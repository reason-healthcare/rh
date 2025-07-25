# FHIR Code Generation

This directory contains tools for generating Rust types from FHIR StructureDefinition JSON files.

## Overview

The codegen library (`crates/codegen`) provides functionality to:
- Parse FHIR StructureDefinition JSON files
- Generate corresponding Rust struct definitions
- Handle FHIR primitive type mappings
- Support optional fields and arrays
- Generate serde annotations for JSON serialization/deserialization
- **Download FHIR packages from npm-style registries**
- **Extract and process StructureDefinitions from package tarballs**

The CLI tool (`apps/fhir-codegen`) provides a command-line interface to:
- Generate single files or batch process directories
- Configure type mappings and output settings
- Customize generated code style
- **Download and install FHIR packages with automatic code generation**

## Quick Start

1. **Initialize configuration:**
   ```bash
   cargo run -p fhir-codegen -- init
   ```

2. **Generate from a single file:**
   ```bash
   cargo run -p fhir-codegen -- generate -i patient.json -o src/patient.rs
   ```

3. **Batch process a directory:**
   ```bash
   cargo run -p fhir-codegen -- batch -i fhir-definitions/ -o src/generated/
   ```

4. **Download a FHIR package:**
   ```bash
   cargo run -p fhir-codegen -- download hl7.fhir.r4.core 4.0.1 -o ./packages/
   ```

5. **Install and generate types from a FHIR package:**
   ```bash
   cargo run -p fhir-codegen -- install hl7.fhir.r4.core 4.0.1 -o ./generated/
   ```

## FHIR Package Management

The codegen tool now supports downloading FHIR packages from npm-style registries, following the FHIR package distribution conventions used by the FHIR community.

### Download Command

Downloads a FHIR package to a local directory:

```bash
cargo run -p fhir-codegen -- download <package> <version> [OPTIONS]
```

**Options:**
- `-o, --output <OUTPUT>`: Output directory (default: `./packages`)
- `--registry <REGISTRY>`: Registry URL (default: `https://packages.fhir.org`)
- `--token <TOKEN>`: Authentication token for private registries

**Example:**
```bash
# Download FHIR R4 core package
cargo run -p fhir-codegen -- download hl7.fhir.r4.core 4.0.1

# Download from custom registry with authentication
cargo run -p fhir-codegen -- download my.custom.package 1.0.0 \
  --registry https://my-fhir-registry.com \
  --token your-auth-token
```

### Install Command

Downloads a FHIR package and automatically generates Rust types for all StructureDefinitions:

```bash
cargo run -p fhir-codegen -- install <package> <version> [OPTIONS]
```

**Options:**
- `-o, --output <OUTPUT>`: Output directory for generated Rust files (default: `./generated`)
- `-c, --config <CONFIG>`: Configuration file path (default: `codegen.json`)
- `--registry <REGISTRY>`: Registry URL (default: `https://packages.fhir.org`)
- `--token <TOKEN>`: Authentication token for private registries

**Example:**
```bash
# Install FHIR R4 core and generate types
cargo run -p fhir-codegen -- install hl7.fhir.r4.core 4.0.1 -o ./src/fhir/

# Use custom configuration
cargo run -p fhir-codegen -- install hl7.fhir.r4.core 4.0.1 \
  -c my-config.json \
  -o ./src/generated/
```

### How It Works

1. **Registry Interaction**: Fetches package metadata from npm-style registries using HTTP
2. **Package Download**: Downloads the package as a compressed tarball (.tgz)
3. **Extraction**: Extracts JSON files from the tarball
4. **Processing**: Identifies StructureDefinition files and generates Rust types
5. **Code Generation**: Uses the same generation pipeline as file-based processing

### Supported Registries

- **FHIR Package Registry**: https://packages.fhir.org (default)
- **Custom npm-style registries**: Any registry following npm package conventions
- **Private registries**: With authentication token support

## Configuration

The `codegen.json` file controls code generation:

```json
{
  "output_dir": "src/generated",
  "module_name": "fhir_types", 
  "with_serde": true,
  "with_docs": true,
  "type_mappings": {
    "string": "String",
    "integer": "i32",
    "boolean": "bool",
    "decimal": "f64"
  }
}
```

### Configuration Options

- `output_dir`: Default output directory for generated files
- `module_name`: Module name for generated types  
- `with_serde`: Generate serde Serialize/Deserialize derives
- `with_docs`: Include documentation comments
- `type_mappings`: Custom FHIR type to Rust type mappings

## Example Generated Code

From a FHIR Patient StructureDefinition, the generator creates:

```rust
use serde::{Deserialize, Serialize};

/// FHIR Patient resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    /// Logical id of this artifact
    pub id: Option<String>,
    
    /// A name associated with the individual
    pub name: Option<Vec<HumanName>>,
    
    /// The date of birth for the individual
    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,
    
    /// Administrative Gender
    pub gender: Option<String>,
}
```

## FHIR Type Mappings

The generator includes default mappings for FHIR primitive types:

| FHIR Type | Rust Type | Notes |
|-----------|-----------|-------|
| string | String | |
| integer | i32 | |
| boolean | bool | |
| decimal | f64 | |
| uri, url, canonical | String | Could use custom URI types |
| date, dateTime, instant | String | Could use chrono types |
| code, id, oid | String | |
| base64Binary | String | Could use Vec<u8> |
| markdown | String | |

Complex FHIR types (starting with uppercase) are converted to PascalCase Rust struct names.

## Library Architecture

The codegen library is organized into modular components:

- **`generator.rs`**: Core type generation functionality
  - `CodeGenerator`: Main struct for generating Rust types with built-in caching
  - `CodegenConfig`: Configuration for type generation
  - FHIR structure definitions and element parsing
  - Token generation and file output
  - Type caching to avoid regenerating identical structs

- **`download.rs`**: Package downloading functionality  
  - `PackageDownloader`: HTTP client for registry interaction
  - `PackageDownloadConfig`: Configuration for registry access
  - Tarball download and extraction
  - Registry response parsing

- **`lib.rs`**: Public API and error definitions
  - Re-exports from modules for convenience
  - Centralized error types (`CodegenError`)
  - Common result type (`CodegenResult<T>`)

This modular design separates concerns and makes the codebase easier to maintain and extend.

```rust
use codegen::{CodeGenerator, CodegenConfig};

let config = CodegenConfig::default();
let mut generator = CodeGenerator::new(config);

// Load and generate from a StructureDefinition
let structure_def = generator.load_structure_definition("patient.json")?;
let rust_struct = generator.generate_struct(&structure_def)?;
let tokens = generator.generate_tokens(&rust_struct);

// Or generate directly to file
generator.generate_to_file(&structure_def, Path::new("patient.rs"))?;
```

## Features

- ✅ Parse FHIR StructureDefinition JSON
- ✅ Generate Rust structs with proper field types
- ✅ Handle optional fields (min: 0)
- ✅ Handle array fields (max: "*" or > 1)
- ✅ FHIR primitive type mappings
- ✅ serde rename for snake_case fields
- ✅ Documentation generation
- ✅ CLI tool for batch processing
- ✅ **FHIR package downloading from npm-style registries**
- ✅ **Automatic extraction and processing of package tarballs**
- ✅ **Authentication support for private registries**
- ✅ **Install command for download + generation workflow**
- 🔄 Complex type references (in progress)
- 🔄 Enum generation for coded values
- 🔄 Extension handling
- 🔄 Profile validation

## Contributing

To add new features or fix bugs:

1. Update the `codegen` library in `crates/codegen/`
2. Add tests for new functionality
3. Update the CLI tool if needed in `apps/fhir-codegen/`
4. Update this documentation

The generated code aims to be idiomatic Rust that integrates well with the serde ecosystem for JSON handling.
