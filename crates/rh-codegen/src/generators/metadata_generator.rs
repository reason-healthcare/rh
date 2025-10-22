//! Metadata generation for FHIR types
//!
//! This module generates a metadata.rs file containing type information for all FHIR resources
//! and datatypes. This metadata enables runtime path resolution like "Patient.name.given" -> string.

use crate::fhir_types::{ElementDefinition, StructureDefinition};
use crate::metadata::{
    FhirFieldType, FhirPrimitiveType, FieldInfo, MetadataRegistry, TypeMetadata,
};
use std::collections::HashMap;

/// Build metadata registry from StructureDefinitions
pub fn build_metadata_registry(structure_defs: &[StructureDefinition]) -> MetadataRegistry {
    let mut registry = MetadataRegistry::new();

    for structure_def in structure_defs {
        if let Some(type_metadata) = extract_type_metadata(structure_def) {
            // Only add if not already present (avoid duplicates)
            if !registry.types.contains_key(&type_metadata.name) {
                registry.add_type(type_metadata);
            }
        }
    }

    registry
}

/// Extract metadata from a single StructureDefinition
fn extract_type_metadata(structure_def: &StructureDefinition) -> Option<TypeMetadata> {
    let type_name = structure_def.name.as_str();
    let mut fields = HashMap::new();

    // Get the snapshot elements
    let snapshot = structure_def.snapshot.as_ref()?;
    let elements = &snapshot.element;

    // Skip the first element (it's the resource itself, e.g., "Patient")
    for element in elements.iter().skip(1) {
        if let Some(field_info) = extract_field_info(element, type_name) {
            if let Some(field_name) = extract_field_name(&element.path, type_name) {
                fields.insert(field_name, field_info);
            }
        }
    }

    Some(TypeMetadata {
        name: type_name.to_string(),
        fields,
    })
}

/// Extract field name from element path (e.g., "Patient.birthDate" -> "birthDate")
fn extract_field_name(path: &str, type_name: &str) -> Option<String> {
    let prefix = format!("{type_name}.");

    if !path.starts_with(&prefix) {
        return None;
    }

    let field_path = &path[prefix.len()..];

    // Only take the immediate child field (not nested paths like "name.given")
    // We'll handle those through recursive resolution
    let field_name = field_path.split('.').next()?;

    Some(field_name.to_string())
}

/// Extract field information from an ElementDefinition
fn extract_field_info(element: &ElementDefinition, _type_name: &str) -> Option<FieldInfo> {
    let element_types = element.element_type.as_ref()?;

    if element_types.is_empty() {
        return None;
    }

    // Get cardinality
    let min = element.min.unwrap_or(0);
    let max = element.max.as_ref().and_then(|m| {
        if m == "*" {
            None
        } else {
            m.parse::<u32>().ok()
        }
    });

    // Determine if this is a choice type (has [x] suffix in name)
    let is_choice_type = element.path.contains("[x]");

    // Collect all types (for choice types)
    let choice_types: Vec<String> = element_types
        .iter()
        .filter_map(|et| et.code.clone())
        .collect();

    // Use the first type as the primary field type
    let primary_type_code = element_types[0].code.as_ref()?;
    let field_type = determine_field_type(primary_type_code);

    Some(FieldInfo {
        field_type,
        min,
        max,
        is_choice_type,
        choice_types,
    })
}

/// Determine the FhirFieldType from a FHIR type code string
fn determine_field_type(type_code: &str) -> FhirFieldType {
    // Check if it's a primitive type
    if let Some(primitive) = FhirPrimitiveType::from_fhir_type(type_code) {
        return FhirFieldType::Primitive(primitive);
    }

    // Check for Reference
    if type_code == "Reference" {
        return FhirFieldType::Reference;
    }

    // Check for BackboneElement (typically internal structures)
    if type_code == "BackboneElement" {
        return FhirFieldType::BackboneElement(type_code.to_string());
    }

    // Otherwise it's a complex type
    FhirFieldType::Complex(type_code.to_string())
}

/// Generate the metadata.rs file content
pub fn generate_metadata_code(registry: &MetadataRegistry) -> String {
    let mut code = String::new();

    // File header and imports
    code.push_str(
        r#"//! FHIR type metadata
//!
//! This module provides compile-time metadata about FHIR types, enabling
//! path resolution like "Patient.name.given" -> FhirPrimitiveType::String.
//!
//! Generated automatically - do not edit manually.

use phf::{phf_map, Map};

"#,
    );

    // Generate FhirPrimitiveType enum
    code.push_str(
        r#"/// FHIR primitive types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

"#,
    );

    // Generate FhirFieldType enum
    code.push_str(
        r#"/// FHIR field type (primitive, complex, reference, or backbone element)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FhirFieldType {
    Primitive(FhirPrimitiveType),
    Complex(&'static str),
    Reference,
    BackboneElement(&'static str),
}

"#,
    );

    // Generate FieldInfo struct
    code.push_str(
        r#"/// Information about a field in a FHIR resource or datatype
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub field_type: FhirFieldType,
    pub min: u32,
    pub max: Option<u32>,
    pub is_choice_type: bool,
}

"#,
    );

    // Track generated const names to avoid duplicates
    let mut generated_consts = std::collections::HashSet::new();

    // Generate phf maps for each type (skip duplicates)
    for (type_name, type_metadata) in &registry.types {
        // Create sanitized const name
        let sanitized_name: String = type_name
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect();
        let const_name = format!("{}_FIELDS", sanitized_name.to_uppercase());

        // Skip if already generated
        if generated_consts.contains(&const_name) {
            continue;
        }

        generate_type_map(&mut code, type_name, type_metadata);
        generated_consts.insert(const_name);
    }

    // Generate the main registry (only include types that were generated)
    generate_registry_map(&mut code, &registry.types, &generated_consts);

    // Generate helper functions
    code.push_str(
        r#"
/// Get field information for a specific field in a type
pub fn get_field_info(type_name: &str, field_name: &str) -> Option<&'static FieldInfo> {
    FHIR_TYPE_REGISTRY
        .get(type_name)
        .and_then(|fields| fields.get(field_name))
}

/// Resolve a nested path like "Patient.name.given" to its field type
pub fn resolve_path(path: &str) -> Option<&'static FhirFieldType> {
    let parts: Vec<&str> = path.split('.').collect();
    if parts.is_empty() {
        return None;
    }

    let mut current_type_name = parts[0];
    
    for (idx, &field_name) in parts[1..].iter().enumerate() {
        let field_info = get_field_info(current_type_name, field_name)?;
        
        // If this is the last field, return its type
        if idx == parts.len() - 2 {
            return Some(&field_info.field_type);
        }
        
        // Otherwise, navigate to the next type
        match &field_info.field_type {
            FhirFieldType::Complex(type_name) | FhirFieldType::BackboneElement(type_name) => {
                current_type_name = type_name;
            }
            _ => return None, // Can't navigate further
        }
    }
    
    None
}
"#,
    );

    code
}

/// Generate a phf map for a single type's fields
fn generate_type_map(code: &mut String, type_name: &str, type_metadata: &TypeMetadata) {
    // Sanitize type name to create valid Rust identifier
    // Replace any non-alphanumeric characters with underscores
    let sanitized_name: String = type_name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    let const_name = format!("{}_FIELDS", sanitized_name.to_uppercase());

    code.push_str(&format!("/// Field metadata for {type_name}\n"));
    code.push_str(&format!(
        "pub static {const_name}: Map<&'static str, FieldInfo> = phf_map! {{\n"
    ));

    for (field_name, field_info) in &type_metadata.fields {
        code.push_str(&format!("    \"{field_name}\" => FieldInfo {{\n"));

        // Generate field_type
        code.push_str("        field_type: ");
        match &field_info.field_type {
            FhirFieldType::Primitive(prim) => {
                code.push_str(&format!(
                    "FhirFieldType::Primitive(FhirPrimitiveType::{})",
                    prim.variant_name()
                ));
            }
            FhirFieldType::Complex(name) => {
                code.push_str(&format!("FhirFieldType::Complex(\"{name}\")"));
            }
            FhirFieldType::Reference => {
                code.push_str("FhirFieldType::Reference");
            }
            FhirFieldType::BackboneElement(name) => {
                code.push_str(&format!("FhirFieldType::BackboneElement(\"{name}\")"));
            }
        }
        code.push_str(",\n");

        // Generate cardinality
        code.push_str(&format!("        min: {},\n", field_info.min));
        code.push_str("        max: ");
        if let Some(max) = field_info.max {
            code.push_str(&format!("Some({max})"));
        } else {
            code.push_str("None");
        }
        code.push_str(",\n");

        // Generate is_choice_type
        code.push_str(&format!(
            "        is_choice_type: {},\n",
            field_info.is_choice_type
        ));

        code.push_str("    },\n");
    }

    code.push_str("};\n\n");
}

/// Generate the main registry map
fn generate_registry_map(
    code: &mut String,
    types: &HashMap<String, TypeMetadata>,
    generated_consts: &std::collections::HashSet<String>,
) {
    code.push_str("/// Main FHIR type registry mapping type names to their field metadata\n");
    code.push_str(
        "pub static FHIR_TYPE_REGISTRY: Map<&'static str, &'static Map<&'static str, FieldInfo>> = phf_map! {\n",
    );

    for type_name in types.keys() {
        // Sanitize type name to create valid Rust identifier
        // Replace any non-alphanumeric characters with underscores
        let sanitized_name: String = type_name
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect();
        let const_name = format!("{}_FIELDS", sanitized_name.to_uppercase());

        // Only include if the const was actually generated
        if generated_consts.contains(&const_name) {
            code.push_str(&format!("    \"{type_name}\" => &{const_name},\n"));
        }
    }

    code.push_str("};\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_field_name() {
        assert_eq!(
            extract_field_name("Patient.birthDate", "Patient"),
            Some("birthDate".to_string())
        );

        assert_eq!(
            extract_field_name("Patient.name.given", "Patient"),
            Some("name".to_string())
        );

        assert_eq!(extract_field_name("Patient", "Patient"), None);
    }

    #[test]
    fn test_determine_field_type() {
        assert_eq!(
            determine_field_type("date"),
            FhirFieldType::Primitive(FhirPrimitiveType::Date)
        );

        assert_eq!(
            determine_field_type("string"),
            FhirFieldType::Primitive(FhirPrimitiveType::String)
        );

        assert_eq!(determine_field_type("Reference"), FhirFieldType::Reference);

        assert!(matches!(
            determine_field_type("HumanName"),
            FhirFieldType::Complex(_)
        ));
    }
}
