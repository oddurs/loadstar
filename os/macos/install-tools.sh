#!/usr/bin/env bash

# macOS Modern Tools Installation
# World-class CLI tools for ultimate productivity

set -euo pipefail

# Colors
readonly GREEN='\033[0;32m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m'

log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }

# Check if command exists
command_exists() { command -v "$1" >/dev/null 2>&1; }

# Install Homebrew if not present
install_homebrew() {
    if ! command_exists brew; then
        log_info "Installing Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        
        # Add to PATH for Apple Silicon Macs
        if [[ $(uname -m) == "arm64" ]]; then
            eval "$(/opt/homebrew/bin/brew shellenv)"
        fi
    else
        log_success "Homebrew already installed"
    fi
}

# Install essential modern CLI tools
install_modern_tools() {
    log_info "Installing modern CLI tools..."
    
    # Core modern replacements
    local tools=(
        # Rust-based modern alternatives
        "fd"                    # find replacement
        "ripgrep"              # grep replacement  
        "eza"                  # ls replacement
        "bat"                  # cat replacement
        "zoxide"               # cd replacement
        "fzf"                  # fuzzy finder
        "starship"             # cross-shell prompt
        "mise"                 # version manager
        "direnv"               # environment per directory
        
        # Enhanced git tools
        "git-delta"            # better git diff
        "gh"                   # GitHub CLI
        "lazygit"              # TUI for git
        "tig"                  # text-mode interface for git
        
        # System monitoring and utilities
        "btop"                 # system monitor
        "duf"                  # disk usage
        "dust"                 # du replacement
        "hyperfine"            # benchmarking
        "procs"                # ps replacement
        "tokei"                # code counter
        
        # Network tools
        "bandwhich"            # network utilization
        "dog"                  # dig replacement
        "httpie"               # HTTP client
        
        # Text processing
        "jq"                   # JSON processor
        "yq"                   # YAML processor
        "sd"                   # sed replacement
        "choose"               # cut replacement
        
        # Development tools
        "tmux"                 # terminal multiplexer
        "neovim"               # modern vim
        "helix"                # modern editor
        "code-minimap"         # code minimap
        
        # Security tools
        "age"                  # encryption
        "sops"                 # secrets management
        "gnupg"                # GPG
        
        # Container tools
        "podman"               # container runtime
        "dive"                 # container layer explorer
        "ctop"                 # container metrics
        
        # Compression
        "zstd"                 # modern compression
        "brotli"               # compression
        
        # Fonts
        "font-jetbrains-mono"
        "font-jetbrains-mono-nerd-font"
        "font-fira-code"
        "font-fira-code-nerd-font"
    )
    
    # Install tools in parallel for speed
    for tool in "${tools[@]}"; do
        if [[ "$tool" == font-* ]]; then
            if ! brew list --cask "$tool" &>/dev/null; then
                brew install --cask "$tool" &
            fi
        else
            if ! brew list "$tool" &>/dev/null; then
                brew install "$tool" &
            fi
        fi
        
        # Limit parallel jobs
        (($(jobs -r | wc -l) >= 5)) && wait
    done
    wait # Wait for all background jobs
    
    log_success "Modern CLI tools installed"
}

# Install development environments
install_dev_environments() {
    log_info "Installing development environments..."
    
    local dev_tools=(
        # Language runtimes (managed by mise)
        # Will be configured in mise config
        
        # IDEs and editors
        "visual-studio-code"
        "iterm2"
        "wezterm"
        "alacritty"
        
        # Development utilities
        "docker"
        "orbstack"              # Docker alternative
        "bruno"                 # API client
        "tableplus"             # Database client
        
        # Productivity
        "raycast"               # Spotlight replacement
        "rectangle"             # Window management
        "hiddenbar"             # Menu bar management
        
        # Optional AI tools
        "cursor"                # AI-powered IDE
    )
    
    for tool in "${dev_tools[@]}"; do
        if ! brew list --cask "$tool" &>/dev/null; then
            brew install --cask "$tool" &
        fi
        (($(jobs -r | wc -l) >= 3)) && wait
    done
    wait
    
    log_success "Development environments installed"
}

# Configure mise for version management
setup_mise() {
    if command_exists mise; then
        log_info "Setting up mise version manager..."
        
        # Create mise config directory
        mkdir -p "$HOME/.config/mise"
        
        # Install common language runtimes
        mise install node@lts
        mise install python@latest
        mise install go@latest
        mise install rust@latest
        mise use -g node@lts python@latest go@latest rust@latest
        
        log_success "Mise configured with common runtimes"
    fi
}

# Setup shell completions
setup_completions() {
    log_info "Setting up shell completions..."
    
    # Create completions directory
    mkdir -p "$HOME/.config/zsh/completions"
    
    # Generate completions for tools that support it
    if command_exists gh; then
        gh completion -s zsh > "$HOME/.config/zsh/completions/_gh"
    fi
    
    if command_exists mise; then
        mise completion zsh > "$HOME/.config/zsh/completions/_mise"
    fi
    
    log_success "Shell completions configured"
}

# Main installation
main() {
    echo "🛠️  Installing world-class macOS development tools..."
    
    install_homebrew
    install_modern_tools
    install_dev_environments
    setup_mise
    setup_completions
    
    log_success "🚀 macOS tools installation complete!"
}

main "$@"