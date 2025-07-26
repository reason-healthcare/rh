# FHIRPath Examples

This directory contains comprehensive examples demonstrating the FHIRPath crate capabilities. These examples show how to parse, evaluate, and work with FHIRPath expressions using both the library API and real FHIR data.

## 🔍 Examples Overview

The examples are organized into categories for better navigation:

### 📚 Basic Examples

#### [`basic_demo.rs`](basic_demo.rs)
**Comprehensive FHIRPath demonstration**
- Basic expression parsing and evaluation
- Working with FHIR Patient resources
- String operations and mathematical expressions
- Error handling patterns

**Run with:**
```bash
cargo run --example basic_demo
```

### 🧮 Core Operations

#### [`arithmetic_operations.rs`](arithmetic_operations.rs)
**Mathematical operations in FHIRPath**
- Addition, subtraction, multiplication, division
- Modulo operations
- Operator precedence and parentheses
- Working with numeric data in FHIR resources
- Complex mathematical expressions

**Run with:**
```bash
cargo run --example arithmetic_operations
```

#### [`string_functions.rs`](string_functions.rs)
**String manipulation functions**
- Length, case conversion (upper/lower)
- Substring operations
- String searching (contains, startsWith, endsWith)
- String replacement and trimming
- String concatenation
- Validation patterns

**Run with:**
```bash
cargo run --example string_functions
```

#### [`math_functions.rs`](math_functions.rs)
**Mathematical functions for healthcare calculations**
- Basic math: abs(), ceiling(), floor(), round(), truncate()
- Power and roots: power(), sqrt()
- Logarithms: ln(), log(), exp()
- Function chaining for complex calculations
- Healthcare examples: BMI, temperature conversion, dosage calculations
- Statistical analysis components
- Error handling for invalid mathematical operations

**Run with:**
```bash
cargo run --example math_functions
```

#### [`simple_math.rs`](simple_math.rs)
**Quick demonstration of basic math functions**
- Essential mathematical operations with clear examples
- Function chaining demonstrations
- Clean, readable output format
- Perfect for learning the basics

**Run with:**
```bash
cargo run --example simple_math
```

#### [`collection_operations.rs`](collection_operations.rs)
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
cargo run --example collection_operations
```

#### [`indexing_operations.rs`](indexing_operations.rs)
**Array indexing and navigation**
- Basic array indexing: `name[0]`, `telecom[1]`
- Nested indexing: `name[0].given[0]`
- Bounds checking and error handling
- Out-of-bounds access patterns
- Complex indexing scenarios

**Run with:**
```bash
cargo run --example indexing_operations
```

#### [`union_operations.rs`](union_operations.rs)
**Union operations and collection combining**
- Basic unions: `(1 | 2 | 3)`
- FHIR data unions: `name.given | name.family`
- Mixed type unions: `(42 | 'hello' | true)`
- Nested unions: `((1 | 2) | (3 | 4))`
- Union with indexing: `(10 | 20 | 30)[1]`
- Empty value handling

**Run with:**
```bash
cargo run --example union_operations
```

### 🏥 FHIR-Specific Examples

#### [`fhir_patient_navigation.rs`](fhir_patient_navigation.rs)
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
cargo run --example fhir_patient_navigation
```

### 🔧 Verification and Debug Examples

#### [`verify_union.rs`](verify_union.rs)
**Simple union verification**
- Quick verification of union functionality
- Basic test cases for union operations
- Simple output for CI/testing

**Run with:**
```bash
cargo run --example verify_union
```

#### [`verify_indexing.rs`](verify_indexing.rs)
**Indexing verification and debugging**
- Detailed indexing operation testing
- Step-by-step indexing examples
- Error case demonstrations

**Run with:**
```bash
cargo run --example verify_indexing
```

#### [`verify_union_debug.rs`](verify_union_debug.rs)
**Debug union expression evaluation**
- Detailed union operation debugging
- Step-by-step union evaluation
- Troubleshooting union edge cases

**Run with:**
```bash
cargo run --example verify_union_debug
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
cargo run -p fhirpath --example basic_demo

# Core operations
cargo run -p fhirpath --example arithmetic_operations
cargo run -p fhirpath --example string_functions
cargo run -p fhirpath --example collection_operations
cargo run -p fhirpath --example indexing_operations
cargo run -p fhirpath --example union_operations

# FHIR-specific examples
cargo run -p fhirpath --example fhir_patient_navigation

# Verification examples
cargo run -p fhirpath --example verify_union
cargo run -p fhirpath --example verify_indexing
cargo run -p fhirpath --example verify_union_debug
```

### Running with Verbose Output

Enable detailed logging for any example:
```bash
RUST_LOG=debug cargo run -p fhirpath --example basic_demo
```

### Running All Core Examples

Run all main FHIRPath examples sequentially:
```bash
examples=(
    "basic_demo"
    "arithmetic_operations"
    "quantity_literals"
    "string_functions"
    "collection_operations"
    "indexing_operations"
    "union_operations"
    "fhir_patient_navigation"
)

for example in "${examples[@]}"; do
    echo "Running $example..."
    cargo run -p fhirpath --example "$example"
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

# Core examples that should always work
core_examples=(
    "basic_demo"
    "arithmetic_operations"
    "quantity_literals"
    "string_functions"
    "collection_operations"
    "indexing_operations"
    "union_operations"
    "fhir_patient_navigation"
)

# Verification examples for testing specific functionality
verify_examples=(
    "verify_union"
    "verify_indexing"
    "verify_union_debug"
)

echo "🧪 Testing core examples..."
for example in "${core_examples[@]}"; do
    echo "Testing: $example"
    if cargo run -p fhirpath --example "$example" > /dev/null 2>&1; then
        echo "✅ $example - PASSED"
    else
        echo "❌ $example - FAILED"
        exit 1
    fi
done

echo "🔧 Testing verification examples..."
for example in "${verify_examples[@]}"; do
    echo "Testing: $example"
    if cargo run -p fhirpath --example "$example" > /dev/null 2>&1; then
        echo "✅ $example - PASSED"
    else
        echo "❌ $example - FAILED"
        exit 1
    fi
done

echo "🎉 All FHIRPath examples passed!"
```

### Quick Verification

Run the verification examples to quickly test core functionality:
```bash
# Test union operations
cargo run -p fhirpath --example verify_union

# Test indexing operations  
cargo run -p fhirpath --example verify_indexing

# Debug union edge cases
cargo run -p fhirpath --example verify_union_debug
```

## 💡 Learning Path

### For Beginners
1. **Start with `basic_demo.rs`** - Get familiar with basic concepts and API usage
2. **Try `arithmetic_operations.rs`** - Learn mathematical operations and operator precedence
3. **Explore `string_functions.rs`** - Understand string manipulation and text processing
4. **Practice with `indexing_operations.rs`** - Learn array access and navigation patterns

### For FHIR Developers
1. **Begin with `fhir_patient_navigation.rs`** - See real FHIR resource usage patterns
2. **Study `collection_operations.rs`** - Learn to work with FHIR arrays and collections
3. **Experiment with `union_operations.rs`** - Understand how to combine data from different sources
4. **Use verification examples** to understand edge cases and debugging

### For Advanced Users
- **Modify examples** to test edge cases and boundary conditions
- **Combine concepts** from different examples for complex use cases
- **Use examples as templates** for your own FHIRPath applications
- **Study verification examples** to understand implementation details

### Recommended Order
```
basic_demo → arithmetic_operations → string_functions → indexing_operations 
    ↓
union_operations → collection_operations → fhir_patient_navigation
    ↓
verify_* examples for testing and debugging
```

## 📖 Key Concepts Demonstrated

### FHIRPath Language Features
- **Expression Parsing** - Converting text to Abstract Syntax Trees (`basic_demo.rs`)
- **Evaluation Context** - Providing data for expression evaluation (all examples)
- **Type System** - Working with FHIRPath's type system (`union_operations.rs`)
- **Error Handling** - Proper error management patterns (all examples)

### Mathematical Operations (`arithmetic_operations.rs`)
- **Arithmetic** - Addition, subtraction, multiplication, division, modulo
- **Operator Precedence** - Understanding evaluation order and parentheses
- **Numeric Types** - Integers, decimals, and mixed-type operations
- **Complex Expressions** - Combining multiple operations with proper precedence

### String Processing (`string_functions.rs`)
- **Case Conversion** - `upper()`, `lower()` functions
- **Substring Operations** - `substring()`, `length()` functions  
- **String Search** - `contains()`, `startsWith()`, `endsWith()`
- **String Modification** - `replace()`, `trim()` functions
- **Concatenation** - Joining strings with `&` operator
- **Collection Processing** - `split()`, `join()` for string arrays

### Collection Operations (`collection_operations.rs`)
- **Array Access** - Working with FHIR arrays and collections
- **Field Extraction** - Getting values from object collections
- **Filtering** - Conditional selection with `where()` clauses
- **Aggregation** - `count()`, `exists()`, and other aggregate functions
- **Collection Functions** - `empty()`, `distinct()`, `isDistinct()`

### Array Indexing (`indexing_operations.rs`)
- **Basic Indexing** - Simple array access with `[0]`, `[1]`, etc.
- **Nested Indexing** - Deep navigation like `name[0].given[0]`
- **Bounds Checking** - Handling out-of-bounds access gracefully
- **Edge Cases** - Empty collections, single-element arrays
- **Complex Navigation** - Combining indexing with filtering

### Union Operations (`union_operations.rs`)
- **Basic Unions** - Combining values with `|` operator
- **Collection Combining** - Merging arrays and collections
- **Mixed Types** - Unions with different data types
- **Nested Unions** - Complex union expressions
- **Union with Indexing** - Accessing specific elements from unions
- **Empty Handling** - How unions deal with empty values

### FHIR Resource Navigation (`fhir_patient_navigation.rs`)
- **Resource Fields** - Accessing top-level FHIR properties
- **Nested Navigation** - Traversing complex FHIR structures
- **Collection Handling** - Working with FHIR arrays and repeating elements
- **Filtering** - Using `where()` clauses for conditional selection
- **Existence Checks** - Testing for presence of FHIR data
- **Real-world Patterns** - Practical FHIR data access patterns

### Verification and Debugging
- **Unit Testing** - Simple verification patterns (`verify_union.rs`)
- **Debug Output** - Detailed tracing of evaluation (`verify_union_debug.rs`)
- **Edge Case Testing** - Boundary condition verification (`verify_indexing.rs`)
- **CI Integration** - Examples suitable for automated testing

## 🔗 Related Documentation

- **[FHIRPath Crate Documentation](../README.md)** - Complete library documentation
- **[Main Workspace Examples](../../../examples/README.md)** - Other workspace examples
- **[RH CLI Documentation](../../../apps/rh/README.md)** - Command-line interface
- **[FHIRPath Specification](https://hl7.org/fhirpath/)** - Official FHIRPath language specification

## 📊 Examples Summary

| Example | Purpose | Complexity | Key Features |
|---------|---------|------------|--------------|
| `basic_demo.rs` | Getting started | ⭐ Beginner | Basic API, parsing, simple evaluation |
| `arithmetic_operations.rs` | Math operations | ⭐⭐ Intermediate | All arithmetic operators, precedence |
| `string_functions.rs` | Text processing | ⭐⭐ Intermediate | 12+ string functions, chaining |
| `collection_operations.rs` | Array handling | ⭐⭐⭐ Advanced | Filtering, aggregation, complex navigation |
| `indexing_operations.rs` | Array access | ⭐⭐ Intermediate | Basic/nested indexing, bounds checking |
| `union_operations.rs` | Data combining | ⭐⭐⭐ Advanced | All union patterns, mixed types |
| `fhir_patient_navigation.rs` | Real FHIR data | ⭐⭐⭐ Advanced | Complete FHIR resource examples |
| `verify_union.rs` | Testing | ⭐ Beginner | Quick verification, CI-friendly |
| `verify_indexing.rs` | Debugging | ⭐⭐ Intermediate | Detailed testing, edge cases |
| `verify_union_debug.rs` | Troubleshooting | ⭐⭐ Intermediate | Step-by-step debugging |

**Total Examples**: 10 examples covering all major FHIRPath functionality

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

    println!("\n✅ All [feature] examples completed successfully!");
    println!("💡 Key takeaways:");
    println!("   - [Learning point 1]");
    println!("   - [Learning point 2]");
    println!("   - [Learning point 3]");

    Ok(())
}
```

### File Organization

Examples are organized by functionality:

```
examples/
├── README.md                    # This documentation
├── basic_demo.rs               # 📚 Basic concepts
├── arithmetic_operations.rs    # 🧮 Math operations
├── string_functions.rs         # 🔤 String processing
├── collection_operations.rs    # 📋 Collection handling
├── indexing_operations.rs      # 🎯 Array indexing
├── union_operations.rs         # 🔗 Union operations
├── fhir_patient_navigation.rs  # 🏥 FHIR examples
├── verify_union.rs            # ✅ Quick verification
├── verify_indexing.rs         # ✅ Indexing tests
└── verify_union_debug.rs      # 🔧 Debug examples
```

### Example Categories

- **📚 Basic**: Fundamental concepts and getting started
- **🧮 Operations**: Core FHIRPath operations (arithmetic, string, collection)
- **🏥 FHIR**: Healthcare-specific examples and real-world usage
- **✅ Verification**: Testing, debugging, and CI examples

## 🎯 Best Practices

1. **Start Simple** - Begin with basic expressions before complex ones
2. **Test Incrementally** - Verify each step works before building complexity
3. **Handle Errors** - Always use proper error handling patterns
4. **Document Assumptions** - Explain data structures and expected formats
5. **Use Real Data** - Test with actual FHIR resources when possible
6. **Performance Awareness** - Consider efficiency for large datasets

The FHIRPath examples provide practical, hands-on learning for working with FHIR data using the FHIRPath expression language. They demonstrate both the power and flexibility of FHIRPath for healthcare data processing.
