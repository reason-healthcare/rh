# FHIRPath Extensions Development Guide

This directory contains extensions to the core FHIRPath specification, providing additional functionality specific to FHIR resources and healthcare use cases.

## Overview

FHIRPath extensions allow you to add domain-specific functions and variables that go beyond the standard FHIRPath specification. This includes FHIR-specific functions, custom healthcare operations, and specialized data processing capabilities.

## Extension Architecture

Extensions are organized into modules, each focusing on a specific domain:

```
extensions/
├── README.md                    # This file
├── mod.rs                      # Extension registry and coordination
├── fhir/                       # FHIR-specific extensions
│   ├── mod.rs                  # FHIR extension registry
│   ├── variables.rs            # FHIR variables (%resource, %context, etc.)
│   ├── functions.rs            # FHIR functions (extension(), resolve(), etc.)
│   └── navigation.rs           # FHIR resource navigation
└── sql_on_fhir/               # SQL-on-FHIR extensions
    ├── mod.rs                  # SQL-on-FHIR registry
    └── functions.rs            # SQL-on-FHIR functions
```

## Creating a New Extension Module

### Step 1: Create the Module Structure

1. Create a new directory under `extensions/` for your domain (e.g., `custom_healthcare/`)
2. Add a `mod.rs` file to define the module registry
3. Create individual files for different types of functionality (functions, variables, operators)

### Step 2: Define Functions

Use the following template for function modules:

```rust
//! Custom extension functions for [DOMAIN_NAME]
//!
//! This module provides [DESCRIPTION] functionality for FHIRPath expressions.

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use std::collections::HashMap;

/// Function signature for FHIRPath extension functions
type ExtensionFunction = Box<dyn Fn(&FhirPathValue, &[FhirPathValue]) -> FhirPathResult<FhirPathValue>>;

/// Register all [DOMAIN] extension functions
pub fn register_functions(functions: &mut HashMap<String, ExtensionFunction>) {
    // Example function registration
    functions.insert(
        "customFunction".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            custom_function(target, params)
        }),
    );
}

/// Example custom function implementation
/// 
/// # Arguments
/// * `target` - The target value the function is called on
/// * `params` - Array of parameter values passed to the function
/// 
/// # Returns
/// * `Ok(FhirPathValue)` - The result of the function
/// * `Err(FhirPathError)` - Error if the function fails
fn custom_function(
    target: &FhirPathValue,
    params: &[FhirPathValue],
) -> FhirPathResult<FhirPathValue> {
    // Validate parameters
    if params.len() != 1 {
        return Err(FhirPathError::InvalidOperation {
            message: "customFunction() requires exactly one parameter".to_string(),
        });
    }

    // Example implementation - customize as needed
    match target {
        FhirPathValue::String(s) => {
            // Process string target
            Ok(FhirPathValue::String(format!("processed: {}", s)))
        }
        _ => Err(FhirPathError::TypeError {
            message: "customFunction() can only be called on String values".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_function() {
        let target = FhirPathValue::String("test".to_string());
        let params = vec![FhirPathValue::String("param".to_string())];
        
        let result = custom_function(&target, &params).unwrap();
        assert_eq!(result, FhirPathValue::String("processed: test".to_string()));
    }

    #[test]
    fn test_custom_function_wrong_type() {
        let target = FhirPathValue::Integer(42);
        let params = vec![FhirPathValue::String("param".to_string())];
        
        let result = custom_function(&target, &params);
        assert!(result.is_err());
    }

    #[test]
    fn test_custom_function_wrong_param_count() {
        let target = FhirPathValue::String("test".to_string());
        let params = vec![]; // No parameters
        
        let result = custom_function(&target, &params);
        assert!(result.is_err());
    }
}
```

### Step 3: Define Variables

For custom variables (like `%resource`, `%context`), use this template:

```rust
//! Custom extension variables for [DOMAIN_NAME]

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use crate::evaluator::EvaluationContext;
use std::collections::HashMap;

/// Variable resolver function signature
type VariableResolver = Box<dyn Fn(&str, &EvaluationContext) -> FhirPathResult<Option<FhirPathValue>>>;

/// Register all [DOMAIN] extension variables
pub fn register_variables(variables: &mut HashMap<String, VariableResolver>) {
    variables.insert(
        "customVar".to_string(),
        Box::new(|name: &str, context: &EvaluationContext| {
            resolve_custom_variable(name, context)
        }),
    );
}

/// Resolve custom variable values
fn resolve_custom_variable(
    name: &str,
    context: &EvaluationContext,
) -> FhirPathResult<Option<FhirPathValue>> {
    match name {
        "customVar" => {
            // Example: return a constant value
            Ok(Some(FhirPathValue::String("custom_value".to_string())))
        }
        _ => Ok(None), // Variable not handled by this resolver
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluator::EvaluationContext;
    use serde_json::json;

    #[test]
    fn test_custom_variable_resolution() {
        let context = EvaluationContext::new(json!({}));
        let result = resolve_custom_variable("customVar", &context).unwrap();
        
        assert!(result.is_some());
        assert_eq!(result.unwrap(), FhirPathValue::String("custom_value".to_string()));
    }
}
```

### Step 4: Create Module Registry

Create a `mod.rs` file for your extension module:

```rust
//! [DOMAIN_NAME] Extensions for FHIRPath
//!
//! This module provides [DOMAIN] specific functionality including:
//! - Custom functions for [DOMAIN] operations
//! - Domain-specific variables and constants
//! - Specialized navigation and data processing

pub mod functions;
pub mod variables;

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use crate::evaluator::EvaluationContext;
use std::collections::HashMap;

/// Function signature for extension functions
type ExtensionFunction = Box<dyn Fn(&FhirPathValue, &[FhirPathValue]) -> FhirPathResult<FhirPathValue>>;

/// Variable resolver signature
type VariableResolver = Box<dyn Fn(&str, &EvaluationContext) -> FhirPathResult<Option<FhirPathValue>>>;

/// Extension registry for [DOMAIN] functionality
pub struct DomainExtensions {
    functions: HashMap<String, ExtensionFunction>,
    variables: HashMap<String, VariableResolver>,
}

impl DomainExtensions {
    /// Create a new extension registry with all [DOMAIN] extensions registered
    pub fn new() -> Self {
        let mut extensions = Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        };
        
        extensions.register_all();
        extensions
    }

    /// Register all extension functions and variables
    fn register_all(&mut self) {
        functions::register_functions(&mut self.functions);
        variables::register_variables(&mut self.variables);
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&ExtensionFunction> {
        self.functions.get(name)
    }

    /// Resolve a variable by name
    pub fn resolve_variable(&self, name: &str, context: &EvaluationContext) -> FhirPathResult<Option<FhirPathValue>> {
        for resolver in self.variables.values() {
            if let Some(value) = resolver(name, context)? {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }
}

impl Default for DomainExtensions {
    fn default() -> Self {
        Self::new()
    }
}
```

### Step 5: Integration with Main FHIRPath

To integrate your extension with the main FHIRPath implementation:

1. **Update the main extensions module** (`extensions/mod.rs`):
```rust
pub mod fhir;
pub mod sql_on_fhir;
pub mod your_domain;  // Add your extension here

// Register in the main extension registry
```

2. **Update the evaluator** to check extension functions:
```rust
// In the function call evaluation logic
if let Some(extension_fn) = extensions.get_function(function_name) {
    return extension_fn(target, params);
}
```

3. **Update variable resolution** to check extension variables:
```rust
// In variable resolution logic
if let Some(value) = extensions.resolve_variable(var_name, context)? {
    return Ok(value);
}
```

## Testing Guidelines

### Unit Tests

Each extension module should include comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_success_case() {
        // Test successful function execution
    }
    
    #[test] 
    fn test_function_error_cases() {
        // Test various error conditions
    }
    
    #[test]
    fn test_parameter_validation() {
        // Test parameter count and type validation
    }
}
```

### Integration Tests

Create integration tests in `tests/` directory:

```rust
// tests/extension_integration_test.rs
use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

#[test]
fn test_custom_extension_integration() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));
    
    // Test your extension functions in complete FHIRPath expressions
    let expr = parser.parse("someValue.customFunction('param')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    
    // Assert expected results
}
```

## Creating Examples

Create example files in `examples/` directory to demonstrate extension usage:

```rust
//! Example demonstrating [DOMAIN] extensions
//!
//! This example shows how to use the custom extensions for [DOMAIN] operations.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> anyhow::Result<()> {
    println!("=== [DOMAIN] Extension Examples ===\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    
    // Example data
    let context = EvaluationContext::new(json!({
        "example": "data"
    }));

    // Example 1: Custom function usage
    let expr = parser.parse("'test'.customFunction('param')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Custom function result: {result:?}");

    // Example 2: Custom variable usage  
    let expr = parser.parse("%customVar")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Custom variable result: {result:?}");

    Ok(())
}
```

## Best Practices

### Function Design

1. **Follow FHIRPath conventions**: Return empty collections for invalid operations rather than errors when appropriate
2. **Validate parameters**: Check parameter count and types early
3. **Handle edge cases**: Empty collections, null values, type mismatches
4. **Document behavior**: Clear documentation of what each function does
5. **Performance**: Consider performance implications for large datasets

### Error Handling

1. **Use appropriate error types**: `TypeError` for type mismatches, `InvalidOperation` for parameter errors
2. **Provide helpful messages**: Include context about what went wrong
3. **Follow FHIRPath semantics**: When in doubt, return empty rather than error

### Testing

1. **Comprehensive coverage**: Test success cases, error cases, and edge cases
2. **Performance tests**: For functions that may process large amounts of data
3. **Integration tests**: Test functions within complete FHIRPath expressions
4. **Documentation tests**: Include examples in documentation comments

### Documentation

1. **Clear function descriptions**: What the function does, parameters, return values
2. **Usage examples**: Show how to use the function in practice
3. **Error conditions**: Document when the function will return errors
4. **Performance notes**: Any performance considerations or limitations

## Available Extension Points

The extension system supports:

- **Custom functions**: Add new functions callable in FHIRPath expressions
- **Custom variables**: Add new variables accessible via `%variableName` syntax
- **Custom operators**: Add new operators (requires parser modifications)
- **Custom types**: Add new data types and their operations
- **Navigation helpers**: Specialized navigation for domain-specific data structures

## Deployment and Distribution

Extensions can be:

1. **Built-in**: Included in the main FHIRPath crate
2. **Optional features**: Enabled via Cargo features (`fhir-extensions`, `sql-extensions`)
3. **Separate crates**: Distributed as independent crates that extend FHIRPath
4. **Runtime plugins**: Loaded dynamically (advanced use case)

## Migration and Compatibility

When creating extensions:

1. **Follow semantic versioning**: Breaking changes require major version bumps
2. **Deprecation process**: Mark old functions as deprecated before removal
3. **Backward compatibility**: Try to maintain compatibility when possible
4. **Clear migration guides**: Document how to migrate between versions

## Support and Community

- **Documentation**: Keep this README and function documentation up to date
- **Examples**: Provide working examples for all major functionality
- **Tests**: Maintain comprehensive test coverage
- **Issues**: Use GitHub issues for bug reports and feature requests
- **Contributions**: Follow the project's contribution guidelines

## Extension Template Checklist

When creating a new extension:

- [ ] Create module directory structure
- [ ] Implement function registration
- [ ] Add comprehensive unit tests
- [ ] Create integration tests
- [ ] Write usage examples
- [ ] Update documentation
- [ ] Register with main extension system
- [ ] Verify clippy passes
- [ ] Run full test suite
- [ ] Update implementation.md status
