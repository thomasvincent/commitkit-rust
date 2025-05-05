# Contributing to CommitKit

Thank you for considering contributing to CommitKit! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and considerate of others when contributing to this project.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with the following information:

1. A clear, descriptive title
2. A detailed description of the issue
3. Steps to reproduce the bug
4. Expected and actual behavior
5. Any relevant logs or screenshots
6. Your environment (OS, Rust version, etc.)

### Suggesting Features

Feature suggestions are welcome! Please create an issue with:

1. A clear, descriptive title
2. A detailed description of the proposed feature
3. Any relevant examples or mockups
4. Why this feature would be useful to the project

### Pull Requests

1. Fork the repository
2. Create a new branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run the tests (`cargo test`)
5. Format your code (`cargo fmt`)
6. Check for linting issues (`cargo clippy`)
7. Commit your changes using conventional commit format:
   ```
   type(scope): subject
   
   body
   
   footer
   ```
8. Push to your branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## Development Setup

1. Install Rust (https://www.rust-lang.org/tools/install)
2. Clone the repository
3. Build the project: `cargo build`
4. Run tests: `cargo test`

## Project Structure

- `src/main.rs` - Entry point and CLI handling
- `src/lib.rs` - Library functionality and tests
- `src/config.rs` - Configuration handling
- `src/error.rs` - Error types
- `src/git.rs` - Git operations
- `src/prompt.rs` - User prompting

## Coding Guidelines

- Follow Rust's standard style guide (enforced by `rustfmt`)
- Use meaningful variable and function names
- Write descriptive comments
- Add tests for new functionality
- Keep functions small and focused

## License

By contributing, you agree that your contributions will be licensed under the project's MIT License.