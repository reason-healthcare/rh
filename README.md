# FHIR Rust Monorepo

A comprehensive Rust workspace for FHIR (Fast Healthcare Interoperability Resources) code generation and processing. This monorepo provides libraries and CLI tools for working with FHIR StructureDefinitions, package management, and type-safe Rust code generation.

## 🚀 Quick Start

```bash
# Clone and build the entire workspace
git clone <repo-url>
cd research
cargo build

# Generate Rust types from a FHIR StructureDefinition
cargo run -p fhir-codegen -- generate -i examples/patient.json -o examples/patient.rs

# Download and install a FHIR package with automatic code generation
cargo run -p fhir-codegen -- install hl7.fhir.r4.core 4.0.1 -o ./generated/
```

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
│   ├── example-cli/       # 📋 Example CLI application
│   └── fhir-codegen/      # ⚡ CLI tool for FHIR code generation
├── docs/                  # 📚 Documentation and guides
├── tools/                 # 🛠️ Development tools and scripts
├── examples/              # 💡 Example files and generated code samples
├── setup.sh              # 🔨 Development setup script
└── build.sh              # 🏭 Build script for CI/CD
```

## 📚 Documentation

### Core Libraries

- **[FHIR Code Generation](crates/codegen/README.md)** - Generate type-safe Rust code from FHIR StructureDefinitions with package management support
- **[FHIRPath](crates/fhirpath/README.md)** - Parse and evaluate FHIRPath expressions with comprehensive operator support

### Applications

- **FHIR Codegen CLI** (`apps/fhir-codegen/`) - Command-line interface for batch processing and package management

## 🏗️ Core Features

### FHIR Code Generation
- **Type-Safe Code Generation**: Convert FHIR StructureDefinitions to idiomatic Rust types
- **Package Management**: Download FHIR packages from npm-style registries
- **Enum Generation**: Create type-safe enums for required value set bindings
- **Batch Processing**: Process entire directories of FHIR definitions
- **Serde Integration**: Automatic JSON serialization/deserialization support

### FHIRPath Expression Engine
- **Complete FHIRPath Support**: Parse and evaluate FHIRPath expressions
- **Mathematical Operations**: Arithmetic, comparison, and logical operators
- **String Functions**: Comprehensive string manipulation capabilities
- **Collection Operations**: Work with FHIR collections and lists
- **Type Safety**: Rust-native type checking and error handling

### Workspace Benefits
- **Unified Dependencies**: Shared dependencies across all crates
- **Consistent Versioning**: Coordinated releases and dependency management  
- **Cross-crate Testing**: Easy integration testing between components
- **Simplified Build Process**: Single `cargo build` command for everything

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

## ➕ Adding New Packages

### Library Crate

```bash
cd crates
cargo new my-lib --lib
```

### Application

```bash
cd apps
cargo new my-app --bin
```

After creating a new package, it will automatically be included in the workspace due to the wildcard patterns in `Cargo.toml`.

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

## 🔥 FHIR Code Generation

This monorepo includes a specialized code generation library for creating Rust types from FHIR StructureDefinition JSON files. See the **[FHIR Code Generation documentation](crates/codegen/README.md)** for comprehensive details.

### Quick Start with FHIR Codegen

1. **Initialize configuration:**
   ```bash
   cargo run -p fhir-codegen -- init
   ```

2. **Generate from a FHIR StructureDefinition:**
   ```bash
   cargo run -p fhir-codegen -- generate -i examples/patient.json -o examples/patient.rs
   ```

3. **Batch process multiple files:**
   ```bash
   cargo run -p fhir-codegen -- batch -i fhir-definitions/ -o src/generated/
   ```

### FHIR Package Management

The codegen tool supports downloading FHIR packages from npm-style registries, following FHIR package conventions:

4. **Download a FHIR package:**
   ```bash
   cargo run -p fhir-codegen -- download hl7.fhir.r4.core 4.0.1 -o ./packages/
   ```

5. **Install and generate types from a FHIR package:**
   ```bash
   cargo run -p fhir-codegen -- install hl7.fhir.r4.core 4.0.1 -o ./generated/
   ```

6. **Use custom registry or authentication:**
   ```bash
   cargo run -p fhir-codegen -- download my.custom.package 1.0.0 \
     --registry https://my-fhir-registry.com \
     --token your-auth-token
   ```

The package management system:
- Downloads packages as compressed tarballs from npm-style registries
- Extracts StructureDefinition JSON files automatically  
- Supports authentication for private registries
- Uses the default FHIR package registry at https://packages.fhir.org
- Generates Rust types for all StructureDefinitions in the package

## 🔍 FHIRPath Expressions

The workspace includes a comprehensive FHIRPath expression parser and evaluator. See the **[FHIRPath documentation](crates/fhirpath/README.md)** for detailed information.

### Quick FHIRPath Examples

```rust
use fhirpath::{FhirPathParser, FhirPathEvaluator};

// Parse and evaluate FHIRPath expressions
let parser = FhirPathParser::new();
let evaluator = FhirPathEvaluator::new();

// Mathematical operations
let expr = parser.parse("5 + 3 * 2")?;
let result = evaluator.evaluate(&expr, &context)?;

// String functions  
let expr = parser.parse("name.family.upper()")?;
let result = evaluator.evaluate(&expr, &patient_data)?;

// Collection operations
let expr = parser.parse("telecom.where(system = 'email').value")?;
let result = evaluator.evaluate(&expr, &patient_data)?;
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
