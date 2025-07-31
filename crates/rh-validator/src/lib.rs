//! FHIR resource validation library
//!
//! This crate provides functionality for validating FHIR resources, starting with:
//! - JSON syntax validation
//!
//! # Example
//!
//! ```rust
//! use rh_validator::{JsonValidator, ValidationResult};
//!
//! let validator = JsonValidator::new();
//! let json_content = r#"{"resourceType": "Patient", "id": "123"}"#;
//!
//! match validator.validate(json_content) {
//!     Ok(ValidationResult::Valid) => println!("Valid JSON"),
//!     Ok(ValidationResult::Invalid(errors)) => println!("Validation errors: {:?}", errors),
//!     Err(e) => println!("Validation failed: {}", e),
//! }
//! ```

use anyhow::Result;
use serde_json::Value;
use thiserror::Error;

/// Validation errors that can occur during JSON validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ValidationError {
    #[error("JSON syntax error: {message} at line {line}, column {column}")]
    JsonSyntax {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Schema validation error: {message}")]
    Schema { message: String },
}

/// Result of validation operation
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    /// JSON is valid
    Valid,
    /// JSON has validation errors
    Invalid(Vec<ValidationError>),
}

impl ValidationResult {
    /// Check if the validation result is valid (no errors)
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    /// Get validation errors if any
    pub fn errors(&self) -> Option<&[ValidationError]> {
        match self {
            ValidationResult::Valid => None,
            ValidationResult::Invalid(errors) => Some(errors),
        }
    }

    /// Get the number of errors
    pub fn error_count(&self) -> usize {
        match self {
            ValidationResult::Valid => 0,
            ValidationResult::Invalid(errors) => errors.len(),
        }
    }
}

/// JSON syntax validator
#[derive(Debug)]
pub struct JsonValidator {
    max_nesting_depth: usize,
}

impl JsonValidator {
    /// Create a new JSON validator with default settings
    pub fn new() -> Self {
        Self {
            max_nesting_depth: 100,
        }
    }

    /// Create a new JSON validator with custom maximum nesting depth
    pub fn with_max_depth(max_nesting_depth: usize) -> Self {
        Self { max_nesting_depth }
    }

    /// Validate JSON syntax and basic structure
    pub fn validate(&self, json_content: &str) -> Result<ValidationResult> {
        let mut errors = Vec::new();

        // First, try to parse the JSON to check basic syntax
        match serde_json::from_str::<Value>(json_content) {
            Ok(value) => {
                // JSON is syntactically valid, now check structure
                if let Err(structural_errors) = self.validate_structure(&value, 0) {
                    errors.extend(structural_errors);
                }
            }
            Err(e) => {
                // JSON syntax is invalid
                let error = self.parse_json_error(e);
                errors.push(error);
            }
        }

        if errors.is_empty() {
            Ok(ValidationResult::Valid)
        } else {
            Ok(ValidationResult::Invalid(errors))
        }
    }

    /// Validate multiple JSON documents from NDJSON format
    pub fn validate_multiple(&self, content: &str) -> Result<Vec<(usize, ValidationResult)>> {
        let mut results = Vec::new();

        for (line_number, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let result = self.validate(trimmed)?;
                results.push((line_number + 1, result));
            }
        }

        Ok(results)
    }

    /// Validate the structure of parsed JSON value
    fn validate_structure(
        &self,
        value: &Value,
        current_depth: usize,
    ) -> std::result::Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Check nesting depth
        if current_depth > self.max_nesting_depth {
            errors.push(ValidationError::Schema {
                message: format!(
                    "Maximum nesting depth of {} exceeded at depth {}",
                    self.max_nesting_depth, current_depth
                ),
            });
            return Err(errors);
        }

        // Recursively validate nested structures
        match value {
            Value::Object(obj) => {
                for (key, nested_value) in obj {
                    if let Err(mut nested_errors) =
                        self.validate_structure(nested_value, current_depth + 1)
                    {
                        errors.append(&mut nested_errors);
                    }

                    // Validate key is not empty
                    if key.is_empty() {
                        errors.push(ValidationError::Schema {
                            message: "Object keys cannot be empty".to_string(),
                        });
                    }
                }
            }
            Value::Array(arr) => {
                for nested_value in arr.iter() {
                    if let Err(mut nested_errors) =
                        self.validate_structure(nested_value, current_depth + 1)
                    {
                        errors.append(&mut nested_errors);
                    }
                }
            }
            Value::String(s) => {
                // Check for extremely long strings that might cause issues
                if s.len() > 1_000_000 {
                    errors.push(ValidationError::Schema {
                        message: format!(
                            "String value exceeds maximum length of 1,000,000 characters: {} characters",
                            s.len()
                        ),
                    });
                }
            }
            Value::Number(n) => {
                // Validate number is finite
                if let Some(f) = n.as_f64() {
                    if !f.is_finite() {
                        errors.push(ValidationError::Schema {
                            message: format!("Number must be finite, found: {f}"),
                        });
                    }
                }
            }
            Value::Bool(_) | Value::Null => {
                // These are always valid
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Parse a serde_json error and convert it to a ValidationError
    fn parse_json_error(&self, error: serde_json::Error) -> ValidationError {
        let line = error.line();
        let column = error.column();

        // Get more specific error message
        let message = if error.is_eof() {
            "Unexpected end of JSON input".to_string()
        } else if error.is_syntax() {
            format!("JSON syntax error: {error}")
        } else if error.is_data() {
            format!("Invalid JSON data: {error}")
        } else {
            format!("JSON parsing error: {error}")
        };

        ValidationError::JsonSyntax {
            message,
            line,
            column,
        }
    }
}

impl Default for JsonValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_json() {
        let validator = JsonValidator::new();

        let valid_cases = vec![
            r#"{}"#,
            r#"[]"#,
            r#"{"name": "test", "value": 42}"#,
            r#"[1, 2, 3, "hello"]"#,
            r#"{"nested": {"object": {"value": true}}}"#,
            r#"null"#,
            r#"42"#,
            r#""simple string""#,
            r#"true"#,
        ];

        for case in valid_cases {
            let result = validator.validate(case).unwrap();
            assert!(result.is_valid(), "Should be valid: {case}");
        }
    }

    #[test]
    fn test_invalid_json() {
        let validator = JsonValidator::new();

        let invalid_cases = vec![
            r#"{"#,
            r#"{"name": "test""#,
            r#"{"name": test}"#,
            r#"[1, 2, 3,"#,
            r#"{"trailing": "comma",}"#,
            r#"undefined"#,
            r#"{name: "no quotes"}"#,
        ];

        for case in invalid_cases {
            let result = validator.validate(case).unwrap();
            assert!(!result.is_valid(), "Should be invalid: {case}");

            if let ValidationResult::Invalid(errors) = result {
                assert!(!errors.is_empty());
                // Should have a JSON syntax error
                assert!(errors
                    .iter()
                    .any(|e| matches!(e, ValidationError::JsonSyntax { .. })));
            }
        }
    }

    #[test]
    fn test_nesting_depth_validation() {
        let validator = JsonValidator::with_max_depth(2);

        // Within depth limit
        let shallow = r#"{"level1": {"level2": "value"}}"#;
        let result = validator.validate(shallow).unwrap();
        assert!(result.is_valid());

        // Exceeds depth limit
        let deep = r#"{"level1": {"level2": {"level3": {"level4": "value"}}}}"#;
        let result = validator.validate(deep).unwrap();
        assert!(!result.is_valid());

        if let ValidationResult::Invalid(errors) = result {
            assert!(errors
                .iter()
                .any(|e| matches!(e, ValidationError::Schema { .. })));
        }
    }

    #[test]
    fn test_empty_object_keys() {
        let validator = JsonValidator::new();

        let empty_key = r#"{"": "value"}"#;
        let result = validator.validate(empty_key).unwrap();
        assert!(!result.is_valid());

        if let ValidationResult::Invalid(errors) = result {
            assert!(errors
                .iter()
                .any(|e| matches!(e, ValidationError::Schema { .. })));
        }
    }

    #[test]
    fn test_multiple_validation() {
        let validator = JsonValidator::new();

        let ndjson = r#"{"valid": "json"}
{"also": "valid"}
{invalid json}
{"another": "valid"}"#;

        let results = validator.validate_multiple(ndjson).unwrap();
        assert_eq!(results.len(), 4);

        assert!(results[0].1.is_valid()); // Line 1: valid
        assert!(results[1].1.is_valid()); // Line 2: valid
        assert!(!results[2].1.is_valid()); // Line 3: invalid
        assert!(results[3].1.is_valid()); // Line 4: valid
    }

    #[test]
    fn test_error_line_column_reporting() {
        let validator = JsonValidator::new();

        let invalid_json = r#"{
    "name": "test",
    "value": broken
}"#;

        let result = validator.validate(invalid_json).unwrap();
        assert!(!result.is_valid());

        if let ValidationResult::Invalid(errors) = result {
            if let Some(ValidationError::JsonSyntax { line, column, .. }) = errors.first() {
                assert!(*line > 1); // Should be on line 3 or so
                assert!(*column > 0);
            } else {
                panic!("Expected JSON syntax error with line/column info");
            }
        }
    }
}
