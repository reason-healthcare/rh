# RH Validator Examples

This directory contains runnable examples demonstrating the RH FHIR validator functionality.

## Running Examples

Run any example with:
```bash
cargo run -p rh-validator --example <example_name>
```

For example:
```bash
cargo run -p rh-validator --example basic_validation
```

## Import Pattern

When working with FHIR resources, use the idiomatic import pattern:

```rust
// Option 1: Import resource module with re-exported traits
use hl7_fhir_r4_core::resources::patient::{Patient, PatientMutators};
use hl7_fhir_r4_core::prelude::*;  // Gets base traits

// Option 2: Just use prelude for base traits
use hl7_fhir_r4_core::prelude::*;
use hl7_fhir_r4_core::resources::patient::Patient;
```

The prelude includes commonly used traits:
- `ValidatableResource` - For accessing invariants
- `ResourceMutators` - For building any resource
- `DomainResourceMutators` - For building domain resources

## Examples

### Basic Validation

- **`basic_validation.rs`** - Simple resource validation from JSON strings
- **`structural_validation.rs`** - Demonstrates structural validation catching type errors
- **`invariant_validation.rs`** - Shows FHIRPath invariant evaluation (uses prelude)

### Advanced Usage

- **`custom_config.rs`** - Configure validator with custom settings
- **`error_handling.rs`** - Handle validation errors and warnings

### Real-World Scenarios

- **`patient_validation.rs`** - Complete Patient resource validation examples
- **`resource_builder.rs`** - Build FHIR resources programmatically (uses new import pattern)

## Testing Examples

Examples are automatically tested as part of CI to ensure they compile and run:

```bash
# Test that all examples compile and run
just test-examples

# Run a specific example
cargo run -p rh-validator --example basic_validation
```
