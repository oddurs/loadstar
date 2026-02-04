#!/usr/bin/env bash

# World-Class Dotfiles Bootstrap Script
# One-command setup for the ultimate development environment

set -euo pipefail

# Colors for beautiful output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly PURPLE='\033[0;35m'
readonly CYAN='\033[0;36m'
readonly WHITE='\033[1;37m'
readonly NC='\033[0m' # No Color

# Emojis for visual appeal
readonly ROCKET="🚀"
readonly CHECKMARK="✅"
readonly WARNING="⚠️"
readonly ERROR="❌"
readonly INFO="ℹ️"
readonly GEAR="⚙️"
readonly SPARKLES="✨"

# Helper functions
log() { echo -e "${WHITE}${1}${NC}"; }
log_info() { echo -e "${CYAN}${INFO} ${1}${NC}"; }
log_success() { echo -e "${GREEN}${CHECKMARK} ${1}${NC}"; }
log_warning() { echo -e "${YELLOW}${WARNING} ${1}${NC}"; }
log_error() { echo -e "${RED}${ERROR} ${1}${NC}"; }
log_gear() { echo -e "${BLUE}${GEAR} ${1}${NC}"; }

# Detect OS and architecture
detect_platform() {
    local os
    local arch
    
    case "$(uname -s)" in
        Darwin)  os="darwin" ;;
        Linux)   os="linux" ;;
        CYGWIN*|MINGW*|MSYS*) os="windows" ;;
        *) log_error "Unsupported operating system: $(uname -s)" && exit 1 ;;
    esac
    
    case "$(uname -m)" in
        x86_64|amd64) arch="amd64" ;;
        arm64|aarch64) arch="arm64" ;;
        *) log_error "Unsupported architecture: $(uname -m)" && exit 1 ;;
    esac
    
    echo "${os}-${arch}"
}

# Check if command exists
command_exists() { command -v "$1" >/dev/null 2>&1; }

# Download and verify file
download_file() {
    local url="$1"
    local output="$2"
    local tmp_file="${output}.tmp"
    
    if command_exists curl; then
        curl -fsSL "$url" -o "$tmp_file"
    elif command_exists wget; then
        wget -q "$url" -O "$tmp_file"
    else
        log_error "Neither curl nor wget found. Please install one of them."
        exit 1
    fi
    
    mv "$tmp_file" "$output"
}

# Install Chezmoi
install_chezmoi() {
    if command_exists chezmoi; then
        log_success "Chezmoi already installed"
        return
    fi
    
    log_gear "Installing Chezmoi..."
    
    # Create bin directory if it doesn't exist
    mkdir -p "$HOME/.local/bin"
    
    local platform
    platform=$(detect_platform)
    local url="https://github.com/twpayne/chezmoi/releases/latest/download/chezmoi_${platform}"
    
    download_file "$url" "$HOME/.local/bin/chezmoi"
    chmod +x "$HOME/.local/bin/chezmoi"
    
    # Add to PATH if not already there
    if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
        export PATH="$HOME/.local/bin:$PATH"
    fi
    
    log_success "Chezmoi installed successfully"
}

# Install modern CLI tools
install_modern_tools() {
    log_gear "Installing modern CLI tools..."
    
    local tools_script=""
    case "$(uname -s)" in
        Darwin)
            tools_script="$HOME/.local/share/chezmoi/os/macos/install-tools.sh"
            ;;
        Linux)
            tools_script="$HOME/.local/share/chezmoi/os/linux/install-tools.sh"
            ;;
        *)
            log_warning "OS-specific tool installation not available for this platform"
            return
            ;;
    esac
    
    if [ -f "$tools_script" ]; then
        bash "$tools_script"
    fi
}

# Setup SSH key if needed
setup_ssh() {
    if [ ! -f "$HOME/.ssh/id_ed25519" ] && [ ! -f "$HOME/.ssh/id_rsa" ]; then
        log_gear "Setting up SSH key..."
        
        read -p "Enter your email for SSH key: " email
        ssh-keygen -t ed25519 -C "$email" -f "$HOME/.ssh/id_ed25519" -N ""
        
        log_info "SSH key generated. Add this public key to your GitHub/GitLab:"
        cat "$HOME/.ssh/id_ed25519.pub"
        
        read -p "Press Enter once you've added the key to your git provider..."
    fi
}

# Main installation function
main() {
    log "${ROCKET} Welcome to World-Class Dotfiles Setup!"
    log "This script will set up the ultimate development environment."
    log ""
    
    # Check for required commands
    if ! command_exists git; then
        log_error "Git is required but not installed. Please install Git first."
        exit 1
    fi
    
    # Install Chezmoi
    install_chezmoi
    
    # Prompt for dotfiles repository
    local repo_url
    if [ -z "${DOTFILES_REPO:-}" ]; then
        read -p "Enter your dotfiles repository URL (or press Enter for local setup): " repo_url
    else
        repo_url="$DOTFILES_REPO"
    fi
    
    if [ -n "$repo_url" ]; then
        # Setup SSH if using SSH URL
        if [[ "$repo_url" == git@* ]] || [[ "$repo_url" == ssh://* ]]; then
            setup_ssh
        fi
        
        log_gear "Initializing Chezmoi with repository: $repo_url"
        chezmoi init --apply "$repo_url"
    else
        log_gear "Initializing Chezmoi locally..."
        chezmoi init
        
        # Copy existing dotfiles structure to chezmoi
        if [ -d "$PWD/common" ]; then
            cp -r "$PWD"/* "$HOME/.local/share/chezmoi/"
            log_success "Copied existing dotfiles to Chezmoi"
        fi
    fi
    
    # Install modern tools
    install_modern_tools
    
    # Apply dotfiles
    log_gear "Applying dotfiles configuration..."
    chezmoi apply -v
    
    # Setup shell
    local shell_path
    if command_exists zsh; then
        shell_path="$(which zsh)"
    elif command_exists bash; then
        shell_path="$(which bash)"
    else
        log_warning "No suitable shell found"
        shell_path=""
    fi
    
    if [ -n "$shell_path" ] && [ "$SHELL" != "$shell_path" ]; then
        log_gear "Changing default shell to: $shell_path"
        chsh -s "$shell_path" || log_warning "Could not change default shell"
    fi
    
    # Final steps
    log ""
    log_success "${SPARKLES} World-Class Dotfiles setup complete!"
    log ""
    log_info "Next steps:"
    log "  1. Restart your terminal or run: exec \$SHELL"
    log "  2. Run 'dev' to start your development environment"
    log "  3. Use 'chezmoi edit' to customize your configuration"
    log "  4. Use 'chezmoi add' to manage new dotfiles"
    log ""
    log_info "Useful commands:"
    log "  chezmoi status       - Show status of managed files"
    log "  chezmoi diff         - Show differences"
    log "  chezmoi apply        - Apply changes"
    log "  chezmoi update       - Pull and apply latest changes"
    log "  health-check         - Validate your environment"
    log ""
    log "${ROCKET} Happy coding!"
}

# Run main function
main "$@"