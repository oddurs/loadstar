//! GitHub CLI setup — SSH keys, git config, gh auth
//! Runs as part of the installation flow after packages are installed.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc;

use crate::executor::InstallMessage;
use crate::system::SystemInfo;
use crate::wizard::WizardState;

/// Run the full GitHub/git setup sequence
pub fn setup_github(wizard: &WizardState, system: &SystemInfo, tx: &mpsc::Sender<InstallMessage>) {
    let _ = tx.send(InstallMessage::PhaseStart {
        phase: "Git & GitHub Setup".to_string(),
    });

    // Step 1: Configure git identity
    configure_git_identity(wizard, tx);

    // Step 2: SSH key generation
    let ssh_key_path = generate_ssh_key(wizard, system, tx);

    // Step 3: Start ssh-agent and add key
    if let Some(ref key_path) = ssh_key_path {
        add_key_to_agent(key_path, tx);
    }

    // Step 4: gh CLI auth (if gh is installed/selected)
    if wizard.selected_apps.contains("gh") {
        setup_gh_cli(ssh_key_path.as_deref(), tx);
    }

    // Step 5: GPG signing (if opted in)
    if wizard.setup_git_signing {
        setup_gpg_signing(wizard, tx);
    }

    let _ = tx.send(InstallMessage::Log(
        "[GIT] Git & GitHub setup complete".to_string(),
    ));
}

// ─── Git identity ────────────────────────────────────────────────────

fn configure_git_identity(wizard: &WizardState, tx: &mpsc::Sender<InstallMessage>) {
    let _ = tx.send(InstallMessage::Log(
        "[GIT] Configuring git identity...".to_string(),
    ));

    if !wizard.identity.name.is_empty() {
        let _ = run_git_config("user.name", &wizard.identity.name, tx);
    }

    if !wizard.identity.email.is_empty() {
        let _ = run_git_config("user.email", &wizard.identity.email, tx);
    }

    // Sane defaults
    let _ = run_git_config("init.defaultBranch", "main", tx);
    let _ = run_git_config("push.autoSetupRemote", "true", tx);
    let _ = run_git_config("pull.rebase", "true", tx);
    let _ = run_git_config("fetch.prune", "true", tx);
    let _ = run_git_config("rebase.autoStash", "true", tx);

    // Delta as pager if selected
    if wizard.selected_apps.contains("delta") {
        let _ = run_git_config("core.pager", "delta", tx);
        let _ = run_git_config("interactive.diffFilter", "delta --color-only", tx);
        let _ = run_git_config("delta.navigate", "true", tx);
        let _ = run_git_config("delta.line-numbers", "true", tx);
    }

    // SSH for GitHub URLs
    let _ = run_git_config("url.git@github.com:.insteadOf", "https://github.com/", tx);

    let _ = tx.send(InstallMessage::Log(
        "[GIT] Git identity configured".to_string(),
    ));
}

fn run_git_config(key: &str, value: &str, tx: &mpsc::Sender<InstallMessage>) -> Result<(), String> {
    let output = Command::new("git")
        .args(["config", "--global", key, value])
        .output()
        .map_err(|e| format!("git config failed: {}", e))?;

    if output.status.success() {
        let _ = tx.send(InstallMessage::Log(format!(
            "  git config --global {} = {}",
            key, value
        )));
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let err = format!("git config {} failed: {}", key, stderr.trim());
        let _ = tx.send(InstallMessage::Log(format!("  [WARN] {}", err)));
        Err(err)
    }
}

// ─── SSH key generation ──────────────────────────────────────────────

fn generate_ssh_key(
    wizard: &WizardState,
    system: &SystemInfo,
    tx: &mpsc::Sender<InstallMessage>,
) -> Option<PathBuf> {
    let ssh_dir = system.home_dir.join(".ssh");
    let key_path = ssh_dir.join("id_ed25519");

    // Check for existing keys
    if key_path.exists() {
        let _ = tx.send(InstallMessage::Log(
            "[SSH] Existing SSH key found — skipping generation".to_string(),
        ));

        // Show the public key
        show_public_key(&key_path, tx);
        return Some(key_path);
    }

    // Also check for RSA keys
    let rsa_path = ssh_dir.join("id_rsa");
    if rsa_path.exists() {
        let _ = tx.send(InstallMessage::Log(
            "[SSH] Existing RSA key found at ~/.ssh/id_rsa — skipping generation".to_string(),
        ));
        show_public_key(&rsa_path, tx);
        return Some(rsa_path);
    }

    let _ = tx.send(InstallMessage::Log(
        "[SSH] Generating ed25519 SSH key...".to_string(),
    ));

    // Ensure .ssh directory exists with correct permissions
    if let Err(e) = std::fs::create_dir_all(&ssh_dir) {
        let _ = tx.send(InstallMessage::Log(format!(
            "  [ERROR] Failed to create ~/.ssh: {}",
            e
        )));
        return None;
    }

    // Set permissions on .ssh directory (700)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&ssh_dir, std::fs::Permissions::from_mode(0o700));
    }

    // Build comment for the key
    let comment = if !wizard.identity.email.is_empty() {
        wizard.identity.email.clone()
    } else {
        format!("{}@loadstar", whoami::username())
    };

    // Generate the key
    let output = Command::new("ssh-keygen")
        .args([
            "-t",
            "ed25519",
            "-C",
            &comment,
            "-f",
            &key_path.to_string_lossy(),
            "-N",
            "", // empty passphrase (user can add one later)
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let _ = tx.send(InstallMessage::Log(
                "[SSH] SSH key generated successfully".to_string(),
            ));

            // Set correct permissions on the private key (600)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o600));
            }

            show_public_key(&key_path, tx);
            Some(key_path)
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let _ = tx.send(InstallMessage::Log(format!(
                "  [ERROR] ssh-keygen failed: {}",
                stderr.trim()
            )));
            None
        }
        Err(e) => {
            let _ = tx.send(InstallMessage::Log(format!(
                "  [ERROR] Could not run ssh-keygen: {}",
                e
            )));
            None
        }
    }
}

fn show_public_key(private_key_path: &Path, tx: &mpsc::Sender<InstallMessage>) {
    let pub_path = private_key_path.with_extension("pub");
    if let Ok(pub_key) = std::fs::read_to_string(&pub_path) {
        let trimmed = pub_key.trim();
        // Show truncated key for the log
        if trimmed.len() > 60 {
            let _ = tx.send(InstallMessage::Log(format!(
                "  Public key: {}...{}",
                &trimmed[..30],
                &trimmed[trimmed.len() - 20..]
            )));
        } else {
            let _ = tx.send(InstallMessage::Log(format!("  Public key: {}", trimmed)));
        }
    }
}

// ─── ssh-agent ───────────────────────────────────────────────────────

fn add_key_to_agent(key_path: &Path, tx: &mpsc::Sender<InstallMessage>) {
    let _ = tx.send(InstallMessage::Log(
        "[SSH] Adding key to ssh-agent...".to_string(),
    ));

    // Start ssh-agent if not running
    let _ = Command::new("ssh-agent").arg("-s").output();

    // On macOS, use --apple-use-keychain to store in Keychain
    let result = if cfg!(target_os = "macos") {
        // Also write SSH config to use Keychain
        write_ssh_config(key_path, tx);

        Command::new("ssh-add")
            .args(["--apple-use-keychain", &key_path.to_string_lossy()])
            .output()
    } else {
        Command::new("ssh-add")
            .arg(key_path.to_string_lossy().to_string())
            .output()
    };

    match result {
        Ok(out) if out.status.success() => {
            let _ = tx.send(InstallMessage::Log("  Key added to ssh-agent".to_string()));
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let _ = tx.send(InstallMessage::Log(format!(
                "  [WARN] ssh-add: {}",
                stderr.trim()
            )));
        }
        Err(e) => {
            let _ = tx.send(InstallMessage::Log(format!(
                "  [WARN] Could not run ssh-add: {}",
                e
            )));
        }
    }
}

/// Write ~/.ssh/config for macOS Keychain integration
fn write_ssh_config(key_path: &Path, tx: &mpsc::Sender<InstallMessage>) {
    let ssh_config_path = key_path
        .parent()
        .map(|p| p.join("config"))
        .unwrap_or_else(|| PathBuf::from("~/.ssh/config"));

    // Don't overwrite existing config — append if needed
    let existing = std::fs::read_to_string(&ssh_config_path).unwrap_or_default();

    if existing.contains("AddKeysToAgent") {
        return; // Already configured
    }

    let block = format!(
        r#"
# Added by LOAD"*",8,1
Host github.com
    AddKeysToAgent yes
    UseKeychain yes
    IdentityFile {}

Host *
    AddKeysToAgent yes
    UseKeychain yes
"#,
        key_path.display()
    );

    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&ssh_config_path)
    {
        Ok(mut file) => {
            use std::io::Write;
            let _ = file.write_all(block.as_bytes());

            // Set permissions (644)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = std::fs::set_permissions(
                    &ssh_config_path,
                    std::fs::Permissions::from_mode(0o644),
                );
            }

            let _ = tx.send(InstallMessage::Log(
                "  Wrote ~/.ssh/config (Keychain integration)".to_string(),
            ));
        }
        Err(e) => {
            let _ = tx.send(InstallMessage::Log(format!(
                "  [WARN] Could not write ~/.ssh/config: {}",
                e
            )));
        }
    }
}

// ─── GitHub CLI ──────────────────────────────────────────────────────

fn setup_gh_cli(ssh_key_path: Option<&Path>, tx: &mpsc::Sender<InstallMessage>) {
    // Check if gh is available
    let gh_available = Command::new("gh")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !gh_available {
        let _ = tx.send(InstallMessage::Log(
            "[GH] GitHub CLI not found — skipping auth setup".to_string(),
        ));
        return;
    }

    // Check if already authenticated
    let auth_status = Command::new("gh")
        .args(["auth", "status"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if auth_status {
        let _ = tx.send(InstallMessage::Log(
            "[GH] Already authenticated with GitHub CLI".to_string(),
        ));
    } else {
        // We can't run interactive auth in a background thread,
        // so we log instructions for the user to run after install
        let _ = tx.send(InstallMessage::Log(
            "[GH] GitHub CLI auth requires interactive login".to_string(),
        ));
        let _ = tx.send(InstallMessage::Log(
            "  Run after install: gh auth login --protocol ssh --web".to_string(),
        ));
    }

    // Upload SSH key if we generated one and gh is authenticated
    if auth_status {
        if let Some(key_path) = ssh_key_path {
            upload_ssh_key(key_path, tx);
        }

        // Set gh as credential helper
        let _ = Command::new("gh").args(["auth", "setup-git"]).output();
        let _ = tx.send(InstallMessage::Log(
            "  Set gh as git credential helper".to_string(),
        ));
    }
}

fn upload_ssh_key(key_path: &Path, tx: &mpsc::Sender<InstallMessage>) {
    let pub_path = key_path.with_extension("pub");
    if !pub_path.exists() {
        return;
    }

    // Check if key is already on GitHub
    let existing = Command::new("gh")
        .args(["ssh-key", "list"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    // Read the public key to check if it's already uploaded
    let pub_key = std::fs::read_to_string(&pub_path).unwrap_or_default();
    let key_fingerprint = pub_key.split_whitespace().nth(1).unwrap_or("");

    if !key_fingerprint.is_empty() && existing.contains(key_fingerprint) {
        let _ = tx.send(InstallMessage::Log(
            "  SSH key already on GitHub — skipping upload".to_string(),
        ));
        return;
    }

    let _ = tx.send(InstallMessage::Log(
        "[GH] Uploading SSH key to GitHub...".to_string(),
    ));

    let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "c64".to_string());
    let title = format!("LOAD*,8,1 ({})", hostname);

    let output = Command::new("gh")
        .args([
            "ssh-key",
            "add",
            &pub_path.to_string_lossy(),
            "--title",
            &title,
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let _ = tx.send(InstallMessage::Log(format!(
                "  SSH key uploaded as '{}'",
                title
            )));
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let _ = tx.send(InstallMessage::Log(format!(
                "  [WARN] Failed to upload SSH key: {}",
                stderr.trim()
            )));
        }
        Err(e) => {
            let _ = tx.send(InstallMessage::Log(format!(
                "  [WARN] Could not run gh ssh-key add: {}",
                e
            )));
        }
    }
}

// ─── GPG signing ─────────────────────────────────────────────────────

fn setup_gpg_signing(wizard: &WizardState, tx: &mpsc::Sender<InstallMessage>) {
    // Check if gpg is available
    let gpg_available = Command::new("gpg")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !gpg_available {
        let _ = tx.send(InstallMessage::Log(
            "[GPG] GnuPG not found — skipping signing setup".to_string(),
        ));
        let _ = tx.send(InstallMessage::Log(
            "  Install gnupg and re-run, or set up manually".to_string(),
        ));
        return;
    }

    let _ = tx.send(InstallMessage::Log(
        "[GPG] Checking for existing GPG keys...".to_string(),
    ));

    // Check for existing keys matching the email
    let existing = Command::new("gpg")
        .args([
            "--list-secret-keys",
            "--keyid-format=long",
            &wizard.identity.email,
        ])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    if !existing.trim().is_empty() {
        let _ = tx.send(InstallMessage::Log(
            "  Existing GPG key found — configuring git to use it".to_string(),
        ));

        // Extract key ID from output
        if let Some(key_id) = extract_gpg_key_id(&existing) {
            let _ = run_git_config("user.signingkey", &key_id, tx);
            let _ = run_git_config("commit.gpgsign", "true", tx);
            let _ = run_git_config("tag.gpgsign", "true", tx);
            let _ = tx.send(InstallMessage::Log(format!(
                "  Git configured to sign with key {}",
                key_id
            )));
        }
    } else {
        // GPG key generation is interactive and slow — give instructions
        let _ = tx.send(InstallMessage::Log(
            "[GPG] No GPG key found for this email".to_string(),
        ));
        let _ = tx.send(InstallMessage::Log(
            "  Run after install: gpg --full-generate-key".to_string(),
        ));
        let _ = tx.send(InstallMessage::Log(
            "  Then: git config --global user.signingkey <KEY_ID>".to_string(),
        ));
        let _ = tx.send(InstallMessage::Log(
            "  Then: git config --global commit.gpgsign true".to_string(),
        ));
    }
}

/// Extract the key ID from gpg --list-secret-keys output
fn extract_gpg_key_id(output: &str) -> Option<String> {
    // GPG output format:
    //   sec   ed25519/ABCDEF1234567890 2024-01-01 [SC]
    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("sec") {
            // Extract the part after the /
            if let Some(slash_pos) = trimmed.find('/') {
                let after_slash = &trimmed[slash_pos + 1..];
                // Key ID is the next space-delimited token
                if let Some(key_id) = after_slash.split_whitespace().next() {
                    return Some(key_id.to_string());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_gpg_key_id_ed25519() {
        let output = r#"sec   ed25519/ABCDEF1234567890 2024-01-01 [SC]
      AABBCCDD11223344AABBCCDD11223344ABCDEF1234567890
uid           [ultimate] Test User <test@example.com>
ssb   cv25519/1234567890ABCDEF 2024-01-01 [E]"#;

        let key_id = extract_gpg_key_id(output);
        assert_eq!(key_id, Some("ABCDEF1234567890".to_string()));
    }

    #[test]
    fn extract_gpg_key_id_rsa() {
        let output = r#"sec   rsa4096/DEADBEEF12345678 2023-06-15 [SC] [expires: 2025-06-15]
      AABBCCDDAABBCCDDAABBCCDDAABBCCDDDEADBEEF12345678
uid           [ultimate] Another User <user@example.com>
ssb   rsa4096/FEDCBA9876543210 2023-06-15 [E] [expires: 2025-06-15]"#;

        let key_id = extract_gpg_key_id(output);
        assert_eq!(key_id, Some("DEADBEEF12345678".to_string()));
    }

    #[test]
    fn extract_gpg_key_id_empty_output() {
        assert_eq!(extract_gpg_key_id(""), None);
        assert_eq!(extract_gpg_key_id("no keys here"), None);
    }
}
