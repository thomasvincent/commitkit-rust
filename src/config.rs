use anyhow::{Context, Result};
use home::home_dir;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub sign_off_commits: bool,
    pub prefixes: Vec<Prefix>,
    pub scopes: Vec<String>,
    pub max_subject_len: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Prefix {
    pub title: String,
    pub description: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::find_config_file();
        
        let config_str = match fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(_) => {
                let default_config = Self::default_config_str();
                // Optionally save the default config to user's home directory
                if let Some(home) = home_dir() {
                    let home_config = home.join(".commitkit.toml");
                    if !home_config.exists() {
                        let _ = fs::write(home_config, &default_config);
                    }
                }
                default_config
            },
        };

        toml::from_str(&config_str)
            .with_context(|| format!("Failed to parse config from {:?}", config_path))
    }

    fn find_config_file() -> PathBuf {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        
        let config_paths = [
            current_dir.join(".commitkit.toml"),
            Self::home_config_path(),
        ];

        config_paths
            .iter()
            .find(|path| path.exists())
            .unwrap_or(&config_paths[0])
            .to_path_buf()
    }

    fn home_config_path() -> PathBuf {
        home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".commitkit.toml")
    }

    fn default_config_str() -> String {
        r#"# CommitKit Configuration

# Whether to add a Signed-off-by line to commits (-s flag)
sign_off_commits = false

# Commit message prefixes following Conventional Commits specification
prefixes = [
    { title = "feat", description = "A new feature" },
    { title = "fix", description = "A bug fix" },
    { title = "docs", description = "Documentation changes" },
    { title = "style", description = "Changes that do not affect code meaning" },
    { title = "refactor", description = "Code change that neither fixes a bug nor adds a feature" },
    { title = "perf", description = "Code change that improves performance" },
    { title = "test", description = "Adding missing tests or correcting existing tests" },
    { title = "build", description = "Changes that affect the build system or external dependencies" },
    { title = "ci", description = "Changes to CI configuration files and scripts" },
    { title = "chore", description = "Other changes that don't modify src or test files" },
    { title = "revert", description = "Reverts a previous commit" }
]

# Optional scopes to categorize changes
scopes = [
    "core",
    "ui",
    "docs",
    "tests",
    "deps"
]

# Maximum length of the commit subject line
max_subject_len = 72
"#.to_string()
    }
}