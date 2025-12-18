# FHIR Code Generation Library

A Rust library for generating type-safe Rust code from FHIR StructureDefinition JSON files, with support for FHIR package downloading and management.

## Overview

The `rh-codegen` crate provides comprehensive functionality for:

- **FHIR Type Generation**: Parse FHIR StructureDefinition JSON files and generate corresponding Rust struct definitions
- **Type Safety**: Generate type-safe enums for required value set bindings
- **Metadata Generation**: Generate compile-time metadata for type resolution and FHIRPath evaluation
- **Package Management**: Download FHIR packages from npm-style registries
- **Batch Processing**: Process entire directories of FHIR definitions
- **Automatic Extraction**: Extract and process StructureDefinitions from package tarballs
- **Configuration**: Flexible configuration for type mappings and output settings

## Crate Generation

When generating a crate, the following idiomatic Rust layout will be created:
```
hl7_fhir_r4_core/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── macros.rs
│   ├── metadata.rs
│   ├── bindings/
│   │   ├── mod.rs
│   │   ├── observation_status.rs
│   │   ├── allergy_category.rs
│   │   ├── encounter_class.rs
│   │   └── ...
│   ├── datatypes/
│   │   ├── mod.rs
│   │   ├── narrative.rs
│   │   ├── extension.rs
│   │   ├── coding.rs
│   │   ├── codeable_concept.rs
│   │   └── ...
│   ├── extensions/
│   │   ├── mod.rs
│   │   └── ...
│   ├── primitives/
│   │   ├── mod.rs
│   │   ├── string.rs
│   │   ├── boolean.rs
│   │   ├── date_time.rs
│   │   └── ...
│   ├── profiles/
│   │   ├── mod.rs
│   │   └── ...
│   ├── resources/
│   │   ├── mod.rs
│   │   ├── observation.rs
│   │   ├── patient.rs
│   │   ├── bundle.rs
│   │   └── ...
│   └── traits/
│       ├── mod.rs
│       ├── resource.rs
│       ├── domain_resource.rs
│       ├── has_extensions.rs
│       ├── has_coding.rs
│       └── ...
```

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
rh-codegen = { path = "../path/to/rh-codegen" }
# Or if using from the workspace root:
# rh-codegen = { path = "crates/rh-codegen" }
```

### Basic Usage

```rust
use rh_codegen::{CodeGenerator, CodegenConfig, CodegenResult};
use std::path::Path;

fn main() -> CodegenResult<()> {
    // Create generator with default configuration
    let config = CodegenConfig::default();
    let mut generator = CodeGenerator::new(config);

    // Generate from a single StructureDefinition file
    let structure_def = generator.load_structure_definition("patient.json")?;
    let rust_struct = generator.generate_struct(&structure_def)?;
    
    // Generate tokens and write to file
    let tokens = generator.generate_tokens(&rust_struct);
    generator.write_to_file(&tokens, Path::new("patient.rs"))?;

    Ok(())
}
```

## Metadata Generation

The code generator produces compile-time metadata for FHIR types, enabling runtime type resolution and validation. This metadata is used by the FHIRPath evaluator to return correctly typed values.

### Metadata Module

The generated `metadata.rs` module contains:

- **FhirPrimitiveType enum**: Defines all FHIR primitive types (Boolean, Integer, String, Date, DateTime, etc.)
- **FhirFieldType enum**: Categorizes fields as Primitive, Complex, Reference, or BackboneElement
- **FieldInfo struct**: Contains field metadata including type, cardinality (min/max), and choice type flag
- **Type maps**: Compile-time maps (using `phf`) for efficient field lookup

### Example Metadata Usage

```rust
use hl7_fhir_r4_core::metadata::{get_field_info, FhirFieldType, FhirPrimitiveType};

// Look up field metadata at runtime
if let Some(field_info) = get_field_info("Patient", "birthDate") {
    match &field_info.field_type {
        FhirFieldType::Primitive(FhirPrimitiveType::Date) => {
            println!("birthDate is a Date type");
        }
        _ => {}
    }
}

// Check cardinality
if let Some(field_info) = get_field_info("Patient", "name") {
    println!("min: {}, max: {:?}", field_info.min, field_info.max);
    // Output: min: 0, max: None (unbounded)
}
```

This metadata enables FHIRPath expressions to return properly typed values instead of generic strings. For example, `Patient.birthDate` returns a `Date` type rather than a `String`.

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `output_dir` | `String` | `"src/generated"` | Default output directory for generated files |
| `module_name` | `String` | `"fhir_types"` | Module name for generated types |
| `with_serde` | `bool` | `true` | Generate serde Serialize/Deserialize derives |
| `with_docs` | `bool` | `true` | Include documentation comments |
| `type_mappings` | `Map<String, String>` | See below | Custom FHIR type to Rust type mappings |

### Default Type Mappings

| FHIR Type | Rust Type | Notes |
|-----------|-----------|-------|
| `string` | `String` | Basic string type |
| `integer` | `i32` | Signed 32-bit integer |
| `boolean` | `bool` | Boolean true/false |
| `decimal` | `f64` | 64-bit floating point |
| `uri`, `url`, `canonical` | `String` | Could use custom URI types |
| `date`, `dateTime`, `instant` | `String` | Could use chrono types |
| `code` | `String` or `Enum` | Becomes enum for required value set bindings |
| `id`, `oid` | `String` | Identifier types |
| `base64Binary` | `String` | Could use `Vec<u8>` |
| `markdown` | `String` | Markdown content |

## Generated Code Examples

### Basic Resource Generation

From a FHIR Patient StructureDefinition:

```rust
use serde::{Deserialize, Serialize};

/// FHIR Patient resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    #[serde(flatten)]
    /// Inherited fields from base type
    pub base: DomainResource,
    
    /// A name associated with the individual
    pub name: Option<Vec<HumanName>>,
    
    /// The date of birth for the individual
    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,
    
    /// Administrative Gender
    pub gender: Option<AdministrativeGender>,
    
    /// Contact details for the individual
    pub telecom: Option<Vec<ContactPoint>>,
}
```

### Type-Safe Enums for Value Sets

For `code` fields with required value set bindings:

```rust
/// The gender of a person used for administrative purposes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdministrativeGender {
    /// Male gender
    #[serde(rename = "male")]
    Male,
    /// Female gender  
    #[serde(rename = "female")]
    Female,
    /// Other gender
    #[serde(rename = "other")]
    Other,
    /// Unknown gender
    #[serde(rename = "unknown")]
    Unknown,
}
```

### Choice Types (value[x])

FHIR choice types are expanded into multiple typed fields:

```rust
pub struct Observation {
    #[serde(flatten)]
    pub base: DomainResource,
    
    pub status: String,
    pub code: CodeableConcept,
    
    // Choice type fields - use the appropriate one
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    #[serde(rename = "valueString")]
    pub value_string: Option<String>,
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<bool>,
    // ... other choice type variants
}
```

## Usage Examples

### Creating Resources

```rust
use serde_json;

// Create a new patient
let patient = Patient {
    base: DomainResource::default(),
    name: Some(vec![HumanName {
        family: Some("Doe".to_string()),
        given: Some(vec!["John".to_string()]),
        ..Default::default()
    }]),
    birth_date: Some("1990-01-15".to_string()),
    gender: Some(AdministrativeGender::Male),
    telecom: Some(vec![ContactPoint {
        system: Some("email".to_string()),
        value: Some("john.doe@example.com".to_string()),
        ..Default::default()
    }]),
};

// Serialize to JSON
let json = serde_json::to_string_pretty(&patient)?;
```

### Parsing Resources

```rust
use serde_json;

let json_data = r#"
{
  "resourceType": "Patient",
  "id": "example",
  "name": [{
    "family": "Smith",
    "given": ["Jane"]
  }],
  "birthDate": "1985-06-23",
  "gender": "female"
}
"#;

let patient: Patient = serde_json::from_str(json_data)?;
println!("Patient name: {:?}", patient.name);
```

### Type-Safe Pattern Matching

```rust
match patient.gender {
    Some(AdministrativeGender::Male) => println!("Male patient"),
    Some(AdministrativeGender::Female) => println!("Female patient"), 
    Some(AdministrativeGender::Other) => println!("Other gender"),
    Some(AdministrativeGender::Unknown) | None => println!("Gender not specified"),
}
```

## Generated Code Features

### Primitive Field Extensions

For FHIR primitive fields that can have extensions, the generator creates companion extension fields:

```rust
pub struct Patient {
    // Primitive field
    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,
    
    // Extension field for metadata and extensions on the primitive
    #[serde(rename = "_birthDate")]
    pub birth_date_ext: Option<Element>,
}
```

The `Element` type allows attaching extensions to primitive values while maintaining FHIR compliance.

## FHIR Package Management

### Registry Support

The library supports downloading FHIR packages from npm-style registries:

- **FHIR Package Registry**: https://packages.fhir.org (default)
- **Custom npm-style registries**: Any registry following npm package conventions
- **Private registries**: With authentication token support

### Download Process

1. **Registry Interaction**: Fetches package metadata from npm-style registries using HTTP
2. **Package Download**: Downloads the package as a compressed tarball (.tgz)
3. **Extraction**: Extracts JSON files from the tarball
4. **Processing**: Identifies StructureDefinition files and generates Rust types
5. **Code Generation**: Uses the same generation pipeline as file-based processing

### Example: Package Download and Processing

```rust
use rh_codegen::{CodeGenerator, CodegenConfig};
use rh_foundation::loader::{PackageLoader, LoaderConfig};
use std::path::Path;

async fn process_fhir_package() -> Result<(), Box<dyn std::error::Error>> {
    // Configure package loader
    let loader_config = LoaderConfig {
        registry_url: "https://packages.fhir.org".to_string(),
        auth_token: None,
        timeout_seconds: 30,
        max_retries: 3,
        verify_checksums: false,
    };
    
    // Configure code generator
    let codegen_config = CodegenConfig::default();
    let mut generator = CodeGenerator::new(codegen_config);
    
    // Download package
    let loader = PackageLoader::new(loader_config)?;
    let _manifest = loader.download_package("hl7.fhir.r4.core", "4.0.1", Path::new("./packages")).await?;
    
    // Extract and process StructureDefinitions
    // Note: You would need to implement structure definition extraction from the downloaded package
    // let structure_defs = extract_structure_definitions_from_package(Path::new("./packages"))?;
    
    // Generate Rust types for each StructureDefinition
    for structure_def in structure_defs {
        let rust_struct = generator.generate_struct(&structure_def)?;
        let tokens = generator.generate_tokens(&rust_struct);
        
        let output_path = format!("./generated/{}.rs", rust_struct.name.to_lowercase());
        generator.write_to_file(&tokens, Path::new(&output_path))?;
    }
    
    Ok(())
}
```

## Library Architecture

The crate is organized into modular components with specialized generators:

### Core Modules

- **`generator.rs`**: Main orchestrating code generator
  - `CodeGenerator`: Coordinates specialized sub-generators with built-in caching
  - `CodegenConfig`: Configuration for type generation
  - Type and enum caching to avoid regenerating identical structures

- **`generators/`**: Specialized generation modules
  - `TokenGenerator`: Core token generation using proc-macro2 and quote
  - `StructGenerator`: FHIR struct to Rust struct generation
  - `TraitGenerator`: FHIR resource trait generation
  - `EnumGenerator`: Value set enum generation
  - `FieldGenerator`: Struct field generation with type mapping
  - `CrateGenerator`: Full crate structure generation
  - `FileGenerator`: File organization and writing
  - `NameGenerator`: Rust-idiomatic naming conventions
  - `TypeUtilities`: Type analysis and mapping utilities

- **`rh-foundation::loader`**: Package downloading functionality (module in rh-foundation)
  - `PackageLoader`: HTTP client for registry interaction
  - `LoaderConfig`: Configuration for registry access
  - Tarball download and extraction
  - Registry response parsing

- **`lib.rs`**: Public API and error definitions
  - Re-exports from modules for convenience
  - Centralized error types (`CodegenError`)
  - Common result type (`CodegenResult<T>`)

### Supporting Modules

- **`config.rs`**: Configuration management and serialization
- **`fhir_types.rs`**: FHIR type definitions and parsing
- **`rust_types.rs`**: Rust type generation and formatting
- **`type_mapper.rs`**: FHIR to Rust type mapping logic
- **`value_sets.rs`**: Value set handling and enum generation

### Error Handling

All functions return `CodegenResult<T>` which is an alias for `anyhow::Result<T>`:

```rust
use rh_codegen::{CodegenResult, CodeGenerator, CodegenConfig};

fn my_function() -> CodegenResult<()> {
    // Create configuration
    let config = CodegenConfig::default();
    
    // Create generator
    let generator = CodeGenerator::new(config);
    
    // Use the generator for code generation tasks
    // ... generator operations
    
    Ok(())
}
```

## Features

### Implemented

- Parse FHIR StructureDefinition JSON
- Generate Rust structs with proper field types
- Handle optional fields (min: 0)
- Handle array fields (max: "*" or > 1)
- FHIR primitive type mappings
- Enum generation for required value set bindings
- serde rename for snake_case fields
- Documentation generation with proper Rust doc comment formatting
- FHIR package downloading from npm-style registries
- Automatic extraction and processing of package tarballs
- Authentication support for private registries
- Type caching to avoid duplicate generation
- Modular generator architecture for maintainability
- Compile-time metadata generation for type resolution
- Primitive field extension support

### In Progress

- Complex type references and inheritance
- Dynamic ValueSet fetching from FHIR servers
- Extension handling and extension definitions
- Profile validation and constraint checking

### Planned

- Custom type generators for dates, URIs, etc.
- Integration with FHIR validation libraries
- Support for FHIR GraphQL schema generation
- Performance optimizations for large packages

## Testing

Run the test suite:

```bash
cargo test -p codegen
```

The tests cover:
- StructureDefinition parsing
- Rust type generation
- Configuration loading
- Package downloading (with mocked HTTP)
- Enum generation for value sets
- Error handling and edge cases

## CLI Tool

This library is used by the `rh-cli` CLI tool in `apps/rh-cli/`. See the CLI documentation for command-line usage examples.

## Contributing

To contribute to the rh-codegen library:

1. **Code Style**: Follow the project's Rust conventions (see `.github/copilot-instructions.md`)
2. **Testing**: Add tests for new functionality
3. **Documentation**: Update doc comments and this README
4. **Error Handling**: Use `anyhow::Result` with proper context
5. **Performance**: Consider caching and optimization for large-scale generation

### Development Commands

```bash
# Run tests
cargo test -p rh-codegen

# Check formatting  
cargo fmt -p rh-codegen

# Run clippy
cargo clippy -p rh-codegen --all-targets --all-features

# Generate documentation
cargo doc -p rh-codegen --open
```

The generated code is designed to be idiomatic Rust that integrates seamlessly with the serde ecosystem for JSON serialization and deserialization.
