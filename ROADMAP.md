# LOAD"*",8,1 — Roadmap to v1.0 Release

## Current State — v1.0 READY

- 10 source files, ~5,750 lines of Rust (lean, no dead code)
- 73 apps in catalog across 16 categories
- Wizard UI with 8 phases (Boot -> Identity -> Shell -> DevTools -> Apps -> Review -> Install -> Complete)
- Matrix rain, typewriter, spinners, hacker theme
- **Real installation engine** — Homebrew bootstrap, package install, already-installed detection
- **Config generation** — .gitconfig, starship.toml, .zshrc, tmux.conf from wizard selections
- **GitHub setup** — SSH key generation, git config, gh CLI auth scaffolding, GPG signing
- **Background thread** installation with mpsc channel for real-time progress
- **Polished UI** — per-package status, color-coded log, install summary, system info bar, graceful Ctrl+C
- **42 unit tests**, zero warnings, zero clippy warnings
- 7 runtime dependencies, release binary: 737KB stripped
- Git repo initialized, MIT LICENSE, GitHub Actions CI/CD configured

---

## Phase 1: CLEANUP (Dead Code Removal & Dependency Pruning) -- COMPLETE

- [x] Deleted 8 dead modules (~2,191 lines)
- [x] Pruned Cargo.toml from 30 deps to 7
- [x] Zero compiler warnings, zero clippy warnings
- [x] `cargo fmt --check` passes
- [x] Binary renamed to `load`, package to `loadstar`

---

## Phase 2: CORE ENGINE (Real Installation) -- COMPLETE

- [x] `system.rs` — OS/arch detection, package manager discovery, pre-flight checks
- [x] `executor.rs` — real package installation via brew, brew cask, cargo, npm, pip, go, scripts
- [x] Homebrew bootstrap — detect, install if missing, `brew update`
- [x] Already-installed detection (skip packages that exist)
- [x] Background thread with `mpsc` channel for real-time TUI updates (no tokio needed)
- [x] `config.rs` — generates .gitconfig, starship.toml, .zshrc, tmux.conf, .editorconfig
- [x] Configs adapt to wizard selections (editor, tools, OS, selected apps)
- [x] Backup existing configs before overwriting (`.load-backup` extension)
- [x] Error recovery — failed packages are skipped and reported, install continues
- [x] Wired into main.rs — install phase runs real commands

**Architecture decision:** Used `std::thread` + `std::sync::mpsc` instead of tokio. This keeps the dependency count at 7 and binary at ~700KB. The install thread sends `InstallMessage` variants back to the TUI main loop which processes them non-blocking via `try_recv()`.

---

## Phase 3: GITHUB CLI SETUP -- COMPLETE

- [x] `github.rs` — full Git & GitHub setup module
- [x] Git identity from wizard (user.name, user.email, defaultBranch, pull.rebase, etc.)
- [x] Delta as pager when selected
- [x] SSH URL rewrite (git@github.com: instead of https)
- [x] SSH key: check for existing ed25519/RSA keys, generate new ed25519 if none
- [x] Set correct permissions (700 on .ssh, 600 on private key)
- [x] macOS: write ~/.ssh/config with Keychain integration, ssh-add --apple-use-keychain
- [x] Linux: standard ssh-add
- [x] gh CLI: detect auth status, upload SSH key if authenticated, setup-git credential helper
- [x] Non-interactive: gh auth login instructions printed for post-install (can't do device flow in background thread)
- [x] GPG signing: detect existing keys, configure git to use them, or print setup instructions
- [x] generate_ssh_key flag in wizard state, shown on review screen
- [x] Review screen shows Git & GitHub section and Config Files section

---

## Phase 4: DESIGN REVIEW & UI REWORK -- COMPLETE

- [x] Install screen: per-package status line (current package, ok/skip/fail counters, N/M progress)
- [x] Install screen: color-coded log (green for OK, red for FAIL/FATAL, yellow for SKIP/WARN, cyan for PHASE/DONE, dim for subprocess output)
- [x] Completion screen: install summary with succeeded/failed/skipped counts
- [x] Completion screen: dynamic next steps (gh auth login if not authed, etc.)
- [x] Identity screen: system info bar (OS, arch, brew status, hostname)
- [x] Review screen: scrollable with ↑↓ keys
- [x] Review screen: Git & GitHub section and Config Files section
- [x] App descriptions shown by default (show_details defaults to true)
- [x] Graceful Ctrl+C during install: skips to Complete with partial results instead of hard exit

**Deferred to future:**
- [ ] Show which apps are already installed (requires brew list at startup — latency concern)
- [ ] Search/filter in app selection
- [ ] Pre-flight check UI screen
- [ ] Glitch transition effects between phases

---

## Phase 5: TESTING -- COMPLETE

### 5.1 Unit tests
- [x] Test `catalog.rs`: 8 tests — filtering, searching, category grouping, unique IDs, install methods
- [x] Test `wizard.rs`: 14 tests — state transitions, app selection, phase ordering, metadata
- [x] Test `config.rs`: 12 tests — gitconfig variants, starship, zshrc, tmux, editorconfig, display_path
- [x] Test `system.rs`: 6 tests — OS detection, arch, brew prefix, which, package managers
- [x] Test `github.rs`: 3 tests — GPG key ID extraction (ed25519, RSA, empty)

### 5.2 Integration tests
- [ ] Deferred — requires platform matrix CI (covered by GitHub Actions)

### 5.3 Manual testing matrix
- [x] macOS Apple Silicon (M-series) - existing dev setup
- [ ] Remaining platforms covered by CI matrix (macOS + Linux)

### 5.4 Code quality
- [x] `cargo clippy -- -D warnings` passes
- [x] `cargo fmt --check` passes
- [x] `cargo test` passes — 42 tests, 0 failures
- [x] No `unwrap()` in production paths (audited, zero instances)

**Exit criteria:** 42 tests pass, zero clippy warnings, zero compiler warnings

---

## Phase 6: PACKAGING & RELEASE -- COMPLETE

### 6.1 Git repository setup
- [x] `git init`
- [x] `.gitignore` reviewed (updated to keep Cargo.lock for binary crate)
- [x] MIT `LICENSE` file created
- [x] Initial commit

### 6.2 GitHub repository
- [ ] Create repo on GitHub and push (manual step)
- [ ] Set repo description, topics, URL

### 6.3 CI/CD with GitHub Actions
- [x] Workflow: `ci.yml` — build + test + clippy + fmt on push/PR (macOS + Linux matrix)
- [x] Workflow: `release.yml` — build release binaries on tag push
- [x] Matrix: macOS arm64, macOS x86_64, Linux x86_64
- [x] Upload binaries as release assets via softprops/action-gh-release

### 6.4 Release artifacts
- [x] CI builds pre-built binaries for macOS arm64, macOS x86_64, Linux x86_64
- [ ] Install script (future)
- [ ] Homebrew formula (future)

### 6.5 Documentation
- [x] `README.md` rewritten with install instructions, feature list, project structure
- [ ] Screenshots / terminal recordings (future)
- [ ] Contributing guide (future)

**Exit criteria:** Git repo initialized, CI/CD configured, README complete, ready for GitHub push

---

## Estimated Timeline

| Phase | Effort | Status |
|-------|--------|--------|
| Phase 1: Cleanup | ~1 session | COMPLETE |
| Phase 2: Core Engine | ~3 sessions | COMPLETE |
| Phase 3: GitHub CLI | ~1 session | COMPLETE |
| Phase 4: Design Review | ~2 sessions | COMPLETE |
| Phase 5: Testing | ~2 sessions | COMPLETE |
| Phase 6: Release | ~1 session | COMPLETE |

Total: ~10 working sessions to v1.0

---

## File Structure

```
installer/
├── Cargo.toml              # 7 deps
├── build.rs                # Build-time git hash + timestamp
├── assets/complete.txt     # Post-install ASCII banner
├── src/
│   ├── main.rs             # App struct, event loop, input handling, boot sequence
│   ├── wizard.rs           # WizardState, WizardPhase, Identity, ShellConfig, choices
│   ├── render.rs           # All TUI rendering (one function per phase)
│   ├── catalog.rs          # 73 App entries across 16 categories
│   ├── effects.rs          # MatrixRain, TypeWriter, Spinner, HackerTheme
│   ├── ascii_art.rs        # MATRIX_CHARS, GLITCH_CHARS, SPINNER_* constants
│   ├── system.rs           # OS/arch detection, package managers, pre-flight checks
│   ├── executor.rs         # Package installation engine (brew, cargo, npm, etc.)
│   ├── config.rs           # Dotfile generation (.gitconfig, .zshrc, starship, tmux)
│   └── github.rs           # SSH key gen, git config, gh CLI auth, GPG signing
```
