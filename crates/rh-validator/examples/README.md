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

## Examples

### Basic Validation

- **`basic_validation.rs`** - Simple resource validation from JSON strings
- **`structural_validation.rs`** - Demonstrates structural validation catching type errors
- **`invariant_validation.rs`** - Shows FHIRPath invariant evaluation

### Advanced Usage

- **`custom_config.rs`** - Configure validator with custom settings
- **`error_handling.rs`** - Handle validation errors and warnings

### Real-World Scenarios

- **`patient_validation.rs`** - Complete Patient resource validation examples
- **`resource_builder.rs`** - Build FHIR resources programmatically and validate them

## Testing Examples

Examples are automatically tested as part of CI to ensure they compile and run:

```bash
# Test that all examples compile and run
just test-examples

# Run a specific example
cargo run -p rh-validator --example basic_validation
```
