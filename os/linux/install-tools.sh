#!/usr/bin/env bash

# Linux Modern Tools Installation
# World-class CLI tools for ultimate productivity

set -euo pipefail

# Colors
readonly GREEN='\033[0;32m'
readonly BLUE='\033[0;34m'
readonly YELLOW='\033[1;33m'
readonly NC='\033[0m'

log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }

# Detect Linux distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        echo "$ID"
    elif command -v lsb_release >/dev/null 2>&1; then
        lsb_release -si | tr '[:upper:]' '[:lower:]'
    else
        echo "unknown"
    fi
}

# Check if command exists
command_exists() { command -v "$1" >/dev/null 2>&1; }

# Install using cargo (Rust package manager)
install_rust_tools() {
    if ! command_exists cargo; then
        log_info "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    log_info "Installing Rust-based tools..."
    
    local rust_tools=(
        "fd-find"              # find replacement
        "ripgrep"              # grep replacement
        "eza"                  # ls replacement  
        "bat"                  # cat replacement
        "zoxide"               # cd replacement
        "starship"             # cross-shell prompt
        "git-delta"            # better git diff
        "dust"                 # du replacement
        "hyperfine"            # benchmarking
        "procs"                # ps replacement
        "tokei"                # code counter
        "bandwhich"            # network utilization
        "sd"                   # sed replacement
        "choose"               # cut replacement
        "bottom"               # system monitor (btop alternative)
        "duf"                  # disk usage
    )
    
    for tool in "${rust_tools[@]}"; do
        if ! command_exists "${tool%%-*}"; then  # Remove suffix for command check
            cargo install "$tool" &
        fi
        (($(jobs -r | wc -l) >= 4)) && wait
    done
    wait
    
    log_success "Rust tools installed"
}

# Install with distribution package manager
install_distro_packages() {
    local distro
    distro=$(detect_distro)
    
    log_info "Installing packages for $distro..."
    
    case "$distro" in
        ubuntu|debian)
            # Update package list
            sudo apt update
            
            # Essential packages
            sudo apt install -y \
                curl wget git unzip \
                build-essential \
                tmux neovim \
                fzf \
                jq yq \
                age gnupg \
                direnv \
                httpie \
                tree htop \
                zsh \
                python3-pip \
                nodejs npm
            ;;
            
        fedora|centos|rhel)
            sudo dnf install -y \
                curl wget git unzip \
                gcc gcc-c++ make \
                tmux neovim \
                fzf \
                jq \
                age gnupg2 \
                direnv \
                httpie \
                tree htop \
                zsh \
                python3-pip \
                nodejs npm
            ;;
            
        arch|manjaro)
            sudo pacman -S --noconfirm \
                curl wget git unzip \
                base-devel \
                tmux neovim \
                fzf \
                jq yq \
                age gnupg \
                direnv \
                httpie \
                tree htop \
                zsh \
                python-pip \
                nodejs npm
            ;;
            
        *)
            log_warning "Unknown distribution: $distro. Installing minimal set..."
            ;;
    esac
    
    log_success "Distribution packages installed"
}

# Install GitHub CLI
install_github_cli() {
    if command_exists gh; then
        log_success "GitHub CLI already installed"
        return
    fi
    
    log_info "Installing GitHub CLI..."
    
    local distro
    distro=$(detect_distro)
    
    case "$distro" in
        ubuntu|debian)
            type -p curl >/dev/null || sudo apt install curl -y
            curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
            sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg
            echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
            sudo apt update
            sudo apt install gh -y
            ;;
            
        fedora|centos|rhel)
            sudo dnf install 'dnf-command(config-manager)' -y
            sudo dnf config-manager --add-repo https://cli.github.com/packages/rpm/gh-cli.repo
            sudo dnf install gh -y
            ;;
            
        arch|manjaro)
            sudo pacman -S github-cli --noconfirm
            ;;
            
        *)
            # Fallback: install from GitHub releases
            local latest_url="https://api.github.com/repos/cli/cli/releases/latest"
            local download_url
            download_url=$(curl -s "$latest_url" | jq -r '.assets[] | select(.name | contains("linux") and contains("amd64") and endswith(".tar.gz")) | .browser_download_url')
            
            if [ -n "$download_url" ]; then
                cd /tmp
                curl -L "$download_url" | tar xz
                sudo cp gh_*/bin/gh /usr/local/bin/
                log_success "GitHub CLI installed from release"
            fi
            ;;
    esac
}

# Install mise (version manager)
install_mise() {
    if command_exists mise; then
        log_success "mise already installed"
        return
    fi
    
    log_info "Installing mise version manager..."
    curl https://mise.run | sh
    
    # Add to PATH
    export PATH="$HOME/.local/bin:$PATH"
    
    log_success "mise installed"
}

# Setup development runtimes
setup_development() {
    log_info "Setting up development runtimes..."
    
    # Install mise if not present
    install_mise
    
    # Add mise to current session
    if [ -f "$HOME/.local/bin/mise" ]; then
        export PATH="$HOME/.local/bin:$PATH"
        
        # Install common runtimes
        mise install node@lts
        mise install python@latest
        mise install go@latest
        mise use -g node@lts python@latest go@latest
        
        log_success "Development runtimes configured"
    fi
}

# Install fonts
install_fonts() {
    log_info "Installing fonts..."
    
    local font_dir="$HOME/.local/share/fonts"
    mkdir -p "$font_dir"
    
    # JetBrains Mono
    if [ ! -f "$font_dir/JetBrainsMono-Regular.ttf" ]; then
        local font_url="https://github.com/JetBrains/JetBrainsMono/releases/download/v2.304/JetBrainsMono-2.304.zip"
        cd /tmp
        curl -L "$font_url" -o JetBrainsMono.zip
        unzip -q JetBrainsMono.zip
        cp fonts/ttf/* "$font_dir/"
        rm -rf JetBrainsMono.zip fonts/
    fi
    
    # JetBrains Mono Nerd Font
    if [ ! -f "$font_dir/JetBrainsMonoNerdFont-Regular.ttf" ]; then
        local nerd_font_url="https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/JetBrainsMono.zip"
        cd /tmp
        curl -L "$nerd_font_url" -o JetBrainsMonoNerd.zip
        unzip -q JetBrainsMonoNerd.zip -d JetBrainsMonoNerd/
        cp JetBrainsMonoNerd/*.ttf "$font_dir/"
        rm -rf JetBrainsMonoNerd.zip JetBrainsMonoNerd/
    fi
    
    # Update font cache
    fc-cache -fv
    
    log_success "Fonts installed"
}

# Setup shell completions
setup_completions() {
    log_info "Setting up shell completions..."
    
    mkdir -p "$HOME/.config/zsh/completions"
    
    # Generate completions
    if command_exists gh; then
        gh completion -s zsh > "$HOME/.config/zsh/completions/_gh" 2>/dev/null || true
    fi
    
    if command_exists mise; then
        mise completion zsh > "$HOME/.config/zsh/completions/_mise" 2>/dev/null || true
    fi
    
    log_success "Shell completions configured"
}

# Main installation
main() {
    echo "🛠️  Installing world-class Linux development tools..."
    
    install_distro_packages
    install_github_cli
    install_rust_tools
    setup_development
    install_fonts
    setup_completions
    
    log_success "🚀 Linux tools installation complete!"
    log_info "Note: Restart your terminal to ensure all tools are in PATH"
}

main "$@"