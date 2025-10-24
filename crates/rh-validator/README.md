# FHIR Validator

A comprehensive FHIR (Fast Healthcare Interoperability Resources) validation library written in Rust. This library provides multi-layered validation capabilities for FHIR resources, from basic JSON syntax checking to advanced profile-based conformance validation.

## Features

### ✅ Currently Implemented

#### JSON Syntax Validation
- **Complete JSON parsing and validation** with detailed error reporting
- **Line and column error positioning** for precise debugging
- **Structural validation** including nesting depth limits and schema checks
- **Batch processing** support for NDJSON (newline-delimited JSON) files
- **Multiple output formats** (human-readable text and structured JSON)
- **Configurable validation parameters** (max depth, strict mode, etc.)

### 🚧 Planned Features

#### FHIR Resource Validation
- **Resource type validation** against FHIR specification
- **FHIR version compliance** checking (R4, R5, etc.)
- **Base resource structure** validation (required fields, data types)
- **Reference validation** (internal and external references)
- **Extension validation** (standard and custom extensions)

#### Profile-Based Validation
- **StructureDefinition conformance** validation
- **Cardinality constraints** enforcement (min/max occurrences)
- **Data type validation** with FHIR primitive and complex types
- **Terminology binding validation** (required, extensible, preferred)
- **FHIRPath constraint evaluation** (invariants and custom rules)
- **Slice validation** for profiled elements

#### Advanced Validation Features
- **Bundle validation** with entry relationships
- **Terminology server integration** for code validation
- **Custom validation rules** and business logic
- **Severity levels** (error, warning, information)
- **Validation context** tracking for complex scenarios
- **Performance optimization** for large datasets

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
rh-validator = { path = "../path/to/rh-validator" }
```

### Basic Usage

```rust
use hl7_fhir_r4_core::resources::patient::Patient;
use rh_validator::FhirValidator;

// Create a validator
let validator = FhirValidator::new()?;

// Validate a FHIR Patient resource
let patient_json = r#"{
    "resourceType": "Patient",
    "id": "example",
    "extension": [{
        "url": "http://example.org/test",
        "valueString": "test"
    }],
    "name": [{
        "family": "Doe",
        "given": ["John"]
    }],
    "gender": "male"
}"#;

let result = validator.validate_full::<Patient>(patient_json)?;
if result.is_valid() {
    println!("✅ Patient is valid");
} else {
    println!("❌ Validation failed with {} errors", result.error_count());
    for issue in &result.issues {
        println!("  - {}: {}", issue.code, issue.details);
    }
}
```

### Advanced Configuration

```rust
use rh_validator::{FhirValidator, ValidatorConfig};

// Custom validator configuration
let config = ValidatorConfig::new()
    .with_max_depth(128)
    .with_skip_invariants(false);

let validator = FhirValidator::with_config(config)?;

// Validate with full structural + invariant checks
let result = validator.validate_full::<Patient>(patient_json)?;
```

## Examples

The crate includes comprehensive examples in the [`examples/`](examples/) directory:

- **`basic_validation.rs`** - Simple resource validation from JSON strings
- **`structural_validation.rs`** - Demonstrates structural validation catching type errors  
- **`invariant_validation.rs`** - Shows FHIRPath invariant evaluation
- **`custom_config.rs`** - Configure validator with custom settings
- **`error_handling.rs`** - Handle validation errors and warnings
- **`patient_validation.rs`** - Complete Patient resource validation examples

Run any example with:
```bash
cargo run -p rh-validator --example basic_validation
```

Test all examples:
```bash
just test-examples
```

## Command Line Interface

The validator integrates with the `rh` CLI tool:

```bash
# Validate JSON syntax from file
rh validate json --input patient.json

# Validate from stdin with JSON output
echo '{"resourceType": "Patient"}' | rh validate json --format json

# Validate multiple resources (NDJSON)
rh validate json --input bundle.ndjson --multiple

# Strict mode (exit with error on validation failure)
rh validate json --input data.json --strict

# Custom validation parameters
rh validate json --input deep.json --max-depth 20 --stats
```

### CLI Options

- `--input, -i`: Input file path (reads from stdin if not provided)
- `--format, -f`: Output format (`text` or `json`)
- `--multiple`: Process as NDJSON (multiple JSON documents)
- `--max-depth`: Maximum allowed nesting depth (default: 100)
- `--stats`: Show detailed statistics for valid JSON
- `--strict`: Exit with non-zero code on validation failure

## Validation Layers

### 1. Syntax Validation (✅ Implemented)

The first layer ensures the input is valid JSON:

- **JSON parsing** with detailed error messages
- **Structural integrity** checks (balanced brackets, proper escaping)
- **Schema validation** (empty keys, infinite numbers, excessive nesting)
- **Performance safeguards** (string length limits, depth limits)

Example errors:
- `JSON syntax error: expected ',' or ']' at line 5, column 12`
- `Schema validation error: Maximum nesting depth of 100 exceeded`

### 2. FHIR Resource Validation (🚧 Planned)

Validates that JSON represents a valid FHIR resource:

- **Resource type recognition** (Patient, Observation, etc.)
- **Required field validation** (resourceType, mandatory elements)
- **Data type conformance** (strings, integers, dates, codes)
- **FHIR version compatibility** (R4, R5 differences)
- **Reference format validation** (Resource/id, #fragment)

Expected validation capabilities:
```rust
// Future API
let fhir_validator = FhirValidator::new("4.0.1")?;
let result = fhir_validator.validate_resource(json_content)?;
```

### 3. Profile Validation (🚧 Planned)

Advanced conformance validation against StructureDefinitions:

- **Profile conformance** (US Core, IPA, custom profiles)
- **Cardinality enforcement** (0..1, 1..*, specific ranges)
- **Terminology binding** validation with terminology servers
- **FHIRPath constraint** evaluation (invariants)
- **Extension validation** (known extensions, custom extensions)
- **Slice discrimination** and validation

Expected validation workflow:
```rust
// Future API
let profile_validator = ProfileValidator::new()
    .with_structure_definition(us_core_patient_profile)?
    .with_terminology_server("https://tx.fhir.org")?;

let result = profile_validator.validate(patient_json)?;
```

## Error Types and Reporting

### Current Error Types

```rust
pub enum ValidationError {
    JsonSyntax { message: String, line: usize, column: usize },
    Schema { message: String },
}
```

### Planned Error Types

```rust
pub enum ValidationError {
    // Syntax errors
    JsonSyntax { message: String, line: usize, column: usize },
    Schema { message: String },

    // FHIR resource errors
    MissingField { field: String, path: String },
    InvalidResourceType { found: String, expected: Vec<String> },
    InvalidFieldValue { field: String, value: String, reason: String },
    VersionMismatch { expected: String, found: String },

    // Profile validation errors
    CardinalityViolation { path: String, expected: String, found: usize },
    TerminologyBinding { path: String, code: String, system: String },
    FhirPathConstraint { path: String, expression: String, message: String },
    ProfileConformance { profile: String, element: String, issue: String },
}
```

## Integration Examples

### Healthcare System Integration

```rust
use rh_validator::{FhirValidator, ValidationConfig, ValidationSeverity};

// Configure validator for production use
let config = ValidationConfig {
    fhir_version: "4.0.1".to_string(),
    strict_mode: true,
    validate_terminology: true,
    terminology_server: Some("https://tx.fhir.org".to_string()),
    profiles: vec![
        "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string(),
    ],
};

let validator = FhirValidator::new(config)?;

// Validate incoming FHIR data
let validation_result = validator.validate_resource(&incoming_json)?;
match validation_result.severity() {
    ValidationSeverity::Valid => process_resource(&incoming_json),
    ValidationSeverity::Warning => {
        log_warnings(&validation_result.warnings());
        process_resource(&incoming_json);
    },
    ValidationSeverity::Error => {
        return Err(format!("Validation failed: {:?}", validation_result.errors()));
    }
}
```

### Batch Processing Pipeline

```rust
use rh_validator::JsonValidator;
use std::fs::File;
use std::io::{BufRead, BufReader};

let validator = JsonValidator::new();
let file = File::open("fhir_bundle.ndjson")?;
let reader = BufReader::new(file);

let mut valid_count = 0;
let mut errors = Vec::new();

for (line_num, line) in reader.lines().enumerate() {
    let line = line?;
    match validator.validate(&line)? {
        ValidationResult::Valid => valid_count += 1,
        ValidationResult::Invalid(line_errors) => {
            errors.push((line_num + 1, line_errors));
        }
    }
}

println!("Processed {} valid resources", valid_count);
if !errors.is_empty() {
    println!("Found {} invalid resources:", errors.len());
    for (line_num, line_errors) in errors {
        println!("  Line {}: {} errors", line_num, line_errors.len());
    }
}
```

## Performance Considerations

### Current Performance Features

- **Streaming JSON parsing** with minimal memory allocation
- **Configurable depth limits** to prevent stack overflow
- **Early termination** on first syntax error (when appropriate)
- **Batch processing** support for large datasets

### Planned Optimizations

- **Profile caching** for repeated validation scenarios
- **Parallel validation** for independent resources
- **Incremental validation** for large bundles
- **Memory-mapped file** support for extremely large datasets
- **Validation result caching** for identical resources

## FHIR Version Support

### Planned Support Matrix

| FHIR Version | Status | Notes |
|--------------|--------|-------|
| DSTU2 | 🚧 Planned | Legacy support |
| STU3 | 🚧 Planned | Limited support |
| R4 (4.0.1) | 🚧 Planned | Primary target |
| R4B (4.3.0) | 🚧 Planned | Full support |
| R5 (5.0.0) | 🚧 Planned | Latest standard |

### Version-Specific Features

- **Automatic version detection** from resource metadata
- **Cross-version compatibility** warnings
- **Version-specific validation rules** and constraints
- **Migration assistance** for version upgrades

## Related Projects

- **rh-codegen** - Generate Rust types from FHIR StructureDefinitions
- **rh-fhirpath** - FHIRPath expression evaluation for constraints
- **rh-cli** - Command-line interface for FHIR processing tools

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

## Roadmap

### Phase 1: Foundation (✅ Complete)
- [x] JSON syntax validation
- [x] CLI integration
- [x] Error reporting system
- [x] Basic testing framework

### Phase 2: FHIR Basics (In Progress)
- [ ] Resource type validation
- [ ] Element validation
- [ ] Reference format checking

### Phase 3: Profile Validation (Planned)
- [ ] StructureDefinition parsing
- [ ] Cardinality validation
- [ ] Data type validation
- [ ] FHIRPath constraint evaluation

### Phase 4: Advanced Features (Planned)
- [ ] Terminology server integration
- [ ] Bundle validation
- [ ] Custom validation rules
- [ ] Performance optimization

### Phase 5: Production Features (Future)
- [ ] Validation caching
- [ ] Parallel processing
- [ ] Integration APIs
- [ ] Monitoring and metrics
