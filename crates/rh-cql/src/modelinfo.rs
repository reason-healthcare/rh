//! ModelInfo type definitions for CQL data model resolution.
//!
//! ModelInfo describes the structure of data models (like FHIR) so that CQL
//! can understand the types and properties available for querying.
//!
//! # Example
//!
//! ```
//! use rh_cql::modelinfo::{ModelInfo, ClassInfo, ClassInfoElement};
//!
//! let model_info = ModelInfo {
//!     name: Some("FHIR".to_string()),
//!     version: Some("4.0.1".to_string()),
//!     url: Some("http://hl7.org/fhir".to_string()),
//!     ..Default::default()
//! };
//! ```

use serde::{Deserialize, Serialize};

/// Top-level ModelInfo container describing a data model.
///
/// ModelInfo provides metadata about a data model including:
/// - Model identification (name, version, URL)
/// - Type definitions (classes, simple types, etc.)
/// - Patient context information
/// - Conversion mappings
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    /// The name of the model (e.g., "FHIR", "QDM", "QUICK").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The version of the model (e.g., "4.0.1").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The XML namespace URI for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Schema location for the model XSD (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,

    /// Target qualifier for generated code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_qualifier: Option<String>,

    /// The fully qualified name of the Patient class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_class_name: Option<String>,

    /// The identifier (URL) for the Patient class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_class_identifier: Option<String>,

    /// The property name for patient birth date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_birth_date_property_name: Option<String>,

    /// Whether the model is case-sensitive for type resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,

    /// Whether the model uses strict retrieval mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_retrievable: Option<bool>,

    /// Default context for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_context: Option<String>,

    /// Type information for all types in the model.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_info: Vec<TypeInfo>,

    /// Conversion information for implicit type conversions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conversion_info: Vec<ConversionInfo>,

    /// Context information for query contexts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_info: Vec<ContextInfo>,

    /// Required model dependencies.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_model_info: Vec<ModelSpecifier>,
}

/// Reference to a required model.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelSpecifier {
    /// The name of the required model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The version of the required model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Information about an execution context (e.g., Patient, Practitioner).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextInfo {
    /// The name of the context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The key element for the context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_element: Option<String>,

    /// The birth date element for age calculations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date_element: Option<String>,
}

/// Information about an implicit type conversion.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionInfo {
    /// The source type for the conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_type: Option<String>,

    /// The source type specifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_type_specifier: Option<TypeSpecifier>,

    /// The target type for the conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_type: Option<String>,

    /// The target type specifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_type_specifier: Option<TypeSpecifier>,

    /// The function that performs the conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
}

// =============================================================================
// Type Info
// =============================================================================

/// Type information - polymorphic container for different type kinds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TypeInfo {
    /// A simple (primitive) type.
    SimpleTypeInfo(SimpleTypeInfo),
    /// A class type with properties.
    ClassInfo(ClassInfo),
    /// A profile type (derived from a class).
    ProfileInfo(ProfileInfo),
    /// An interval type.
    IntervalTypeInfo(IntervalTypeInfo),
    /// A list type.
    ListTypeInfo(ListTypeInfo),
    /// A tuple type.
    TupleTypeInfo(TupleTypeInfo),
    /// A choice type (union of types).
    ChoiceTypeInfo(ChoiceTypeInfo),
}

/// Base fields common to all TypeInfo variants.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeInfoBase {
    /// Base type name (qualified).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,

    /// Base type specifier (for complex base types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type_specifier: Option<TypeSpecifier>,
}

/// Simple type information (primitives like String, Integer, etc.).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleTypeInfo {
    /// The namespace of the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The name of the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Base type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,

    /// Base type specifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type_specifier: Option<TypeSpecifier>,
}

/// Class type information (complex types with elements/properties).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassInfo {
    /// The namespace of the class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The name of the class (may include namespace prefix).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A unique identifier for the class (e.g., profile URL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,

    /// Human-readable label for the class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Base type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,

    /// Base type specifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type_specifier: Option<TypeSpecifier>,

    /// Whether this class can be the target of a Retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrievable: Option<bool>,

    /// The primary code path for terminology filtering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_code_path: Option<String>,

    /// The primary value set path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_value_set_path: Option<String>,

    /// Generic type parameters.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<TypeParameterInfo>,

    /// Elements (properties) of the class.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub element: Vec<ClassInfoElement>,

    /// Context relationships for the class.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_relationship: Vec<RelationshipInfo>,

    /// Target context relationships.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_context_relationship: Vec<RelationshipInfo>,
}

/// Profile type information (profile of a base class).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfo {
    /// The namespace of the profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The name of the profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A unique identifier for the profile (URL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,

    /// Human-readable label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Base type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,

    /// Base type specifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type_specifier: Option<TypeSpecifier>,

    /// Whether this profile can be the target of a Retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrievable: Option<bool>,

    /// The primary code path for terminology filtering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_code_path: Option<String>,

    /// Elements (properties) of the profile.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub element: Vec<ClassInfoElement>,
}

/// Interval type information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntervalTypeInfo {
    /// Base type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,

    /// The point type name (for simple types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_type: Option<String>,

    /// The point type specifier (for complex types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_type_specifier: Option<TypeSpecifier>,
}

/// List type information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTypeInfo {
    /// Base type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,

    /// The element type name (for simple element types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,

    /// The element type specifier (for complex element types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type_specifier: Option<TypeSpecifier>,
}

/// Tuple type information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TupleTypeInfo {
    /// Base type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,

    /// Base type specifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type_specifier: Option<TypeSpecifier>,

    /// Elements of the tuple.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub element: Vec<TupleTypeInfoElement>,
}

/// Element of a tuple type.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TupleTypeInfoElement {
    /// The name of the element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The element type name (for simple types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,

    /// The element type specifier (for complex types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type_specifier: Option<TypeSpecifier>,

    /// @deprecated - use element_type instead.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    /// @deprecated - use element_type_specifier instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_specifier: Option<TypeSpecifier>,
}

/// Choice type information (union of multiple types).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceTypeInfo {
    /// Base type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,

    /// Base type specifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_type_specifier: Option<TypeSpecifier>,

    /// The choice type names (for simple types).
    #[serde(rename = "type", default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,

    /// The choice type specifiers (for complex types).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub choice: Vec<TypeSpecifier>,
}

// =============================================================================
// Type Parameters
// =============================================================================

/// Generic type parameter information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeParameterInfo {
    /// The name of the type parameter (e.g., "T").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The constraint kind (e.g., "NONE", "CLASS", "VALUE", "TUPLE", "INTERVAL", "CHOICE", "TYPE").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint: Option<String>,

    /// The constraint type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_type: Option<String>,
}

// =============================================================================
// Class Elements
// =============================================================================

/// Element (property) of a class.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassInfoElement {
    /// The name of the element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The element type name (for simple types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,

    /// The element type specifier (for complex types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type_specifier: Option<TypeSpecifier>,

    /// @deprecated - use element_type instead.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    /// @deprecated - use element_type_specifier instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_specifier: Option<TypeSpecifier>,

    /// Whether the element is prohibited (max cardinality 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prohibited: Option<bool>,

    /// Whether the element uses one-based indexing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_based: Option<bool>,

    /// Target mapping for profile-informed authoring.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// Relationship information for context relationships.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipInfo {
    /// The context name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    /// The related key element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_key_element: Option<String>,
}

// =============================================================================
// Type Specifiers
// =============================================================================

/// Type specifier - polymorphic type reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TypeSpecifier {
    /// A named type reference.
    NamedTypeSpecifier(NamedTypeSpecifier),
    /// An interval type reference.
    IntervalTypeSpecifier(IntervalTypeSpecifier),
    /// A list type reference.
    ListTypeSpecifier(ListTypeSpecifier),
    /// A tuple type reference.
    TupleTypeSpecifier(TupleTypeSpecifier),
    /// A choice type reference.
    ChoiceTypeSpecifier(ChoiceTypeSpecifier),
    /// A type parameter reference.
    ParameterTypeSpecifier(ParameterTypeSpecifier),
    /// A bound parameter type reference.
    BoundParameterTypeSpecifier(BoundParameterTypeSpecifier),
}

/// Named type specifier.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedTypeSpecifier {
    /// The model/namespace name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,

    /// The namespace (preferred over model_name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Interval type specifier.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntervalTypeSpecifier {
    /// The point type name (for simple point types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_type: Option<String>,

    /// The point type specifier (for complex point types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_type_specifier: Option<Box<TypeSpecifier>>,
}

/// List type specifier.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTypeSpecifier {
    /// The element type name (for simple element types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,

    /// The element type specifier (for complex element types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type_specifier: Option<Box<TypeSpecifier>>,
}

/// Tuple type specifier.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TupleTypeSpecifier {
    /// Elements of the tuple type.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub element: Vec<TupleElementDefinition>,
}

/// Element definition in a tuple type specifier.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TupleElementDefinition {
    /// The name of the element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The element type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,

    /// The element type specifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type_specifier: Option<Box<TypeSpecifier>>,

    /// @deprecated - use element_type instead.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

/// Choice type specifier (union of types).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceTypeSpecifier {
    /// The choice type specifiers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub choice: Vec<TypeSpecifier>,

    /// @deprecated - use choice instead.
    #[serde(rename = "type", default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
}

/// Parameter type specifier (generic type parameter reference).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterTypeSpecifier {
    /// The name of the type parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
}

/// Bound parameter type specifier (generic type with bound parameter).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundParameterTypeSpecifier {
    /// The name of the type parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,

    /// The bound type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bound_type: Option<String>,

    /// The bound type specifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bound_type_specifier: Option<Box<TypeSpecifier>>,
}

// =============================================================================
// System Model Types
// =============================================================================

/// System model primitive types.
///
/// These are the built-in CQL types that exist in the System model.
pub mod system {
    /// The System model namespace.
    pub const NAMESPACE: &str = "System";

    /// System.Any - the root type of all types.
    pub const ANY: &str = "System.Any";
    /// System.Boolean
    pub const BOOLEAN: &str = "System.Boolean";
    /// System.Integer
    pub const INTEGER: &str = "System.Integer";
    /// System.Long
    pub const LONG: &str = "System.Long";
    /// System.Decimal
    pub const DECIMAL: &str = "System.Decimal";
    /// System.String
    pub const STRING: &str = "System.String";
    /// System.Date
    pub const DATE: &str = "System.Date";
    /// System.DateTime
    pub const DATE_TIME: &str = "System.DateTime";
    /// System.Time
    pub const TIME: &str = "System.Time";
    /// System.Quantity
    pub const QUANTITY: &str = "System.Quantity";
    /// System.Ratio
    pub const RATIO: &str = "System.Ratio";
    /// System.Code
    pub const CODE: &str = "System.Code";
    /// System.Concept
    pub const CONCEPT: &str = "System.Concept";
    /// System.ValueSet
    pub const VALUE_SET: &str = "System.ValueSet";
    /// System.CodeSystem
    pub const CODE_SYSTEM: &str = "System.CodeSystem";
    /// System.Vocabulary
    pub const VOCABULARY: &str = "System.Vocabulary";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_info_serialization() {
        let model_info = ModelInfo {
            name: Some("FHIR".to_string()),
            version: Some("4.0.1".to_string()),
            url: Some("http://hl7.org/fhir".to_string()),
            patient_class_name: Some("FHIR.Patient".to_string()),
            patient_birth_date_property_name: Some("birthDate".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&model_info).unwrap();
        assert!(json.contains("\"name\": \"FHIR\""));
        assert!(json.contains("\"version\": \"4.0.1\""));

        let parsed: ModelInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, Some("FHIR".to_string()));
    }

    #[test]
    fn test_class_info_serialization() {
        let class_info = ClassInfo {
            namespace: Some("FHIR".to_string()),
            name: Some("Patient".to_string()),
            identifier: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
            base_type: Some("FHIR.DomainResource".to_string()),
            retrievable: Some(true),
            primary_code_path: None,
            element: vec![
                ClassInfoElement {
                    name: Some("identifier".to_string()),
                    element_type: Some("list<FHIR.Identifier>".to_string()),
                    ..Default::default()
                },
                ClassInfoElement {
                    name: Some("active".to_string()),
                    element_type: Some("FHIR.boolean".to_string()),
                    ..Default::default()
                },
                ClassInfoElement {
                    name: Some("name".to_string()),
                    element_type: Some("list<FHIR.HumanName>".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&class_info).unwrap();
        assert!(json.contains("\"name\": \"Patient\""));
        assert!(json.contains("\"retrievable\": true"));
    }

    #[test]
    fn test_type_specifier_named() {
        let specifier = TypeSpecifier::NamedTypeSpecifier(NamedTypeSpecifier {
            namespace: Some("FHIR".to_string()),
            name: Some("Patient".to_string()),
            ..Default::default()
        });

        let json = serde_json::to_string(&specifier).unwrap();
        assert!(json.contains("NamedTypeSpecifier"));
        assert!(json.contains("Patient"));
    }

    #[test]
    fn test_type_specifier_list() {
        let specifier = TypeSpecifier::ListTypeSpecifier(ListTypeSpecifier {
            element_type: Some("FHIR.Observation".to_string()),
            ..Default::default()
        });

        let json = serde_json::to_string(&specifier).unwrap();
        assert!(json.contains("ListTypeSpecifier"));
        assert!(json.contains("Observation"));
    }

    #[test]
    fn test_type_specifier_interval() {
        let specifier = TypeSpecifier::IntervalTypeSpecifier(IntervalTypeSpecifier {
            point_type: Some("System.DateTime".to_string()),
            ..Default::default()
        });

        let json = serde_json::to_string(&specifier).unwrap();
        assert!(json.contains("IntervalTypeSpecifier"));
        assert!(json.contains("DateTime"));
    }

    #[test]
    fn test_type_specifier_choice() {
        let specifier = TypeSpecifier::ChoiceTypeSpecifier(ChoiceTypeSpecifier {
            choice: vec![
                TypeSpecifier::NamedTypeSpecifier(NamedTypeSpecifier {
                    namespace: Some("FHIR".to_string()),
                    name: Some("string".to_string()),
                    ..Default::default()
                }),
                TypeSpecifier::NamedTypeSpecifier(NamedTypeSpecifier {
                    namespace: Some("FHIR".to_string()),
                    name: Some("CodeableConcept".to_string()),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        });

        let json = serde_json::to_string(&specifier).unwrap();
        assert!(json.contains("ChoiceTypeSpecifier"));
    }

    #[test]
    fn test_simple_type_info() {
        let type_info = TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
            namespace: Some("System".to_string()),
            name: Some("Boolean".to_string()),
            base_type: Some("System.Any".to_string()),
            ..Default::default()
        });

        let json = serde_json::to_string(&type_info).unwrap();
        assert!(json.contains("SimpleTypeInfo"));
        assert!(json.contains("Boolean"));
    }

    #[test]
    fn test_interval_type_info() {
        let type_info = TypeInfo::IntervalTypeInfo(IntervalTypeInfo {
            point_type: Some("System.DateTime".to_string()),
            ..Default::default()
        });

        let json = serde_json::to_string(&type_info).unwrap();
        assert!(json.contains("IntervalTypeInfo"));
    }

    #[test]
    fn test_list_type_info() {
        let type_info = TypeInfo::ListTypeInfo(ListTypeInfo {
            element_type: Some("FHIR.Observation".to_string()),
            ..Default::default()
        });

        let json = serde_json::to_string(&type_info).unwrap();
        assert!(json.contains("ListTypeInfo"));
    }

    #[test]
    fn test_tuple_type_info() {
        let type_info = TypeInfo::TupleTypeInfo(TupleTypeInfo {
            element: vec![
                TupleTypeInfoElement {
                    name: Some("id".to_string()),
                    element_type: Some("System.String".to_string()),
                    ..Default::default()
                },
                TupleTypeInfoElement {
                    name: Some("value".to_string()),
                    element_type: Some("System.Integer".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        let json = serde_json::to_string(&type_info).unwrap();
        assert!(json.contains("TupleTypeInfo"));
    }

    #[test]
    fn test_conversion_info() {
        let conversion = ConversionInfo {
            from_type: Some("FHIR.string".to_string()),
            to_type: Some("System.String".to_string()),
            function_name: Some("FHIRHelpers.ToString".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&conversion).unwrap();
        assert!(json.contains("FHIRHelpers.ToString"));
    }

    #[test]
    fn test_context_info() {
        let context = ContextInfo {
            name: Some("Patient".to_string()),
            key_element: Some("id".to_string()),
            birth_date_element: Some("birthDate".to_string()),
        };

        let json = serde_json::to_string(&context).unwrap();
        assert!(json.contains("Patient"));
        assert!(json.contains("birthDate"));
    }
}
