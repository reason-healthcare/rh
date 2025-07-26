# FHIR Code Generation Examples

This directory contains comprehensive examples demonstrating the FHIR code generation crate capabilities. These examples show how to generate type-safe Rust code from FHIR StructureDefinitions, manage FHIR packages, and work with generated types.

✅ **All examples are fully functional and demonstrate real working code.**

## 🔧 Examples Overview

### [`codegen_basic_usage.rs`](codegen_basic_usage.rs)
**Basic code generation workflow**
- Creating CodegenConfig with proper settings
- Creating StructureDefinition instances
- Generating Rust structs from FHIR definitions
- Writing generated code to files

**Run with:**
```bash
cargo run -p codegen --example codegen_basic_usage
```

### [`codegen_package_management.rs`](codegen_package_management.rs)
**FHIR package management**
- Configuring package downloaders for FHIR registries
- Setting up authentication for custom registries
- Demonstrating async package download workflows
- Configuration examples for package management
- Package processing workflows

**Run with:**
```bash
cargo run -p codegen --example codegen_package_management
```

### [`codegen_generated_types.rs`](codegen_generated_types.rs)
**Working with generated Rust types**
- Setting up code generation with Serde support
- Creating multiple StructureDefinitions 
- Generating multiple Rust structs
- Demonstrating benefits of type-safe generated code
- Examples of generated code patterns

**Run with:**
```bash
cargo run -p codegen --example codegen_generated_types
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
# Basic usage demonstration
cargo run -p codegen --example codegen_basic_usage

# Package management
cargo run -p codegen --example codegen_package_management

# Working with generated types
cargo run -p codegen --example codegen_generated_types
```

### Running with Verbose Output

Enable detailed logging for any example:
```bash
RUST_LOG=debug cargo run -p codegen --example codegen_basic_usage
```

### Running All Codegen Examples

Run all codegen examples sequentially:
```bash
for example in codegen_basic_usage codegen_package_management codegen_generated_types; do
    echo "Running $example..."
    cargo run -p codegen --example $example
    echo "Completed $example"
    echo "---"
done
```

## 🧪 Testing Examples

The examples serve as both documentation and informal tests. You can verify they work correctly by running them and checking their output.

### Example Testing Script

```bash
#!/bin/bash
# test-codegen-examples.sh

set -e

echo "Testing codegen examples..."

examples=(
    "codegen_basic_usage"
    "codegen_package_management"
    "codegen_generated_types"
)

for example in "${examples[@]}"; do
    echo "🧪 Testing: $example"
    if cargo run -p codegen --example "$example" > /dev/null 2>&1; then
        echo "✅ $example - PASSED"
    else
        echo "❌ $example - FAILED"
        exit 1
    fi
done

echo "🎉 All codegen examples passed!"
```

## 💡 Learning Path

### For Beginners
1. Start with **`codegen_basic_usage.rs`** - Get familiar with basic concepts
2. Try **`codegen_package_management.rs`** - Learn FHIR package workflows
3. Explore **`codegen_generated_types.rs`** - Understand working with generated code

### For FHIR Developers
1. Begin with **`codegen_package_management.rs`** - See real FHIR package usage
2. Study **`codegen_generated_types.rs`** - Learn type-safe FHIR development
3. Experiment with modifications to understand code generation patterns

### For Advanced Users
- Modify examples to test different FHIR profiles
- Combine concepts from different examples
- Use examples as templates for your own FHIR applications

## 📖 Key Concepts Demonstrated

### Code Generation Workflow
- **Configuration** - Setting up code generation parameters
- **Input Processing** - Handling FHIR StructureDefinitions
- **Type Generation** - Creating Rust structs and enums
- **Output Management** - Organizing generated code files

### FHIR Package Management
- **Package Discovery** - Finding packages in registries
- **Download Process** - Retrieving package content
- **Authentication** - Working with private registries
- **Package Installation** - Extracting and processing definitions

### Generated Type Usage
- **Resource Creation** - Building FHIR resources in Rust
- **Serialization** - Converting between Rust and JSON
- **Type Safety** - Leveraging Rust's type system for FHIR
- **Validation** - Ensuring data integrity

### Error Handling
- **Configuration Errors** - Invalid settings and parameters
- **Network Errors** - Registry access and download issues
- **Processing Errors** - StructureDefinition parsing problems
- **Generation Errors** - Code generation failures

## 🔗 Related Documentation

- **[FHIR Code Generation Crate Documentation](../README.md)** - Complete library documentation
- **[Main Workspace Examples](../../../examples/README.md)** - Other workspace examples
- **[RH CLI Documentation](../../../apps/rh/README.md)** - Command-line interface
- **[FHIR Package Specification](https://confluence.hl7.org/display/FHIR/NPM+Package+Specification)** - Official FHIR package format

## 🛠️ Development

### Adding New Examples

To add a new codegen example:

1. **Create the example file** in this directory (e.g., `codegen_new_feature.rs`)
2. **Follow the example template** with comprehensive documentation
3. **Include error handling** using `anyhow::Result<()>`
4. **Add comprehensive comments** explaining each step
5. **Update this README** with a description and run instructions
6. **Test the example** to ensure it works correctly

### Example Template

```rust
/// FHIR Code Generation - [Feature Name] Example
/// 
/// This example demonstrates [specific functionality]

use anyhow::Result;
use codegen::{CodeGenerator, CodegenConfig};
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("🔧 FHIR Code Generation - [Feature Name]");
    println!("========================================\n");

    // Example implementation with detailed comments

    println!("\n✅ Example completed successfully!");
    println!("💡 Key learning points or usage tips");

    Ok(())
}
```

## 🎯 Best Practices

1. **Start Simple** - Begin with basic configuration before complex setups
2. **Test Incrementally** - Verify each step works before building complexity
3. **Handle Errors** - Always use proper error handling patterns
4. **Document Assumptions** - Explain expected input formats and structures
5. **Use Real Data** - Test with actual FHIR StructureDefinitions when possible
6. **Performance Awareness** - Consider efficiency for large FHIR packages

### Configuration Best Practices

- **Organize Output** - Use clear directory structures for generated code
- **Package Naming** - Choose descriptive names for generated packages
- **Module Organization** - Structure modules logically within packages
- **Enum Generation** - Enable for type-safe value set handling

### Package Management Best Practices

- **Registry Selection** - Use appropriate registries for your needs
- **Authentication** - Secure token management for private registries
- **Version Pinning** - Specify exact versions for reproducible builds
- **Caching** - Leverage package caching for improved performance

### Generated Code Best Practices

- **Type Safety** - Leverage Rust's type system for FHIR validation
- **Serialization** - Use serde features for JSON compatibility
- **Documentation** - Include generated documentation for clarity
- **Testing** - Write tests for generated type usage patterns

The codegen examples provide practical, hands-on learning for generating type-safe Rust code from FHIR StructureDefinitions. They demonstrate both the power and flexibility of the code generation approach for healthcare data processing.
