# RH Validator Design Document

## Overview

The RH validator provides fast, accurate, and parallel validation of FHIR resources against FHIR specifications. It leverages Rust's type system, the generated FHIR types, and the FHIRPath engine to deliver comprehensive validation with excellent performance.

## Design Goals

1. **Speed**: Leverage Rust's performance and enable parallel processing for batch validation
2. **Accuracy**: Validate both structural conformance and business rules (invariants)
3. **Scalability**: Support multi-threaded validation of large resource sets
4. **Type Safety**: Utilize generated Rust types for compile-time structural guarantees
5. **Extensibility**: Support custom validation rules and profiles

## Architecture

### Separation of Concerns

The validation functionality is split across two main components:

#### 1. Generated Crate (`rh-hl7_fhir_r4_core`)

**Responsibilities:**
- Structural validation through Rust's type system
- Compile-time guarantees for required fields and cardinality
- Type-safe deserialization with immediate structural validation
- Embedded invariant metadata (FHIRPath expressions and severity levels)
- Auto-generated trait implementations for validation support

**Generated Code Includes:**
```rust
// Each resource/datatype file
pub struct Patient {
    // Fields with proper types and Option<T> for optional fields
}

impl Patient {
    /// Returns all FHIRPath invariants for this resource
    pub fn invariants() -> &'static [Invariant] {
        &[
            Invariant {
                key: "pat-1",
                severity: Severity::Error,
                human: "SHALL at least contain a contact's details or a reference to an organization",
                expression: "contact.name.exists() or contact.telecom.exists() or contact.address.exists() or contact.organization.exists()",
            },
            // ... more invariants
        ]
    }
}

// Trait for validation support
pub trait ValidatableResource {
    fn resource_type() -> &'static str;
    fn invariants() -> &'static [Invariant];
}

impl ValidatableResource for Patient {
    fn resource_type() -> &'static str { "Patient" }
    fn invariants() -> &'static [Invariant] { Self::invariants() }
}
```

**Why NOT include a `validate()` method on generated types:**

1. **Separation of Concerns**: The generated crate should be dependency-free (only serde), while validation requires `rh-fhirpath` engine
2. **Circular Dependencies**: Generated types are used by the validator, not the other way around
3. **Performance**: Users might want to validate 10,000 resources with a single FHIRPath engine instance, not create 10,000 instances
4. **Flexibility**: Different validation contexts need different configurations (strict mode, profile validation, custom rules)
5. **Type Purity**: Generated types should be simple data structures, not behavior-heavy objects

**Benefits:**
- Zero runtime cost for structural validation (handled by serde)
- Invariants are version-locked with the resource definitions
- No runtime parsing of StructureDefinitions needed
- Type-safe API for validation consumers

#### 2. Validator Crate (`rh-validator`)

**Responsibilities:**
- Orchestrating the validation process
- Executing FHIRPath invariants using `rh-fhirpath`
- Collecting and reporting validation results
- Parallel validation coordination
- Profile validation (beyond core resources)
- Custom validation rules
- Validation result aggregation and formatting

**Core Components:**

```rust
// Main validator coordinating all validation
pub struct FhirValidator {
    fhirpath_engine: FhirPathEngine,
    thread_pool: Option<ThreadPool>,
    config: ValidatorConfig,
}

// Configuration options
pub struct ValidatorConfig {
    pub max_threads: usize,
    pub validate_invariants: bool,
    pub validate_profiles: bool,
    pub fail_fast: bool,
    pub severity_threshold: Severity,
}

// Validation results
pub struct ValidationResult {
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub issues: Vec<ValidationIssue>,
}

pub struct ValidationIssue {
    pub severity: Severity,
    pub code: IssueCode,
    pub details: String,
    pub expression: Option<String>, // FHIRPath expression for invariants
    pub location: Option<String>,   // JSON path to problematic element
}

pub enum Severity {
    Error,      // Validation failure
    Warning,    // Best practice violation
    Information // Informational message
}
```

## Validation API Design

### Recommended Usage Pattern

Instead of `patient.validate()`, the validator provides a clean external API:

```rust
use rh_validator::FhirValidator;
use rh_hl7_fhir_r4_core::resources::Patient;

// Create validator once (reusable across many resources)
let validator = FhirValidator::new()?;

// Validate from Rust type
let patient: Patient = serde_json::from_str(json)?;
let result = validator.validate(&patient)?;

// Or validate from JSON directly (combines parsing + validation)
let result = validator.validate_json::<Patient>(json)?;

// Batch validation (parallel)
let results = validator.validate_batch::<Patient>(json_array)?;
```

### Why This Design is Superior

**1. Performance:**
```rust
// ❌ BAD: Creates FHIRPath engine for each resource
for json in resources {
    let patient: Patient = serde_json::from_str(json)?;
    let result = patient.validate()?; // New engine every time!
}

// ✅ GOOD: Reuses FHIRPath engine
let validator = FhirValidator::new()?;
for json in resources {
    let patient: Patient = serde_json::from_str(json)?;
    let result = validator.validate(&patient)?; // Shared engine
}

// ✅ BEST: Parallel validation with shared engine pool
let validator = FhirValidator::new()?;
let results = validator.validate_batch::<Patient>(resources)?;
```

**2. Flexibility:**
```rust
// Different validation contexts without changing generated code
let strict_validator = FhirValidator::builder()
    .strict_mode(true)
    .min_severity(Severity::Warning)
    .build()?;

let lenient_validator = FhirValidator::builder()
    .skip_invariants(true)
    .build()?;

// Same Patient type works with both
let result1 = strict_validator.validate(&patient)?;
let result2 = lenient_validator.validate(&patient)?;
```

**3. Dependency Management:**
```rust
// Generated crate: minimal dependencies
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

// Validator crate: full dependencies
[dependencies]
rh-hl7_fhir_r4_core = { path = "../rh-hl7_fhir_r4_core" }
rh-fhirpath = { path = "../rh-fhirpath" }
rayon = "1.7"
# ... other validation-specific deps
```

### Alternative: Extension Trait Pattern (Advanced Users)

For users who really want `patient.validate()` syntax, we can provide an optional extension trait:

```rust
// In rh-validator crate
pub trait ValidateExt: ValidatableResource + Serialize {
    fn validate_with(&self, validator: &FhirValidator) -> Result<ValidationResult> {
        validator.validate(self)
    }
}

// Blanket implementation
impl<T: ValidatableResource + Serialize> ValidateExt for T {}

// Usage (requires explicit validator)
let validator = FhirValidator::new()?;
let result = patient.validate_with(&validator)?;
```

**Benefits of this approach:**
- Optional (requires importing trait)
- Still requires passing validator (makes dependency explicit)
- Enables method-call syntax for those who prefer it
- Zero cost abstraction (compiles to same code)

### Comparison with Other FHIR Libraries

**HAPI FHIR (Java):**
```java
// HAPI has validate() on resources, but:
// - Requires validator instance passed as parameter
// - Heavy object with lots of state
Patient patient = new Patient();
FhirValidator validator = ctx.newValidator();
ValidationResult result = validator.validateWithResult(patient);
```

**FHIR.js (JavaScript):**
```javascript
// Most JS libraries use external validation
const fhir = require('fhir');
const validator = new fhir.Validator();
const result = validator.validate(patient);
```

**Our approach is more like Rust's design philosophy:**
- Data and behavior are separate
- Dependencies are explicit
- Performance is predictable
- Composition over inheritance

## Validation Pipeline

### Single Resource Validation

```
1. JSON Input
   ↓
2. Deserialize to Rust Type (serde_json)
   ↓ [Structural validation happens here automatically]
3. Type-specific Invariant Validation
   ↓
4. Profile Validation (if specified)
   ↓
5. Custom Rule Validation (if configured)
   ↓
6. Aggregate Results
   ↓
7. Return ValidationResult
```

### Batch Validation (Parallel)

```
1. Read NDJSON or JSON array
   ↓
2. Split into chunks
   ↓
3. Distribute to thread pool
   ↓ [Each thread runs single resource pipeline]
4. Collect results from all threads
   ↓
5. Aggregate and return ValidationResults
```

## Detailed Component Design

### 1. Deserialization-based Structural Validation

The Rust type system and serde provide first-level validation:

```rust
// This automatically validates:
// - Required fields are present
// - Field types match (string vs number, etc.)
// - Cardinality constraints (single vs array)
// - Nested structure conformance
let patient: Patient = serde_json::from_str(json_str)?;
```

**Error Handling:**
- Serde errors are captured and transformed into `ValidationIssue`s
- Missing required fields → Error severity
- Type mismatches → Error severity
- Unknown fields → Warning severity (configurable)

### 2. Invariant Validation

After successful deserialization, invariants are evaluated:

```rust
impl FhirValidator {
    fn validate_invariants<T: ValidatableResource + Serialize>(
        &self,
        resource: &T,
    ) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        for invariant in T::invariants() {
            match self.fhirpath_engine.evaluate(
                invariant.expression,
                resource
            ) {
                Ok(result) if !result.as_bool() => {
                    issues.push(ValidationIssue {
                        severity: invariant.severity,
                        code: IssueCode::Invariant,
                        details: invariant.human.to_string(),
                        expression: Some(invariant.expression.to_string()),
                        location: None, // Could be enhanced with FHIRPath context
                    });
                }
                Err(e) => {
                    // FHIRPath evaluation error
                    issues.push(ValidationIssue {
                        severity: Severity::Error,
                        code: IssueCode::InvariantEvaluation,
                        details: format!("Failed to evaluate invariant {}: {}", invariant.key, e),
                        expression: Some(invariant.expression.to_string()),
                        location: None,
                    });
                }
                _ => {} // Invariant passed
            }
        }
        
        issues
    }
}
```

### 3. Parallel Validation

For batch validation, use Rayon for work-stealing parallelism:

```rust
use rayon::prelude::*;

impl FhirValidator {
    pub fn validate_batch(
        &self,
        resources: Vec<String>, // JSON strings
    ) -> Vec<ValidationResult> {
        resources
            .par_iter()
            .map(|json| self.validate_json(json))
            .collect()
    }
    
    pub fn validate_ndjson(
        &self,
        ndjson: &str,
    ) -> Vec<ValidationResult> {
        ndjson
            .lines()
            .par_bridge() // Parallel iterator over lines
            .map(|line| self.validate_json(line))
            .collect()
    }
}
```

**Performance Considerations:**
- Thread pool sized based on CPU cores
- Chunk size tuning for optimal parallelism
- Memory management for large batches (streaming vs loading all)
- Progress reporting for long-running validations

### 4. Profile Validation

Profiles extend base resources with additional constraints:

```rust
pub struct ProfileValidator {
    canonical_url: String,
    additional_invariants: Vec<Invariant>,
    element_constraints: HashMap<String, ElementConstraint>,
}

pub struct ElementConstraint {
    path: String,
    min_cardinality: Option<usize>,
    max_cardinality: Option<usize>,
    fixed_value: Option<serde_json::Value>,
    pattern: Option<serde_json::Value>,
    binding: Option<ValueSetBinding>,
}
```

Profiles are validated after core resource validation passes.

## CLI Design

### Command Structure

```bash
rh validate <subcommand> [options]
```

### Subcommands

#### 1. Single Resource Validation

```bash
# Validate from file
rh validate resource -i patient.json

# Validate from stdin
cat patient.json | rh validate resource

# With profile
rh validate resource -i patient.json \
  --profile http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient

# Output formats
rh validate resource -i patient.json --format json
rh validate resource -i patient.json --format text
rh validate resource -i patient.json --format sarif
```

#### 2. Batch Validation

```bash
# Validate NDJSON file
rh validate batch -i resources.ndjson

# Validate directory of JSON files
rh validate batch -i ./fhir-resources/

# Parallel processing with custom thread count
rh validate batch -i resources.ndjson --threads 8

# Progress reporting
rh validate batch -i resources.ndjson --progress

# Summary only (don't show individual issues)
rh validate batch -i resources.ndjson --summary-only
```

#### 3. Configuration Options

```bash
# Strict mode (fail on warnings)
rh validate resource -i patient.json --strict

# Severity threshold
rh validate resource -i patient.json --min-severity error
rh validate resource -i patient.json --min-severity warning

# Skip invariant validation (structure only)
rh validate resource -i patient.json --no-invariants

# Validate against specific FHIR version
rh validate resource -i patient.json --fhir-version 4.0.1

# Custom validator configuration
rh validate resource -i patient.json --config validator.toml
```

#### 4. Output Formats

**Text Format (Default):**
```
✅ Validation passed for Patient (id: patient-123)

Found 2 issues:
  ❌ Error: SHALL at least contain a contact's details or a reference to an organization
     Invariant: pat-1
     Expression: contact.name.exists() or contact.telecom.exists()...
     
  ⚠️  Warning: Identifier system should be an absolute URL
     Location: Patient.identifier[0].system
```

**JSON Format:**
```json
{
  "resourceType": "OperationOutcome",
  "issue": [
    {
      "severity": "error",
      "code": "invariant",
      "details": {
        "text": "SHALL at least contain a contact's details..."
      },
      "expression": ["Patient.contact"],
      "diagnostics": "pat-1: contact.name.exists() or contact.telecom.exists()..."
    }
  ]
}
```

**SARIF Format (for CI/CD integration):**
```json
{
  "version": "2.1.0",
  "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
  "runs": [{
    "tool": {
      "driver": {
        "name": "rh-validator",
        "version": "0.1.0"
      }
    },
    "results": [...]
  }]
}
```

### CLI Examples

```bash
# Validate single resource with detailed output
rh validate resource -i patient.json -v

# Batch validate with progress bar and summary
rh validate batch -i ./fhir-exports/ --progress --summary-only

# Strict validation for CI/CD (exit code 1 on any warning)
rh validate batch -i resources.ndjson --strict --format json > results.json

# Validate against US Core profile
rh validate resource -i patient.json \
  --profile http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient \
  --format text

# Performance benchmark
time rh validate batch -i large-dataset.ndjson --threads 16 --no-output

# Validate and fix common issues (future enhancement)
rh validate resource -i patient.json --fix --output patient-fixed.json
```

### Exit Codes

- `0` - Validation passed (no errors)
- `1` - Validation failed (errors found)
- `2` - Invalid arguments or configuration
- `3` - File I/O error
- `4` - Internal validator error

## Implementation Phases

### Phase 1: Core Validation (Current)
- ✅ JSON structural validation
- ✅ Basic resource type detection
- ⚠️ Limited invariant support

### Phase 2: Generated Invariants
- Generate invariant metadata in `rh-hl7_fhir_r4_core`
- Implement FHIRPath evaluation for invariants
- Add `ValidatableResource` trait to generated types
- Full invariant validation support

### Phase 3: Parallel Processing
- Implement batch validation with Rayon
- Add streaming NDJSON support
- Progress reporting
- Performance optimization

### Phase 4: Profile Validation
- Load and parse StructureDefinition profiles
- Implement element constraint validation
- ValueSet binding validation
- Profile inheritance support

### Phase 5: Advanced Features
- Custom validation rules API
- Validation rule plugins
- SARIF output format
- Validation caching for repeated validations
- Auto-fix capabilities for common issues

## Code Generation Requirements

### For `rh-codegen`

The code generator needs to emit additional code for validation:

1. **Invariant Metadata:**
```rust
// In each resource/datatype file
pub const INVARIANTS: &[Invariant] = &[
    Invariant {
        key: "pat-1",
        severity: Severity::Error,
        human: "SHALL at least contain...",
        expression: "contact.name.exists()...",
        xpath: None, // Optional, for reference
    },
    // ... all invariants from StructureDefinition
];
```

2. **Trait Implementations:**
```rust
impl ValidatableResource for Patient {
    fn resource_type() -> &'static str {
        "Patient"
    }
    
    fn invariants() -> &'static [Invariant] {
        INVARIANTS
    }
    
    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Patient")
    }
}
```

3. **Element Metadata (for detailed errors):**
```rust
pub const ELEMENT_DEFINITIONS: &[ElementDefinition] = &[
    ElementDefinition {
        path: "Patient.identifier",
        min: 0,
        max: "*",
        type_: &["Identifier"],
        short: "An identifier for this patient",
    },
    // ... all elements
];
```

## Performance Targets

Based on Rust's performance characteristics:

- **Single Resource:** < 1ms for simple resources, < 10ms for complex (with invariants)
- **Batch Processing (1000 resources):** < 1 second with 8 threads
- **Memory Usage:** O(n) for batch, streaming for very large batches
- **Startup Time:** < 100ms (no JVM/CLR warmup)

## Testing Strategy

1. **Unit Tests:**
   - Individual invariant evaluation
   - Error message formatting
   - Edge cases (empty resources, deeply nested structures)

2. **Integration Tests:**
   - Full validation pipeline
   - Parallel validation correctness
   - Profile validation

3. **Conformance Tests:**
   - FHIR test suite resources
   - Known-good and known-bad examples
   - Cross-validation with official FHIR validator

4. **Performance Tests:**
   - Benchmark against official Java validator
   - Scalability testing (1, 100, 10k, 100k resources)
   - Memory profiling

## Dependencies

- `rh-foundation`: Error handling, CLI utilities
- `rh-fhirpath`: Invariant evaluation
- `rh-hl7_fhir_r4_core`: Generated types with validation metadata
- `serde_json`: JSON parsing and structural validation
- `rayon`: Parallel iteration
- `indicatif`: Progress bars (optional)
- `clap`: CLI argument parsing

## Future Enhancements

1. **Snapshot Testing:** Generate expected validation results for regression testing
2. **Validation Profiles:** User-defined validation rule sets
3. **Machine Learning:** Suggest fixes for common validation errors
4. **IDE Integration:** Real-time validation in editors via LSP
5. **Web Service:** Expose validator as HTTP API
6. **WASM Support:** Run validator in browser
7. **Differential Validation:** Validate changes between resource versions
8. **Terminology Validation:** Integrate with terminology services for CodeSystem/ValueSet validation

## Open Questions

1. **Terminology Server Integration:** Should we validate coded values against remote terminology servers?
2. **Reference Validation:** Should we validate that references point to existing resources (requires resource resolution)?
3. **Extension Validation:** How deep should we validate extensions without their StructureDefinitions?
4. **Bundle Validation:** Should we validate Bundle resources holistically (e.g., transaction semantics)?
5. **Slicing Validation:** How to handle complex slicing rules from profiles?

## Success Metrics

- ✅ Validates 100% of official FHIR test resources correctly
- ✅ 10x faster than official Java validator for batch operations
- ✅ Zero false positives on known-good resources
- ✅ All invariants from core spec are enforced
- ✅ Memory usage scales linearly with batch size
- ✅ Clean, idiomatic Rust API
