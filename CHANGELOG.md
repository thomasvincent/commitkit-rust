# Changelog

All notable changes to CommitKit will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## 0.1.0 (2024-05-05)

### Added
- **Added** (core): Initial version with basic conventional commit functionality
- **Added** (ui): Interactive prompts for commit components
- **Added**: Emoji support for commit messages
- **Added**: Template support for automated commits
- **Added**: Changelog generation and maintenance
- **Added**: Git hooks for commit validation and preparation
- **Added**: Commit statistics and analysis
- **Added**: Support for custom configurations via .commitkit.toml

### Changed
- **Changed**: Refactored to use proper Rust project structure
- **Changed**: Improved error handling with anyhow and thiserror

### Fixed
- **Fixed**: Proper handling of empty scopes