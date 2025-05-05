# Contributing to CommitKit

Thank you for considering contributing to CommitKit! This document provides guidelines and instructions for contributing to make the process smooth for everyone.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Please be respectful and considerate of others.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork**:
   ```sh
   git clone https://github.com/your-username/commitkit-rust.git
   cd commitkit-rust
   ```
3. **Set up the development environment**:
   ```sh
   cargo build
   ```
4. **Create a branch** for your changes:
   ```sh
   git checkout -b feature/amazing-feature
   ```

## Development Workflow

### Running Tests

Run the tests to make sure everything works:

```sh
cargo test
```

### Code Formatting

Run rustfmt to ensure your code is properly formatted:

```sh
cargo fmt
```

### Linting

Check for potential issues with clippy:

```sh
cargo clippy -- -D warnings
```

## Conventional Commits

This project uses [Conventional Commits](https://www.conventionalcommits.org/) for commit messages. The format is:

```
<type>([optional scope]): <description>

[optional body]

[optional footer(s)]
```

Types include:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Changes that do not affect code meaning
- `refactor`: Code changes that neither fix a bug nor add a feature
- `perf`: Performance improvements
- `test`: Adding or fixing tests
- `build`: Changes to the build system or dependencies
- `ci`: Changes to CI configuration
- `chore`: Other changes that don't modify source files
- `revert`: Revert a previous commit

Examples:
```
feat(cli): add support for custom templates
fix(parser): handle empty commit messages
docs: update installation instructions
```

## Pull Request Process

1. **Update the README.md** if necessary with details of changes to the interface.
2. **Add tests** for new functionality.
3. **Update the CHANGELOG.md** with details of your changes under the "Unreleased" section.
4. **Submit your pull request** against the `main` branch.
5. The pull request must **pass all CI checks**.
6. Your pull request will be reviewed by maintainers who may suggest changes.

## Release Process

Releases are managed by maintainers using the following process:

1. Determine the appropriate version bump based on [Semantic Versioning](https://semver.org/).
2. Use the "Semantic Versioning" GitHub Action workflow to create a new version.
3. The workflow will automatically:
   - Update the version in Cargo.toml
   - Update CHANGELOG.md
   - Commit and tag the changes
   - Push the changes to GitHub
   - Trigger the release workflow

## Documentation

Good documentation is crucial. When adding new features, please add appropriate documentation:

- Code comments explaining "why" not "what"
- Update README.md with usage instructions if applicable
- Update CHANGELOG.md with a description of your changes

## Community

For questions or discussions, please use GitHub Discussions.

---

Thank you for contributing to CommitKit!