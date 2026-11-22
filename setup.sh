#!/usr/bin/env bash

set -euo pipefail

echo "🦀 Setting up RH workspace..."

# Rust toolchain
if ! command -v rustc >/dev/null 2>&1; then
  echo "❌ Rust is not installed. Install via https://rustup.rs/"
  exit 1
fi
if ! command -v rustup >/dev/null 2>&1; then
  echo "❌ rustup not found. Install via https://rustup.rs/"
  exit 1
fi

RUST_VERSION=$(rustc --version | awk '{print $2}')
REQUIRED_RUST="1.95.0"
ver_ge() { printf '%s\n%s\n' "$1" "$2" | sort -Vr -C; }
if ! ver_ge "$RUST_VERSION" "$REQUIRED_RUST"; then
  echo "⚠️  rustc $RUST_VERSION detected; workspace requires >= $REQUIRED_RUST"
  echo "   Run: rustup update stable"
fi

TOOLCHAIN=$(rustup show active-toolchain | awk '{print $1}')
echo "✅ Rust version: $RUST_VERSION ($TOOLCHAIN)"

echo "📦 Ensuring development components..."
rustup component add --toolchain "$TOOLCHAIN" clippy rustfmt 2>/dev/null || rustup component add clippy rustfmt

# Optional tool installation (set SKIP_INSTALL=1 to skip)
if [[ -z "${SKIP_INSTALL:-}" ]]; then
  if ! command -v cargo-audit >/dev/null 2>&1; then
    echo "Installing cargo-audit..."
    cargo install cargo-audit || echo "⚠️  Failed to install cargo-audit; continuing"
  fi
  if ! command -v cargo-watch >/dev/null 2>&1; then
    echo "Installing cargo-watch..."
    cargo install cargo-watch --locked || echo "⚠️  Failed to install cargo-watch; continuing"
  fi
  if ! command -v cargo-nextest >/dev/null 2>&1; then
    echo "Installing cargo-nextest..."
    cargo install cargo-nextest --locked || echo "⚠️  Failed to install cargo-nextest; continuing"
  fi
else
  echo "⏭️  SKIP_INSTALL set; skipping dev tool installation"
fi

# Encourage installing 'just' for workspace tasks
if ! command -v just >/dev/null 2>&1; then
  echo "ℹ️  'just' not found. Install for helper tasks: https://github.com/casey/just"
fi

echo "🔧 Running initial checks..."

# Format code first for a smooth start
cargo fmt --all

# Prefer unified workspace checks if 'just' is available
if command -v just >/dev/null 2>&1; then
  just check
else
  echo "🎯 No 'just' detected; running equivalent checks"
  echo "📎 Running clippy..."
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  echo "🏗️  Building workspace..."
  cargo build --workspace --all-targets --all-features
  echo "🧪 Running tests..."
  cargo test --workspace --all-features --lib --bins --tests
  if command -v cargo-audit >/dev/null 2>&1; then
    echo "🛡️  Running cargo audit..."
    cargo audit || true
  fi
fi

echo "✅ Setup complete! RH workspace is ready."
echo ""
echo "Useful commands:"
echo "  just check                 - Format check, lint, tests, audit"
echo "  just build                 - Build all packages"
echo "  cargo run -p rh -- --help  - CLI help"
echo "  just test-fhir             - Quick validator tests (5 cases)"
echo "  cargo watch -x test        - Run tests on changes"
