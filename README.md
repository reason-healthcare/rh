# Rust Monorepo

A Rust workspace containing multiple related packages.

## Structure

- `crates/`: Library crates that provide reusable functionality
  - `common/`: Shared utilities and error handling
  - `codegen/`: FHIR StructureDefinition to Rust type code generation
- `apps/`: Executable applications that may depend on the library crates
  - `example-cli/`: Example CLI application demonstrating workspace usage
  - `fhir-codegen/`: CLI tool for generating Rust types from FHIR StructureDefinitions
- `docs/`: Documentation, examples, and guides
- `tools/`: Development tools, scripts, and utilities
- `examples/`: Example files and generated code samples

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

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

### Development

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

## Adding New Packages

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

## FHIR Code Generation

This monorepo includes a specialized code generation library for creating Rust types from FHIR StructureDefinition JSON files.

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

See `docs/fhir-codegen.md` for detailed documentation.

## Workspace Dependencies

Common dependencies are defined in the workspace `Cargo.toml` file. Individual crates can reference these using:

```toml
[dependencies]
serde = { workspace = true }
tokio = { workspace = true }
```

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
