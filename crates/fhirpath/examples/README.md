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
```

Enable detailed logging for any example:
```bash
RUST_LOG=debug cargo run -p fhirpath --example basic_demo
```

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
