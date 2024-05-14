# # CommitKit-rust

[![Rust](https://img.shields.io/badge/Rust-1.x-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

CommitKit is a command-line tool that helps developers create well-structured and consistent commit messages following the [Conventional Commits](https://www.conventionalcommits.org/) specification. It provides an interactive interface to guide users through the process of constructing a commit message with a prefix, optional scope, subject, body, and footer.

## Features

- Customizable prefixes and scopes through a configuration file (`.commitkit.toml`)
- Enforcement of a maximum length for the commit subject
- Support for multi-line commit body
- Optional footer for additional information
- Integration with git to execute the commit command
- Cross-platform compatibility (Windows, macOS, Linux)

## Installation

To install CommitKit, you need to have Rust installed on your system. If you don't have Rust installed, you can download and install it from the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Once Rust is installed, you can install CommitKit using Cargo, the Rust package manager:

```shell
cargo install commitkit
This will download and compile CommitKit, and install it in your system's PATH.
# Usage
To use CommitKit, navigate to your git repository in the terminal and run the following command:
### shell
