use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Add build-time information
    println!(
        "cargo:rustc-env=BUILD_TIME={}",
        chrono::Utc::now().to_rfc3339()
    );
    println!("cargo:rustc-env=GIT_HASH={}", get_git_hash());

    // Ensure required directories exist
    let out_dir = env::var("OUT_DIR").unwrap();
    let assets_dir = Path::new(&out_dir).join("assets");
    fs::create_dir_all(&assets_dir).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/HEAD");
}

fn get_git_hash() -> String {
    std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
