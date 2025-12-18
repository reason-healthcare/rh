# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Reason Health (rh)** is a high-performance FHIR (Fast Healthcare Interoperability Resources) toolkit in Rust. Monorepo workspace with library crates in `/crates/` and the CLI application in `/apps/rh-cli/`.

## Build Commands

```bash
cargo build                              # Build all
cargo build -p <package-name>            # Build specific crate
just build                               # Shortcut for full build
```

## Testing

```bash
just test                                # All tests (recommended)
cargo test -p <package-name>             # Single crate
cargo test -p <pkg> <test_name>          # Single test

# FHIR validation test suites
just test-fhir                           # Quick (5 cases)
just test-fhir-50                        # Extended (50 cases)
just test-fhir-all                       # Full suite (~570 cases)
```

## Linting and Formatting

```bash
just lint                                # Clippy with warnings as errors
just fmt                                 # Format code
just fmt-check                           # Check formatting
just check                               # Full check: fmt-check, lint, test, test-examples, audit
```

## Architecture

### Crate Dependency Structure
```
rh-cli (binary)
├── rh-codegen       # FHIR → Rust type generator
├── rh-fhirpath      # FHIRPath expression engine
├── rh-validator     # FHIR resource validator
├── rh-vcl           # ValueSet Compose Language
├── rh-loader        # FHIR package loader
├── rh-snapshot      # StructureDefinition snapshots
└── rh-foundation    # Shared utilities (errors, HTTP, I/O)
```

### Key Crates
- **rh-foundation**: Base utilities, error types, HTTP client wrappers
- **rh-codegen**: Generates Rust types from FHIR StructureDefinitions
- **rh-fhirpath**: Parser and evaluator for FHIRPath expressions
- **rh-validator**: Profile-based FHIR validation with LRU caching
- **rh-vcl**: ValueSet Compose Language parser (WASM-compatible)
- **rh-loader**: Loads FHIR packages from npm-style registries
- **hl7_fhir_r4_core**: Pre-generated R4 FHIR types

## Code Conventions

- **Line length**: 100 characters
- **Error handling**: `anyhow::Result<T>` for apps, `thiserror` for libraries. Never `.unwrap()` in production—use `?` with context.
- **Naming**: `snake_case` (functions/vars), `PascalCase` (types/traits), `SCREAMING_SNAKE_CASE` (constants)
- **Dependencies**: Use workspace dependencies from root `Cargo.toml` when available
- **No code comments**: Only `///` doc comments for public APIs
- **Tests**: Unit tests in `#[cfg(test)]` modules, use `#[tokio::test]` for async
- **Commits**: Conventional format `<type>(<scope>): <description>`

## Running the CLI

```bash
cargo run -p rh -- <subcommand>          # Development
cargo run -p rh -- --help                # Show help
cargo run -p rh -- fhirpath eval '...'   # FHIRPath evaluation
cargo run -p rh -- validate <resource>   # Validate FHIR resource
```

## WASM Builds

```bash
just build-wasm                          # Web target
just build-wasm-node                     # Node.js target
```

## Before Committing

```bash
just check                               # Runs all checks
cargo fmt                                # Final format pass
```
