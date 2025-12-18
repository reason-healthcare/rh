# Agent Instructions for RH Repository

This file provides guidance for AI agents working with code in this repository.

## Project Overview

**Reason Health (rh)** is a high-performance FHIR (Fast Healthcare Interoperability Resources) toolkit in Rust. Monorepo workspace with library crates in `/crates/` and the CLI application in `/apps/rh-cli/`.

## Architecture

### Crate Dependency Structure
```
rh-cli (binary)
├── rh-codegen       # FHIR → Rust type generator
├── rh-fhirpath      # FHIRPath expression engine
├── rh-validator     # FHIR resource validator
├── rh-vcl           # ValueSet Compose Language
├── rh-foundation    # Shared utilities (errors, HTTP, I/O)
├── rh-snapshot      # StructureDefinition snapshots
└── rh-loader        # FHIR package loader
```

### Key Crates
- **rh-foundation**: Base utilities, error types, HTTP client wrappers
- **rh-codegen**: Generates Rust types from FHIR StructureDefinitions
- **rh-fhirpath**: Parser and evaluator for FHIRPath expressions
- **rh-validator**: Profile-based FHIR validation with LRU caching
- **rh-vcl**: ValueSet Compose Language parser (WASM-compatible)
- **rh-loader**: Loads FHIR packages from npm-style registries
- **hl7_fhir_r4_core**: Pre-generated R4 FHIR types

## Build & Test Commands

- **Build All**: `cargo build` (or `just build`)
- **Build Package**: `cargo build -p <package-name>`
- **Test All**: `cargo test --workspace --all-features` (or `just test`)
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

## Code Style & Conventions

- **Formatting**: Run `cargo fmt` before committing. 100 char line length.
- **Naming**: `snake_case` (functions/vars/modules), `PascalCase` (types/structs/enums/traits), `SCREAMING_SNAKE_CASE` (constants).
- **Error Handling**: Use `anyhow::Result<T>` for apps, `thiserror` for libraries. Never `.unwrap()` in production code - use `?` operator. Add context with `anyhow::Context`.
- **Imports**: Use workspace dependencies from root `Cargo.toml` when available. Check existing code for library usage before adding new deps.
- **Documentation**: Write `///` doc comments for all public APIs with examples. Use `cargo doc --open` to review.
- **Comments**: **NO code comments** unless explicitly requested. Focus on code clarity. Doc comments for public APIs only.
- **Async**: Use `tokio` runtime, prefer `async fn`, use `reqwest` for HTTP, handle timeouts explicitly.
- **Tests**: Place unit tests in `#[cfg(test)]` modules, integration tests in `tests/` dir. Use `#[tokio::test]` for async tests.

## Project-Specific Conventions

- **Commit Format**: Conventional commits with `<type>(<scope>): <description>`.
    - Types: feat/fix/docs/style/refactor/test/chore.
    - Scopes: codegen/cli/fhirpath/vcl/validator/loader/deps/ci/docs.
- **FHIR Types Mapping**:
    - `string` → `String`
    - `integer` → `i32`
    - `boolean` → `bool`
    - `decimal` → `f64`
    - `dateTime` → `String`
- **Generated Code**: Includes serde annotations, uses `Option<T>` for optional fields, `Vec<T>` for arrays.

## Task Completion Protocol
After completing tasks, always run:
1. `just check` (preferred) or `cargo test && cargo clippy --all-targets --all-features`
2. `cargo fmt`