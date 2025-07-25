#!/bin/bash

# Build script for CI/CD pipelines

set -e

echo "🏗️  Building Rust monorepo..."

# Ensure we're using the stable toolchain
rustup default stable

# Update dependencies
echo "📦 Updating dependencies..."
cargo update

# Check formatting
echo "🎨 Checking code formatting..."
cargo fmt --all -- --check

# Run clippy
echo "📎 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Build all packages
echo "🔨 Building all packages..."
cargo build --all-targets --all-features

# Run tests
echo "🧪 Running tests..."
cargo test --all-features

# Check for security vulnerabilities
echo "🛡️  Checking for security vulnerabilities..."
if command -v cargo-audit &> /dev/null; then
    cargo audit
else
    echo "⚠️  cargo-audit not installed, skipping security check"
fi

echo "✅ Build completed successfully!"
