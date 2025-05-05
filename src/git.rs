use anyhow::{Context, Result};
use std::io::{self, Write};
use std::process::{Command, Stdio};

/// Runs the git commit command with the provided message
pub fn run_git_commit(message: &str, sign_off: bool) -> Result<()> {
    let mut command = Command::new("git");
    command
        .arg("commit")
        .arg("-m")
        .arg(message)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    
    if sign_off {
        command.arg("-s");
    }

    let output = command
        .output()
        .context("Failed to execute git commit command")?;
    
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Git commit failed with status: {}",
            output.status
        ))
    }
}

/// Checks if the current directory is in a git repository
pub fn is_git_repo() -> Result<bool> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .context("Failed to execute git command")?;
    
    Ok(output.status.success())
}

/// Checks if there are staged changes ready to commit
pub fn has_staged_changes() -> Result<bool> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .status()
        .context("Failed to check for staged changes")?;
    
    // Exit code 1 means there are differences (staged changes),
    // Exit code 0 means no differences (no staged changes)
    Ok(!output.success())
}