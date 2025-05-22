use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommitKitError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse config: {0}")]
    ConfigParse(#[from] toml::de::Error),

    #[error("Invalid selection: {0}")]
    InvalidSelection(String),

    #[error("Git error: {0}")]
    GitError(String),

    #[error("Input error: {0}")]
    InputError(String),
}

pub type Result<T> = std::result::Result<T, CommitKitError>;
