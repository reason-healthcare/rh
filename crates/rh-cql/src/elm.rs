//! ELM (Expression Logical Model) type definitions.
//!
//! This module provides Rust representations of the HL7 ELM specification,
//! which is the canonical representation of CQL (Clinical Quality Language).
//!
//! # Overview
//!
//! ELM defines a structured, machine-processable representation of clinical logic.
//! CQL source code is translated into ELM for execution. The type system includes:
//!
//! - **Library** - The top-level container for CQL/ELM content
//! - **Type Specifiers** - Represent CQL types (Named, List, Interval, Tuple, Choice)
//! - **Expressions** - The core expression tree (literals, operators, queries, etc.)
//! - **Definitions** - Named expressions, functions, parameters, code systems, etc.
//!
//! # ELM JSON Serialization
//!
//! All types implement serde serialization matching the official ELM JSON schema.
//! Polymorphic types use the `type` field as a discriminator.
//!
//! # Reference
//!
//! - [ELM Specification](https://cql.hl7.org/elm.html)
//! - [ELM XML Schema](https://cql.hl7.org/elm/schema/expression.xsd)
//!
//! # Example
//!
//! ```
//! use rh_cql::elm::{Library, Expression, Literal};
//! use serde_json;
//!
//! let literal = Expression::Literal(Literal {
//!     value_type: Some("{urn:hl7-org:elm-types:r1}Integer".into()),
//!     value: Some("42".into()),
//!     ..Default::default()
//! });
//!
//! let json = serde_json::to_string(&literal).unwrap();
//! assert!(json.contains("\"type\":\"Literal\""));
//! ```

mod annotation;
mod definitions;
mod expression;
mod library;
mod type_specifier;

pub use annotation::*;
pub use definitions::*;
pub use expression::*;
pub use library::*;
pub use type_specifier::*;

use serde::{Deserialize, Serialize};

/// A qualified name in the format `{namespace}name` or `namespace:name`.
///
/// In ELM, qualified names are used to reference types, typically in the
/// format `{urn:hl7-org:elm-types:r1}Integer` for built-in types.
pub type QName = String;

/// Access level for definitions (expressions, functions, code systems, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AccessModifier {
    /// Accessible from other libraries.
    #[default]
    Public,
    /// Only accessible within the defining library.
    Private,
}

/// Precision for date/time operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DateTimePrecision {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
}

/// Sort direction for query sort clauses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    #[default]
    #[serde(alias = "ascending")]
    Asc,
    #[serde(alias = "descending")]
    Desc,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_modifier_default() {
        let access: AccessModifier = Default::default();
        assert_eq!(access, AccessModifier::Public);
    }

    #[test]
    fn test_access_modifier_serialization() {
        let public = serde_json::to_string(&AccessModifier::Public).unwrap();
        assert_eq!(public, "\"Public\"");

        let private = serde_json::to_string(&AccessModifier::Private).unwrap();
        assert_eq!(private, "\"Private\"");
    }

    #[test]
    fn test_datetime_precision_serialization() {
        let precision = DateTimePrecision::Millisecond;
        let json = serde_json::to_string(&precision).unwrap();
        assert_eq!(json, "\"millisecond\"");
    }

    #[test]
    fn test_sort_direction_default() {
        let dir: SortDirection = Default::default();
        assert_eq!(dir, SortDirection::Asc);
    }

    #[test]
    fn test_sort_direction_alias_deserialization() {
        let asc: SortDirection = serde_json::from_str("\"ascending\"").unwrap();
        assert_eq!(asc, SortDirection::Asc);

        let desc: SortDirection = serde_json::from_str("\"descending\"").unwrap();
        assert_eq!(desc, SortDirection::Desc);
    }
}
