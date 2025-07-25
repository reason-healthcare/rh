# FHIR Code Generation

This directory contains tools for generating Rust types from FHIR StructureDefinition JSON files.

## Overview

The codegen library (`crates/codegen`) provides functionality to:
- Parse FHIR StructureDefinition JSON files
- Generate corresponding Rust struct definitions
- Handle FHIR primitive type mappings
- Support optional fields and arrays
- Generate serde annotations for JSON serialization/deserialization

The CLI tool (`apps/fhir-codegen`) provides a command-line interface to:
- Generate single files or batch process directories
- Configure type mappings and output settings
- Customize generated code style

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

## Usage in Code

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
