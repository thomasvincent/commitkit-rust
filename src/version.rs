//! Version information for the crate
//!
//! This module contains version information about the crate, including the version from
//! Cargo.toml and git information when available.

// Include the generated version info
include!(concat!(env!("OUT_DIR"), "/version_info.rs"));

/// Returns a string with the full version information
pub fn version_string() -> String {
    match (GIT_HASH, GIT_DATE) {
        (Some(hash), Some(date)) => format!("{} (git: {} - {})", VERSION, hash, date),
        (Some(hash), None) => format!("{} (git: {})", VERSION, hash),
        _ => VERSION.to_string(),
    }
}

/// Returns the semantic version (without git info)
pub fn semantic_version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_string() {
        let version = version_string();
        assert!(!version.is_empty());
        assert!(version.contains(VERSION));
    }

    #[test]
    fn test_semantic_version() {
        let version = semantic_version();
        assert_eq!(version, VERSION);
    }
}
