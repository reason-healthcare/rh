# Contributing to the Rust Monorepo

Thank you for your interest in contributing! This document provides guidelines for contributing to this Rust monorepo.

## Development Setup

1. Ensure you have Rust 1.70 or later installed
2. Run the setup script: `./setup.sh`
3. Verify everything works: `cargo test`

## Project Structure

- `crates/`: Reusable library crates
- `apps/`: Executable applications
- `docs/`: Documentation and examples
- `tools/`: Development utilities

## Coding Standards

### Rust Code Style

We use the standard Rust formatting and linting tools:

- **Formatting**: `cargo fmt` (enforced in CI)
- **Linting**: `cargo clippy` (warnings treated as errors in CI)
- **Documentation**: All public APIs must be documented

### Commit Messages

Follow conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
- `feat(common): add new error handling utilities`
- `fix(cli): resolve config file parsing issue`
- `docs: update README with new examples`

## Adding New Packages

### Library Crates

1. Create in `crates/` directory: `cargo new crates/my-lib --lib`
2. Update `Cargo.toml` to use workspace dependencies
3. Add appropriate documentation and tests
4. Update the main README if it provides significant functionality

### Applications

1. Create in `apps/` directory: `cargo new apps/my-app --bin`
2. Update `Cargo.toml` to use workspace dependencies
3. Add CLI documentation and help text
4. Consider adding integration tests

## Testing

- Unit tests: `cargo test`
- Integration tests: Place in `tests/` directory within each crate
- Documentation tests: Ensure all code examples in docs compile and run

## Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes following the coding standards
4. Add tests for new functionality
5. Ensure all tests pass: `cargo test`
6. Ensure code is formatted: `cargo fmt`
7. Ensure no clippy warnings: `cargo clippy`
8. Update documentation as needed
9. Commit your changes with conventional commit messages
10. Push to your fork and create a Pull Request

## Release Process

1. Update version numbers in relevant `Cargo.toml` files
2. Update CHANGELOG.md
3. Create a release PR
4. After merge, tag the release: `git tag v0.x.y`
5. Push the tag: `git push origin v0.x.y`

## Questions?

Feel free to open an issue for questions or discussion!
