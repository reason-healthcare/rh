//! ELM definition types.
//!
//! This module contains types for library-level definitions like
//! FunctionDef and OperandDef.

use serde::{Deserialize, Serialize};

use super::{AccessModifier, Annotation, Expression, QName, TypeSpecifier};

/// A function definition.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDef {
    /// Unique identifier within the ELM tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,

    /// Source locator information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,

    /// Function name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Evaluation context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    /// Access level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<AccessModifier>,

    /// Result type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type_name: Option<QName>,

    /// Result type specifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type_specifier: Option<TypeSpecifier>,

    /// The function operands (parameters).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operand: Vec<OperandDef>,

    /// The function body expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Box<Expression>>,

    /// Whether this is an external function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,

    /// Whether this function can be called fluently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluent: Option<bool>,

    /// Annotations on this definition.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotation: Vec<Annotation>,
}

/// A function operand (parameter) definition.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperandDef {
    /// Operand name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Operand type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand_type_name: Option<QName>,

    /// Operand type specifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand_type_specifier: Option<TypeSpecifier>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_def_serialization() {
        let func = FunctionDef {
            name: Some("Add".into()),
            operand: vec![
                OperandDef {
                    name: Some("a".into()),
                    operand_type_name: Some("{urn:hl7-org:elm-types:r1}Integer".into()),
                    ..Default::default()
                },
                OperandDef {
                    name: Some("b".into()),
                    operand_type_name: Some("{urn:hl7-org:elm-types:r1}Integer".into()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let json = serde_json::to_string(&func).unwrap();
        assert!(json.contains("\"name\":\"Add\""));
        assert!(json.contains("\"name\":\"a\""));
        assert!(json.contains("\"name\":\"b\""));
    }

    #[test]
    fn test_external_function() {
        let func = FunctionDef {
            name: Some("ExternalFunc".into()),
            external: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&func).unwrap();
        assert!(json.contains("\"external\":true"));
    }
}
