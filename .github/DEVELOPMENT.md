# Development Guide

## Getting Started

This project uses [just](https://github.com/casey/just) as the primary task runner. All common development tasks are defined in the `justfile`.

### Initial Setup

```bash
./setup.sh  # or: just setup
```

This will:
- Verify Rust installation (>= 1.70.0)
- Install development tools (clippy, rustfmt, cargo-audit, cargo-watch, cargo-nextest)
- Run initial formatting and checks

### Available Commands

Run `just` or `just --list` to see all available commands.

## Build & Test Commands

- **Build All**: `just build` (or `cargo build`)
- **Build Package**: `cargo build -p <package-name>`
- **Test All**: `just test` (or `cargo test --workspace --all-features`)
- **Test Package**: `cargo test -p <package-name>`
- **Test Single**: `cargo test -p <package-name> <test-name>`
- **FHIR Validation Tests**:
    - `just test-fhir` (Quick - 5 cases)
    - `just test-fhir-50` (Extended - 50 cases)
    - `just test-fhir-all` (Full suite - ~570 cases)
- **WASM Builds**:
    - `just build-wasm` (Web target)
    - `just build-wasm-node` (Node.js target)

## Linting & Formatting

- **Format**: `cargo fmt --all` (or `just fmt`)
- **Format Check**: `cargo fmt --all -- --check` (or `just fmt-check`)
- **Lint**: `cargo clippy --workspace --all-targets --all-features -- -D warnings` (or `just lint`)
- **Full Check**: `just check` (runs fmt-check, lint, test, audit) - **Run this before committing.**

## Running the CLI

- **Development**: `cargo run -p rh -- <subcommand>`
- **Help**: `cargo run -p rh -- --help`
- **FHIRPath Eval**: `cargo run -p rh -- fhirpath eval '...'`
- **Validate**: `cargo run -p rh -- validate <resource>`

## Task Completion Protocol

After completing tasks, always run:
1. `just check` (preferred) or `cargo test && cargo clippy --all-targets --all-features`
2. `cargo fmt`
