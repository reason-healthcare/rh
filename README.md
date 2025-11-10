# RH - Rust Healthcare Toolkit

[![CI](https://github.com/reason-healthcare/rh/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/reason-healthcare/rh/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)


**RH** is a modern, high-performance toolkit for working with HL7® FHIR®, purpose-built in **Rust**. It provides ergonomic, developer-friendly APIs that are modular, easy to understand, and highly extendable. It also ships with a powerful **command-line interface (CLI)** designed with the **Unix philosophy** in mind: superior UX, small, composable, and ideal for scripting and automation.

Cross-platform and lightning-fast, and running nativelly cross platform, RH eliminates the overhead of heavy virtual machines like the JVM or .NET. RH can even be compiled to **WebAssembly** for seamless integration into web applications and embedded environments.

## Components

| Status | Component | Description |
|--------|-----------|-------------|
| 🚀 | [rh-cli](apps/rh-cli/README.md) | First-class CLI for all RH features |
| 🚀 | [rh-codegen](crates/rh-codegen/README.md)  | FHIR to Rust type code generator |
| 🚀 | [rh-fhirpath](crates/rh-fhirpath/README.md) | Extendable FHIRPath engine |
| 🚀 | [rh-vcl](crates/rh-vcl/README.md) | ValueSet Compose Language (VCL) parser and translator |
| 🔜 | [rh-validator](crates/rh-validator/README.md)  | FHIR validator |
| 🚀 | [rh-loader](crates/rh-loader/README.md)  | FHIR package loader from npm-style registries |
| 🔜 | rh-fsh  | FHIR Shorthand converter |
| 🔜 | rh-publisher  | FHIR Package publisher and related tools |
| 🔜 | rh-sql  | SQL-on-FHIR view runner and related tools |
| 🔜 | rh-cql  | CQL tools |
| 🚀 | [rh-hl7_fhir_r4_core](crates/rh-hl7_fhir_r4_core/README.md)  | **Generated** R4 FHIR for Rust |

## Quick Start

```bash
# Clone and build the entire workspace
git clone <repo-url>
cd research
cargo build
```


## Workspace Structure

```
.
├── Cargo.toml              # Workspace root configuration
├── crates/                 # Library crates
│   ├── rh-codegen/            # FHIR code generation library
│   ├── rh-foundation/         # Foundation utilities (errors, HTTP, I/O, CLI helpers)
│   ├── rh-fhirpath/           # FHIRPath expression parser and evaluator
│   ├── rh-loader/             # FHIR package loader from npm-style registries
│   ├── rh-validator/          # FHIR resource validation library
│   └── rh-vcl/                # ValueSet Compose Language (VCL) parser and translator
├── apps/                   # Executable applications
│   └── rh-cli/                # Unified cross-platform CLI for FHIR
├── setup.sh                # Development setup script
└── build.sh                # Build script for CI/CD
```

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Development Setup

Run the setup script to configure your development environment:

```bash
./setup.sh
```

### Building

Build all packages in the workspace:

```bash
cargo build
```

Build a specific package:

```bash
cargo build -p <package-name>
```

### Testing

Run all tests:

```bash
cargo test
```

Run tests for a specific package:

```bash
cargo test -p <package-name>
```

NOTE: You can also use `just`
```bash
just --help
```

### Development Commands

All functionality is available through the unified `rh` CLI.

**For complete CLI documentation and examples, see the [RH CLI documentation](apps/rh-cli/README.md)**

**Unified CLI (recommended):**
```bash
# Code generation
cargo run -p rh -- codegen --help

# FHIRPath operations
cargo run -p rh -- fhirpath --help

# ValueSet Compose Language (VCL) operations
cargo run -p rh -- vcl --help
```

Check code formatting:

```bash
cargo fmt --check
```

Apply formatting:

```bash
cargo fmt
```

Run clippy lints:

```bash
cargo clippy
```

Run clippy with all features:

```bash
cargo clippy --all-features
```

Run tests:

```bash
cargo tes
```

## Dependency Management

### Workspace Dependencies

Add common dependencies to the root `Cargo.toml`, e.g.:

```toml
[workspace.dependencies]
serde = "1.0"
tokio = "1.0"
anyhow = "1.0"
```

Then reference them in individual crates, e.g.:

```toml
[dependencies]
serde = { workspace = true }
tokio = { workspace = true }
```

### Crate-specific Dependencies

Add directly to the crate's `Cargo.toml`, e.g.:

```toml
[dependencies]
regex = "1.0"
```

## Development Workflow

1. **Setup**: Run `./setup.sh` once to configure your environment
2. **Development**: Work in individual crates with full workspace support
3. **Testing**: `cargo test` runs all tests across the workspace
4. **Building**: `cargo build` builds everything with optimized dependencies
5. **Linting**: `cargo clippy` checks all code for best practices

## Release Management

The workspace supports coordinated releases of all crates or individual crate releases as needed. Use semantic versioning and update `Cargo.toml` files accordingly.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
