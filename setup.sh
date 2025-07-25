#!/bin/bash

# Development setup script for the Rust monorepo

set -e

echo "🦀 Setting up Rust monorepo development environment..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo "✅ Rust version: $RUST_VERSION"

# Install useful development tools
echo "📦 Installing development tools..."

# Clippy for linting (usually comes with Rust)
rustup component add clippy

# Rustfmt for formatting (usually comes with Rust)
rustup component add rustfmt

# Cargo-audit for security auditing
if ! cargo audit --version &> /dev/null; then
    echo "Installing cargo-audit..."
    cargo install cargo-audit
fi

# Cargo-watch for automatic rebuilding
if ! cargo watch --version &> /dev/null; then
    echo "Installing cargo-watch..."
    cargo install cargo-watch
fi

# Cargo-nextest for faster testing
if ! cargo nextest --version &> /dev/null; then
    echo "Installing cargo-nextest..."
    cargo install cargo-nextest --locked
fi

echo "🔧 Running initial setup..."

# Format all code
cargo fmt

# Check that everything builds
echo "🏗️  Building workspace..."
cargo build

# Run tests
echo "🧪 Running tests..."
cargo test

echo "✅ Setup complete! Your Rust monorepo is ready for development."
echo ""
echo "Useful commands:"
echo "  cargo build          - Build all packages"
echo "  cargo test           - Run all tests"
echo "  cargo fmt            - Format code"
echo "  cargo clippy         - Run lints"
echo "  cargo audit          - Check for security vulnerabilities"
echo "  cargo watch -x test  - Continuously run tests on file changes"
