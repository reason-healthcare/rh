# FHIR Code Generation

This directory contains tools for generating Rust types from FHIR StructureDefinition JSON files.

## Overview

The codegen library (`crates/codegen`) provides functionality to:
- Parse FHIR StructureDefinition JSON files
- Generate corresponding Rust struct definitions
- Handle FHIR primitive type mappings
- Support optional fields and arrays
- Generate serde annotations for JSON serialization/deserialization
- **Generate type-safe enums for required value set bindings**
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
cargo run -p fhir-codegen -- install hl7.fhir.r4.core 4.0.1 -o ./generated

# Use custom configuration
cargo run -p fhir-codegen -- install hl7.fhir.r4.core 4.0.1 \
  -c my-config.json \
  -o ./generated
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
    #[serde(flatten)]
    /// Inherited fields from base type
    pub base: DomainResource,
    
    /// A name associated with the individual
    pub name: Option<Vec<HumanName>>,
    
    /// The date of birth for the individual
    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,
    
    /// Administrative Gender
    pub gender: Option<String>,
    
    /// Contact details for the individual
    pub telecom: Option<Vec<ContactPoint>>,
}
```

## Using Generated Types

The generated types are designed to work seamlessly with serde for JSON serialization and deserialization:

### Creating Patient Resources

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
    gender: Some("male".to_string()),
    telecom: Some(vec![ContactPoint {
        system: Some("email".to_string()),
        value: Some("john.doe@example.com".to_string()),
        ..Default::default()
    }]),
};

// Serialize to JSON
let json = serde_json::to_string_pretty(&patient)?;
println!("Patient JSON:\n{}", json);
```

### Parsing Patient Resources

```rust
use serde_json;

// Parse from FHIR JSON
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
println!("Birth date: {:?}", patient.birth_date);
```

### Working with Nested Structures

The generator creates properly typed nested structures for complex FHIR elements:

```rust
// Working with StructureMap nested types
let structure_map = StructureMap {
    base: DomainResource::default(),
    url: "http://example.org/fhir/StructureMap/example".to_string(),
    status: "active".to_string(),
    structure: Some(vec![StructureMapStructure {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        mode: "source".to_string(),
        alias: Some("Patient".to_string()),
        documentation: Some("Source patient resource".to_string()),
    }]),
    group: vec![StructureMapGroup {
        name: "PatientTransform".to_string(),
        type_mode: "none".to_string(),
        documentation: Some("Transform patient data".to_string()),
        input: vec![StructureMapGroupInput {
            name: "source".to_string(),
            type_: Some("Patient".to_string()),
            mode: "source".to_string(),
            documentation: Some("Source patient".to_string()),
        }],
        rule: vec![],
    }],
};
```

### Choice Types (value[x])

FHIR choice types are expanded into multiple typed fields:

```rust
// For an Observation with value[x], you get:
let observation = Observation {
    base: DomainResource::default(),
    status: "final".to_string(),
    code: CodeableConcept::default(),
    
    // Choice type fields - use the appropriate one
    value_quantity: Some(Quantity {
        value: Some(98.6),
        unit: Some("F".to_string()),
        system: Some("http://unitsofmeasure.org".to_string()),
        ..Default::default()
    }),
    value_string: None,
    value_boolean: None,
    // ... other choice type variants
};
```

### Reserved Keyword Handling

Field names that conflict with Rust keywords are automatically renamed:

```rust
// FHIR 'type' field becomes 'type_' with serde rename
let structure_def = StructureDefinition {
    base: DomainResource::default(),
    url: "http://example.org/StructureDefinition/Example".to_string(),
    name: "Example".to_string(),
    status: "active".to_string(),
    kind: "resource".to_string(),
    abstract_: false,  // 'abstract' field renamed to 'abstract_'
    type_: "Example".to_string(),  // 'type' field renamed to 'type_'
    // ... other fields
};

// JSON serialization automatically uses correct field names
assert_eq!(
    serde_json::to_value(&structure_def)["abstract"],
    serde_json::Value::Bool(false)
);
assert_eq!(
    serde_json::to_value(&structure_def)["type"], 
    serde_json::Value::String("Example".to_string())
);
```

### Error Handling

The generated types integrate with standard Rust error handling:

```rust
use anyhow::Result;

fn process_patient_data(json: &str) -> Result<String> {
    let patient: Patient = serde_json::from_str(json)
        .context("Failed to parse patient JSON")?;
    
    let name = patient.name
        .and_then(|names| names.first().cloned())
        .and_then(|name| name.family)
        .ok_or_else(|| anyhow::anyhow!("Patient has no family name"))?;
    
    Ok(format!("Patient: {}", name))
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
| code | String | Becomes enum for required value set bindings |
| id, oid | String | |
| base64Binary | String | Could use Vec<u8> |
| markdown | String | |

Complex FHIR types (starting with uppercase) are converted to PascalCase Rust struct names.

## Enum Generation for Value Sets

The generator automatically creates Rust enums for `code` fields that have **required** value set bindings. This provides compile-time type safety for fields that must only contain specific values.

### Example: Patient Gender

For the FHIR Patient resource, the `gender` field has a required binding to the `administrative-gender` value set:

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

pub struct Patient {
    // ... other fields
    pub gender: Option<AdministrativeGender>,
    // ... other fields  
}
```

### Usage with Type Safety

```rust
use serde_json;

// Create a patient with type-safe gender
let patient = Patient {
    base: DomainResource::default(),
    gender: Some(AdministrativeGender::Female),
    // ... other fields
};

// Serialization works correctly
let json = serde_json::to_string(&patient)?;
// Results in: {"gender": "female", ...}

// Deserialization validates the value
let json_str = r#"{"gender": "male"}"#;  
let parsed: Patient = serde_json::from_str(json_str)?; // ✅ Works

let invalid_json = r#"{"gender": "invalid"}"#;
let result: Result<Patient, _> = serde_json::from_str(invalid_json); // ❌ Fails
```

### Binding Strength Requirements

Only `code` fields with **required** binding strength generate enums:

- **Required**: ✅ Generates enum (e.g., `Patient.gender`)
- **Extensible**: ❌ Uses `String` (allows additional values)
- **Preferred**: ❌ Uses `String` (non-binding recommendation)
- **Example**: ❌ Uses `String` (for demonstration only)

### Known Value Sets

The generator includes built-in enum variants for common FHIR value sets:

- `administrative-gender`: Male, Female, Other, Unknown
- `publication-status`: Draft, Active, Retired, Unknown  
- `structure-definition-kind`: PrimitiveType, ComplexType, Resource, Logical
- `type-derivation-rule`: Specialization, Constraint
- `extension-context-type`: Fhirpath, Element, Extension
- `FHIR-version`: R4, R5

For unknown value sets, a minimal enum with an `Unknown` variant is generated. In a production implementation, the generator could be extended to fetch actual ValueSet definitions from a FHIR server.

### Benefits

1. **Compile-time Safety**: Invalid values are caught at compile time
2. **IntelliSense**: IDEs can provide autocomplete for valid values
3. **Documentation**: Enum variants serve as inline documentation
4. **Serialization**: Automatic conversion between Rust enums and JSON strings
5. **Pattern Matching**: Use Rust's powerful pattern matching on enum values

```rust
match patient.gender {
    Some(AdministrativeGender::Male) => println!("Male patient"),
    Some(AdministrativeGender::Female) => println!("Female patient"), 
    Some(AdministrativeGender::Other) => println!("Other gender"),
    Some(AdministrativeGender::Unknown) | None => println!("Gender not specified"),
}
```

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
- ✅ **Enum generation for required value set bindings**
- ✅ serde rename for snake_case fields
- ✅ Documentation generation
- ✅ CLI tool for batch processing
- ✅ **FHIR package downloading from npm-style registries**
- ✅ **Automatic extraction and processing of package tarballs**
- ✅ **Authentication support for private registries**
- ✅ **Install command for download + generation workflow**
- 🔄 Complex type references (in progress)
- 🔄 Dynamic ValueSet fetching from FHIR servers
- 🔄 Extension handling
- 🔄 Profile validation

## Contributing

To add new features or fix bugs:

1. Update the `codegen` library in `crates/codegen/`
2. Add tests for new functionality
3. Update the CLI tool if needed in `apps/fhir-codegen/`
4. Update this documentation

The generated code aims to be idiomatic Rust that integrates well with the serde ecosystem for JSON handling.
