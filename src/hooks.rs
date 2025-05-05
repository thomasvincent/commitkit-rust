use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;

pub struct GitHookManager {
    repo_path: PathBuf,
}

impl GitHookManager {
    /// Create a new Git hook manager for the given repository
    pub fn new<P: AsRef<Path>>(repo_path: P) -> Self {
        Self {
            repo_path: PathBuf::from(repo_path.as_ref()),
        }
    }

    /// Get the path to the Git hooks directory
    pub fn hooks_dir(&self) -> PathBuf {
        self.repo_path.join(".git").join("hooks")
    }

    /// Check if a repository exists at the given path
    pub fn is_git_repo(&self) -> bool {
        self.repo_path.join(".git").exists()
    }

    /// Install the CommitKit prepare-commit-msg hook
    pub fn install_prepare_commit_msg_hook(&self) -> Result<()> {
        self.ensure_hook_directory()?;

        let hook_path = self.hooks_dir().join("prepare-commit-msg");
        let hook_content = r#"#!/bin/sh
# CommitKit prepare-commit-msg hook
#
# This hook is called by "git commit" with the name of the file that has the
# commit message, followed by the description of the commit message's source.

# If commitkit is installed and available, use it to prepare the commit message
if command -v commitkit > /dev/null 2>&1; then
    # Save the original commit message
    ORIG_MSG=$(cat "$1")
    
    # Run commitkit in prepare-msg mode
    # This will read any existing message and enhance it if needed
    commitkit --prepare-msg "$ORIG_MSG" > "$1"
fi
"#;

        fs::write(&hook_path, hook_content)
            .context("Failed to write prepare-commit-msg hook")?;

        // Make the hook executable
        let mut perms = fs::metadata(&hook_path)?.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(&hook_path, perms)
            .context("Failed to set hook permissions")?;

        Ok(())
    }

    /// Install the CommitKit commit-msg hook for validation
    pub fn install_commit_msg_hook(&self) -> Result<()> {
        self.ensure_hook_directory()?;

        let hook_path = self.hooks_dir().join("commit-msg");
        let hook_content = r#"#!/bin/sh
# CommitKit commit-msg hook
#
# This hook is called by "git commit" with one argument, the name of the file
# that has the commit message. The hook should exit with non-zero status after
# modifying the message file if it wants to stop the commit.

# If commitkit is installed and available, use it to validate the commit message
if command -v commitkit > /dev/null 2>&1; then
    # Validate the commit message
    commitkit --validate "$1"
    exit $?
fi

# If commitkit is not available, allow the commit
exit 0
"#;

        fs::write(&hook_path, hook_content)
            .context("Failed to write commit-msg hook")?;

        // Make the hook executable
        let mut perms = fs::metadata(&hook_path)?.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(&hook_path, perms)
            .context("Failed to set hook permissions")?;

        Ok(())
    }

    /// Remove a git hook
    pub fn remove_hook(&self, hook_name: &str) -> Result<()> {
        let hook_path = self.hooks_dir().join(hook_name);
        
        if hook_path.exists() {
            fs::remove_file(&hook_path)
                .context(format!("Failed to remove {} hook", hook_name))?;
        }
        
        Ok(())
    }

    /// Ensure the hooks directory exists
    fn ensure_hook_directory(&self) -> Result<()> {
        let hooks_dir = self.hooks_dir();
        
        if !hooks_dir.exists() {
            fs::create_dir_all(&hooks_dir)
                .context("Failed to create hooks directory")?;
        }
        
        Ok(())
    }

    /// Check if a hook is installed
    pub fn is_hook_installed(&self, hook_name: &str) -> bool {
        let hook_path = self.hooks_dir().join(hook_name);
        hook_path.exists()
    }

    /// Find the root of the Git repository from any subdirectory
    pub fn find_repo_root(start_dir: &Path) -> Option<PathBuf> {
        let mut current = start_dir.to_path_buf();
        
        loop {
            if current.join(".git").exists() {
                return Some(current);
            }
            
            if !current.pop() {
                return None;
            }
        }
    }
}

pub struct CommitMessageValidator {
    min_subject_length: usize,
    max_subject_length: usize,
    required_types: Vec<String>,
    validate_scope: bool,
}

impl Default for CommitMessageValidator {
    fn default() -> Self {
        Self {
            min_subject_length: 10,
            max_subject_length: 72,
            required_types: vec![
                "feat".to_string(),
                "fix".to_string(),
                "docs".to_string(),
                "style".to_string(),
                "refactor".to_string(),
                "perf".to_string(),
                "test".to_string(),
                "build".to_string(),
                "ci".to_string(),
                "chore".to_string(),
                "revert".to_string(),
            ],
            validate_scope: false,
        }
    }
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidFormat,
    InvalidType,
    SubjectTooShort,
    SubjectTooLong,
    InvalidScope,
}

impl CommitMessageValidator {
    /// Create a new commit message validator
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the minimum subject length
    pub fn min_subject_length(mut self, length: usize) -> Self {
        self.min_subject_length = length;
        self
    }

    /// Set the maximum subject length
    pub fn max_subject_length(mut self, length: usize) -> Self {
        self.max_subject_length = length;
        self
    }

    /// Set the required commit types
    pub fn required_types(mut self, types: Vec<String>) -> Self {
        self.required_types = types;
        self
    }

    /// Enable or disable scope validation
    pub fn validate_scope(mut self, validate: bool) -> Self {
        self.validate_scope = validate;
        self
    }

    /// Validate a commit message
    pub fn validate(&self, message: &str) -> Result<(), ValidationError> {
        // Get the first line (subject line)
        let subject = message.lines().next().unwrap_or("");

        // Check if it follows the conventional commit format
        let re = regex::Regex::new(r"^(\w+)(\(([\w-]+)\))?: (.+)$").unwrap();
        let captures = re.captures(subject).ok_or(ValidationError::InvalidFormat)?;

        // Validate commit type
        let commit_type = captures.get(1).unwrap().as_str();
        if !self.required_types.iter().any(|t| t == commit_type) {
            return Err(ValidationError::InvalidType);
        }

        // Validate scope if required
        if self.validate_scope {
            if captures.get(3).is_none() {
                return Err(ValidationError::InvalidScope);
            }
        }

        // Validate subject text
        let subject_text = captures.get(4).unwrap().as_str();
        if subject_text.len() < self.min_subject_length {
            return Err(ValidationError::SubjectTooShort);
        }

        if subject_text.len() > self.max_subject_length {
            return Err(ValidationError::SubjectTooLong);
        }

        Ok(())
    }

    /// Validate a commit message file
    pub fn validate_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ValidationError> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|_| ValidationError::InvalidFormat)?;

        self.validate(&content)
    }
}