# Development Guide

## Getting Started

This project uses [just](https://github.com/casey/just) as the primary task runner. All common development tasks are defined in the `justfile`.

### Initial Setup

```bash
./setup.sh  # or: just setup
```

This will:
- Verify Rust installation (>= 1.91.0, the workspace MSRV)
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
    - `just wasm-check` (compile-check all WASM-capable crates)
    - `just wasm-build fhirpath all`
    - `just wasm-build vcl all`
    - `just wasm-build cql all`
- **Docs Drift**:
    - `just docs-sync` (checks architecture dependency docs and CLI help docs)

## Linting & Formatting

- **Format**: `cargo fmt --all` (or `just fmt`)
- **Format Check**: `cargo fmt --all -- --check` (or `just fmt-check`)
- **Lint**: `cargo clippy --workspace --all-targets --all-features -- -D warnings` (or `just lint`)
- **Full Check**: `just check` (runs fmt-check, lint, test, audit) - **Run this before committing.**

## Running the CLI

- **Development**: `cargo run -p rh-cli -- <subcommand>`
- **Help**: `cargo run -p rh-cli -- --help`
- **FHIRPath Eval**: `cargo run -p rh-cli -- fhirpath eval '...'`
- **Validate**: `cargo run -p rh-cli -- validate resource --input patient.json`

## Task Completion Protocol

After completing tasks, always run:
1. `just check` (preferred) or `cargo test && cargo clippy --all-targets --all-features`
2. `just docs-sync` for documentation, CLI, architecture, or workspace dependency changes
3. `cargo fmt`
