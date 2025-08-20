# Justfile for common development tasks
# Install just: https://github.com/casey/just

# Default recipe - show available commands
default:
    @just --list

# Run all tests
test:
    cargo test --workspace --all-features

# Run tests with nextest if available
test-fast:
    #!/usr/bin/env bash
    if command -v cargo-nextest &> /dev/null; then
        cargo nextest run --all-features
    else
        cargo test --workspace --all-features
    fi

# Build all packages
build:
    cargo build --workspace --all-targets --all-features

# Check code formatting
fmt-check:
    cargo fmt --all -- --check

# Format all code
fmt:
    cargo fmt --all

# Run clippy lints
lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run security audit
audit:
    cargo audit

# Clean build artifacts
clean:
    cargo clean

# Check everything (format, lint, test, audit)
check: fmt-check lint test audit

# Watch for changes and run tests
watch:
    cargo watch -x test

# Watch for changes and run clippy
watch-lint:
    cargo watch -x clippy

# Create a new library crate
new-lib name:
    cd crates && cargo new {{name}} --lib
    @echo "Don't forget to update the Cargo.toml to use workspace dependencies!"

# Create a new binary crate
new-bin name:
    cd apps && cargo new {{name}} --bin
    @echo "Don't forget to update the Cargo.toml to use workspace dependencies!"

# Build release versions
build-release:
    cargo build --release --all-targets --all-features

# Run the example CLI
run-example *args:
    cargo run -p example-cli -- {{args}}

# Generate documentation
docs:
    cargo doc --all-features --no-deps --open

# Update all dependencies
update:
    cargo update
