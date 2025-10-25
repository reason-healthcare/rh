# Justfile for common development tasks
# Install just: https://github.com/casey/just

# Default recipe - show available commands
default:
    @just --list

# Run all tests (skipping doctests due to incremental compilation issues)
test:
    cargo test --workspace --all-features --lib --bins --tests

# Run tests with nextest if available
test-fast:
    #!/usr/bin/env bash
    if command -v cargo-nextest &> /dev/null; then
        cargo nextest run --all-features
    else
        cargo test --workspace --all-features
    fi

# Test that all examples compile and run
test-examples:
    @echo "Building all examples..."
    cargo build --examples --workspace
    @echo "Running validator examples..."
    cargo run -p rh-validator --example basic_validation > /dev/null
    cargo run -p rh-validator --example structural_validation > /dev/null
    cargo run -p rh-validator --example invariant_validation > /dev/null
    cargo run -p rh-validator --example custom_config > /dev/null
    cargo run -p rh-validator --example error_handling > /dev/null
    cargo run -p rh-validator --example patient_validation > /dev/null
    cargo run -p rh-validator --example resource_builder > /dev/null
    @echo "✅ All examples compiled and ran successfully!"

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

# Check everything (format, lint, test, audit, examples)
check: fmt-check lint test test-examples audit

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

# Build WASM package for VCL (web target)
build-wasm:
    #!/usr/bin/env bash
    echo "Building VCL WASM (web target)..."
    cd crates/rh-vcl && just wasm-web
    # Copy to workspace pkg directory for compatibility
    mkdir -p ../../pkg/wasm
    cp -r pkg/web/* ../../pkg/wasm/
    echo "✅ WASM web build complete! Output in pkg/wasm/ and crates/rh-vcl/pkg/web/"

# Build WASM package for Node.js
build-wasm-node:
    #!/usr/bin/env bash
    echo "Building VCL WASM (Node.js target)..."
    cd crates/rh-vcl && just wasm-node
    # Copy to workspace pkg directory for compatibility
    mkdir -p ../../pkg/wasm-node
    cp -r pkg/node/* ../../pkg/wasm-node/
    echo "✅ WASM Node.js build complete! Output in pkg/wasm-node/ and crates/rh-vcl/pkg/node/"

# Build WASM package for bundlers (webpack, etc.)
build-wasm-bundler:
    #!/usr/bin/env bash
    echo "Building VCL WASM (bundler target)..."
    cd crates/rh-vcl && just wasm-bundler
    # Copy to workspace pkg directory for compatibility
    mkdir -p ../../pkg/wasm-bundler
    cp -r pkg/bundler/* ../../pkg/wasm-bundler/
    echo "✅ WASM bundler build complete! Output in pkg/wasm-bundler/ and crates/rh-vcl/pkg/bundler/"

# Test WASM build 
test-wasm:
    #!/usr/bin/env bash
    echo "Testing VCL WASM build..."
    cd crates/rh-vcl && make test-wasm
