# FHIRPath Examples

This directory contains comprehensive examples demonstrating the FHIRPath crate capabilities. These examples show how to parse, evaluate, and work with FHIRPath expressions using both the library API and real FHIR data.

The examples serve as both documentation and informal tests. You can verify they work correctly by running them and checking their output.

## 🚀 Running Examples

### Prerequisites

Ensure you have built the workspace:
```bash
cargo build
```

### Running Individual Examples

From the workspace root, run any example using:

```bash
# Basic demonstration
cargo run -p fhirpath --example basic_demo

# Type operations (is/as operators)
cargo run -p fhirpath --example type_operations

# Logical implies operator
cargo run -p fhirpath --example implies_operations

# Arithmetic operations
cargo run -p fhirpath --example arithmetic_operations

# String functions
cargo run -p fhirpath --example string_functions

# Collection operations
cargo run -p fhirpath --example collection_operations

# Boolean collection functions
cargo run -p fhirpath --example boolean_collection_operations

# FHIR Patient navigation
cargo run -p fhirpath --example fhir_patient_navigation

# Math functions
cargo run -p fhirpath --example math_functions

# DateTime functions
cargo run -p fhirpath --example datetime_functions_example

# And many more - see the files in this directory
```

Enable detailed logging for any example:
```bash
RUST_LOG=debug cargo run -p fhirpath --example type_operations
```

## 📚 Available Examples

### Core Language Features
- **`basic_demo.rs`** - Basic FHIRPath parsing and evaluation
- **`type_operations.rs`** - Type checking (`is`) and casting (`as`) operations  
- **`implies_operations.rs`** - Logical implication operator demonstration
- **`arithmetic_operations.rs`** - Math operations and numeric expressions
- **`string_functions.rs`** - String manipulation and functions
- **`collection_operations.rs`** - Working with collections and arrays
- **`boolean_collection_operations.rs`** - Boolean collection functions (all, allTrue, anyTrue, etc.)
- **`union_operations.rs`** - Union operations and set logic

### Advanced Operations  
- **`math_functions.rs`** - Mathematical functions and calculations
- **`datetime_functions_example.rs`** - Date and time operations
- **`quantity_literals.rs`** - Working with FHIR quantities and units
- **`indexing_operations.rs`** - Array indexing and element access
- **`temporal_literals.rs`** - Temporal data types and operations

### FHIR-Specific Examples
- **`fhir_patient_navigation.rs`** - Navigating FHIR Patient resources
- **`temperature_conversion_example.rs`** - Unit conversion in FHIR context
- **`unit_conversion_example.rs`** - General unit conversion operations

### Verification and Testing
- **`verify_indexing.rs`** - Testing indexing functionality
- **`verify_union.rs`** - Testing union operations
- **`simple_math.rs`** - Simple mathematical expressions

## 🔗 Related Documentation

- **[FHIRPath Crate Documentation](../README.md)** - Complete library documentation
- **[Main Workspace Examples](../../../examples/README.md)** - Other workspace examples
- **[RH CLI Documentation](../../../apps/rh/README.md)** - Command-line interface
- **[FHIRPath Specification](https://hl7.org/fhirpath/)** - Official FHIRPath language specification

## 🛠️ Development

### Adding New Examples

To add a new FHIRPath example:

1. **Choose appropriate naming** following the established patterns:
   - `basic_*` for fundamental concepts
   - `[operation]_operations` for specific functionality
   - `fhir_*` for FHIR-specific examples
   - `verify_*` for testing and verification

2. **Create the example file** in this directory (e.g., `comparison_operations.rs`)
3. **Follow the example template** with comprehensive documentation
4. **Include error handling** using `anyhow::Result<()>`
5. **Add comprehensive comments** explaining each step
6. **Update this README** with a description and run instructions
7. **Test the example** to ensure it works correctly
8. **Add to testing scripts** for CI verification

### Example Template

```rust
//! FHIRPath - [Feature Name] Operations Example
//! 
//! This example demonstrates [specific functionality] including:
//! - [Key concept 1]
//! - [Key concept 2]
//! - [Key concept 3]

use anyhow::Result;
use serde_json::json;
use fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext, FhirPathValue};

fn main() -> Result<()> {
    println!("🎯 FHIRPath [Feature Name] Operations Examples");
    println!("===============================================\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({})); // or sample data

    // Example 1: Basic usage
    println!("1. Basic [feature] usage");
    // Implementation with detailed comments
    
    // Example 2: Advanced usage
    println!("\n2. Advanced [feature] patterns");
    // Implementation with edge cases
    
    // Example 3: Error handling
    println!("\n3. Error handling and edge cases");
    // Implementation showing error scenarios

    Ok(())
}
```
