//! System detection and pre-flight checks
//! Detects OS, architecture, available package managers, and validates
//! the environment before installation begins.

use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Detected operating system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Os {
    MacOS,
    Linux,
}

impl Os {
    pub fn name(&self) -> &'static str {
        match self {
            Os::MacOS => "macOS",
            Os::Linux => "Linux",
        }
    }
}

/// CPU architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    Aarch64,
    X86_64,
}

impl Arch {
    pub fn name(&self) -> &'static str {
        match self {
            Arch::Aarch64 => "arm64",
            Arch::X86_64 => "x86_64",
        }
    }
}

/// Available package managers on the system
#[derive(Debug, Clone)]
pub struct PackageManagers {
    pub homebrew: Option<PathBuf>,
    pub cargo: Option<PathBuf>,
    pub npm: Option<PathBuf>,
    pub pip: Option<PathBuf>,
    pub apt: Option<PathBuf>,
}

/// Linux distribution info (only populated on Linux)
#[derive(Debug, Clone)]
pub struct LinuxDistro {
    pub id: String,
    pub name: String,
}

/// Complete system information
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: Os,
    pub arch: Arch,
    pub hostname: String,
    pub shell: String,
    pub home_dir: PathBuf,
    pub config_dir: PathBuf,
    pub package_managers: PackageManagers,
    pub linux_distro: Option<LinuxDistro>,
}

/// Pre-flight check results (used in Phase 4 when pre-flight UI is added)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PreflightResult {
    pub checks: Vec<PreflightCheck>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PreflightCheck {
    pub name: String,
    pub status: CheckStatus,
    pub detail: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
}

impl SystemInfo {
    /// Detect the current system environment
    pub fn detect() -> anyhow::Result<Self> {
        let os = detect_os()?;
        let arch = detect_arch()?;
        let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string());
        let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());

        let home_dir = dirs_path("HOME").unwrap_or_else(|| PathBuf::from("/tmp"));
        let config_dir = env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home_dir.join(".config"));

        let package_managers = detect_package_managers();
        let linux_distro = if os == Os::Linux {
            detect_linux_distro()
        } else {
            None
        };

        Ok(Self {
            os,
            arch,
            hostname,
            shell,
            home_dir,
            config_dir,
            package_managers,
            linux_distro,
        })
    }

    /// Homebrew install prefix (differs by arch on macOS)
    pub fn brew_prefix(&self) -> &'static str {
        match (self.os, self.arch) {
            (Os::MacOS, Arch::Aarch64) => "/opt/homebrew",
            (Os::MacOS, Arch::X86_64) => "/usr/local",
            (Os::Linux, _) => "/home/linuxbrew/.linuxbrew",
        }
    }

    /// Whether Homebrew is available
    pub fn has_homebrew(&self) -> bool {
        self.package_managers.homebrew.is_some()
    }

    /// Whether this is Apple Silicon
    pub fn is_apple_silicon(&self) -> bool {
        self.os == Os::MacOS && self.arch == Arch::Aarch64
    }
}

#[allow(dead_code)]
impl PreflightResult {
    /// Run all pre-flight checks
    pub fn run(system: &SystemInfo) -> Self {
        let mut checks = Vec::new();

        // Internet connectivity
        checks.push(check_internet());

        // Homebrew
        checks.push(check_homebrew(system));

        // Disk space
        checks.push(check_disk_space());

        // Xcode CLT (macOS only)
        if system.os == Os::MacOS {
            checks.push(check_xcode_clt());
        }

        // Writable home directory
        checks.push(check_home_writable(system));

        // Git available
        checks.push(check_git());

        Self { checks }
    }

    /// Whether all checks passed (no failures)
    pub fn all_passed(&self) -> bool {
        !self.checks.iter().any(|c| c.status == CheckStatus::Fail)
    }

    /// Count of failures
    pub fn fail_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| c.status == CheckStatus::Fail)
            .count()
    }

    /// Count of warnings
    pub fn warn_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| c.status == CheckStatus::Warn)
            .count()
    }
}

// ─── Detection helpers ───────────────────────────────────────────────

fn detect_os() -> anyhow::Result<Os> {
    match env::consts::OS {
        "macos" => Ok(Os::MacOS),
        "linux" => Ok(Os::Linux),
        other => anyhow::bail!("Unsupported operating system: {}", other),
    }
}

fn detect_arch() -> anyhow::Result<Arch> {
    match env::consts::ARCH {
        "aarch64" => Ok(Arch::Aarch64),
        "x86_64" => Ok(Arch::X86_64),
        other => anyhow::bail!("Unsupported architecture: {}", other),
    }
}

fn dirs_path(var: &str) -> Option<PathBuf> {
    env::var(var).ok().map(PathBuf::from)
}

fn detect_package_managers() -> PackageManagers {
    PackageManagers {
        homebrew: which("brew"),
        cargo: which("cargo"),
        npm: which("npm"),
        pip: which("pip3").or_else(|| which("pip")),
        apt: which("apt"),
    }
}

fn detect_linux_distro() -> Option<LinuxDistro> {
    // Parse /etc/os-release
    let content = std::fs::read_to_string("/etc/os-release").ok()?;
    let mut id = String::new();
    let mut name = String::new();

    for line in content.lines() {
        if let Some(val) = line.strip_prefix("ID=") {
            id = val.trim_matches('"').to_string();
        } else if let Some(val) = line.strip_prefix("NAME=") {
            name = val.trim_matches('"').to_string();
        }
    }

    if id.is_empty() {
        None
    } else {
        Some(LinuxDistro { id, name })
    }
}

/// Find a command in PATH
fn which(cmd: &str) -> Option<PathBuf> {
    Command::new("which")
        .arg(cmd)
        .output()
        .ok()
        .and_then(|out| {
            if out.status.success() {
                let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if path.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(path))
                }
            } else {
                None
            }
        })
}

// ─── Pre-flight checks (used in Phase 4) ────────────────────────────

#[allow(dead_code)]
fn check_internet() -> PreflightCheck {
    // Quick DNS check - try to resolve github.com
    let result = Command::new("ping")
        .args(["-c", "1", "-W", "3", "github.com"])
        .output();

    match result {
        Ok(output) if output.status.success() => PreflightCheck {
            name: "Internet connectivity".to_string(),
            status: CheckStatus::Pass,
            detail: "Connected".to_string(),
        },
        _ => PreflightCheck {
            name: "Internet connectivity".to_string(),
            status: CheckStatus::Fail,
            detail: "Cannot reach github.com - check your connection".to_string(),
        },
    }
}

#[allow(dead_code)]
fn check_homebrew(system: &SystemInfo) -> PreflightCheck {
    if system.has_homebrew() {
        PreflightCheck {
            name: "Homebrew".to_string(),
            status: CheckStatus::Pass,
            detail: format!(
                "Found at {}",
                system
                    .package_managers
                    .homebrew
                    .as_deref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default()
            ),
        }
    } else {
        PreflightCheck {
            name: "Homebrew".to_string(),
            status: CheckStatus::Warn,
            detail: "Not installed - will be installed during setup".to_string(),
        }
    }
}

#[allow(dead_code)]
fn check_disk_space() -> PreflightCheck {
    // Use df to check available space on /
    let result = Command::new("df").args(["-g", "/"]).output();

    match result {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Parse df output - available space is typically 4th column
            let available_gb: u64 = stdout
                .lines()
                .nth(1)
                .and_then(|line| line.split_whitespace().nth(3))
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);

            if available_gb >= 10 {
                PreflightCheck {
                    name: "Disk space".to_string(),
                    status: CheckStatus::Pass,
                    detail: format!("{}GB available", available_gb),
                }
            } else if available_gb >= 5 {
                PreflightCheck {
                    name: "Disk space".to_string(),
                    status: CheckStatus::Warn,
                    detail: format!("Only {}GB available - may be tight", available_gb),
                }
            } else {
                PreflightCheck {
                    name: "Disk space".to_string(),
                    status: CheckStatus::Fail,
                    detail: format!("Only {}GB available - need at least 5GB", available_gb),
                }
            }
        }
        _ => PreflightCheck {
            name: "Disk space".to_string(),
            status: CheckStatus::Warn,
            detail: "Could not determine available disk space".to_string(),
        },
    }
}

#[allow(dead_code)]
fn check_xcode_clt() -> PreflightCheck {
    let result = Command::new("xcode-select").arg("-p").output();

    match result {
        Ok(output) if output.status.success() => PreflightCheck {
            name: "Xcode CLT".to_string(),
            status: CheckStatus::Pass,
            detail: "Installed".to_string(),
        },
        _ => PreflightCheck {
            name: "Xcode CLT".to_string(),
            status: CheckStatus::Warn,
            detail: "Not installed - will prompt during Homebrew install".to_string(),
        },
    }
}

#[allow(dead_code)]
fn check_home_writable(system: &SystemInfo) -> PreflightCheck {
    let test_file = system.home_dir.join(".load_write_test");
    match std::fs::write(&test_file, "test") {
        Ok(()) => {
            let _ = std::fs::remove_file(&test_file);
            PreflightCheck {
                name: "Home directory".to_string(),
                status: CheckStatus::Pass,
                detail: format!("{} is writable", system.home_dir.display()),
            }
        }
        Err(e) => PreflightCheck {
            name: "Home directory".to_string(),
            status: CheckStatus::Fail,
            detail: format!("{} is not writable: {}", system.home_dir.display(), e),
        },
    }
}

#[allow(dead_code)]
fn check_git() -> PreflightCheck {
    match which("git") {
        Some(path) => {
            // Get version
            let version = Command::new("git")
                .arg("--version")
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|| "unknown version".to_string());

            PreflightCheck {
                name: "Git".to_string(),
                status: CheckStatus::Pass,
                detail: format!("{} ({})", version, path.display()),
            }
        }
        None => PreflightCheck {
            name: "Git".to_string(),
            status: CheckStatus::Warn,
            detail: "Not installed - will be installed via Homebrew".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_system_info() {
        let info = SystemInfo::detect();
        assert!(info.is_ok(), "SystemInfo::detect() should succeed");

        let info = info.unwrap();
        assert!(!info.hostname.is_empty());
        assert!(!info.shell.is_empty());
        assert!(info.home_dir.exists());
    }

    #[test]
    fn detect_os_is_known() {
        let os = detect_os();
        assert!(os.is_ok());
        let os = os.unwrap();
        assert!(os == Os::MacOS || os == Os::Linux);
        assert!(!os.name().is_empty());
    }

    #[test]
    fn detect_arch_is_known() {
        let arch = detect_arch();
        assert!(arch.is_ok());
        let arch = arch.unwrap();
        assert!(arch == Arch::Aarch64 || arch == Arch::X86_64);
        assert!(!arch.name().is_empty());
    }

    #[test]
    fn brew_prefix_varies_by_platform() {
        let mut info = SystemInfo::detect().unwrap();

        info.os = Os::MacOS;
        info.arch = Arch::Aarch64;
        assert_eq!(info.brew_prefix(), "/opt/homebrew");

        info.os = Os::MacOS;
        info.arch = Arch::X86_64;
        assert_eq!(info.brew_prefix(), "/usr/local");

        info.os = Os::Linux;
        assert_eq!(info.brew_prefix(), "/home/linuxbrew/.linuxbrew");
    }

    #[test]
    fn which_finds_common_commands() {
        assert!(which("sh").is_some(), "sh should be findable");
        assert!(
            which("nonexistent_command_xyz").is_none(),
            "Nonexistent command should not be found"
        );
    }

    #[test]
    fn package_manager_detection_doesnt_panic() {
        let pm = detect_package_managers();
        // Just verify it doesn't panic — presence depends on the machine
        let _ = pm.homebrew;
        let _ = pm.cargo;
        let _ = pm.npm;
    }
}
