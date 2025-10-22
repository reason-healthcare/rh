//! FHIR type metadata structures
//!
//! This module defines the data structures used for FHIR type metadata that will be
//! generated into the target crate's metadata.rs file.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// FHIR primitive types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FhirPrimitiveType {
    Boolean,
    Integer,
    String,
    Date,
    DateTime,
    Instant,
    Time,
    Decimal,
    Uri,
    Url,
    Canonical,
    Code,
    Oid,
    Id,
    Markdown,
    Base64Binary,
    UnsignedInt,
    PositiveInt,
}

impl FhirPrimitiveType {
    /// Convert FHIR type string to FhirPrimitiveType
    pub fn from_fhir_type(fhir_type: &str) -> Option<Self> {
        match fhir_type {
            "boolean" => Some(Self::Boolean),
            "integer" => Some(Self::Integer),
            "string" => Some(Self::String),
            "date" => Some(Self::Date),
            "dateTime" => Some(Self::DateTime),
            "instant" => Some(Self::Instant),
            "time" => Some(Self::Time),
            "decimal" => Some(Self::Decimal),
            "uri" => Some(Self::Uri),
            "url" => Some(Self::Url),
            "canonical" => Some(Self::Canonical),
            "code" => Some(Self::Code),
            "oid" => Some(Self::Oid),
            "id" => Some(Self::Id),
            "markdown" => Some(Self::Markdown),
            "base64Binary" => Some(Self::Base64Binary),
            "unsignedInt" => Some(Self::UnsignedInt),
            "positiveInt" => Some(Self::PositiveInt),
            _ => None,
        }
    }

    /// Get the enum variant name as a string for code generation
    pub fn variant_name(&self) -> &'static str {
        match self {
            Self::Boolean => "Boolean",
            Self::Integer => "Integer",
            Self::String => "String",
            Self::Date => "Date",
            Self::DateTime => "DateTime",
            Self::Instant => "Instant",
            Self::Time => "Time",
            Self::Decimal => "Decimal",
            Self::Uri => "Uri",
            Self::Url => "Url",
            Self::Canonical => "Canonical",
            Self::Code => "Code",
            Self::Oid => "Oid",
            Self::Id => "Id",
            Self::Markdown => "Markdown",
            Self::Base64Binary => "Base64Binary",
            Self::UnsignedInt => "UnsignedInt",
            Self::PositiveInt => "PositiveInt",
        }
    }
}

/// FHIR field type (primitive, complex, reference, or backbone element)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FhirFieldType {
    Primitive(FhirPrimitiveType),
    Complex(String),         // e.g., "HumanName", "Address"
    Reference,               // Reference to another resource
    BackboneElement(String), // Nested structure defined within the parent
}

/// Information about a field in a FHIR resource or datatype
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldInfo {
    pub field_type: FhirFieldType,
    pub min: u32,
    pub max: Option<u32>, // None = unbounded (*)
    pub is_choice_type: bool,
    pub choice_types: Vec<String>, // If choice type, all possible types (e.g., ["boolean", "dateTime"])
}

/// Metadata for a FHIR type (resource or datatype)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeMetadata {
    pub name: String,
    pub fields: HashMap<String, FieldInfo>,
}

/// Complete metadata registry for all FHIR types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataRegistry {
    pub types: HashMap<String, TypeMetadata>,
}

impl MetadataRegistry {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    pub fn add_type(&mut self, type_metadata: TypeMetadata) {
        self.types.insert(type_metadata.name.clone(), type_metadata);
    }

    /// Resolve a path like "Patient.name.given" to its field type
    pub fn resolve_path(&self, path: &str) -> Option<&FhirFieldType> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return None;
        }

        // Start with the root type
        let mut current_type_name = parts[0];
        let mut current_type = self.types.get(current_type_name)?;

        // Navigate through the path
        for &field_name in &parts[1..] {
            let field_info = current_type.fields.get(field_name)?;

            // If this is the last part of the path, return this field's type
            if field_name == *parts.last().unwrap() {
                return Some(&field_info.field_type);
            }

            // Otherwise, continue navigating
            match &field_info.field_type {
                FhirFieldType::Complex(type_name) | FhirFieldType::BackboneElement(type_name) => {
                    current_type_name = type_name;
                    current_type = self.types.get(current_type_name)?;
                }
                FhirFieldType::Reference => {
                    // Can't navigate further into a Reference
                    return None;
                }
                FhirFieldType::Primitive(_) => {
                    // Can't navigate further into a primitive
                    return None;
                }
            }
        }

        None
    }
}

impl Default for MetadataRegistry {
    fn default() -> Self {
        Self::new()
    }
}
