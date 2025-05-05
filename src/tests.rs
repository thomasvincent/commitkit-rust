#![cfg(test)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Test helper for creating a temporary config file
pub fn create_test_config(content: &str) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join(".commitkit.toml");
    
    fs::write(&config_path, content).expect("Failed to write test config");
    
    (temp_dir, config_path)
}

/// Test helper for creating a temporary git repository
pub fn setup_test_repo() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Initialize git repo
    let status = std::process::Command::new("git")
        .args(&["init"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to run git init");
    
    assert!(status.success(), "Failed to initialize git repository");
    
    // Configure git for tests
    std::process::Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user name");
    
    std::process::Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user email");
    
    temp_dir
}

/// Run function in a temporary directory
pub fn with_temp_dir<F, T>(f: F) -> T
where
    F: FnOnce(&Path) -> T,
{
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let old_dir = env::current_dir().expect("Failed to get current directory");
    
    env::set_current_dir(temp_dir.path()).expect("Failed to change to temp dir");
    let result = f(temp_dir.path());
    env::set_current_dir(old_dir).expect("Failed to restore directory");
    
    result
}

/// Create a file with content in the specified directory
pub fn create_test_file(dir: &Path, name: &str, content: &str) -> PathBuf {
    let file_path = dir.join(name);
    fs::write(&file_path, content).expect("Failed to write test file");
    file_path
}