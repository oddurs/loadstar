#!/usr/bin/env bash

# Encryption Setup for Dotfiles
# Secure management of sensitive configuration data

set -euo pipefail

# Colors
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly RED='\033[0;31m'
readonly CYAN='\033[0;36m'
readonly NC='\033[0m'

log_info() { echo -e "${CYAN}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }

# Check if command exists
command_exists() { command -v "$1" >/dev/null 2>&1; }

# Generate age key pair
generate_age_key() {
    local key_dir="$HOME/.config/chezmoi"
    local private_key="$key_dir/key.txt"
    local public_key="$key_dir/key.pub"
    
    mkdir -p "$key_dir"
    
    if [[ -f "$private_key" ]]; then
        log_warning "Age private key already exists at $private_key"
        return 0
    fi
    
    log_info "Generating age key pair..."
    
    if ! command_exists age-keygen; then
        log_error "age-keygen not found. Please install age first."
        return 1
    fi
    
    # Generate private key
    age-keygen -o "$private_key"
    
    # Extract public key
    age-keygen -y "$private_key" > "$public_key"
    
    # Secure permissions
    chmod 600 "$private_key"
    chmod 644 "$public_key"
    
    local public_key_value
    public_key_value=$(cat "$public_key")
    
    log_success "Age key pair generated:"
    log_info "Private key: $private_key"
    log_info "Public key: $public_key_value"
    
    echo "$public_key_value"
}

# Setup chezmoi encryption
setup_chezmoi_encryption() {
    local public_key="$1"
    local chezmoi_config="$HOME/.config/chezmoi/chezmoi.toml"
    
    log_info "Configuring chezmoi encryption..."
    
    # Backup existing config
    if [[ -f "$chezmoi_config" ]]; then
        cp "$chezmoi_config" "$chezmoi_config.backup"
    fi
    
    # Add encryption configuration
    cat >> "$chezmoi_config" << EOF

[encryption]
command = "age"
args = ["-r", "$public_key"]

[textconv]
[textconv.age]
command = "age"
args = ["-d", "-i", "$HOME/.config/chezmoi/key.txt"]
EOF
    
    log_success "Chezmoi encryption configured"
}

# Create example encrypted files
create_examples() {
    local chezmoi_source
    chezmoi_source="$(chezmoi source-path 2>/dev/null || echo "$HOME/.local/share/chezmoi")"
    
    if [[ ! -d "$chezmoi_source" ]]; then
        log_warning "Chezmoi source directory not found. Skipping examples."
        return 0
    fi
    
    log_info "Creating encrypted file examples..."
    
    # Create encrypted environment file
    cat > "$chezmoi_source/private_dot_env.age" << 'EOF'
# Encrypted environment variables
# Edit with: chezmoi edit ~/.env
# View with: chezmoi cat ~/.env

# Example API keys (replace with real values)
API_KEY=your-secret-api-key-here
DATABASE_URL=postgresql://user:password@localhost/db
JWT_SECRET=your-jwt-secret-here

# Cloud provider credentials
AWS_ACCESS_KEY_ID=your-aws-access-key
AWS_SECRET_ACCESS_KEY=your-aws-secret-key

# Personal tokens
GITHUB_TOKEN=ghp_your-github-token-here
OPENAI_API_KEY=sk-your-openai-key-here
EOF

    # Encrypt the file
    age -r "$(cat "$HOME/.config/chezmoi/key.pub")" \
        -o "$chezmoi_source/private_dot_env.age" \
        < "$chezmoi_source/private_dot_env.age"
    
    # Create SSH config template
    mkdir -p "$chezmoi_source/private_dot_ssh"
    cat > "$chezmoi_source/private_dot_ssh/config.age" << 'EOF'
# Encrypted SSH configuration
# Edit with: chezmoi edit ~/.ssh/config

Host github.com
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_ed25519
    AddKeysToAgent yes
    UseKeychain yes

Host work-server
    HostName work-server.company.com
    User your-username
    IdentityFile ~/.ssh/work_key
    Port 22

Host personal-vps
    HostName your-vps.example.com
    User root
    IdentityFile ~/.ssh/personal_key
    Port 2222
EOF

    # Encrypt SSH config
    age -r "$(cat "$HOME/.config/chezmoi/key.pub")" \
        -o "$chezmoi_source/private_dot_ssh/config.age" \
        < "$chezmoi_source/private_dot_ssh/config.age"
    
    log_success "Example encrypted files created"
}

# Setup GPG signing
setup_gpg_signing() {
    if ! command_exists gpg; then
        log_warning "GPG not installed. Skipping GPG setup."
        return 0
    fi
    
    log_info "Checking GPG configuration..."
    
    # Check if GPG key exists
    if gpg --list-secret-keys | grep -q "sec"; then
        log_success "GPG signing key already configured"
    else
        log_warning "No GPG signing key found. Consider generating one with:"
        log_info "  gpg --full-generate-key"
        log_info "  Then configure git signing with:"
        log_info "  git config --global user.signingkey YOUR_KEY_ID"
        log_info "  git config --global commit.gpgsign true"
    fi
}

# Verify encryption setup
verify_setup() {
    log_info "Verifying encryption setup..."
    
    local key_file="$HOME/.config/chezmoi/key.txt"
    local pub_file="$HOME/.config/chezmoi/key.pub"
    
    # Check key files
    if [[ -f "$key_file" && -f "$pub_file" ]]; then
        log_success "Age key pair exists"
    else
        log_error "Age key pair missing"
        return 1
    fi
    
    # Test encryption/decryption
    local test_data="test encryption data"
    local encrypted
    local decrypted
    
    if encrypted=$(echo "$test_data" | age -r "$(cat "$pub_file")" 2>/dev/null) && \
       decrypted=$(echo "$encrypted" | age -d -i "$key_file" 2>/dev/null) && \
       [[ "$decrypted" == "$test_data" ]]; then
        log_success "Encryption/decryption test passed"
    else
        log_error "Encryption/decryption test failed"
        return 1
    fi
    
    # Check chezmoi integration
    if chezmoi --help | grep -q "encryption"; then
        log_success "Chezmoi encryption support available"
    else
        log_warning "Chezmoi may not support encryption"
    fi
}

# Main function
main() {
    echo "🔐 Setting up dotfiles encryption..."
    echo
    
    # Install age if not present
    if ! command_exists age && ! command_exists age-keygen; then
        log_error "Age encryption tool not found. Please install it first:"
        case "$(uname -s)" in
            Darwin)
                echo "  brew install age"
                ;;
            Linux)
                echo "  # On Ubuntu/Debian:"
                echo "  sudo apt install age"
                echo "  # On Fedora:"
                echo "  sudo dnf install age"
                echo "  # On Arch:"
                echo "  sudo pacman -S age"
                ;;
        esac
        exit 1
    fi
    
    # Generate age key pair
    local public_key
    public_key=$(generate_age_key)
    
    # Setup chezmoi encryption
    setup_chezmoi_encryption "$public_key"
    
    # Create example encrypted files
    create_examples
    
    # Setup GPG signing
    setup_gpg_signing
    
    # Verify setup
    verify_setup
    
    echo
    log_success "🎉 Encryption setup complete!"
    echo
    log_info "Next steps:"
    log_info "  1. Backup your private key securely: $HOME/.config/chezmoi/key.txt"
    log_info "  2. Edit encrypted files with: chezmoi edit ~/.env"
    log_info "  3. Add sensitive files with: chezmoi add --encrypt ~/.ssh/config"
    log_info "  4. Apply changes with: chezmoi apply"
    echo
    log_warning "Keep your age private key safe! Store it in a password manager."
}

main "$@"