//! ELM type specifier types.
//!
//! Type specifiers represent CQL types in ELM. They are used to express
//! the types of expressions, parameters, and return values.

use serde::{Deserialize, Serialize};

use super::QName;

/// A type specifier represents a CQL type.
///
/// ELM uses type specifiers to represent various kinds of types including
/// named types (like `Integer`, `Patient`), list types, interval types,
/// tuple types, and choice types.
///
/// # Example
///
/// ```
/// use rh_cql::elm::{TypeSpecifier, NamedTypeSpecifier, ListTypeSpecifier};
///
/// // A named type
/// let integer_type = TypeSpecifier::Named(NamedTypeSpecifier {
///     name: "{urn:hl7-org:elm-types:r1}Integer".into(),
///     ..Default::default()
/// });
///
/// // A list type
/// let list_of_integers = TypeSpecifier::List(ListTypeSpecifier {
///     element_type: Some(Box::new(integer_type)),
///     ..Default::default()
/// });
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TypeSpecifier {
    /// A named type reference (e.g., `Integer`, `Patient`).
    #[serde(rename = "NamedTypeSpecifier")]
    Named(NamedTypeSpecifier),

    /// A list type (e.g., `List<Integer>`).
    #[serde(rename = "ListTypeSpecifier")]
    List(ListTypeSpecifier),

    /// An interval type (e.g., `Interval<Integer>`).
    #[serde(rename = "IntervalTypeSpecifier")]
    Interval(IntervalTypeSpecifier),

    /// A tuple type with named elements.
    #[serde(rename = "TupleTypeSpecifier")]
    Tuple(TupleTypeSpecifier),

    /// A choice type representing a union of types.
    #[serde(rename = "ChoiceTypeSpecifier")]
    Choice(ChoiceTypeSpecifier),

    /// A parameterized type specifier.
    #[serde(rename = "ParameterTypeSpecifier")]
    Parameter(ParameterTypeSpecifier),
}

impl Default for TypeSpecifier {
    fn default() -> Self {
        TypeSpecifier::Named(NamedTypeSpecifier::default())
    }
}

/// A named type specifier referencing a type by qualified name.
///
/// Named type specifiers are used for primitive types, model types,
/// and user-defined types.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedTypeSpecifier {
    /// Unique identifier within the ELM tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,

    /// Source locator information (e.g., "10:5-10:20").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,

    /// The qualified name of the type.
    pub name: QName,
}

/// A list type specifier representing `List<T>`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTypeSpecifier {
    /// Unique identifier within the ELM tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,

    /// Source locator information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,

    /// The type of elements in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<Box<TypeSpecifier>>,
}

/// An interval type specifier representing `Interval<T>`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntervalTypeSpecifier {
    /// Unique identifier within the ELM tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,

    /// Source locator information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,

    /// The point type of the interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_type: Option<Box<TypeSpecifier>>,
}

/// A tuple type specifier with named elements.
///
/// Tuples are anonymous record types with named, typed elements.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TupleTypeSpecifier {
    /// Unique identifier within the ELM tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,

    /// Source locator information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,

    /// The elements of the tuple type.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub element: Vec<TupleElementDefinition>,
}

/// Definition of a tuple element.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TupleElementDefinition {
    /// The name of the element.
    pub name: String,

    /// The type of the element.
    #[serde(rename = "elementType", skip_serializing_if = "Option::is_none")]
    pub element_type: Option<Box<TypeSpecifier>>,

    /// The type of the element (deprecated, use elementType).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_specifier: Option<Box<TypeSpecifier>>,
}

/// A choice type specifier representing a union of types.
///
/// Choice types allow a value to be one of several possible types.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceTypeSpecifier {
    /// Unique identifier within the ELM tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,

    /// Source locator information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,

    /// The types that comprise this choice.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub choice: Vec<TypeSpecifier>,
}

/// A parameterized type specifier for generic types.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterTypeSpecifier {
    /// Unique identifier within the ELM tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,

    /// Source locator information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,

    /// The parameter name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_type_specifier_serialization() {
        let ts = TypeSpecifier::Named(NamedTypeSpecifier {
            name: "{urn:hl7-org:elm-types:r1}Integer".into(),
            ..Default::default()
        });
        let json = serde_json::to_string(&ts).unwrap();
        assert!(json.contains("\"type\":\"NamedTypeSpecifier\""));
        assert!(json.contains("\"name\":\"{urn:hl7-org:elm-types:r1}Integer\""));
    }

    #[test]
    fn test_list_type_specifier_serialization() {
        let element_type = TypeSpecifier::Named(NamedTypeSpecifier {
            name: "{urn:hl7-org:elm-types:r1}String".into(),
            ..Default::default()
        });
        let ts = TypeSpecifier::List(ListTypeSpecifier {
            element_type: Some(Box::new(element_type)),
            ..Default::default()
        });
        let json = serde_json::to_string(&ts).unwrap();
        assert!(json.contains("\"type\":\"ListTypeSpecifier\""));
        assert!(json.contains("\"elementType\""));
    }

    #[test]
    fn test_interval_type_specifier_serialization() {
        let point_type = TypeSpecifier::Named(NamedTypeSpecifier {
            name: "{urn:hl7-org:elm-types:r1}DateTime".into(),
            ..Default::default()
        });
        let ts = TypeSpecifier::Interval(IntervalTypeSpecifier {
            point_type: Some(Box::new(point_type)),
            ..Default::default()
        });
        let json = serde_json::to_string(&ts).unwrap();
        assert!(json.contains("\"type\":\"IntervalTypeSpecifier\""));
        assert!(json.contains("\"pointType\""));
    }

    #[test]
    fn test_tuple_type_specifier_serialization() {
        let ts = TypeSpecifier::Tuple(TupleTypeSpecifier {
            element: vec![
                TupleElementDefinition {
                    name: "id".into(),
                    element_type: Some(Box::new(TypeSpecifier::Named(NamedTypeSpecifier {
                        name: "{urn:hl7-org:elm-types:r1}String".into(),
                        ..Default::default()
                    }))),
                    type_specifier: None,
                },
                TupleElementDefinition {
                    name: "count".into(),
                    element_type: Some(Box::new(TypeSpecifier::Named(NamedTypeSpecifier {
                        name: "{urn:hl7-org:elm-types:r1}Integer".into(),
                        ..Default::default()
                    }))),
                    type_specifier: None,
                },
            ],
            ..Default::default()
        });
        let json = serde_json::to_string(&ts).unwrap();
        assert!(json.contains("\"type\":\"TupleTypeSpecifier\""));
        assert!(json.contains("\"name\":\"id\""));
        assert!(json.contains("\"name\":\"count\""));
    }

    #[test]
    fn test_choice_type_specifier_serialization() {
        let ts = TypeSpecifier::Choice(ChoiceTypeSpecifier {
            choice: vec![
                TypeSpecifier::Named(NamedTypeSpecifier {
                    name: "{urn:hl7-org:elm-types:r1}Integer".into(),
                    ..Default::default()
                }),
                TypeSpecifier::Named(NamedTypeSpecifier {
                    name: "{urn:hl7-org:elm-types:r1}Decimal".into(),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        });
        let json = serde_json::to_string(&ts).unwrap();
        assert!(json.contains("\"type\":\"ChoiceTypeSpecifier\""));
        assert!(json.contains("\"choice\""));
    }

    #[test]
    fn test_type_specifier_deserialization() {
        let json = r#"{"type":"NamedTypeSpecifier","name":"{urn:hl7-org:elm-types:r1}Boolean"}"#;
        let ts: TypeSpecifier = serde_json::from_str(json).unwrap();
        match ts {
            TypeSpecifier::Named(named) => {
                assert_eq!(named.name, "{urn:hl7-org:elm-types:r1}Boolean");
            }
            _ => panic!("Expected NamedTypeSpecifier"),
        }
    }

    #[test]
    fn test_nested_type_specifiers() {
        let inner = TypeSpecifier::Named(NamedTypeSpecifier {
            name: "{urn:hl7-org:elm-types:r1}Integer".into(),
            ..Default::default()
        });
        let list = TypeSpecifier::List(ListTypeSpecifier {
            element_type: Some(Box::new(inner)),
            ..Default::default()
        });
        let interval_of_list = TypeSpecifier::Interval(IntervalTypeSpecifier {
            point_type: Some(Box::new(list)),
            ..Default::default()
        });

        let json = serde_json::to_string(&interval_of_list).unwrap();
        let roundtrip: TypeSpecifier = serde_json::from_str(&json).unwrap();
        assert_eq!(interval_of_list, roundtrip);
    }
}
