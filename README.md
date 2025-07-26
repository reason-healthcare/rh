# FHIR Rust

[![CI](https://github.com/reason-healthcare/rh/workflows/CI/badge.svg)](https://github.com/reason-healthcare/rh/actions)
[![codecov](https://codecov.io/gh/reason-healthcare/rh/branch/main/graph/badge.svg)](https://codecov.io/gh/reason-healthcare/rh)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

A comprehensive Rust workspace for FHIR (Fast Healthcare Interoperability Resources) code generation and processing. This monorepo provides libraries and CLI tools for working with FHIR StructureDefinitions, package management, and type-safe Rust code generation.

## 🚀 Quick Start

```bash
# Clone and build the entire workspace
git clone <repo-url>
cd research
cargo build
```

For detailed usage examples and comprehensive documentation, see:
- **[RH CLI Documentation](apps/rh/README.md)** - Complete guide to the unified CLI
- **[FHIR Code Generation](crates/codegen/README.md)** - Library usage and advanced features
- **[FHIRPath Operations](crates/fhirpath/README.md)** - Expression syntax and evaluation examples

## 📁 Workspace Structure

```
.
├── Cargo.toml              # Workspace root configuration
├── crates/                 # Library crates
│   ├── codegen/           # 🔧 FHIR code generation library
│   ├── common/            # 📦 Shared utilities and error handling  
│   ├── core/              # 🏗️ Core functionality
│   └── fhirpath/          # 🔍 FHIRPath expression parser and evaluator
├── apps/                  # Executable applications
│   └── rh/                # 🎯 Unified CLI with codegen and fhirpath subcommands
├── setup.sh              # 🔨 Development setup script
└── build.sh              # 🏭 Build script for CI/CD
```

## Libraries

- **[FHIR Code Generation](crates/codegen/README.md)** - Generate type-safe Rust code from FHIR StructureDefinitions with package management support
- **[FHIRPath](crates/fhirpath/README.md)** - Parse and evaluate FHIRPath expressions with comprehensive operator support
- **Common** (`crates/common/`) - Shared utilities and error handling
- **Core** (`crates/core/`) - Core functionality and types

## Applications

- **[RH CLI](apps/rh/README.md)** - Unified CLI application with subcommands for:
  - **Code Generation** (`rh codegen`) - Convert FHIR StructureDefinitions to Rust types
  - **FHIRPath Operations** (`rh fhirpath`) - Parse, evaluate, and test FHIRPath expressions  

## 🛠️ Getting Started

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

### Development Commands

All functionality is now available through the unified `rh` CLI. Legacy applications remain available but are deprecated.

**For complete CLI documentation and examples, see the [RH CLI documentation](apps/rh/README.md)**

**Unified CLI (recommended):**
```bash
# Code generation
cargo run -p rh -- codegen --help

# FHIRPath operations  
cargo run -p rh -- fhirpath --help

# Enable verbose logging for any subcommand
cargo run -p rh -- -v codegen init
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

## 📦 Dependency Management

### Workspace Dependencies

Add common dependencies to the root `Cargo.toml`:

```toml
[workspace.dependencies]
serde = "1.0"
tokio = "1.0"
anyhow = "1.0"
```

Then reference them in individual crates:

```toml
[dependencies]
serde = { workspace = true }
tokio = { workspace = true }
```

### Crate-specific Dependencies

Add directly to the crate's `Cargo.toml`:

```toml
[dependencies]
regex = "1.0"
```

## 🔄 Development Workflow

1. **Setup**: Run `./setup.sh` once to configure your environment
2. **Development**: Work in individual crates with full workspace support
3. **Testing**: `cargo test` runs all tests across the workspace
4. **Building**: `cargo build` builds everything with optimized dependencies
5. **Linting**: `cargo clippy` checks all code for best practices

## 🚀 Release Management

The workspace supports coordinated releases of all crates or individual crate releases as needed. Use semantic versioning and update `Cargo.toml` files accordingly.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
