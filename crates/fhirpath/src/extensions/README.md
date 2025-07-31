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
│   ├── values.rs               # FHIR-specific data types (Reference, CodeableConcept, etc.)
│   ├── variables.rs            # FHIR variables (%resource, %context, etc.)
│   ├── functions.rs            # FHIR functions (extension(), resolve(), etc.)
│   └── navigation.rs           # FHIR resource navigation
├── sql_on_fhir/               # SQL-on-FHIR extensions
│   ├── mod.rs                  # SQL-on-FHIR registry
│   ├── values.rs               # SQL-on-FHIR specific types (if needed)
│   └── functions.rs            # SQL-on-FHIR functions
└── custom_healthcare/         # Example custom extension
    ├── mod.rs                  # Custom extension registry
    ├── values.rs               # Custom healthcare data types
    ├── functions.rs            # Custom functions
    └── variables.rs            # Custom variables
```

## Creating a New Extension Module

### Step 1: Create the Module Structure

1. Create a new directory under `extensions/` for your domain (e.g., `custom_healthcare/`)
2. Add a `mod.rs` file to define the module registry
3. Create individual files for different types of functionality:
   - `values.rs` - Custom data types specific to your domain
   - `functions.rs` - Custom functions that work with your types
   - `variables.rs` - Custom variables and constants
   - `operators.rs` - Custom operators (if needed)

### Step 2: Define Custom Data Types

Each extension can define its own data types in a `values.rs` file. These types extend the base `FhirPathValue` enum additively, meaning they work alongside the core types without replacing them.

Use the following template for extension-specific types:

```rust
//! Custom data types for [DOMAIN_NAME] extensions
//!
//! This module defines domain-specific data types that extend the base FhirPathValue
//! enum. These types are used in conjunction with the core types, not as replacements.

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use serde_json::Value;
use std::collections::HashMap;

/// Extension-specific value types for [DOMAIN]
/// 
/// These types extend the base FhirPathValue enum for domain-specific use cases.
/// They are designed to be additive - working alongside core types.
#[derive(Debug, Clone, PartialEq)]
pub enum DomainValue {
    /// FHIR Reference type with reference and display
    Reference { 
        reference: String, 
        display: Option<String> 
    },
    
    /// FHIR CodeableConcept with coding array and text
    CodeableConcept {
        coding: Vec<Coding>,
        text: Option<String>,
    },
    
    /// FHIR Coding with system, code, and display
    Coding {
        system: Option<String>,
        code: Option<String>,
        display: Option<String>,
    },
    
    /// FHIR Identifier with system and value
    Identifier { 
        system: Option<String>, 
        value: String,
        use_: Option<String>,
    },
    
    /// FHIR Period with start and end
    Period {
        start: Option<String>,
        end: Option<String>,
    },
    
    /// FHIR Range with low and high values
    Range {
        low: Option<FhirPathValue>,
        high: Option<FhirPathValue>,
    },
}

impl DomainValue {
    /// Convert extension type to core FhirPathValue
    /// This allows extension types to participate in core FHIRPath operations
    pub fn to_fhir_path_value(self) -> FhirPathValue {
        match self {
            DomainValue::Reference { reference, display } => {
                let mut obj = serde_json::Map::new();
                obj.insert("reference".to_string(), Value::String(reference));
                if let Some(d) = display {
                    obj.insert("display".to_string(), Value::String(d));
                }
                FhirPathValue::Object(Value::Object(obj))
            }
            DomainValue::Coding { system, code, display } => {
                let mut obj = serde_json::Map::new();
                if let Some(s) = system {
                    obj.insert("system".to_string(), Value::String(s));
                }
                if let Some(c) = code {
                    obj.insert("code".to_string(), Value::String(c));
                }
                if let Some(d) = display {
                    obj.insert("display".to_string(), Value::String(d));
                }
                FhirPathValue::Object(Value::Object(obj))
            }
            DomainValue::Identifier { system, value, use_ } => {
                let mut obj = serde_json::Map::new();
                if let Some(s) = system {
                    obj.insert("system".to_string(), Value::String(s));
                }
                obj.insert("value".to_string(), Value::String(value));
                if let Some(u) = use_ {
                    obj.insert("use".to_string(), Value::String(u));
                }
                FhirPathValue::Object(Value::Object(obj))
            }
            // ... handle other types similarly
            _ => FhirPathValue::Empty, // Fallback for unhandled types
        }
    }
    
    /// Try to create extension type from core FhirPathValue
    /// This enables automatic recognition of extension types from JSON/Objects
    pub fn from_fhir_path_value(value: &FhirPathValue) -> Option<Self> {
        match value {
            FhirPathValue::Object(obj) => Self::from_json_object(obj),
            _ => None,
        }
    }
    
    /// Create extension type from JSON object
    pub fn from_json_object(obj: &Value) -> Option<Self> {
        if let Value::Object(map) = obj {
            // Detect FHIR Reference pattern
            if map.contains_key("reference") {
                let reference = map.get("reference")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let display = map.get("display")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                return Some(DomainValue::Reference { reference, display });
            }
            
            // Detect FHIR Identifier pattern
            if map.contains_key("value") && (map.contains_key("system") || map.contains_key("use")) {
                let system = map.get("system")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let value = map.get("value")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let use_ = map.get("use")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                return Some(DomainValue::Identifier { system, value, use_ });
            }
            
            // Detect FHIR Coding pattern
            if map.contains_key("system") || map.contains_key("code") || map.contains_key("display") {
                let system = map.get("system")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let code = map.get("code")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let display = map.get("display")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                return Some(DomainValue::Coding { system, code, display });
            }
        }
        None
    }
    
    /// Get string representation for debugging and display
    pub fn to_string_representation(&self) -> String {
        match self {
            DomainValue::Reference { reference, display } => {
                match display {
                    Some(d) => format!("{} ({})", reference, d),
                    None => reference.clone(),
                }
            }
            DomainValue::Coding { system, code, display } => {
                match (system, code, display) {
                    (Some(s), Some(c), Some(d)) => format!("{}|{}|{}", s, c, d),
                    (Some(s), Some(c), None) => format!("{}|{}", s, c),
                    (None, Some(c), Some(d)) => format!("{}|{}", c, d),
                    (None, Some(c), None) => c.clone(),
                    _ => "".to_string(),
                }
            }
            DomainValue::Identifier { system, value, .. } => {
                match system {
                    Some(s) => format!("{}|{}", s, value),
                    None => value.clone(),
                }
            }
            _ => format!("{:?}", self), // Fallback to debug representation
        }
    }
    
    /// Check equality with another domain value
    pub fn equals(&self, other: &DomainValue) -> bool {
        match (self, other) {
            (
                DomainValue::Reference { reference: r1, display: d1 },
                DomainValue::Reference { reference: r2, display: d2 },
            ) => r1 == r2 && d1 == d2,
            (
                DomainValue::Coding { system: s1, code: c1, display: d1 },
                DomainValue::Coding { system: s2, code: c2, display: d2 },
            ) => s1 == s2 && c1 == c2 && d1 == d2,
            (
                DomainValue::Identifier { system: s1, value: v1, use_: u1 },
                DomainValue::Identifier { system: s2, value: v2, use_: u2 },
            ) => s1 == s2 && v1 == v2 && u1 == u2,
            _ => false,
        }
    }
}

/// Type registry for extension-specific types
/// 
/// This allows the extension system to recognize and work with custom types
/// alongside the core FhirPathValue types.
pub struct DomainTypeRegistry {
    type_constructors: HashMap<String, fn(&[FhirPathValue]) -> FhirPathResult<DomainValue>>,
    type_checkers: HashMap<String, fn(&FhirPathValue) -> bool>,
}

impl DomainTypeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            type_constructors: HashMap::new(),
            type_checkers: HashMap::new(),
        };
        registry.register_default_types();
        registry
    }
    
    fn register_default_types(&mut self) {
        // Register type constructors
        self.type_constructors.insert(
            "Reference".to_string(),
            |params| {
                if params.is_empty() || params.len() > 2 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "Reference() requires 1 or 2 parameters (reference, optional display)".to_string(),
                    });
                }
                
                let reference = match &params[0] {
                    FhirPathValue::String(s) => s.clone(),
                    _ => return Err(FhirPathError::TypeError {
                        message: "Reference() first parameter must be a string".to_string(),
                    }),
                };
                
                let display = if params.len() > 1 {
                    match &params[1] {
                        FhirPathValue::String(s) => Some(s.clone()),
                        FhirPathValue::Empty => None,
                        _ => return Err(FhirPathError::TypeError {
                            message: "Reference() second parameter must be a string or empty".to_string(),
                        }),
                    }
                } else {
                    None
                };
                
                Ok(DomainValue::Reference { reference, display })
            }
        );
        
        // Register type checkers
        self.type_checkers.insert(
            "Reference".to_string(),
            |value| {
                if let Some(domain_val) = DomainValue::from_fhir_path_value(value) {
                    matches!(domain_val, DomainValue::Reference { .. })
                } else {
                    false
                }
            }
        );
    }
    
    /// Check if a value is of a specific extension type
    pub fn is_type(&self, value: &FhirPathValue, type_name: &str) -> bool {
        if let Some(checker) = self.type_checkers.get(type_name) {
            checker(value)
        } else {
            false
        }
    }
    
    /// Construct a new extension type from parameters
    pub fn construct_type(&self, type_name: &str, params: &[FhirPathValue]) -> FhirPathResult<Option<DomainValue>> {
        if let Some(constructor) = self.type_constructors.get(type_name) {
            constructor(params).map(Some)
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_reference_creation_and_conversion() {
        let reference = DomainValue::Reference {
            reference: "Patient/123".to_string(),
            display: Some("John Doe".to_string()),
        };
        
        let fhir_value = reference.to_fhir_path_value();
        let back_to_domain = DomainValue::from_fhir_path_value(&fhir_value).unwrap();
        
        assert!(matches!(back_to_domain, DomainValue::Reference { .. }));
    }

    #[test]
    fn test_coding_string_representation() {
        let coding = DomainValue::Coding {
            system: Some("http://loinc.org".to_string()),
            code: Some("15074-8".to_string()),
            display: Some("Blood glucose".to_string()),
        };
        
        let string_rep = coding.to_string_representation();
        assert_eq!(string_rep, "http://loinc.org|15074-8|Blood glucose");
    }

    #[test]
    fn test_type_registry() {
        let registry = DomainTypeRegistry::new();
        
        let params = vec![
            FhirPathValue::String("Patient/123".to_string()),
            FhirPathValue::String("John Doe".to_string()),
        ];
        
        let result = registry.construct_type("Reference", &params).unwrap();
        assert!(result.is_some());
    }
}
```

### Step 3: Define Functions

Create a `functions.rs` file with custom function implementations. Functions can work with both core and extension-specific types defined in your `values.rs` file:

```rust
//! Custom functions for [DOMAIN_NAME] extensions
//!
//! Functions that work with both core FhirPathValue types and extension-specific
//! types defined in values.rs

use crate::error::*;
use crate::evaluator::context::EvaluationContext;
use crate::evaluator::types::FhirPathValue;
use super::values::{DomainValue, DomainTypeRegistry};

/// Get the reference URL from a FHIR Reference
/// Usage: reference.getReference()
pub fn get_reference(
    context: &EvaluationContext,
    input: &[FhirPathValue],
    _args: &[FhirPathValue],
) -> FhirPathResult<Vec<FhirPathValue>> {
    let mut results = Vec::new();
    
    for value in input {
        // Try to convert to domain-specific type first
        if let Some(domain_value) = DomainValue::from_fhir_path_value(value) {
            match domain_value {
                DomainValue::Reference { reference, .. } => {
                    results.push(FhirPathValue::String(reference));
                }
                _ => {
                    // Not a reference type, return empty
                }
            }
        } else {
            // Fallback to generic object handling
            if let FhirPathValue::Object(obj) = value {
                if let Some(ref_value) = obj.get("reference") {
                    if let Some(ref_str) = ref_value.as_str() {
                        results.push(FhirPathValue::String(ref_str.to_string()));
                    }
                }
            }
        }
    }
    
    Ok(results)
}

/// Get the display text from a FHIR Reference  
/// Usage: reference.getDisplay()
pub fn get_display(
    context: &EvaluationContext,
    input: &[FhirPathValue],
    _args: &[FhirPathValue],
) -> FhirPathResult<Vec<FhirPathValue>> {
    let mut results = Vec::new();
    
    for value in input {
        if let Some(domain_value) = DomainValue::from_fhir_path_value(value) {
            match domain_value {
                DomainValue::Reference { display: Some(display), .. } => {
                    results.push(FhirPathValue::String(display));
                }
                DomainValue::Coding { display: Some(display), .. } => {
                    results.push(FhirPathValue::String(display));
                }
                _ => {
                    // No display available
                }
            }
        }
    }
    
    Ok(results)
}

/// Create a new FHIR Reference
/// Usage: Reference('Patient/123', 'John Doe') or Reference('Patient/123')  
pub fn create_reference(
    context: &EvaluationContext,
    input: &[FhirPathValue],
    args: &[FhirPathValue],
) -> FhirPathResult<Vec<FhirPathValue>> {
    if args.is_empty() || args.len() > 2 {
        return Err(FhirPathError::InvalidOperation {
            message: "Reference() requires 1 or 2 arguments (reference, optional display)".to_string(),
        });
    }
    
    let registry = DomainTypeRegistry::new();
    
    if let Some(domain_ref) = registry.construct_type("Reference", args)? {
        let fhir_value = domain_ref.to_fhir_path_value();
        Ok(vec![fhir_value])
    } else {
        Err(FhirPathError::InvalidOperation {
            message: "Failed to create Reference".to_string(),
        })
    }
}

/// Check if a value matches a specific coding system
/// Usage: coding.hasSystem('http://loinc.org')
pub fn has_system(
    context: &EvaluationContext,
    input: &[FhirPathValue],
    args: &[FhirPathValue],
) -> FhirPathResult<Vec<FhirPathValue>> {
    if args.len() != 1 {
        return Err(FhirPathError::InvalidOperation {
            message: "hasSystem() requires exactly 1 argument (system URL)".to_string(),
        });
    }
    
    let target_system = match &args[0] {
        FhirPathValue::String(s) => s,
        _ => return Err(FhirPathError::TypeError {
            message: "hasSystem() argument must be a string".to_string(),
        }),
    };
    
    for value in input {
        if let Some(domain_value) = DomainValue::from_fhir_path_value(value) {
            match domain_value {
                DomainValue::Coding { system: Some(system), .. } => {
                    if &system == target_system {
                        return Ok(vec![FhirPathValue::Boolean(true)]);
                    }
                }
                _ => continue,
            }
        }
    }
    
    Ok(vec![FhirPathValue::Boolean(false)])
}

/// Get all coding systems from a CodeableConcept
/// Usage: codeableConcept.getSystems()
pub fn get_systems(
    context: &EvaluationContext,
    input: &[FhirPathValue],
    _args: &[FhirPathValue],
) -> FhirPathResult<Vec<FhirPathValue>> {
    let mut results = Vec::new();
    
    for value in input {
        if let Some(domain_value) = DomainValue::from_fhir_path_value(value) {
            match domain_value {
                DomainValue::CodeableConcept { coding, .. } => {
                    for code in coding {
                        if let Some(system) = code.system {
                            results.push(FhirPathValue::String(system));
                        }
                    }
                }
                DomainValue::Coding { system: Some(system), .. } => {
                    results.push(FhirPathValue::String(system));
                }
                _ => continue,
            }
        }
    }
    
    Ok(results)
}

/// Function registry for domain-specific functions
pub struct DomainFunctionRegistry {
    functions: std::collections::HashMap<String, fn(&EvaluationContext, &[FhirPathValue], &[FhirPathValue]) -> FhirPathResult<Vec<FhirPathValue>>>,
}

impl DomainFunctionRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            functions: std::collections::HashMap::new(),
        };
        
        // Register functions
        registry.functions.insert("getReference".to_string(), get_reference);
        registry.functions.insert("getDisplay".to_string(), get_display);
        registry.functions.insert("Reference".to_string(), create_reference);
        registry.functions.insert("hasSystem".to_string(), has_system);
        registry.functions.insert("getSystems".to_string(), get_systems);
        
        registry
    }
    
    pub fn get_function(&self, name: &str) -> Option<&fn(&EvaluationContext, &[FhirPathValue], &[FhirPathValue]) -> FhirPathResult<Vec<FhirPathValue>>> {
        self.functions.get(name)
    }
    
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_reference_function() {
        let context = EvaluationContext::default();
        
        // Create a reference object
        let ref_obj = json!({
            "reference": "Patient/123",
            "display": "John Doe"
        });
        
        let input = vec![FhirPathValue::Object(ref_obj)];
        let result = get_reference(&context, &input, &[]).unwrap();
        
        assert_eq!(result.len(), 1);
        if let FhirPathValue::String(ref_str) = &result[0] {
            assert_eq!(ref_str, "Patient/123");
        } else {
            panic!("Expected string result");
        }
    }

    #[test]
    fn test_create_reference_function() {
        let context = EvaluationContext::default();
        
        let args = vec![
            FhirPathValue::String("Patient/456".to_string()),
            FhirPathValue::String("Jane Doe".to_string()),
        ];
        
        let result = create_reference(&context, &[], &args).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify the created reference has the right structure
        if let FhirPathValue::Object(obj) = &result[0] {
            assert_eq!(obj.get("reference").unwrap().as_str().unwrap(), "Patient/456");
            assert_eq!(obj.get("display").unwrap().as_str().unwrap(), "Jane Doe");
        } else {
            panic!("Expected object result");
        }
    }
}
```

### Step 4: Define Variables

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

### Step 5: Create Module Registry

Create a `mod.rs` file for your extension module that brings together all components:

```rust
//! [DOMAIN_NAME] Extensions for FHIRPath
//!
//! This module provides [DOMAIN] specific functionality including:
//! - Custom data types defined in values.rs 
//! - Custom functions for [DOMAIN] operations
//! - Domain-specific variables and constants
//! - Specialized navigation and data processing

pub mod values;
pub mod functions;  
pub mod variables;

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use crate::evaluator::EvaluationContext;
use std::collections::HashMap;

// Re-export key types for easier access
pub use values::{DomainValue, DomainTypeRegistry};
pub use functions::DomainFunctionRegistry;

/// Function signature for extension functions
type ExtensionFunction = Box<dyn Fn(&FhirPathValue, &[FhirPathValue]) -> FhirPathResult<FhirPathValue>>;

/// Variable resolver signature
type VariableResolver = Box<dyn Fn(&str, &EvaluationContext) -> FhirPathResult<Option<FhirPathValue>>>;

/// Extension registry for [DOMAIN] functionality
/// 
/// This registry coordinates all extension components:
/// - Type system (values.rs)
/// - Function library (functions.rs)  
/// - Variable resolvers (variables.rs)
pub struct DomainExtensions {
    functions: HashMap<String, ExtensionFunction>,
    variables: HashMap<String, VariableResolver>,
    type_registry: DomainTypeRegistry,
    function_registry: DomainFunctionRegistry,
}

impl DomainExtensions {
    /// Create a new extension registry with all [DOMAIN] extensions registered
    pub fn new() -> Self {
        let mut extensions = Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
            type_registry: DomainTypeRegistry::new(),
            function_registry: DomainFunctionRegistry::new(),
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
    
    /// Get a function from the function registry
    pub fn get_registry_function(&self, name: &str) -> Option<&fn(&EvaluationContext, &[FhirPathValue], &[FhirPathValue]) -> FhirPathResult<Vec<FhirPathValue>>> {
        self.function_registry.get_function(name)
    }

    /// Check if a value is of a specific extension type
    pub fn is_extension_type(&self, value: &FhirPathValue, type_name: &str) -> bool {
        self.type_registry.is_type(value, type_name)
    }
    
    /// Construct a new extension type from parameters
    pub fn construct_extension_type(&self, type_name: &str, params: &[FhirPathValue]) -> FhirPathResult<Option<DomainValue>> {
        self.type_registry.construct_type(type_name, params)
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
    
    /// Try to convert a core FhirPathValue to an extension-specific type
    pub fn try_convert_to_extension_type(&self, value: &FhirPathValue) -> Option<DomainValue> {
        DomainValue::from_fhir_path_value(value)
    }
}

impl Default for DomainExtensions {
    fn default() -> Self {
        Self::new()
    }
}
```

### Step 6: Integration with Main FHIRPath

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
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
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

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
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
