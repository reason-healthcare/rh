//! ELM Library type definition.
//!
//! The Library is the top-level container for CQL/ELM content.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{
    AccessModifier, Annotation, CqlToElmError, CqlToElmInfo, Expression, FunctionDef, QName,
    TypeSpecifier,
};

/// Serialize Option<VersionedIdentifier> - always outputs, even if None (as empty object).
fn serialize_optional_identifier<S>(
    value: &Option<VersionedIdentifier>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(id) => id.serialize(serializer),
        None => VersionedIdentifier::default().serialize(serializer),
    }
}

/// Deserialize Option<VersionedIdentifier> - empty object becomes None.
fn deserialize_optional_identifier<'de, D>(
    deserializer: D,
) -> Result<Option<VersionedIdentifier>, D::Error>
where
    D: Deserializer<'de>,
{
    let id = VersionedIdentifier::deserialize(deserializer)?;
    // If all fields are None, treat as None
    if id.id.is_none() && id.system.is_none() && id.version.is_none() {
        Ok(None)
    } else {
        Ok(Some(id))
    }
}

/// An ELM Library - the top-level container for CQL content.
///
/// A library contains all the definitions from a CQL file including:
/// - Metadata (identifier, schema, version info)
/// - Using declarations (data model references)
/// - Include declarations (library dependencies)
/// - Code system, value set, code, and concept definitions
/// - Parameter definitions
/// - Context definitions
/// - Expression and function definitions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    /// Local identifier for tracking (Library extends Element).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,

    /// Library identifier (name and version).
    /// Always serialized, even if None (as empty object).
    #[serde(
        default,
        serialize_with = "serialize_optional_identifier",
        deserialize_with = "deserialize_optional_identifier"
    )]
    pub identifier: Option<VersionedIdentifier>,

    /// Schema identifier for this ELM version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_identifier: Option<VersionedIdentifier>,

    /// Using declarations for data models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usings: Option<UsingDefs>,

    /// Include declarations for library dependencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<IncludeDefs>,

    /// Parameter definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefs>,

    /// Code system definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_systems: Option<CodeSystemDefs>,

    /// Value set definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_sets: Option<ValueSetDefs>,

    /// Code definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codes: Option<CodeDefs>,

    /// Concept definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concepts: Option<ConceptDefs>,

    /// Context definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<ContextDefs>,

    /// Expression definitions (define statements).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<ExpressionDefs>,

    /// Annotations including CqlToElmInfo.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotation: Vec<LibraryAnnotation>,
}

/// A versioned identifier (name + version).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedIdentifier {
    /// The identifier/name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The system/namespace URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// The version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Library-level annotation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LibraryAnnotation {
    /// General annotation.
    Annotation(Annotation),
    /// CQL-to-ELM translator info.
    CqlToElmInfo(CqlToElmInfo),
    /// CQL-to-ELM error/warning.
    CqlToElmError(CqlToElmError),
}

/// Container for using definitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsingDefs {
    #[serde(rename = "def", default, skip_serializing_if = "Vec::is_empty")]
    pub defs: Vec<UsingDef>,
}

/// A using declaration referencing a data model.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsingDef {
    /// Local alias for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_identifier: Option<String>,

    /// The model URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// The model version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Container for include definitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncludeDefs {
    #[serde(rename = "def", default, skip_serializing_if = "Vec::is_empty")]
    pub defs: Vec<IncludeDef>,
}

/// An include declaration referencing another library.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncludeDef {
    /// Local alias for the library.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_identifier: Option<String>,

    /// The library path/name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// The library version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Container for parameter definitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDefs {
    #[serde(rename = "def", default, skip_serializing_if = "Vec::is_empty")]
    pub defs: Vec<ParameterDef>,
}

/// A parameter definition.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDef {
    /// Parameter name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Access level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<AccessModifier>,

    /// Parameter type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_type_name: Option<QName>,

    /// Parameter type specifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_type_specifier: Option<TypeSpecifier>,

    /// Default value expression.
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Box<Expression>>,
}

/// Container for code system definitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemDefs {
    #[serde(rename = "def", default, skip_serializing_if = "Vec::is_empty")]
    pub defs: Vec<CodeSystemDef>,
}

/// A code system definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemDef {
    /// Code system name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Code system identifier (URI).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Code system version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Access level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<AccessModifier>,
}

/// Container for value set definitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetDefs {
    #[serde(rename = "def", default, skip_serializing_if = "Vec::is_empty")]
    pub defs: Vec<ValueSetDef>,
}

/// A value set definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetDef {
    /// Value set name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Value set identifier (URI).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Value set version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Access level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<AccessModifier>,

    /// Code systems referenced by this value set.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_system: Vec<CodeSystemDefRef>,
}

/// A code system reference within a value set definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemDefRef {
    /// The name of the code system definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The library containing the code system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_name: Option<String>,
}

/// Container for code definitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeDefs {
    #[serde(rename = "def", default, skip_serializing_if = "Vec::is_empty")]
    pub defs: Vec<CodeDef>,
}

/// A code definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeDef {
    /// Code definition name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The code value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Display text for the code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    /// Access level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<AccessModifier>,

    /// The code system this code belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_system: Option<CodeSystemDefRef>,
}

/// Container for concept definitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptDefs {
    #[serde(rename = "def", default, skip_serializing_if = "Vec::is_empty")]
    pub defs: Vec<ConceptDef>,
}

/// A concept definition (grouping of codes).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptDef {
    /// Concept definition name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Display text for the concept.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    /// Access level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<AccessModifier>,

    /// The codes in this concept.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<CodeDefRef>,
}

/// A code reference within a concept definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeDefRef {
    /// The name of the code definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The library containing the code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_name: Option<String>,
}

/// Container for context definitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextDefs {
    #[serde(rename = "def", default, skip_serializing_if = "Vec::is_empty")]
    pub defs: Vec<ContextDef>,
}

/// A context definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextDef {
    /// Context name (e.g., "Patient", "Practitioner").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Container for statement definitions (expressions and functions).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionDefs {
    #[serde(rename = "def", default, skip_serializing_if = "Vec::is_empty")]
    pub defs: Vec<StatementDef>,
}

/// A statement definition (expression or function).
#[derive(Debug, Clone, PartialEq)]
pub enum StatementDef {
    /// An expression definition.
    Expression(ExpressionDef),
    /// A function definition.
    Function(FunctionDef),
}

impl Serialize for StatementDef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            StatementDef::Expression(definition) => serde_json::to_value(definition),
            StatementDef::Function(definition) => serde_json::to_value(definition),
        };
        let type_name = match self {
            StatementDef::Expression(_) => "ExpressionDef",
            StatementDef::Function(_) => "FunctionDef",
        };
        let mut value = value.map_err(serde::ser::Error::custom)?;
        let object = value.as_object_mut().ok_or_else(|| {
            serde::ser::Error::custom("statement definition must serialize as an object")
        })?;
        object.insert(
            "type".to_owned(),
            serde_json::Value::String(type_name.to_owned()),
        );
        value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StatementDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let object = value
            .as_object()
            .ok_or_else(|| serde::de::Error::custom("statement definition must be an object"))?;
        match object.get("type") {
            Some(serde_json::Value::String(type_name)) => match type_name.as_str() {
                "ExpressionDef" => serde_json::from_value(value)
                    .map(StatementDef::Expression)
                    .map_err(serde::de::Error::custom),
                "FunctionDef" => serde_json::from_value(value)
                    .map(StatementDef::Function)
                    .map_err(serde::de::Error::custom),
                other => Err(serde::de::Error::custom(format!(
                    "unknown statement definition type {other}"
                ))),
            },
            Some(_) => Err(serde::de::Error::custom(
                "statement definition type must be a string",
            )),
            // RH's early native ELM omitted this discriminator. `operand`,
            // including an empty array, identifies FunctionDef; all other
            // legacy entries are ExpressionDef. New output is tagged above.
            None if object.contains_key("operand") => serde_json::from_value(value)
                .map(StatementDef::Function)
                .map_err(serde::de::Error::custom),
            None => serde_json::from_value(value)
                .map(StatementDef::Expression)
                .map_err(serde::de::Error::custom),
        }
    }
}

/// An expression definition (CQL define statement).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionDef {
    /// Unique identifier within the ELM tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,

    /// Source locator information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,

    /// Expression name.
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

    /// The expression body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Box<Expression>>,

    /// Annotations on this definition.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotation: Vec<Annotation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_serialization() {
        let library = Library {
            identifier: Some(VersionedIdentifier {
                id: Some("TestLibrary".into()),
                version: Some("1.0.0".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&library).unwrap();
        assert!(json.contains("\"id\":\"TestLibrary\""));
        assert!(json.contains("\"version\":\"1.0.0\""));
    }

    #[test]
    fn test_using_def_serialization() {
        let using = UsingDef {
            local_identifier: Some("FHIR".into()),
            uri: Some("http://hl7.org/fhir".into()),
            version: Some("4.0.1".into()),
        };
        let json = serde_json::to_string(&using).unwrap();
        assert!(json.contains("\"localIdentifier\":\"FHIR\""));
    }

    #[test]
    fn test_expression_def_serialization() {
        let def = ExpressionDef {
            name: Some("InPopulation".into()),
            context: Some("Patient".into()),
            access_level: Some(AccessModifier::Public),
            ..Default::default()
        };
        let json = serde_json::to_string(&def).unwrap();
        assert!(json.contains("\"name\":\"InPopulation\""));
        assert!(json.contains("\"context\":\"Patient\""));
    }

    #[test]
    fn statement_definition_reads_legacy_untagged_entries_and_writes_tags() {
        let expression: StatementDef = serde_json::from_str(
            r#"{"name":"Population","context":"Patient","expression":{"type":"Literal","valueType":"{urn:hl7-org:elm-types:r1}Boolean","value":"true"}}"#,
        )
        .unwrap();
        assert!(matches!(expression, StatementDef::Expression(_)));
        let expression_json = serde_json::to_value(expression).unwrap();
        assert_eq!(expression_json["type"], "ExpressionDef");

        let function: StatementDef =
            serde_json::from_str(r#"{"name":"Utility","operand":[]}"#).unwrap();
        assert!(matches!(function, StatementDef::Function(_)));
        let function_json = serde_json::to_value(function).unwrap();
        assert_eq!(function_json["type"], "FunctionDef");
    }

    #[test]
    fn statement_definition_rejects_unknown_explicit_tag() {
        let error = serde_json::from_str::<StatementDef>(r#"{"type":"OtherDef"}"#)
            .expect_err("unknown explicit statement type must fail");
        assert!(error
            .to_string()
            .contains("unknown statement definition type"));
    }
}
