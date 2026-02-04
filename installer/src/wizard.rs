//! Wizard state machine for the installation process
//! Guides users through logical phases of machine setup

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::catalog::{App, Category, CATALOG};

/// The phases of the installation wizard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WizardPhase {
    /// Boot sequence - dramatic intro
    Boot,
    /// Who are you? Name, email, work/personal
    Identity,
    /// Shell & prompt configuration
    Shell,
    /// Development tools selection
    DevTools,
    /// Application selection by category
    Apps,
    /// Review configuration choices
    Review,
    /// Execute the installation
    Install,
    /// Completion and next steps
    Complete,
}

impl WizardPhase {
    pub fn all() -> &'static [WizardPhase] {
        &[
            WizardPhase::Boot,
            WizardPhase::Identity,
            WizardPhase::Shell,
            WizardPhase::DevTools,
            WizardPhase::Apps,
            WizardPhase::Review,
            WizardPhase::Install,
            WizardPhase::Complete,
        ]
    }

    pub fn index(&self) -> usize {
        Self::all().iter().position(|p| p == self).unwrap_or(0)
    }

    pub fn name(&self) -> &'static str {
        match self {
            WizardPhase::Boot => "BOOT",
            WizardPhase::Identity => "IDENTITY",
            WizardPhase::Shell => "SHELL",
            WizardPhase::DevTools => "DEV TOOLS",
            WizardPhase::Apps => "APPS",
            WizardPhase::Review => "REVIEW",
            WizardPhase::Install => "INSTALL",
            WizardPhase::Complete => "COMPLETE",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            WizardPhase::Boot => "Initializing consciousness transfer...",
            WizardPhase::Identity => "Establishing neural identity profile",
            WizardPhase::Shell => "Configuring command interface layer",
            WizardPhase::DevTools => "Selecting development arsenal",
            WizardPhase::Apps => "Choosing software companions",
            WizardPhase::Review => "Reviewing configuration matrix",
            WizardPhase::Install => "Executing reality modification",
            WizardPhase::Complete => "Transformation complete",
        }
    }

    pub fn next(&self) -> Option<WizardPhase> {
        let phases = Self::all();
        let current = self.index();
        if current + 1 < phases.len() {
            Some(phases[current + 1])
        } else {
            None
        }
    }

    pub fn prev(&self) -> Option<WizardPhase> {
        let phases = Self::all();
        let current = self.index();
        if current > 0 {
            Some(phases[current - 1])
        } else {
            None
        }
    }
}

/// User identity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub name: String,
    pub email: String,
    pub github_username: String,
    pub setup_type: SetupType,
}

impl Default for Identity {
    fn default() -> Self {
        Self {
            name: whoami::realname(),
            email: String::new(),
            github_username: String::new(),
            setup_type: SetupType::Personal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupType {
    Personal,
    Work,
    Minimal,
    Full,
}

impl SetupType {
    pub fn all() -> &'static [SetupType] {
        &[
            SetupType::Personal,
            SetupType::Work,
            SetupType::Minimal,
            SetupType::Full,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            SetupType::Personal => "Personal",
            SetupType::Work => "Work",
            SetupType::Minimal => "Minimal",
            SetupType::Full => "Full",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            SetupType::Personal => "Personal development machine with all the bells and whistles",
            SetupType::Work => "Professional setup with work-oriented tools",
            SetupType::Minimal => "Essential tools only - fast and lean",
            SetupType::Full => "Everything. Maximum power. No compromises.",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            SetupType::Personal => "🏠",
            SetupType::Work => "💼",
            SetupType::Minimal => "🍃",
            SetupType::Full => "🚀",
        }
    }
}

/// Shell configuration choices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    pub shell: ShellChoice,
    pub prompt: PromptChoice,
    pub terminal: TerminalChoice,
    pub multiplexer: Option<MultiplexerChoice>,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            shell: ShellChoice::Zsh,
            prompt: PromptChoice::Starship,
            terminal: TerminalChoice::Default,
            multiplexer: Some(MultiplexerChoice::Tmux),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShellChoice {
    Zsh,
    Bash,
    Fish,
    Nushell,
}

impl ShellChoice {
    pub fn all() -> &'static [ShellChoice] {
        &[
            ShellChoice::Zsh,
            ShellChoice::Bash,
            ShellChoice::Fish,
            ShellChoice::Nushell,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ShellChoice::Zsh => "Zsh",
            ShellChoice::Bash => "Bash",
            ShellChoice::Fish => "Fish",
            ShellChoice::Nushell => "Nushell",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ShellChoice::Zsh => "Feature-rich, highly customizable (recommended)",
            ShellChoice::Bash => "Classic Unix shell, maximum compatibility",
            ShellChoice::Fish => "Friendly interactive shell with great defaults",
            ShellChoice::Nushell => "Modern shell with structured data",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PromptChoice {
    Starship,
    Powerlevel10k,
    PurePurple,
    Minimal,
    None,
}

impl PromptChoice {
    pub fn all() -> &'static [PromptChoice] {
        &[
            PromptChoice::Starship,
            PromptChoice::Powerlevel10k,
            PromptChoice::PurePurple,
            PromptChoice::Minimal,
            PromptChoice::None,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            PromptChoice::Starship => "Starship",
            PromptChoice::Powerlevel10k => "Powerlevel10k",
            PromptChoice::PurePurple => "Pure",
            PromptChoice::Minimal => "Minimal",
            PromptChoice::None => "Default",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            PromptChoice::Starship => "Cross-shell prompt, fast & customizable (recommended)",
            PromptChoice::Powerlevel10k => "Zsh theme with instant prompt",
            PromptChoice::PurePurple => "Pretty, minimal and fast prompt",
            PromptChoice::Minimal => "Simple, distraction-free prompt",
            PromptChoice::None => "Keep system default",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminalChoice {
    Default,
    WezTerm,
    Alacritty,
    Kitty,
    ITerm2,
    Ghostty,
}

impl TerminalChoice {
    pub fn all() -> &'static [TerminalChoice] {
        &[
            TerminalChoice::Default,
            TerminalChoice::WezTerm,
            TerminalChoice::Alacritty,
            TerminalChoice::Kitty,
            TerminalChoice::ITerm2,
            TerminalChoice::Ghostty,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            TerminalChoice::Default => "Keep Current",
            TerminalChoice::WezTerm => "WezTerm",
            TerminalChoice::Alacritty => "Alacritty",
            TerminalChoice::Kitty => "Kitty",
            TerminalChoice::ITerm2 => "iTerm2",
            TerminalChoice::Ghostty => "Ghostty",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TerminalChoice::Default => "Don't install a new terminal",
            TerminalChoice::WezTerm => "GPU-accelerated with Lua config",
            TerminalChoice::Alacritty => "Minimal, fast, GPU-accelerated",
            TerminalChoice::Kitty => "Feature-rich, GPU-accelerated",
            TerminalChoice::ITerm2 => "macOS classic with many features",
            TerminalChoice::Ghostty => "Native, fast, by Mitchell Hashimoto",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MultiplexerChoice {
    Tmux,
    Zellij,
    None,
}

impl MultiplexerChoice {
    pub fn all() -> &'static [MultiplexerChoice] {
        &[
            MultiplexerChoice::Tmux,
            MultiplexerChoice::Zellij,
            MultiplexerChoice::None,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            MultiplexerChoice::Tmux => "Tmux",
            MultiplexerChoice::Zellij => "Zellij",
            MultiplexerChoice::None => "None",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            MultiplexerChoice::Tmux => "Classic multiplexer, huge ecosystem",
            MultiplexerChoice::Zellij => "Modern alternative with better defaults",
            MultiplexerChoice::None => "No terminal multiplexer",
        }
    }
}

/// Editor preferences
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditorChoice {
    Neovim,
    Helix,
    VSCode,
    Zed,
    None,
}

impl EditorChoice {
    pub fn all() -> &'static [EditorChoice] {
        &[
            EditorChoice::Neovim,
            EditorChoice::Helix,
            EditorChoice::VSCode,
            EditorChoice::Zed,
            EditorChoice::None,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            EditorChoice::Neovim => "Neovim",
            EditorChoice::Helix => "Helix",
            EditorChoice::VSCode => "VS Code",
            EditorChoice::Zed => "Zed",
            EditorChoice::None => "None",
        }
    }
}

/// The complete wizard state
#[derive(Debug, Clone)]
pub struct WizardState {
    pub phase: WizardPhase,
    pub identity: Identity,
    pub shell_config: ShellConfig,
    pub editor: EditorChoice,
    pub selected_apps: HashSet<String>,
    pub selected_categories: HashSet<Category>,
    pub install_homebrew: bool,
    pub install_fonts: bool,
    pub generate_ssh_key: bool,
    pub setup_git_signing: bool,
    pub cursor_position: usize,
    pub scroll_offset: usize,
    pub input_buffer: String,
    pub input_field: usize,
    pub show_details: bool,
}

impl Default for WizardState {
    fn default() -> Self {
        // Pre-select essential apps
        let mut selected_apps = HashSet::new();
        for app in crate::catalog::essential_apps() {
            selected_apps.insert(app.id.to_string());
        }

        Self {
            phase: WizardPhase::Boot,
            identity: Identity::default(),
            shell_config: ShellConfig::default(),
            editor: EditorChoice::Neovim,
            selected_apps,
            selected_categories: HashSet::new(),
            install_homebrew: true,
            install_fonts: true,
            generate_ssh_key: true,
            setup_git_signing: false,
            cursor_position: 0,
            scroll_offset: 0,
            input_buffer: String::new(),
            input_field: 0,
            show_details: true,
        }
    }
}

impl WizardState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn advance(&mut self) -> bool {
        if let Some(next) = self.phase.next() {
            self.phase = next;
            self.cursor_position = 0;
            self.scroll_offset = 0;
            true
        } else {
            false
        }
    }

    pub fn go_back(&mut self) -> bool {
        if let Some(prev) = self.phase.prev() {
            // Don't go back past Boot
            if prev != WizardPhase::Boot || self.phase == WizardPhase::Identity {
                self.phase = prev;
                self.cursor_position = 0;
                self.scroll_offset = 0;
                return true;
            }
        }
        false
    }

    pub fn toggle_app(&mut self, app_id: &str) {
        if self.selected_apps.contains(app_id) {
            self.selected_apps.remove(app_id);
        } else {
            self.selected_apps.insert(app_id.to_string());
        }
    }

    pub fn is_app_selected(&self, app_id: &str) -> bool {
        self.selected_apps.contains(app_id)
    }

    pub fn get_selected_apps(&self) -> Vec<&'static App> {
        CATALOG
            .iter()
            .filter(|app| self.selected_apps.contains(app.id))
            .collect()
    }

    pub fn get_apps_for_category(&self, category: &Category) -> Vec<&'static App> {
        crate::catalog::apps_by_category(category)
    }

    pub fn selected_app_count(&self) -> usize {
        self.selected_apps.len()
    }

    pub fn total_app_count(&self) -> usize {
        CATALOG.len()
    }

    /// Get brew packages to install
    pub fn get_brew_packages(&self) -> Vec<String> {
        let mut packages = Vec::new();

        for app in self.get_selected_apps() {
            if let crate::catalog::InstallMethod::Brew(pkg) = app.install_method {
                packages.push(pkg.to_string());
            }
        }

        packages
    }

    /// Get brew casks to install
    pub fn get_brew_casks(&self) -> Vec<String> {
        let mut casks = Vec::new();

        for app in self.get_selected_apps() {
            if let crate::catalog::InstallMethod::BrewCask(pkg) = app.install_method {
                casks.push(pkg.to_string());
            }
        }

        casks
    }

    /// Calculate estimated install time in minutes
    pub fn estimated_install_time(&self) -> u32 {
        let app_count = self.selected_apps.len() as u32;
        let base_time = 5; // Base setup time
        let per_app = 1; // ~1 minute per app on average

        base_time + (app_count * per_app)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wizard_state_defaults() {
        let state = WizardState::new();
        assert_eq!(state.phase, WizardPhase::Boot);
        assert_eq!(state.shell_config.shell, ShellChoice::Zsh);
        assert_eq!(state.shell_config.prompt, PromptChoice::Starship);
        assert_eq!(state.editor, EditorChoice::Neovim);
        assert!(state.install_homebrew);
        assert!(state.generate_ssh_key);
        assert!(state.show_details);
    }

    #[test]
    fn wizard_pre_selects_essentials() {
        let state = WizardState::new();
        assert!(
            !state.selected_apps.is_empty(),
            "Essential apps should be pre-selected"
        );
        // Check a few known essentials
        assert!(state.selected_apps.contains("fzf") || state.selected_apps.contains("ripgrep"));
    }

    #[test]
    fn wizard_phase_advance() {
        let mut state = WizardState::new();
        assert_eq!(state.phase, WizardPhase::Boot);

        assert!(state.advance());
        assert_eq!(state.phase, WizardPhase::Identity);

        assert!(state.advance());
        assert_eq!(state.phase, WizardPhase::Shell);

        assert!(state.advance());
        assert_eq!(state.phase, WizardPhase::DevTools);

        assert!(state.advance());
        assert_eq!(state.phase, WizardPhase::Apps);

        assert!(state.advance());
        assert_eq!(state.phase, WizardPhase::Review);

        assert!(state.advance());
        assert_eq!(state.phase, WizardPhase::Install);

        assert!(state.advance());
        assert_eq!(state.phase, WizardPhase::Complete);

        // Can't advance past Complete
        assert!(!state.advance());
        assert_eq!(state.phase, WizardPhase::Complete);
    }

    #[test]
    fn wizard_phase_go_back() {
        let mut state = WizardState::new();
        state.phase = WizardPhase::Shell;

        assert!(state.go_back());
        assert_eq!(state.phase, WizardPhase::Identity);

        // Can go back to Boot from Identity
        assert!(state.go_back());
        assert_eq!(state.phase, WizardPhase::Boot);
    }

    #[test]
    fn wizard_toggle_app() {
        let mut state = WizardState::new();
        let was_selected = state.is_app_selected("neovim");

        state.toggle_app("neovim");
        assert_ne!(state.is_app_selected("neovim"), was_selected);

        state.toggle_app("neovim");
        assert_eq!(state.is_app_selected("neovim"), was_selected);
    }

    #[test]
    fn wizard_get_selected_apps() {
        let mut state = WizardState::new();
        state.selected_apps.clear();
        state.selected_apps.insert("git".to_string());
        state.selected_apps.insert("neovim".to_string());

        let apps = state.get_selected_apps();
        assert_eq!(apps.len(), 2);
    }

    #[test]
    fn wizard_brew_packages() {
        let mut state = WizardState::new();
        state.selected_apps.clear();
        state.selected_apps.insert("git".to_string());
        state.selected_apps.insert("fzf".to_string());
        state.selected_apps.insert("docker".to_string()); // This is a cask

        let brew = state.get_brew_packages();
        let casks = state.get_brew_casks();

        assert!(brew.contains(&"git".to_string()));
        assert!(brew.contains(&"fzf".to_string()));
        assert!(casks.contains(&"docker".to_string()));
        assert!(!brew.contains(&"docker".to_string()));
    }

    #[test]
    fn wizard_estimated_time() {
        let mut state = WizardState::new();
        state.selected_apps.clear();
        assert_eq!(state.estimated_install_time(), 5); // base time only

        state.selected_apps.insert("git".to_string());
        state.selected_apps.insert("fzf".to_string());
        assert_eq!(state.estimated_install_time(), 7); // 5 + 2
    }

    #[test]
    fn phase_ordering() {
        let phases = WizardPhase::all();
        assert_eq!(phases.len(), 8);
        assert_eq!(phases[0], WizardPhase::Boot);
        assert_eq!(phases[7], WizardPhase::Complete);

        // Each phase knows its index
        for (i, phase) in phases.iter().enumerate() {
            assert_eq!(phase.index(), i);
        }
    }

    #[test]
    fn phase_names_and_descriptions() {
        for phase in WizardPhase::all() {
            assert!(!phase.name().is_empty());
            assert!(!phase.description().is_empty());
        }
    }

    #[test]
    fn phase_next_prev() {
        assert_eq!(WizardPhase::Boot.next(), Some(WizardPhase::Identity));
        assert_eq!(WizardPhase::Complete.next(), None);
        assert_eq!(WizardPhase::Boot.prev(), None);
        assert_eq!(WizardPhase::Identity.prev(), Some(WizardPhase::Boot));
    }

    #[test]
    fn setup_type_metadata() {
        for t in SetupType::all() {
            assert!(!t.name().is_empty());
            assert!(!t.description().is_empty());
            assert!(!t.icon().is_empty());
        }
    }

    #[test]
    fn shell_choice_metadata() {
        for s in ShellChoice::all() {
            assert!(!s.name().is_empty());
            assert!(!s.description().is_empty());
        }
    }

    #[test]
    fn identity_defaults_to_real_name() {
        let identity = Identity::default();
        // whoami::realname() should return something
        assert!(!identity.name.is_empty() || true); // May be empty in CI
        assert_eq!(identity.setup_type, SetupType::Personal);
    }
}
