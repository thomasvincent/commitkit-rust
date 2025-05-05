# CommitKit

[![Rust](https://img.shields.io/badge/Rust-1.x-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

CommitKit is a command-line tool that helps developers create well-structured and consistent commit messages following the [Conventional Commits](https://www.conventionalcommits.org/) specification. It provides an interactive interface to guide users through the process of constructing a commit message with a type, optional scope, subject, body, and footer.

## Features

- Interactive selection of commit types and scopes
- Customizable configuration via `.commitkit.toml`
- Enforcement of maximum subject length
- Support for multi-line commit bodies
- Automatic validation of input
- Optional signed-off commits
- Cross-platform compatibility

## Installation

### From crates.io

```shell
cargo install commitkit
```

### From binaries

Download the latest binary from the [Releases](https://github.com/thomasvincent/commitkit-rust/releases) page.

#### Linux/macOS

```shell
# Make it executable
chmod +x commitkit
# Move to a directory in your PATH
sudo mv commitkit /usr/local/bin/
```

#### Windows

Download the `.exe` file and add it to your PATH.

### From source

```shell
git clone https://github.com/thomasvincent/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Usage

Navigate to your git repository and run:

```shell
commitkit
```

### Command-line options

```
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run     Skip git commit and print the message to stdout
    -v, --verbose     Show verbose output
    -c, --config      Path to config file (default: .commitkit.toml in current or home directory)
    -h, --help        Print help information
    -V, --version     Print version information
```

### Interactive process

The tool will guide you through the process of creating a conventional commit message:

1. Select a commit type (feat, fix, docs, etc.)
2. Select a scope (optional)
3. Enter the commit subject
4. Enter the commit body (optional)
5. Enter the commit footer (optional)

After completing these steps, CommitKit will run `git commit` with your constructed message.

### Examples

```shell
# Basic usage
commitkit

# Dry run - display the resulting commit message without committing
commitkit --dry-run

# Use a custom config file
commitkit --config /path/to/custom-config.toml

# Verbose output
commitkit --verbose
```

## Configuration

CommitKit looks for a configuration file named `.commitkit.toml` in the following locations (in order):

1. Current directory
2. User's home directory

If no configuration file is found, a default one will be generated in your home directory.

Example configuration:

```toml
# CommitKit Configuration

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
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes using conventional commits
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request