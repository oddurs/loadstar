#!/usr/bin/env bash

# Dotfiles Health Check System
# Validates and reports on development environment status

set -euo pipefail

# Colors for output
readonly GREEN='\033[0;32m'
readonly RED='\033[0;31m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly CYAN='\033[0;36m'
readonly WHITE='\033[1;37m'
readonly NC='\033[0m'

# Emojis for visual appeal
readonly CHECKMARK="✅"
readonly CROSS="❌"
readonly WARNING="⚠️"
readonly INFO="ℹ️"
readonly ROCKET="🚀"
readonly GEAR="⚙️"

# Counters
passed=0
failed=0
warnings=0

# Helper functions
log_header() { echo -e "\n${WHITE}=== $1 ===${NC}"; }
log_success() { echo -e "${GREEN}${CHECKMARK} $1${NC}"; ((passed++)); }
log_error() { echo -e "${RED}${CROSS} $1${NC}"; ((failed++)); }
log_warning() { echo -e "${YELLOW}${WARNING} $1${NC}"; ((warnings++)); }
log_info() { echo -e "${CYAN}${INFO} $1${NC}"; }

# Check if command exists
command_exists() { command -v "$1" >/dev/null 2>&1; }

# Check file exists and is readable
file_check() {
    if [[ -f "$1" && -r "$1" ]]; then
        log_success "$2"
    else
        log_error "$2 (missing: $1)"
    fi
}

# Check directory exists and is readable
dir_check() {
    if [[ -d "$1" && -r "$1" ]]; then
        log_success "$2"
    else
        log_error "$2 (missing: $1)"
    fi
}

# Check tool version
version_check() {
    local tool="$1"
    local version_cmd="$2"
    local name="${3:-$tool}"
    
    if command_exists "$tool"; then
        local version
        version=$(eval "$version_cmd" 2>/dev/null | head -1 || echo "unknown")
        log_success "$name: $version"
    else
        log_error "$name not installed"
    fi
}

# Check Core Tools
check_core_tools() {
    log_header "Core Development Tools"
    
    version_check "git" "git --version" "Git"
    version_check "zsh" "zsh --version" "Zsh"
    version_check "tmux" "tmux -V" "Tmux"
    version_check "nvim" "nvim --version | head -1" "Neovim"
    
    # Check default shell
    if [[ "$SHELL" == *"zsh"* ]]; then
        log_success "Default shell: zsh"
    else
        log_warning "Default shell is not zsh: $SHELL"
    fi
}

# Check Modern CLI Tools
check_modern_tools() {
    log_header "Modern CLI Tools"
    
    version_check "fd" "fd --version" "fd (find replacement)"
    version_check "rg" "rg --version | head -1" "ripgrep"
    version_check "eza" "eza --version" "eza (ls replacement)"
    version_check "bat" "bat --version" "bat (cat replacement)"
    version_check "zoxide" "zoxide --version" "zoxide (cd replacement)"
    version_check "fzf" "fzf --version" "fzf (fuzzy finder)"
    version_check "starship" "starship --version" "Starship prompt"
    version_check "delta" "delta --version" "Delta (git diff)"
    version_check "gh" "gh --version | head -1" "GitHub CLI"
    version_check "mise" "mise --version" "Mise (version manager)"
    version_check "direnv" "direnv --version" "Direnv"
}

# Check Package Managers
check_package_managers() {
    log_header "Package Managers"
    
    case "$(uname -s)" in
        Darwin)
            version_check "brew" "brew --version | head -1" "Homebrew"
            ;;
        Linux)
            if command_exists apt; then
                version_check "apt" "apt --version | head -1" "APT"
            elif command_exists dnf; then
                version_check "dnf" "dnf --version" "DNF"
            elif command_exists pacman; then
                version_check "pacman" "pacman --version | head -1" "Pacman"
            fi
            ;;
    esac
    
    # Language package managers
    version_check "npm" "npm --version" "npm"
    version_check "pip" "pip --version" "pip"
    version_check "cargo" "cargo --version" "Cargo"
}

# Check Configuration Files
check_config_files() {
    log_header "Configuration Files"
    
    file_check "$HOME/.zshrc" "Zsh configuration"
    file_check "$HOME/.tmux.conf" "Tmux configuration"
    file_check "$HOME/.gitconfig" "Git configuration"
    file_check "$HOME/.config/starship.toml" "Starship configuration"
    file_check "$HOME/.config/nvim/init.lua" "Neovim configuration"
    
    # Chezmoi specific
    if command_exists chezmoi; then
        local chezmoi_dir
        chezmoi_dir="$(chezmoi source-path 2>/dev/null || echo "")"
        if [[ -n "$chezmoi_dir" && -d "$chezmoi_dir" ]]; then
            log_success "Chezmoi source directory: $chezmoi_dir"
        else
            log_warning "Chezmoi not initialized"
        fi
    fi
}

# Check Environment Variables
check_environment() {
    log_header "Environment Variables"
    
    # Essential environment variables
    [[ -n "${EDITOR:-}" ]] && log_success "EDITOR: $EDITOR" || log_warning "EDITOR not set"
    [[ -n "${SHELL:-}" ]] && log_success "SHELL: $SHELL" || log_error "SHELL not set"
    [[ -n "${HOME:-}" ]] && log_success "HOME: $HOME" || log_error "HOME not set"
    [[ -n "${PATH:-}" ]] && log_success "PATH configured (${#PATH} chars)" || log_error "PATH not set"
    
    # XDG Base Directory Specification
    [[ -n "${XDG_CONFIG_HOME:-}" ]] && log_success "XDG_CONFIG_HOME: $XDG_CONFIG_HOME" || log_info "XDG_CONFIG_HOME using default"
    [[ -n "${XDG_DATA_HOME:-}" ]] && log_success "XDG_DATA_HOME: $XDG_DATA_HOME" || log_info "XDG_DATA_HOME using default"
    [[ -n "${XDG_CACHE_HOME:-}" ]] && log_success "XDG_CACHE_HOME: $XDG_CACHE_HOME" || log_info "XDG_CACHE_HOME using default"
}

# Check Development Runtimes
check_dev_runtimes() {
    log_header "Development Runtimes"
    
    if command_exists mise; then
        log_info "Active mise tools:"
        mise current 2>/dev/null | while IFS= read -r line; do
            [[ -n "$line" ]] && log_success "  $line"
        done || log_warning "No mise tools active"
    fi
    
    # Check individual runtimes
    version_check "node" "node --version" "Node.js"
    version_check "python" "python --version" "Python"
    version_check "python3" "python3 --version" "Python 3"
    version_check "go" "go version" "Go"
    version_check "rustc" "rustc --version" "Rust"
    version_check "java" "java -version 2>&1 | head -1" "Java"
}

# Check SSH and GPG
check_security() {
    log_header "Security Configuration"
    
    # SSH keys
    if [[ -f "$HOME/.ssh/id_ed25519" ]] || [[ -f "$HOME/.ssh/id_rsa" ]]; then
        log_success "SSH keys configured"
        
        # Check SSH agent
        if ssh-add -l &>/dev/null; then
            local key_count
            key_count=$(ssh-add -l | wc -l)
            log_success "SSH agent running with $key_count key(s)"
        else
            log_warning "SSH agent not running or no keys loaded"
        fi
    else
        log_warning "No SSH keys found"
    fi
    
    # GPG
    if command_exists gpg; then
        local gpg_keys
        gpg_keys=$(gpg --list-secret-keys 2>/dev/null | grep -c "^sec" || echo "0")
        if [[ "$gpg_keys" -gt 0 ]]; then
            log_success "GPG configured with $gpg_keys key(s)"
        else
            log_warning "No GPG keys found"
        fi
    fi
}

# Check Performance
check_performance() {
    log_header "Performance Metrics"
    
    # Shell startup time
    if command_exists zsh; then
        log_info "Measuring shell startup time..."
        local startup_time
        startup_time=$(time zsh -lic exit 2>&1 | grep real | awk '{print $2}' || echo "unknown")
        if [[ "$startup_time" != "unknown" ]]; then
            log_success "Zsh startup time: $startup_time"
        else
            log_warning "Could not measure shell startup time"
        fi
    fi
    
    # Tmux plugin status
    if [[ -d "$HOME/.tmux/plugins/tpm" ]]; then
        log_success "Tmux Plugin Manager installed"
    else
        log_warning "Tmux Plugin Manager not installed"
    fi
}

# Check Integration Health
check_integrations() {
    log_header "Tool Integration Health"
    
    # Git delta integration
    if command_exists git && command_exists delta; then
        if git config --get core.pager | grep -q delta; then
            log_success "Git-Delta integration configured"
        else
            log_warning "Git not configured to use Delta"
        fi
    fi
    
    # Starship integration
    if command_exists starship && command_exists zsh; then
        if grep -q "starship init" "$HOME/.zshrc" 2>/dev/null; then
            log_success "Starship-Zsh integration configured"
        else
            log_warning "Starship not integrated with Zsh"
        fi
    fi
    
    # FZF integration
    if command_exists fzf; then
        if [[ -n "${FZF_DEFAULT_COMMAND:-}" ]]; then
            log_success "FZF default command configured"
        else
            log_info "FZF using default settings"
        fi
    fi
}

# Main health check function
main() {
    echo -e "${ROCKET} ${WHITE}Dotfiles Health Check${NC}\n"
    
    check_core_tools
    check_modern_tools
    check_package_managers
    check_config_files
    check_environment
    check_dev_runtimes
    check_security
    check_performance
    check_integrations
    
    # Summary
    local total=$((passed + failed + warnings))
    echo -e "\n${WHITE}=== Health Check Summary ===${NC}"
    echo -e "${GREEN}Passed: $passed${NC}"
    echo -e "${YELLOW}Warnings: $warnings${NC}"
    echo -e "${RED}Failed: $failed${NC}"
    echo -e "Total checks: $total"
    
    if [[ $failed -eq 0 ]]; then
        echo -e "\n${GREEN}${CHECKMARK} Your development environment is healthy!${NC}"
        exit 0
    else
        echo -e "\n${RED}${CROSS} Some issues need attention.${NC}"
        exit 1
    fi
}

# Run health check
main "$@"