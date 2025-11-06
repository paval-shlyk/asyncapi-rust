# Contributing to asyncapi-rust

Thank you for your interest in contributing! This document provides guidelines for contributing to asyncapi-rust.

## Code of Conduct

This project adheres to the Rust Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to mark@lilback.com.

## How to Contribute

### Reporting Bugs

- Check existing issues to avoid duplicates
- Use the bug report template
- Include minimal reproducible example
- Specify Rust version and OS

### Suggesting Features

- Check existing issues and discussions
- Explain the use case clearly
- Consider implementation complexity
- Be open to feedback

### Pull Requests

1. **Fork and clone** the repository
2. **Create a branch** from `main`: `git checkout -b feature/my-feature`
3. **Make your changes** with clear commit messages
4. **Add tests** for new functionality
5. **Run tests**: `cargo test --all-features`
6. **Run clippy**: `cargo clippy -- -D warnings`
7. **Format code**: `cargo fmt`
8. **Update documentation** if needed
9. **Submit PR** with description of changes

### Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/asyncapi-rust.git
cd asyncapi-rust

# Build the project
cargo build

# Run tests
cargo test --all-features

# Run clippy
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### Testing

- Unit tests in each crate: `cargo test -p asyncapi-rust-codegen`
- Integration tests: `cargo test --test integration_*`
- All tests must pass before merge

### Documentation

- Add doc comments for public APIs
- Use examples in doc comments
- Update README if adding features
- Follow Rust documentation guidelines

### Code Style

- Follow Rust API Guidelines
- Use `cargo fmt` (enforced in CI)
- Pass `cargo clippy` with no warnings
- Keep commits focused and atomic

## License

By contributing, you agree that your contributions will be dual-licensed under MIT OR Apache-2.0, matching the project license.

## Questions?

Feel free to open an issue for questions or reach out to mark@lilback.com.
