//! Comprehensive catalog of CLI-configurable apps and tools
//! Organized by category with descriptions and configuration options

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct App {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub install_method: InstallMethod,
    pub config_files: &'static [&'static str],
    pub dependencies: &'static [&'static str],
    pub tags: &'static [&'static str],
    pub url: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Shell,
    Editor,
    Git,
    Terminal,
    FileManager,
    Search,
    System,
    Network,
    Container,
    Language,
    Database,
    Security,
    Productivity,
    Media,
    Cloud,
    AI,
}

impl Category {
    pub fn icon(&self) -> &'static str {
        match self {
            Category::Shell => "🐚",
            Category::Editor => "✏️",
            Category::Git => "⎇",
            Category::Terminal => "⌨",
            Category::FileManager => "📁",
            Category::Search => "🔍",
            Category::System => "⚙️",
            Category::Network => "🌐",
            Category::Container => "📦",
            Category::Language => "⟨⟩",
            Category::Database => "⛁",
            Category::Security => "🔒",
            Category::Productivity => "⚡",
            Category::Media => "🎬",
            Category::Cloud => "☁️",
            Category::AI => "🤖",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Category::Shell => "Shell & Prompt",
            Category::Editor => "Editors",
            Category::Git => "Git & Version Control",
            Category::Terminal => "Terminal Tools",
            Category::FileManager => "File Management",
            Category::Search => "Search & Navigation",
            Category::System => "System Utilities",
            Category::Network => "Network Tools",
            Category::Container => "Containers & VMs",
            Category::Language => "Languages & Runtimes",
            Category::Database => "Databases",
            Category::Security => "Security",
            Category::Productivity => "Productivity",
            Category::Media => "Media",
            Category::Cloud => "Cloud & DevOps",
            Category::AI => "AI & ML Tools",
        }
    }

    pub fn all() -> &'static [Category] {
        &[
            Category::Shell,
            Category::Editor,
            Category::Git,
            Category::Terminal,
            Category::FileManager,
            Category::Search,
            Category::System,
            Category::Network,
            Category::Container,
            Category::Language,
            Category::Database,
            Category::Security,
            Category::Productivity,
            Category::Media,
            Category::Cloud,
            Category::AI,
        ]
    }
}

#[derive(Debug, Clone)]
pub enum InstallMethod {
    Brew(&'static str),
    BrewCask(&'static str),
    Cargo(&'static str),
    Npm(&'static str),
    Pip(&'static str),
    Go(&'static str),
    Script(&'static str),
    Manual(&'static str),
    Apt(&'static str),
}

impl InstallMethod {
    pub fn command(&self) -> String {
        match self {
            InstallMethod::Brew(pkg) => format!("brew install {}", pkg),
            InstallMethod::BrewCask(pkg) => format!("brew install --cask {}", pkg),
            InstallMethod::Cargo(pkg) => format!("cargo install {}", pkg),
            InstallMethod::Npm(pkg) => format!("npm install -g {}", pkg),
            InstallMethod::Pip(pkg) => format!("pip install {}", pkg),
            InstallMethod::Go(pkg) => format!("go install {}", pkg),
            InstallMethod::Script(url) => format!("curl -fsSL {} | sh", url),
            InstallMethod::Manual(cmd) => cmd.to_string(),
            InstallMethod::Apt(pkg) => format!("sudo apt install -y {}", pkg),
        }
    }
}

/// The complete app catalog
pub const CATALOG: &[App] = &[
    // ═══════════════════════════════════════════════════════════════
    // SHELL & PROMPT
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "zsh",
        name: "Zsh",
        description: "Extended Bourne shell with many improvements",
        category: Category::Shell,
        install_method: InstallMethod::Brew("zsh"),
        config_files: &["~/.zshrc", "~/.zshenv", "~/.zprofile"],
        dependencies: &[],
        tags: &["shell", "essential"],
        url: "https://www.zsh.org/",
    },
    App {
        id: "starship",
        name: "Starship",
        description: "Minimal, blazing-fast, customizable prompt for any shell",
        category: Category::Shell,
        install_method: InstallMethod::Brew("starship"),
        config_files: &["~/.config/starship.toml"],
        dependencies: &[],
        tags: &["prompt", "rust", "essential"],
        url: "https://starship.rs/",
    },
    App {
        id: "zoxide",
        name: "Zoxide",
        description: "Smarter cd command - remembers your most used directories",
        category: Category::Shell,
        install_method: InstallMethod::Brew("zoxide"),
        config_files: &[],
        dependencies: &[],
        tags: &["navigation", "rust", "essential"],
        url: "https://github.com/ajeetdsouza/zoxide",
    },
    App {
        id: "direnv",
        name: "Direnv",
        description: "Environment switcher for the shell - auto-load .envrc files",
        category: Category::Shell,
        install_method: InstallMethod::Brew("direnv"),
        config_files: &["~/.config/direnv/direnvrc"],
        dependencies: &[],
        tags: &["environment", "essential"],
        url: "https://direnv.net/",
    },
    App {
        id: "atuin",
        name: "Atuin",
        description: "Magical shell history - sync, search, and stats",
        category: Category::Shell,
        install_method: InstallMethod::Brew("atuin"),
        config_files: &["~/.config/atuin/config.toml"],
        dependencies: &[],
        tags: &["history", "rust", "sync"],
        url: "https://atuin.sh/",
    },
    // ═══════════════════════════════════════════════════════════════
    // EDITORS
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "neovim",
        name: "Neovim",
        description: "Hyperextensible Vim-based text editor",
        category: Category::Editor,
        install_method: InstallMethod::Brew("neovim"),
        config_files: &["~/.config/nvim/init.lua", "~/.config/nvim/"],
        dependencies: &[],
        tags: &["editor", "vim", "lua"],
        url: "https://neovim.io/",
    },
    App {
        id: "helix",
        name: "Helix",
        description: "Post-modern modal text editor with built-in LSP",
        category: Category::Editor,
        install_method: InstallMethod::Brew("helix"),
        config_files: &[
            "~/.config/helix/config.toml",
            "~/.config/helix/languages.toml",
        ],
        dependencies: &[],
        tags: &["editor", "rust", "modal"],
        url: "https://helix-editor.com/",
    },
    App {
        id: "micro",
        name: "Micro",
        description: "Modern and intuitive terminal-based text editor",
        category: Category::Editor,
        install_method: InstallMethod::Brew("micro"),
        config_files: &["~/.config/micro/settings.json"],
        dependencies: &[],
        tags: &["editor", "simple"],
        url: "https://micro-editor.github.io/",
    },
    App {
        id: "vscode",
        name: "Visual Studio Code",
        description: "Code editor with extensions and integrated terminal",
        category: Category::Editor,
        install_method: InstallMethod::BrewCask("visual-studio-code"),
        config_files: &["~/.config/Code/User/settings.json"],
        dependencies: &[],
        tags: &["editor", "gui", "extensions"],
        url: "https://code.visualstudio.com/",
    },
    // ═══════════════════════════════════════════════════════════════
    // GIT & VERSION CONTROL
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "git",
        name: "Git",
        description: "Distributed version control system",
        category: Category::Git,
        install_method: InstallMethod::Brew("git"),
        config_files: &["~/.gitconfig", "~/.gitignore_global"],
        dependencies: &[],
        tags: &["vcs", "essential"],
        url: "https://git-scm.com/",
    },
    App {
        id: "gh",
        name: "GitHub CLI",
        description: "GitHub's official CLI for PRs, issues, and more",
        category: Category::Git,
        install_method: InstallMethod::Brew("gh"),
        config_files: &["~/.config/gh/config.yml"],
        dependencies: &["git"],
        tags: &["github", "cli"],
        url: "https://cli.github.com/",
    },
    App {
        id: "delta",
        name: "Delta",
        description: "Beautiful syntax-highlighting pager for git and diff",
        category: Category::Git,
        install_method: InstallMethod::Brew("git-delta"),
        config_files: &[],
        dependencies: &["git"],
        tags: &["diff", "rust", "essential"],
        url: "https://github.com/dandavison/delta",
    },
    App {
        id: "lazygit",
        name: "Lazygit",
        description: "Simple terminal UI for git commands",
        category: Category::Git,
        install_method: InstallMethod::Brew("lazygit"),
        config_files: &["~/.config/lazygit/config.yml"],
        dependencies: &["git"],
        tags: &["tui", "go"],
        url: "https://github.com/jesseduffield/lazygit",
    },
    App {
        id: "git-absorb",
        name: "Git Absorb",
        description: "Automatically absorb staged changes into your current branch",
        category: Category::Git,
        install_method: InstallMethod::Brew("git-absorb"),
        config_files: &[],
        dependencies: &["git"],
        tags: &["workflow", "rust"],
        url: "https://github.com/tummychow/git-absorb",
    },
    // ═══════════════════════════════════════════════════════════════
    // TERMINAL TOOLS
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "tmux",
        name: "Tmux",
        description: "Terminal multiplexer - multiple windows in one terminal",
        category: Category::Terminal,
        install_method: InstallMethod::Brew("tmux"),
        config_files: &["~/.tmux.conf", "~/.config/tmux/tmux.conf"],
        dependencies: &[],
        tags: &["multiplexer", "essential"],
        url: "https://github.com/tmux/tmux",
    },
    App {
        id: "zellij",
        name: "Zellij",
        description: "Modern terminal workspace with layouts and plugins",
        category: Category::Terminal,
        install_method: InstallMethod::Brew("zellij"),
        config_files: &["~/.config/zellij/config.kdl"],
        dependencies: &[],
        tags: &["multiplexer", "rust"],
        url: "https://zellij.dev/",
    },
    App {
        id: "wezterm",
        name: "WezTerm",
        description: "GPU-accelerated terminal emulator with Lua config",
        category: Category::Terminal,
        install_method: InstallMethod::BrewCask("wezterm"),
        config_files: &["~/.wezterm.lua", "~/.config/wezterm/wezterm.lua"],
        dependencies: &[],
        tags: &["emulator", "rust", "gpu"],
        url: "https://wezfurlong.org/wezterm/",
    },
    App {
        id: "alacritty",
        name: "Alacritty",
        description: "GPU-accelerated terminal emulator",
        category: Category::Terminal,
        install_method: InstallMethod::BrewCask("alacritty"),
        config_files: &["~/.config/alacritty/alacritty.toml"],
        dependencies: &[],
        tags: &["emulator", "rust", "gpu"],
        url: "https://alacritty.org/",
    },
    App {
        id: "kitty",
        name: "Kitty",
        description: "Fast, feature-rich, GPU-based terminal emulator",
        category: Category::Terminal,
        install_method: InstallMethod::BrewCask("kitty"),
        config_files: &["~/.config/kitty/kitty.conf"],
        dependencies: &[],
        tags: &["emulator", "gpu"],
        url: "https://sw.kovidgoyal.net/kitty/",
    },
    // ═══════════════════════════════════════════════════════════════
    // FILE MANAGEMENT
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "eza",
        name: "Eza",
        description: "Modern replacement for ls with icons and git integration",
        category: Category::FileManager,
        install_method: InstallMethod::Brew("eza"),
        config_files: &[],
        dependencies: &[],
        tags: &["ls", "rust", "essential"],
        url: "https://eza.rocks/",
    },
    App {
        id: "bat",
        name: "Bat",
        description: "Cat clone with syntax highlighting and git integration",
        category: Category::FileManager,
        install_method: InstallMethod::Brew("bat"),
        config_files: &["~/.config/bat/config"],
        dependencies: &[],
        tags: &["cat", "rust", "essential"],
        url: "https://github.com/sharkdp/bat",
    },
    App {
        id: "yazi",
        name: "Yazi",
        description: "Blazing fast terminal file manager with async I/O",
        category: Category::FileManager,
        install_method: InstallMethod::Brew("yazi"),
        config_files: &["~/.config/yazi/yazi.toml", "~/.config/yazi/keymap.toml"],
        dependencies: &[],
        tags: &["tui", "rust", "async"],
        url: "https://yazi-rs.github.io/",
    },
    App {
        id: "broot",
        name: "Broot",
        description: "New way to see and navigate directory trees",
        category: Category::FileManager,
        install_method: InstallMethod::Brew("broot"),
        config_files: &["~/.config/broot/conf.hjson"],
        dependencies: &[],
        tags: &["tree", "rust"],
        url: "https://dystroy.org/broot/",
    },
    App {
        id: "dua",
        name: "Dua",
        description: "Disk usage analyzer with interactive mode",
        category: Category::FileManager,
        install_method: InstallMethod::Brew("dua-cli"),
        config_files: &[],
        dependencies: &[],
        tags: &["disk", "rust"],
        url: "https://github.com/Byron/dua-cli",
    },
    App {
        id: "dust",
        name: "Dust",
        description: "More intuitive version of du in rust",
        category: Category::FileManager,
        install_method: InstallMethod::Brew("dust"),
        config_files: &[],
        dependencies: &[],
        tags: &["disk", "rust"],
        url: "https://github.com/bootandy/dust",
    },
    App {
        id: "trash-cli",
        name: "Trash CLI",
        description: "Move files to trash instead of permanent deletion",
        category: Category::FileManager,
        install_method: InstallMethod::Brew("trash-cli"),
        config_files: &[],
        dependencies: &[],
        tags: &["safety"],
        url: "https://github.com/andreafrancia/trash-cli",
    },
    // ═══════════════════════════════════════════════════════════════
    // SEARCH & NAVIGATION
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "fzf",
        name: "FZF",
        description: "Fuzzy finder for the command line",
        category: Category::Search,
        install_method: InstallMethod::Brew("fzf"),
        config_files: &[],
        dependencies: &[],
        tags: &["fuzzy", "go", "essential"],
        url: "https://github.com/junegunn/fzf",
    },
    App {
        id: "ripgrep",
        name: "Ripgrep",
        description: "Recursively search directories for a regex pattern",
        category: Category::Search,
        install_method: InstallMethod::Brew("ripgrep"),
        config_files: &["~/.ripgreprc"],
        dependencies: &[],
        tags: &["grep", "rust", "essential"],
        url: "https://github.com/BurntSushi/ripgrep",
    },
    App {
        id: "fd",
        name: "Fd",
        description: "Simple, fast alternative to find",
        category: Category::Search,
        install_method: InstallMethod::Brew("fd"),
        config_files: &[],
        dependencies: &[],
        tags: &["find", "rust", "essential"],
        url: "https://github.com/sharkdp/fd",
    },
    App {
        id: "ag",
        name: "The Silver Searcher",
        description: "Code searching tool similar to ack, but faster",
        category: Category::Search,
        install_method: InstallMethod::Brew("the_silver_searcher"),
        config_files: &[],
        dependencies: &[],
        tags: &["grep"],
        url: "https://geoff.greer.fm/ag/",
    },
    // ═══════════════════════════════════════════════════════════════
    // SYSTEM UTILITIES
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "htop",
        name: "Htop",
        description: "Interactive process viewer",
        category: Category::System,
        install_method: InstallMethod::Brew("htop"),
        config_files: &["~/.config/htop/htoprc"],
        dependencies: &[],
        tags: &["process", "essential"],
        url: "https://htop.dev/",
    },
    App {
        id: "btop",
        name: "Btop",
        description: "Resource monitor with beautiful graphs",
        category: Category::System,
        install_method: InstallMethod::Brew("btop"),
        config_files: &["~/.config/btop/btop.conf"],
        dependencies: &[],
        tags: &["monitor", "cpp"],
        url: "https://github.com/aristocratos/btop",
    },
    App {
        id: "bottom",
        name: "Bottom",
        description: "Cross-platform graphical process/system monitor",
        category: Category::System,
        install_method: InstallMethod::Brew("bottom"),
        config_files: &["~/.config/bottom/bottom.toml"],
        dependencies: &[],
        tags: &["monitor", "rust"],
        url: "https://github.com/ClementTsang/bottom",
    },
    App {
        id: "procs",
        name: "Procs",
        description: "Modern replacement for ps",
        category: Category::System,
        install_method: InstallMethod::Brew("procs"),
        config_files: &["~/.config/procs/config.toml"],
        dependencies: &[],
        tags: &["process", "rust"],
        url: "https://github.com/dalance/procs",
    },
    App {
        id: "hyperfine",
        name: "Hyperfine",
        description: "Command-line benchmarking tool",
        category: Category::System,
        install_method: InstallMethod::Brew("hyperfine"),
        config_files: &[],
        dependencies: &[],
        tags: &["benchmark", "rust"],
        url: "https://github.com/sharkdp/hyperfine",
    },
    App {
        id: "tokei",
        name: "Tokei",
        description: "Count lines of code quickly",
        category: Category::System,
        install_method: InstallMethod::Brew("tokei"),
        config_files: &[],
        dependencies: &[],
        tags: &["stats", "rust"],
        url: "https://github.com/XAMPPRocky/tokei",
    },
    // ═══════════════════════════════════════════════════════════════
    // NETWORK TOOLS
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "curl",
        name: "cURL",
        description: "Transfer data with URLs",
        category: Category::Network,
        install_method: InstallMethod::Brew("curl"),
        config_files: &["~/.curlrc"],
        dependencies: &[],
        tags: &["http", "essential"],
        url: "https://curl.se/",
    },
    App {
        id: "httpie",
        name: "HTTPie",
        description: "Human-friendly HTTP client for the API era",
        category: Category::Network,
        install_method: InstallMethod::Brew("httpie"),
        config_files: &["~/.config/httpie/config.json"],
        dependencies: &[],
        tags: &["http", "python"],
        url: "https://httpie.io/",
    },
    App {
        id: "xh",
        name: "xh",
        description: "Friendly and fast tool for sending HTTP requests",
        category: Category::Network,
        install_method: InstallMethod::Brew("xh"),
        config_files: &[],
        dependencies: &[],
        tags: &["http", "rust"],
        url: "https://github.com/ducaale/xh",
    },
    App {
        id: "bandwhich",
        name: "Bandwhich",
        description: "Terminal bandwidth utilization tool",
        category: Category::Network,
        install_method: InstallMethod::Brew("bandwhich"),
        config_files: &[],
        dependencies: &[],
        tags: &["monitor", "rust"],
        url: "https://github.com/imsnif/bandwhich",
    },
    App {
        id: "doggo",
        name: "Doggo",
        description: "Command-line DNS client for humans",
        category: Category::Network,
        install_method: InstallMethod::Brew("doggo"),
        config_files: &[],
        dependencies: &[],
        tags: &["dns", "go"],
        url: "https://github.com/mr-karan/doggo",
    },
    // ═══════════════════════════════════════════════════════════════
    // CONTAINERS & VMs
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "docker",
        name: "Docker",
        description: "Container runtime and build system",
        category: Category::Container,
        install_method: InstallMethod::BrewCask("docker"),
        config_files: &["~/.docker/config.json"],
        dependencies: &[],
        tags: &["container", "essential"],
        url: "https://www.docker.com/",
    },
    App {
        id: "podman",
        name: "Podman",
        description: "Daemonless container engine",
        category: Category::Container,
        install_method: InstallMethod::Brew("podman"),
        config_files: &["~/.config/containers/"],
        dependencies: &[],
        tags: &["container"],
        url: "https://podman.io/",
    },
    App {
        id: "lazydocker",
        name: "Lazydocker",
        description: "Terminal UI for docker and docker-compose",
        category: Category::Container,
        install_method: InstallMethod::Brew("lazydocker"),
        config_files: &["~/.config/lazydocker/config.yml"],
        dependencies: &["docker"],
        tags: &["tui", "go"],
        url: "https://github.com/jesseduffield/lazydocker",
    },
    App {
        id: "dive",
        name: "Dive",
        description: "Explore docker image layers",
        category: Category::Container,
        install_method: InstallMethod::Brew("dive"),
        config_files: &[],
        dependencies: &["docker"],
        tags: &["docker", "go"],
        url: "https://github.com/wagoodman/dive",
    },
    App {
        id: "colima",
        name: "Colima",
        description: "Container runtimes on macOS with minimal setup",
        category: Category::Container,
        install_method: InstallMethod::Brew("colima"),
        config_files: &["~/.colima/default/colima.yaml"],
        dependencies: &[],
        tags: &["docker", "macos"],
        url: "https://github.com/abiosoft/colima",
    },
    // ═══════════════════════════════════════════════════════════════
    // LANGUAGES & RUNTIMES
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "mise",
        name: "Mise",
        description: "Polyglot runtime manager (replaces asdf, nvm, pyenv)",
        category: Category::Language,
        install_method: InstallMethod::Brew("mise"),
        config_files: &["~/.config/mise/config.toml", ".mise.toml"],
        dependencies: &[],
        tags: &["version-manager", "rust", "essential"],
        url: "https://mise.jdx.dev/",
    },
    App {
        id: "rustup",
        name: "Rustup",
        description: "Rust toolchain installer and manager",
        category: Category::Language,
        install_method: InstallMethod::Script("https://sh.rustup.rs"),
        config_files: &["~/.cargo/config.toml"],
        dependencies: &[],
        tags: &["rust"],
        url: "https://rustup.rs/",
    },
    App {
        id: "go",
        name: "Go",
        description: "Go programming language",
        category: Category::Language,
        install_method: InstallMethod::Brew("go"),
        config_files: &[],
        dependencies: &[],
        tags: &["go"],
        url: "https://go.dev/",
    },
    App {
        id: "python",
        name: "Python",
        description: "Python programming language",
        category: Category::Language,
        install_method: InstallMethod::Brew("python"),
        config_files: &["~/.config/pip/pip.conf"],
        dependencies: &[],
        tags: &["python"],
        url: "https://www.python.org/",
    },
    App {
        id: "uv",
        name: "uv",
        description: "Extremely fast Python package installer",
        category: Category::Language,
        install_method: InstallMethod::Brew("uv"),
        config_files: &["~/.config/uv/uv.toml"],
        dependencies: &[],
        tags: &["python", "rust", "package-manager"],
        url: "https://github.com/astral-sh/uv",
    },
    App {
        id: "deno",
        name: "Deno",
        description: "Secure runtime for JavaScript and TypeScript",
        category: Category::Language,
        install_method: InstallMethod::Brew("deno"),
        config_files: &["deno.json"],
        dependencies: &[],
        tags: &["javascript", "typescript", "rust"],
        url: "https://deno.land/",
    },
    App {
        id: "bun",
        name: "Bun",
        description: "Incredibly fast JavaScript runtime and toolkit",
        category: Category::Language,
        install_method: InstallMethod::Brew("bun"),
        config_files: &["bunfig.toml"],
        dependencies: &[],
        tags: &["javascript", "fast"],
        url: "https://bun.sh/",
    },
    // ═══════════════════════════════════════════════════════════════
    // DATABASES
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "sqlite",
        name: "SQLite",
        description: "Self-contained SQL database engine",
        category: Category::Database,
        install_method: InstallMethod::Brew("sqlite"),
        config_files: &["~/.sqliterc"],
        dependencies: &[],
        tags: &["sql"],
        url: "https://www.sqlite.org/",
    },
    App {
        id: "postgresql",
        name: "PostgreSQL",
        description: "Powerful open-source relational database",
        category: Category::Database,
        install_method: InstallMethod::Brew("postgresql@16"),
        config_files: &[],
        dependencies: &[],
        tags: &["sql"],
        url: "https://www.postgresql.org/",
    },
    App {
        id: "redis",
        name: "Redis",
        description: "In-memory data structure store",
        category: Category::Database,
        install_method: InstallMethod::Brew("redis"),
        config_files: &[],
        dependencies: &[],
        tags: &["nosql", "cache"],
        url: "https://redis.io/",
    },
    App {
        id: "usql",
        name: "usql",
        description: "Universal command-line interface for SQL databases",
        category: Category::Database,
        install_method: InstallMethod::Brew("usql"),
        config_files: &[],
        dependencies: &[],
        tags: &["sql", "universal"],
        url: "https://github.com/xo/usql",
    },
    // ═══════════════════════════════════════════════════════════════
    // SECURITY
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "age",
        name: "Age",
        description: "Simple, modern, secure file encryption",
        category: Category::Security,
        install_method: InstallMethod::Brew("age"),
        config_files: &[],
        dependencies: &[],
        tags: &["encryption", "go"],
        url: "https://age-encryption.org/",
    },
    App {
        id: "gnupg",
        name: "GnuPG",
        description: "GNU Privacy Guard - encryption and signing",
        category: Category::Security,
        install_method: InstallMethod::Brew("gnupg"),
        config_files: &["~/.gnupg/gpg.conf", "~/.gnupg/gpg-agent.conf"],
        dependencies: &[],
        tags: &["encryption", "signing"],
        url: "https://gnupg.org/",
    },
    App {
        id: "pass",
        name: "Pass",
        description: "Standard unix password manager",
        category: Category::Security,
        install_method: InstallMethod::Brew("pass"),
        config_files: &["~/.password-store/"],
        dependencies: &["gnupg"],
        tags: &["password"],
        url: "https://www.passwordstore.org/",
    },
    App {
        id: "sops",
        name: "SOPS",
        description: "Secrets OPerationS - manage encrypted files",
        category: Category::Security,
        install_method: InstallMethod::Brew("sops"),
        config_files: &[".sops.yaml"],
        dependencies: &[],
        tags: &["secrets", "go"],
        url: "https://github.com/getsops/sops",
    },
    App {
        id: "1password-cli",
        name: "1Password CLI",
        description: "1Password command-line tool",
        category: Category::Security,
        install_method: InstallMethod::BrewCask("1password-cli"),
        config_files: &[],
        dependencies: &[],
        tags: &["password", "commercial"],
        url: "https://1password.com/downloads/command-line/",
    },
    // ═══════════════════════════════════════════════════════════════
    // PRODUCTIVITY
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "jq",
        name: "jq",
        description: "Lightweight command-line JSON processor",
        category: Category::Productivity,
        install_method: InstallMethod::Brew("jq"),
        config_files: &[],
        dependencies: &[],
        tags: &["json", "essential"],
        url: "https://jqlang.github.io/jq/",
    },
    App {
        id: "yq",
        name: "yq",
        description: "YAML/JSON/XML processor like jq",
        category: Category::Productivity,
        install_method: InstallMethod::Brew("yq"),
        config_files: &[],
        dependencies: &[],
        tags: &["yaml", "json"],
        url: "https://mikefarah.gitbook.io/yq/",
    },
    App {
        id: "jless",
        name: "jless",
        description: "Command-line JSON viewer",
        category: Category::Productivity,
        install_method: InstallMethod::Brew("jless"),
        config_files: &[],
        dependencies: &[],
        tags: &["json", "rust"],
        url: "https://jless.io/",
    },
    App {
        id: "glow",
        name: "Glow",
        description: "Render markdown on the CLI with style",
        category: Category::Productivity,
        install_method: InstallMethod::Brew("glow"),
        config_files: &["~/.config/glow/glow.yml"],
        dependencies: &[],
        tags: &["markdown", "go"],
        url: "https://github.com/charmbracelet/glow",
    },
    App {
        id: "slides",
        name: "Slides",
        description: "Terminal-based presentation tool",
        category: Category::Productivity,
        install_method: InstallMethod::Brew("slides"),
        config_files: &[],
        dependencies: &[],
        tags: &["presentation", "go"],
        url: "https://github.com/maaslalani/slides",
    },
    App {
        id: "chezmoi",
        name: "Chezmoi",
        description: "Manage your dotfiles across multiple machines",
        category: Category::Productivity,
        install_method: InstallMethod::Brew("chezmoi"),
        config_files: &["~/.config/chezmoi/chezmoi.toml"],
        dependencies: &[],
        tags: &["dotfiles", "go", "essential"],
        url: "https://www.chezmoi.io/",
    },
    // ═══════════════════════════════════════════════════════════════
    // MEDIA
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "ffmpeg",
        name: "FFmpeg",
        description: "Complete solution for audio/video processing",
        category: Category::Media,
        install_method: InstallMethod::Brew("ffmpeg"),
        config_files: &[],
        dependencies: &[],
        tags: &["video", "audio"],
        url: "https://ffmpeg.org/",
    },
    App {
        id: "imagemagick",
        name: "ImageMagick",
        description: "Create, edit, compose, or convert images",
        category: Category::Media,
        install_method: InstallMethod::Brew("imagemagick"),
        config_files: &[],
        dependencies: &[],
        tags: &["image"],
        url: "https://imagemagick.org/",
    },
    App {
        id: "yt-dlp",
        name: "yt-dlp",
        description: "Download videos from YouTube and other sites",
        category: Category::Media,
        install_method: InstallMethod::Brew("yt-dlp"),
        config_files: &["~/.config/yt-dlp/config"],
        dependencies: &["ffmpeg"],
        tags: &["video", "download"],
        url: "https://github.com/yt-dlp/yt-dlp",
    },
    // ═══════════════════════════════════════════════════════════════
    // CLOUD & DEVOPS
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "awscli",
        name: "AWS CLI",
        description: "Amazon Web Services command-line interface",
        category: Category::Cloud,
        install_method: InstallMethod::Brew("awscli"),
        config_files: &["~/.aws/config", "~/.aws/credentials"],
        dependencies: &[],
        tags: &["aws", "cloud"],
        url: "https://aws.amazon.com/cli/",
    },
    App {
        id: "terraform",
        name: "Terraform",
        description: "Infrastructure as Code tool",
        category: Category::Cloud,
        install_method: InstallMethod::Brew("terraform"),
        config_files: &["~/.terraformrc"],
        dependencies: &[],
        tags: &["iac", "hashicorp"],
        url: "https://www.terraform.io/",
    },
    App {
        id: "kubectl",
        name: "kubectl",
        description: "Kubernetes command-line tool",
        category: Category::Cloud,
        install_method: InstallMethod::Brew("kubectl"),
        config_files: &["~/.kube/config"],
        dependencies: &[],
        tags: &["kubernetes"],
        url: "https://kubernetes.io/docs/reference/kubectl/",
    },
    App {
        id: "k9s",
        name: "K9s",
        description: "Terminal UI to interact with Kubernetes clusters",
        category: Category::Cloud,
        install_method: InstallMethod::Brew("k9s"),
        config_files: &["~/.config/k9s/config.yml"],
        dependencies: &["kubectl"],
        tags: &["kubernetes", "tui", "go"],
        url: "https://k9scli.io/",
    },
    App {
        id: "helm",
        name: "Helm",
        description: "Kubernetes package manager",
        category: Category::Cloud,
        install_method: InstallMethod::Brew("helm"),
        config_files: &[],
        dependencies: &["kubectl"],
        tags: &["kubernetes"],
        url: "https://helm.sh/",
    },
    // ═══════════════════════════════════════════════════════════════
    // AI & ML TOOLS
    // ═══════════════════════════════════════════════════════════════
    App {
        id: "ollama",
        name: "Ollama",
        description: "Run large language models locally",
        category: Category::AI,
        install_method: InstallMethod::Brew("ollama"),
        config_files: &[],
        dependencies: &[],
        tags: &["llm", "local"],
        url: "https://ollama.ai/",
    },
    App {
        id: "aichat",
        name: "AIChat",
        description: "Chat with AI models in the terminal",
        category: Category::AI,
        install_method: InstallMethod::Brew("aichat"),
        config_files: &["~/.config/aichat/config.yaml"],
        dependencies: &[],
        tags: &["llm", "rust"],
        url: "https://github.com/sigoden/aichat",
    },
    App {
        id: "claude-code",
        name: "Claude Code",
        description: "Anthropic's CLI coding assistant",
        category: Category::AI,
        install_method: InstallMethod::Npm("@anthropic-ai/claude-code"),
        config_files: &["~/.claude/"],
        dependencies: &[],
        tags: &["llm", "coding"],
        url: "https://claude.ai/",
    },
];

/// Get all apps in a specific category
pub fn apps_by_category(category: &Category) -> Vec<&'static App> {
    CATALOG
        .iter()
        .filter(|app| app.category == *category)
        .collect()
}

/// Get all apps with a specific tag
pub fn apps_by_tag(tag: &str) -> Vec<&'static App> {
    CATALOG
        .iter()
        .filter(|app| app.tags.contains(&tag))
        .collect()
}

/// Get essential apps (tagged as essential)
pub fn essential_apps() -> Vec<&'static App> {
    apps_by_tag("essential")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_is_not_empty() {
        assert!(!CATALOG.is_empty());
        assert!(CATALOG.len() >= 50, "Expected at least 50 apps in catalog");
    }

    #[test]
    fn all_categories_have_apps() {
        for cat in Category::all() {
            let apps = apps_by_category(cat);
            assert!(!apps.is_empty(), "Category {:?} has no apps", cat);
        }
    }

    #[test]
    fn essential_apps_exist() {
        let essentials = essential_apps();
        assert!(
            essentials.len() >= 5,
            "Expected at least 5 essential apps, got {}",
            essentials.len()
        );
    }

    #[test]
    fn all_apps_have_unique_ids() {
        let mut ids = std::collections::HashSet::new();
        for app in CATALOG {
            assert!(ids.insert(app.id), "Duplicate app ID: {}", app.id);
        }
    }

    #[test]
    fn all_apps_have_names_and_descriptions() {
        for app in CATALOG {
            assert!(!app.name.is_empty(), "App {} has empty name", app.id);
            assert!(
                !app.description.is_empty(),
                "App {} has empty description",
                app.id
            );
            assert!(!app.url.is_empty(), "App {} has empty URL", app.id);
        }
    }

    #[test]
    fn install_methods_produce_commands() {
        for app in CATALOG {
            let cmd = app.install_method.command();
            assert!(!cmd.is_empty(), "App {} has empty install command", app.id);
        }
    }

    #[test]
    fn apps_by_tag_works() {
        let rust_apps = apps_by_tag("rust");
        assert!(!rust_apps.is_empty(), "No apps tagged 'rust'");

        for app in &rust_apps {
            assert!(
                app.tags.contains(&"rust"),
                "App {} returned by tag 'rust' but doesn't have that tag",
                app.id
            );
        }
    }

    #[test]
    fn category_metadata() {
        for cat in Category::all() {
            assert!(!cat.name().is_empty());
            assert!(!cat.icon().is_empty());
        }
    }
}
