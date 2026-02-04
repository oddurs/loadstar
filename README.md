# LOAD"*",8,1

```


    **** COMMODORE 64 BASIC V2 ****

 64K RAM SYSTEM  38911 BASIC BYTES FREE

READY.
LOAD"*",8,1

SEARCHING FOR *
LOADING
READY.
RUN

```

It's a machine setup wizard. It installs your dev tools, writes your dotfiles, configures git and SSH, and does it all through a TUI with matrix rain because of course it does.

The name is the Commodore 64 command for "load whatever's on the disk." That's what this does to your machine. You run it, and it loads everything.

## What happens when you run it

Eight phases. You can't skip the boot sequence. (You can skip the boot sequence.)

1. **Boot** — matrix rain, system probe, the machine pretends to think
2. **Identity** — who are you, what's your email, are you at work or not
3. **Shell** — zsh, starship, terminal emulator, multiplexer
4. **DevTools** — pick a preset or be difficult about it
5. **Apps** — 73 tools across 16 categories. Scroll through them. Toggle things.
6. **Review** — look at what you've done. Last chance.
7. **Install** — it actually installs everything. Real commands. A progress bar. Color-coded log. Green means good, red means bad, yellow means it was already there.
8. **Complete** — `READY.`

This is not a simulation. It runs `brew install`. It runs `cargo install`. It generates your `.gitconfig` and `.zshrc` from your selections. It creates SSH keys and wires up your GitHub. It backs up your existing configs before touching them.

## Get it

Pre-built binaries:

```bash
# macOS Apple Silicon
curl -fsSL https://github.com/oddurs/loadstar/releases/latest/download/load-macos-arm64 -o load
chmod +x load && ./load

# macOS Intel
curl -fsSL https://github.com/oddurs/loadstar/releases/latest/download/load-macos-x86_64 -o load
chmod +x load && ./load

# Linux x86_64
curl -fsSL https://github.com/oddurs/loadstar/releases/latest/download/load-linux-x86_64 -o load
chmod +x load && ./load
```

Or build it yourself:

```bash
git clone https://github.com/oddurs/loadstar.git
cd loadstar/installer
cargo build --release
./target/release/load
```

## What's in the catalog

73 things. Some of them:

| | |
|---|---|
| **Shell** | zsh, starship, tmux, alacritty, wezterm |
| **Files** | fd, ripgrep, fzf, eza, bat, zoxide |
| **Git** | delta, gh, lazygit, git-lfs |
| **Editors** | neovim, vim, helix, emacs |
| **Languages** | mise, node, python, go, rust, deno |
| **Containers** | docker, colima, lazydocker, dive |
| **Cloud** | awscli, terraform, kubectl, helm |
| **Databases** | postgresql, redis, sqlite, duckdb |
| **Networking** | curl, httpie, wget, nmap |
| **Security** | age, gnupg, 1password-cli |
| **Fonts** | JetBrains Mono, Fira Code, Hack (Nerd Font) |
| **macOS** | Rectangle, Raycast, Arc, Spotify |

You can pick a preset (minimal, standard, full) or hand-select. It doesn't care.

## What it writes to your machine

Dotfiles generated from your wizard answers:

- **`.gitconfig`** — identity, delta, SSH URLs, GPG signing, work/personal includeIf
- **`starship.toml`** — Catppuccin prompt
- **`.zshrc`** — aliases for whatever tools you picked, PATH, initializations
- **`tmux.conf`** — Catppuccin, OS-aware clipboard
- **`.editorconfig`** — spaces, not tabs
- **SSH key** — ed25519, correct permissions, macOS Keychain
- **Git identity** — name, email, default branch

Existing files get backed up as `.load-backup` first. We're not monsters.

## Technical

```
Binary size:    737 KB (stripped, LTO)
Dependencies:   7 (ratatui, crossterm, anyhow, rand, whoami, serde, chrono)
Tests:          42 unit tests across 5 modules
Async runtime:  none (std::thread + mpsc)
Platforms:      macOS arm64, macOS x86_64, Linux x86_64
```

## Inside

```
installer/
├── Cargo.toml
├── build.rs                # build-time git hash + timestamp
├── assets/complete.txt     # the READY. screen
└── src/
    ├── main.rs             # event loop, boot sequence, input handling
    ├── wizard.rs           # state machine, 8 phases, identity, selections
    ├── render.rs           # TUI rendering, one function per phase
    ├── catalog.rs          # 73 apps, 16 categories, install methods
    ├── effects.rs          # matrix rain, typewriter, spinner
    ├── ascii_art.rs        # character sets
    ├── system.rs           # OS/arch detection, package managers
    ├── executor.rs         # the part that actually installs things
    ├── config.rs           # dotfile generation
    └── github.rs           # SSH, git config, gh CLI, GPG
```

## License

[MIT](LICENSE)

---

```
READY.
█
```
