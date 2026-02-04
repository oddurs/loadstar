//! Package installation executor
//! Runs actual install commands, captures output, handles errors and retries.
//! Designed to be driven from an async task that streams results back to the TUI.

use std::process::Command;
use std::sync::mpsc;
use std::time::Instant;

use crate::catalog::{App, InstallMethod};
use crate::system::SystemInfo;

/// Message sent from the install thread back to the TUI
#[derive(Debug, Clone)]
pub enum InstallMessage {
    /// A new phase of installation is starting
    PhaseStart { phase: String },
    /// Starting to install a specific package
    PackageStart { name: String, method: String },
    /// Package installed successfully
    PackageSuccess { name: String, duration_ms: u64 },
    /// Package was already installed
    PackageSkipped { name: String, reason: String },
    /// Package installation failed
    PackageFailed { name: String, error: String },
    /// A log line from command output
    Log(String),
    /// Progress update (completed, total)
    Progress { completed: usize, total: usize },
    /// Installation complete
    Done {
        succeeded: usize,
        failed: usize,
        skipped: usize,
    },
    /// Fatal error — installation cannot continue
    FatalError(String),
}

/// Tracks the result of the entire installation
#[derive(Debug, Default)]
pub struct InstallSummary {
    pub succeeded: Vec<String>,
    pub failed: Vec<(String, String)>,  // (name, error)
    pub skipped: Vec<(String, String)>, // (name, reason)
}

/// Run the full installation sequence.
/// This is designed to run on a background thread.
/// It sends progress messages through the channel.
pub fn run_install(
    system: &SystemInfo,
    apps: Vec<&'static App>,
    tx: &mpsc::Sender<InstallMessage>,
) -> InstallSummary {
    let mut summary = InstallSummary::default();
    let total = apps.len();

    // ─── Phase 1: Homebrew bootstrap ─────────────────────────────
    let _ = tx.send(InstallMessage::PhaseStart {
        phase: "Homebrew Bootstrap".to_string(),
    });

    if !system.has_homebrew() {
        let _ = tx.send(InstallMessage::Log(
            "[BREW] Homebrew not found — installing...".to_string(),
        ));
        match bootstrap_homebrew(system, tx) {
            Ok(()) => {
                let _ = tx.send(InstallMessage::Log(
                    "[BREW] Homebrew installed successfully".to_string(),
                ));
            }
            Err(e) => {
                let _ = tx.send(InstallMessage::FatalError(format!(
                    "Failed to install Homebrew: {}",
                    e
                )));
                return summary;
            }
        }
    } else {
        let _ = tx.send(InstallMessage::Log(
            "[BREW] Homebrew found — updating...".to_string(),
        ));
        let _ = run_command("brew", &["update"], tx);
    }

    // ─── Phase 2: Batch brew installs ────────────────────────────
    let brew_formulae: Vec<&App> = apps
        .iter()
        .filter(|a| matches!(a.install_method, InstallMethod::Brew(_)))
        .copied()
        .collect();

    let brew_casks: Vec<&App> = apps
        .iter()
        .filter(|a| matches!(a.install_method, InstallMethod::BrewCask(_)))
        .copied()
        .collect();

    let other_apps: Vec<&App> = apps
        .iter()
        .filter(|a| {
            !matches!(
                a.install_method,
                InstallMethod::Brew(_) | InstallMethod::BrewCask(_)
            )
        })
        .copied()
        .collect();

    let mut completed = 0;

    // Install brew formulae in batch for speed
    if !brew_formulae.is_empty() {
        let _ = tx.send(InstallMessage::PhaseStart {
            phase: "Homebrew Formulae".to_string(),
        });

        for app in &brew_formulae {
            let start = Instant::now();
            let pkg = match app.install_method {
                InstallMethod::Brew(p) => p,
                _ => unreachable!(),
            };

            let _ = tx.send(InstallMessage::PackageStart {
                name: app.name.to_string(),
                method: format!("brew install {}", pkg),
            });

            // Check if already installed
            if is_brew_installed(pkg) {
                let _ = tx.send(InstallMessage::PackageSkipped {
                    name: app.name.to_string(),
                    reason: "Already installed".to_string(),
                });
                summary
                    .skipped
                    .push((app.name.to_string(), "Already installed".to_string()));
            } else {
                match run_command("brew", &["install", pkg], tx) {
                    Ok(()) => {
                        let _ = tx.send(InstallMessage::PackageSuccess {
                            name: app.name.to_string(),
                            duration_ms: start.elapsed().as_millis() as u64,
                        });
                        summary.succeeded.push(app.name.to_string());
                    }
                    Err(e) => {
                        let _ = tx.send(InstallMessage::PackageFailed {
                            name: app.name.to_string(),
                            error: e.clone(),
                        });
                        summary.failed.push((app.name.to_string(), e));
                    }
                }
            }

            completed += 1;
            let _ = tx.send(InstallMessage::Progress { completed, total });
        }
    }

    // Install brew casks
    if !brew_casks.is_empty() {
        let _ = tx.send(InstallMessage::PhaseStart {
            phase: "Homebrew Casks".to_string(),
        });

        for app in &brew_casks {
            let start = Instant::now();
            let pkg = match app.install_method {
                InstallMethod::BrewCask(p) => p,
                _ => unreachable!(),
            };

            let _ = tx.send(InstallMessage::PackageStart {
                name: app.name.to_string(),
                method: format!("brew install --cask {}", pkg),
            });

            if is_brew_cask_installed(pkg) {
                let _ = tx.send(InstallMessage::PackageSkipped {
                    name: app.name.to_string(),
                    reason: "Already installed".to_string(),
                });
                summary
                    .skipped
                    .push((app.name.to_string(), "Already installed".to_string()));
            } else {
                match run_command("brew", &["install", "--cask", pkg], tx) {
                    Ok(()) => {
                        let _ = tx.send(InstallMessage::PackageSuccess {
                            name: app.name.to_string(),
                            duration_ms: start.elapsed().as_millis() as u64,
                        });
                        summary.succeeded.push(app.name.to_string());
                    }
                    Err(e) => {
                        let _ = tx.send(InstallMessage::PackageFailed {
                            name: app.name.to_string(),
                            error: e.clone(),
                        });
                        summary.failed.push((app.name.to_string(), e));
                    }
                }
            }

            completed += 1;
            let _ = tx.send(InstallMessage::Progress { completed, total });
        }
    }

    // ─── Phase 3: Other install methods ──────────────────────────
    if !other_apps.is_empty() {
        let _ = tx.send(InstallMessage::PhaseStart {
            phase: "Additional Tools".to_string(),
        });

        for app in &other_apps {
            let start = Instant::now();

            let _ = tx.send(InstallMessage::PackageStart {
                name: app.name.to_string(),
                method: app.install_method.command(),
            });

            match install_app(app, system, tx) {
                Ok(()) => {
                    let _ = tx.send(InstallMessage::PackageSuccess {
                        name: app.name.to_string(),
                        duration_ms: start.elapsed().as_millis() as u64,
                    });
                    summary.succeeded.push(app.name.to_string());
                }
                Err(e) => {
                    let _ = tx.send(InstallMessage::PackageFailed {
                        name: app.name.to_string(),
                        error: e.clone(),
                    });
                    summary.failed.push((app.name.to_string(), e));
                }
            }

            completed += 1;
            let _ = tx.send(InstallMessage::Progress { completed, total });
        }
    }

    let _ = tx.send(InstallMessage::Done {
        succeeded: summary.succeeded.len(),
        failed: summary.failed.len(),
        skipped: summary.skipped.len(),
    });

    summary
}

// ─── Homebrew bootstrap ──────────────────────────────────────────────

fn bootstrap_homebrew(
    _system: &SystemInfo,
    tx: &mpsc::Sender<InstallMessage>,
) -> Result<(), String> {
    let _ = tx.send(InstallMessage::Log(
        "[BREW] Downloading Homebrew installer...".to_string(),
    ));

    // The official Homebrew install command
    let output = Command::new("/bin/bash")
        .args([
            "-c",
            "NONINTERACTIVE=1 /bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"",
        ])
        .output()
        .map_err(|e| format!("Failed to run installer: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Homebrew install failed: {}", stderr.trim()))
    }
}

// ─── Individual app installation ─────────────────────────────────────

fn install_app(
    app: &App,
    _system: &SystemInfo,
    tx: &mpsc::Sender<InstallMessage>,
) -> Result<(), String> {
    match &app.install_method {
        InstallMethod::Brew(pkg) => run_command("brew", &["install", pkg], tx),
        InstallMethod::BrewCask(pkg) => run_command("brew", &["install", "--cask", pkg], tx),
        InstallMethod::Cargo(pkg) => run_command("cargo", &["install", pkg], tx),
        InstallMethod::Npm(pkg) => run_command("npm", &["install", "-g", pkg], tx),
        InstallMethod::Pip(pkg) => run_command("pip3", &["install", pkg], tx),
        InstallMethod::Go(pkg) => run_command("go", &["install", pkg], tx),
        InstallMethod::Script(url) => install_via_script(url, tx),
        InstallMethod::Manual(cmd) => run_shell_command(cmd, tx),
        InstallMethod::Apt(pkg) => run_command("sudo", &["apt", "install", "-y", pkg], tx),
    }
}

fn install_via_script(url: &str, tx: &mpsc::Sender<InstallMessage>) -> Result<(), String> {
    let _ = tx.send(InstallMessage::Log(format!("[SCRIPT] Downloading {}", url)));

    // Download the script first, then pipe to sh
    let output = Command::new("/bin/bash")
        .args(["-c", &format!("curl -fsSL {} | sh", url)])
        .output()
        .map_err(|e| format!("Failed to run script: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Script failed: {}", stderr.trim()))
    }
}

fn run_shell_command(cmd: &str, tx: &mpsc::Sender<InstallMessage>) -> Result<(), String> {
    let _ = tx.send(InstallMessage::Log(format!("[CMD] {}", cmd)));

    let output = Command::new("/bin/bash")
        .args(["-c", cmd])
        .output()
        .map_err(|e| format!("Failed to run command: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Command failed: {}", stderr.trim()))
    }
}

// ─── Command runner with log streaming ───────────────────────────────

fn run_command(
    program: &str,
    args: &[&str],
    tx: &mpsc::Sender<InstallMessage>,
) -> Result<(), String> {
    let cmd_str = format!("{} {}", program, args.join(" "));
    let _ = tx.send(InstallMessage::Log(format!("[RUN] {}", cmd_str)));

    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run '{}': {}", cmd_str, e))?;

    // Stream stdout lines
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines().filter(|l| !l.is_empty()) {
        let _ = tx.send(InstallMessage::Log(format!("  {}", line)));
    }

    // Stream stderr lines (not all are errors — brew uses stderr for progress)
    let stderr = String::from_utf8_lossy(&output.stderr);
    for line in stderr.lines().filter(|l| !l.is_empty()) {
        let _ = tx.send(InstallMessage::Log(format!("  {}", line)));
    }

    if output.status.success() {
        Ok(())
    } else {
        let error_msg = if stderr.is_empty() {
            format!("exited with code {}", output.status)
        } else {
            stderr.lines().last().unwrap_or("unknown error").to_string()
        };
        Err(error_msg)
    }
}

// ─── Already-installed detection ─────────────────────────────────────

fn is_brew_installed(formula: &str) -> bool {
    Command::new("brew")
        .args(["list", "--formula", formula])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn is_brew_cask_installed(cask: &str) -> bool {
    Command::new("brew")
        .args(["list", "--cask", cask])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
