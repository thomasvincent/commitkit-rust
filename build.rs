use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Only run if not on CI
    if env::var("CI").is_ok() {
        return;
    }

    // Tell cargo to rerun this build script if Cargo.toml changes
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");

    // Collect git information if available
    let git_hash = get_git_hash();
    let git_date = get_git_date();

    // Write the version information to a generated file
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version_info.rs");

    let content = format!(
        r#"
/// The version of the crate from Cargo.toml
pub const VERSION: &str = "{}";
/// The git hash of the build, if available
pub const GIT_HASH: Option<&str> = {};
/// The date of the git commit, if available
pub const GIT_DATE: Option<&str> = {};
"#,
        env::var("CARGO_PKG_VERSION").unwrap(),
        git_hash
            .as_ref()
            .map(|h| format!("Some(\"{}\")", h))
            .unwrap_or_else(|| "None".to_string()),
        git_date
            .as_ref()
            .map(|d| format!("Some(\"{}\")", d))
            .unwrap_or_else(|| "None".to_string())
    );

    fs::write(dest_path, content).unwrap();
}

fn get_git_hash() -> Option<String> {
    Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

fn get_git_date() -> Option<String> {
    Command::new("git")
        .args(&["log", "-1", "--format=%cd", "--date=short"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}