# FHIRPath Examples

This directory contains comprehensive examples demonstrating the FHIRPath crate capabilities. These examples show how to parse, evaluate, and work with FHIRPath expressions using both the library API and real FHIR data.

## 🔍 Examples Overview

### [`fhirpath_demo.rs`](fhirpath_demo.rs)
**Comprehensive FHIRPath demonstration**
- Basic expression parsing and evaluation
- Working with FHIR Patient resources
- String operations and mathematical expressions
- Error handling patterns

**Run with:**
```bash
cargo run --example fhirpath_demo
```

### [`fhirpath_arithmetic.rs`](fhirpath_arithmetic.rs)
**Mathematical operations in FHIRPath**
- Addition, subtraction, multiplication, division
- Modulo operations
- Operator precedence and parentheses
- Working with numeric data in FHIR resources
- Complex mathematical expressions

**Run with:**
```bash
cargo run --example fhirpath_arithmetic
```

### [`fhirpath_strings.rs`](fhirpath_strings.rs)
**String manipulation functions**
- Length, case conversion (upper/lower)
- Substring operations
- String searching (contains, startsWith, endsWith)
- String replacement and trimming
- String concatenation
- Validation patterns

**Run with:**
```bash
cargo run --example fhirpath_strings
```

### [`fhirpath_patient_navigation.rs`](fhirpath_patient_navigation.rs)
**Working with real FHIR Patient resources**
- Basic resource navigation
- Name operations and filtering
- Contact information extraction
- Address navigation
- Identifier handling
- Emergency contact access
- Metadata operations
- Conditional expressions

**Run with:**
```bash
cargo run --example fhirpath_patient_navigation
```

### [`fhirpath_collections.rs`](fhirpath_collections.rs)
**Collection operations and array handling**
- Basic collection access
- Collection size and existence checks
- Collection filtering with where() clauses
- Field access across collections
- Mathematical operations on collections
- String operations on collections
- Working with FHIR Bundles

**Run with:**
```bash
cargo run --example fhirpath_collections
```

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
cargo run -p fhirpath --example fhirpath_demo

# Mathematical operations
cargo run -p fhirpath --example fhirpath_arithmetic

# String manipulation
cargo run -p fhirpath --example fhirpath_strings

# FHIR resource navigation
cargo run -p fhirpath --example fhirpath_patient_navigation

# Collection operations
cargo run -p fhirpath --example fhirpath_collections
```

### Running with Verbose Output

Enable detailed logging for any example:
```bash
RUST_LOG=debug cargo run -p fhirpath --example fhirpath_demo
```

### Running All FHIRPath Examples

Run all FHIRPath examples sequentially:
```bash
for example in fhirpath_demo fhirpath_arithmetic fhirpath_strings fhirpath_patient_navigation fhirpath_collections; do
    echo "Running $example..."
    cargo run -p fhirpath --example $example
    echo "Completed $example"
    echo "---"
done
```

## 🧪 Testing Examples

The examples serve as both documentation and informal tests. You can verify they work correctly by running them and checking their output.

### Example Testing Script

```bash
#!/bin/bash
# test-fhirpath-examples.sh

set -e

echo "Testing FHIRPath examples..."

examples=(
    "fhirpath_demo"
    "fhirpath_arithmetic"
    "fhirpath_strings"
    "fhirpath_patient_navigation"
    "fhirpath_collections"
)

for example in "${examples[@]}"; do
    echo "🧪 Testing: $example"
    if cargo run -p fhirpath --example "$example" > /dev/null 2>&1; then
        echo "✅ $example - PASSED"
    else
        echo "❌ $example - FAILED"
        exit 1
    fi
done

echo "🎉 All FHIRPath examples passed!"
```

## 💡 Learning Path

### For Beginners
1. Start with **`fhirpath_demo.rs`** - Get familiar with basic concepts
2. Try **`fhirpath_arithmetic.rs`** - Learn mathematical operations
3. Explore **`fhirpath_strings.rs`** - Understand string manipulation

### For FHIR Developers
1. Begin with **`fhirpath_patient_navigation.rs`** - See real FHIR usage
2. Study **`fhirpath_collections.rs`** - Learn collection operations
3. Experiment with modifications to understand edge cases

### For Advanced Users
- Modify examples to test edge cases
- Combine concepts from different examples
- Use examples as templates for your own FHIRPath applications

## 📖 Key Concepts Demonstrated

### FHIRPath Language Features
- **Expression Parsing** - Converting text to Abstract Syntax Trees
- **Evaluation Context** - Providing data for expression evaluation
- **Type System** - Working with FHIRPath's type system
- **Error Handling** - Proper error management patterns

### Mathematical Operations
- **Arithmetic** - Addition, subtraction, multiplication, division, modulo
- **Operator Precedence** - Understanding evaluation order
- **Numeric Types** - Integers and decimals
- **Complex Expressions** - Combining multiple operations

### String Processing
- **Case Conversion** - upper(), lower() functions
- **Substring Operations** - substring(), length() functions  
- **String Search** - contains(), startsWith(), endsWith()
- **String Modification** - replace(), trim() functions
- **Concatenation** - Joining strings with + operator

### FHIR Resource Navigation
- **Resource Fields** - Accessing top-level properties
- **Nested Navigation** - Traversing complex structures
- **Collection Handling** - Working with arrays and lists
- **Filtering** - Using where() clauses for conditional selection
- **Existence Checks** - Testing for presence of data

### Collection Operations
- **Array Access** - Working with FHIR arrays
- **Field Extraction** - Getting values from object collections
- **Filtering** - Conditional selection with where()
- **Aggregation** - Count, existence, and other aggregate functions

## 🔗 Related Documentation

- **[FHIRPath Crate Documentation](../README.md)** - Complete library documentation
- **[Main Workspace Examples](../../../examples/README.md)** - Other workspace examples
- **[RH CLI Documentation](../../../apps/rh/README.md)** - Command-line interface
- **[FHIRPath Specification](https://hl7.org/fhirpath/)** - Official FHIRPath language specification

## 🛠️ Development

### Adding New Examples

To add a new FHIRPath example:

1. **Create the example file** in this directory (e.g., `fhirpath_new_feature.rs`)
2. **Follow the example template** with comprehensive documentation
3. **Include error handling** using `anyhow::Result<()>`
4. **Add comprehensive comments** explaining each step
5. **Update this README** with a description and run instructions
6. **Test the example** to ensure it works correctly

### Example Template

```rust
/// FHIRPath - [Feature Name] Example
/// 
/// This example demonstrates [specific functionality]

use anyhow::Result;
use serde_json::json;
use fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext, FhirPathValue};

fn main() -> Result<()> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    println!("🎯 FHIRPath [Feature Name] Examples");
    println!("==================================\n");

    // Example implementation with detailed comments

    println!("\n✅ All examples completed successfully!");
    println!("💡 Key learning points or usage tips");

    Ok(())
}
```

## 🎯 Best Practices

1. **Start Simple** - Begin with basic expressions before complex ones
2. **Test Incrementally** - Verify each step works before building complexity
3. **Handle Errors** - Always use proper error handling patterns
4. **Document Assumptions** - Explain data structures and expected formats
5. **Use Real Data** - Test with actual FHIR resources when possible
6. **Performance Awareness** - Consider efficiency for large datasets

The FHIRPath examples provide practical, hands-on learning for working with FHIR data using the FHIRPath expression language. They demonstrate both the power and flexibility of FHIRPath for healthcare data processing.
