# Google Rust Style Guide

This project follows the Google Rust Style Guide principles as outlined in [Google's Comprehensive Rust](https://google.github.io/comprehensive-rust/).

## Key Principles

### Code Formatting
- **Line Length**: 100 characters maximum
- **Indentation**: 4 spaces (no tabs)
- **Function Parameters**: Vertical layout for multiple parameters

### Tools
- **rustfmt**: Automatic code formatting (`cargo fmt`)
- **clippy**: Linting and best practices (`cargo clippy`)

### Running Style Checks

```bash
# Format code
cargo fmt

# Check formatting without applying changes
cargo fmt -- --check

# Run linter
cargo clippy

# Run all checks
cargo fmt -- --check && cargo clippy
```

### Configuration Files
- `.rustfmt.toml`: Rustfmt configuration
- `.editorconfig`: Editor settings
- `clippy.toml`: Clippy configuration (if needed)

## Integration

Style checks are integrated into:
- Pre-commit hooks
- CI/CD pipeline
- IDE/editor settings via `.editorconfig`
