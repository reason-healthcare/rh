//! Centralized naming utilities for FHIR code generation
//!
//! This module provides a consistent, clean interface for converting FHIR names
//! to valid Rust identifiers, including struct names, field names, filenames,
//! and module names. All naming logic is centralized here to avoid duplication
//! and ensure consistency across the codebase.
//!
//! ## Key Features
//!
//! - **Struct Naming**: Converts FHIR StructureDefinition names to valid Rust struct names
//! - **Field Naming**: Converts FHIR field names to snake_case with keyword handling
//! - **File Naming**: Generates appropriate filenames for generated Rust code
//! - **Case Conversions**: Handles PascalCase, snake_case, and identifier validation
//! - **Keyword Handling**: Manages Rust keyword conflicts by appending underscores
//! - **FHIR Conventions**: Preserves FHIR naming conventions where appropriate
//!
//! ## Usage
//!
//! ```rust
//! use rh_codegen::naming::Naming;
//! use rh_codegen::fhir_types::StructureDefinition;
//!
//! // Create a sample structure definition
//! let structure_def = StructureDefinition {
//!     name: "Patient".to_string(),
//!     id: "Patient".to_string(),
//!     kind: "resource".to_string(),
//!     ..Default::default()
//! };
//!
//! // Generate struct name
//! let struct_name = Naming::struct_name(&structure_def);
//!
//! // Convert field name
//! let field_name = Naming::field_name("birthDate"); // -> "birth_date"
//!
//! // Generate filename
//! let filename = Naming::filename(&structure_def); // -> "patient.rs"
//!
//! // Case conversions
//! let snake_case = Naming::to_snake_case("PatientName"); // -> "patient_name"
//! let pascal_case = Naming::to_pascal_case("patient_name"); // -> "PatientName"
//! ```
//!
//! ## Migration from Legacy Naming
//!
//! This module replaces the previously scattered naming functions from:
//! - `GeneratorUtils::generate_struct_name` → `Naming::struct_name`
//! - `GeneratorUtils::to_rust_field_name` → `Naming::field_name`
//! - `GeneratorUtils::to_snake_case` → `Naming::to_snake_case`
//! - `GeneratorUtils::to_filename` → `Naming::filename`
//! - `NameGenerator::*` → `Naming::*`
//! - `FieldGenerator::to_rust_field_name` → `Naming::field_name`

use crate::fhir_types::StructureDefinition;

/// Central naming utility for converting FHIR names to Rust identifiers
pub struct Naming;

impl Naming {
    // =============================================================================
    // STRUCT NAMING
    // =============================================================================

    /// Generate a proper Rust struct name from StructureDefinition
    pub fn struct_name(structure_def: &StructureDefinition) -> String {
        let raw_name = if structure_def.name == "alternate" {
            // Special case for "alternate" name - use ID
            Self::to_rust_identifier(&structure_def.id)
        } else if structure_def.name.is_empty() {
            // No name provided - use ID
            Self::to_rust_identifier(&structure_def.id)
        } else if structure_def.name != structure_def.id && !structure_def.id.is_empty() {
            // Name and ID differ - prefer ID for uniqueness, especially for extensions
            // This handles cases like cqf-library where name="library" but id="cqf-library"
            Self::to_rust_identifier(&structure_def.id)
        } else {
            // Use name when it matches ID or ID is empty
            Self::to_rust_identifier(&structure_def.name)
        };

        // FHIR convention is to have capitalized names for non-primitive types
        if structure_def.kind != "primitive-type" {
            Self::capitalize_first(&raw_name)
        } else {
            raw_name
        }
    }

    // =============================================================================
    // FIELD NAMING
    // =============================================================================

    /// Convert a FHIR field name to a valid Rust field name
    pub fn field_name(name: &str) -> String {
        // Handle FHIR choice types (fields ending with [x])
        let clean_name = if name.ends_with("[x]") {
            name.strip_suffix("[x]").unwrap_or(name)
        } else {
            name
        };

        // Handle field name conflicts with inherited base field
        let conflict_resolved_name = if clean_name == "base" {
            // Rename FHIR 'base' elements to avoid conflict with the inherited base field
            "base_definition"
        } else {
            clean_name
        };

        // Convert to snake_case and handle Rust keywords
        let snake_case = conflict_resolved_name
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<String>();

        // Handle Rust keywords by appending underscore
        Self::handle_rust_keywords(&snake_case)
    }

    /// Convert FHIR type code to snake_case for field suffix in choice types
    pub fn type_suffix(type_code: &str) -> String {
        type_code
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect()
    }

    // =============================================================================
    // FILE NAMING
    // =============================================================================

    /// Convert a StructureDefinition to a filename using snake_case
    pub fn filename(structure_def: &StructureDefinition) -> String {
        let struct_name = Self::struct_name(structure_def);
        let snake_case_name = Self::to_snake_case(&struct_name);
        format!("{snake_case_name}.rs")
    }

    /// Convert an enum name to a filename using snake_case
    pub fn enum_filename(enum_name: &str) -> String {
        let snake_case_name = Self::to_snake_case(enum_name);
        format!("{snake_case_name}.rs")
    }

    // =============================================================================
    // MODULE NAMING
    // =============================================================================

    /// Convert an enum name to a module name using snake_case
    pub fn module_name(enum_name: &str) -> String {
        Self::to_snake_case(enum_name)
    }

    /// Convert a trait name to a module name using snake_case
    pub fn trait_module_name(name: &str) -> String {
        // First handle spaces, dashes, dots, and other separators
        let cleaned = name
            .replace([' ', '-', '.'], "_")
            .replace(['(', ')', '[', ']'], "")
            .replace(['/', '\\', ':'], "_");

        // Then apply snake_case conversion for CamelCase
        Self::to_snake_case(&cleaned)
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect()
    }

    // =============================================================================
    // CASE CONVERSIONS
    // =============================================================================

    /// Convert a PascalCase type name to snake_case
    pub fn to_snake_case(name: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = name.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            if ch.is_uppercase() && i > 0 {
                // Check if this is part of an acronym or start of a new word
                let is_acronym_continuation = i > 0 && chars[i - 1].is_uppercase();
                let is_followed_by_lowercase = i + 1 < chars.len() && chars[i + 1].is_lowercase();

                // Add underscore if:
                // 1. Previous char was lowercase (start of new word like "someWord")
                // 2. This is an acronym followed by lowercase (like "HTTPRequest" -> "http_request")
                if (i > 0 && chars[i - 1].is_lowercase())
                    || (is_acronym_continuation && is_followed_by_lowercase)
                {
                    result.push('_');
                }
            }

            result.push(ch.to_lowercase().next().unwrap());
        }

        result
    }

    /// Convert a snake_case string to PascalCase
    pub fn to_pascal_case(s: &str) -> String {
        s.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect()
    }

    /// Capitalize the first letter of a string
    pub fn capitalize_first(s: &str) -> String {
        if s.is_empty() {
            return s.to_string();
        }
        s[0..1].to_uppercase() + &s[1..]
    }

    // =============================================================================
    // RUST IDENTIFIER VALIDATION AND CONVERSION
    // =============================================================================

    /// Convert a FHIR name to a valid Rust identifier while preserving the original as much as possible
    pub fn to_rust_identifier(name: &str) -> String {
        // For names that are already valid Rust identifiers, use them as-is
        if Self::is_valid_rust_identifier(name) {
            return name.to_string();
        }

        // For names with spaces, dashes, or other characters, convert to PascalCase
        let mut result = String::new();
        let mut capitalize_next = true;

        for ch in name.chars() {
            if ch.is_alphanumeric() {
                if capitalize_next {
                    result.push(ch.to_uppercase().next().unwrap());
                    capitalize_next = false;
                } else {
                    result.push(ch);
                }
            } else {
                // Skip non-alphanumeric characters and capitalize the next letter
                capitalize_next = true;
            }
        }

        // Ensure it starts with a letter or underscore (Rust requirement)
        if result.is_empty() || result.chars().next().unwrap().is_numeric() {
            result = format!("_{result}");
        }

        // Handle common FHIR acronyms that should remain uppercase
        Self::fix_acronyms(&result)
    }

    /// Check if a string is a valid Rust identifier
    pub fn is_valid_rust_identifier(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        let mut chars = name.chars();
        let first_char = chars.next().unwrap();

        // First character must be a letter or underscore
        if !first_char.is_alphabetic() && first_char != '_' {
            return false;
        }

        // Remaining characters must be alphanumeric or underscore
        for ch in chars {
            if !ch.is_alphanumeric() && ch != '_' {
                return false;
            }
        }

        // Check if it's a Rust keyword
        !Self::is_rust_keyword(name)
    }

    /// Check if a string is a Rust keyword
    pub fn is_rust_keyword(name: &str) -> bool {
        matches!(
            name,
            "as" | "break"
                | "const"
                | "continue"
                | "crate"
                | "else"
                | "enum"
                | "extern"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "impl"
                | "in"
                | "let"
                | "loop"
                | "match"
                | "mod"
                | "move"
                | "mut"
                | "pub"
                | "ref"
                | "return"
                | "self"
                | "Self"
                | "static"
                | "struct"
                | "super"
                | "trait"
                | "true"
                | "type"
                | "unsafe"
                | "use"
                | "where"
                | "while"
                | "async"
                | "await"
                | "dyn"
                | "abstract"
                | "become"
                | "box"
                | "do"
                | "final"
                | "macro"
                | "override"
                | "priv"
                | "typeof"
                | "unsized"
                | "virtual"
                | "yield"
                | "try"
        )
    }

    // =============================================================================
    // HELPER FUNCTIONS
    // =============================================================================

    /// Handle Rust keywords by appending underscore
    fn handle_rust_keywords(name: &str) -> String {
        match name {
            "type" => "type_".to_string(),
            "match" => "match_".to_string(),
            "loop" => "loop_".to_string(),
            "move" => "move_".to_string(),
            "ref" => "ref_".to_string(),
            "mod" => "mod_".to_string(),
            "use" => "use_".to_string(),
            "self" => "self_".to_string(),
            "super" => "super_".to_string(),
            "crate" => "crate_".to_string(),
            "async" => "async_".to_string(),
            "await" => "await_".to_string(),
            "fn" => "fn_".to_string(),
            "let" => "let_".to_string(),
            "const" => "const_".to_string(),
            "static" => "static_".to_string(),
            "struct" => "struct_".to_string(),
            "enum" => "enum_".to_string(),
            "impl" => "impl_".to_string(),
            "trait" => "trait_".to_string(),
            "for" => "for_".to_string(),
            "if" => "if_".to_string(),
            "else" => "else_".to_string(),
            "while" => "while_".to_string(),
            "return" => "return_".to_string(),
            "where" => "where_".to_string(),
            "abstract" => "abstract_".to_string(),
            _ => name.to_string(),
        }
    }

    /// Fix common FHIR acronyms to maintain proper casing
    fn fix_acronyms(name: &str) -> String {
        let mut result = name.to_string();

        // Common FHIR acronyms that should be uppercase
        let acronyms = [
            ("Cqf", "CQF"),     // Clinical Quality Framework
            ("Fhir", "FHIR"),   // Fast Healthcare Interoperability Resources
            ("Hl7", "HL7"),     // Health Level 7
            ("Http", "HTTP"),   // HyperText Transfer Protocol
            ("Https", "HTTPS"), // HTTP Secure
            ("Json", "JSON"),   // JavaScript Object Notation
            ("Xml", "XML"),     // eXtensible Markup Language
            ("Uuid", "UUID"),   // Universally Unique Identifier
            ("Uri", "URI"),     // Uniform Resource Identifier
            ("Url", "URL"),     // Uniform Resource Locator
            ("Api", "API"),     // Application Programming Interface
        ];

        for (from, to) in &acronyms {
            result = result.replace(from, to);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_name_generation() {
        let structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Patient".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            name: "Patient".to_string(),
            title: Some("Patient".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("A patient resource".to_string()),
            purpose: None,
            base_type: "DomainResource".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
            ),
            version: None,
            differential: None,
            snapshot: None,
        };

        assert_eq!(Naming::struct_name(&structure), "Patient");
    }

    #[test]
    fn test_field_name_conversion() {
        // Test basic field names
        assert_eq!(Naming::field_name("active"), "active");
        assert_eq!(Naming::field_name("name"), "name");

        // Test PascalCase to snake_case conversion
        assert_eq!(Naming::field_name("birthDate"), "birth_date");
        assert_eq!(
            Naming::field_name("multipleBirthBoolean"),
            "multiple_birth_boolean"
        );

        // Test choice types with [x] suffix
        assert_eq!(Naming::field_name("value[x]"), "value");
        assert_eq!(Naming::field_name("deceased[x]"), "deceased");

        // Test Rust keywords
        assert_eq!(Naming::field_name("type"), "type_");
        assert_eq!(Naming::field_name("use"), "use_");
        assert_eq!(Naming::field_name("ref"), "ref_");
        assert_eq!(Naming::field_name("for"), "for_");
        assert_eq!(Naming::field_name("match"), "match_");

        // Test base conflict resolution
        assert_eq!(Naming::field_name("base"), "base_definition");
    }

    #[test]
    fn test_filename_generation() {
        let patient_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Patient".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            name: "Patient".to_string(),
            title: Some("Patient".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("A patient resource".to_string()),
            purpose: None,
            base_type: "DomainResource".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
            ),
            version: None,
            differential: None,
            snapshot: None,
        };

        assert_eq!(Naming::filename(&patient_structure), "patient.rs");

        let structure_definition = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "StructureDefinition".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/StructureDefinition".to_string(),
            name: "StructureDefinition".to_string(),
            title: Some("StructureDefinition".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("A structure definition resource".to_string()),
            purpose: None,
            base_type: "MetadataResource".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/MetadataResource".to_string(),
            ),
            version: None,
            differential: None,
            snapshot: None,
        };

        assert_eq!(
            Naming::filename(&structure_definition),
            "structure_definition.rs"
        );
    }

    #[test]
    fn test_snake_case_conversion() {
        assert_eq!(Naming::to_snake_case("Patient"), "patient");
        assert_eq!(
            Naming::to_snake_case("StructureDefinition"),
            "structure_definition"
        );
        assert_eq!(Naming::to_snake_case("HTTPRequest"), "http_request");
        assert_eq!(Naming::to_snake_case("someField"), "some_field");
    }

    #[test]
    fn test_pascal_case_conversion() {
        assert_eq!(Naming::to_pascal_case("patient_name"), "PatientName");
        assert_eq!(Naming::to_pascal_case("some_field"), "SomeField");
        assert_eq!(Naming::to_pascal_case("http_request"), "HttpRequest");
    }

    #[test]
    fn test_rust_identifier_conversion() {
        // Test FHIR resource names that should preserve original case
        assert_eq!(
            Naming::to_rust_identifier("StructureDefinition"),
            "StructureDefinition"
        );
        assert_eq!(Naming::to_rust_identifier("Patient"), "Patient");
        assert_eq!(Naming::to_rust_identifier("Observation"), "Observation");
        assert_eq!(Naming::to_rust_identifier("CodeSystem"), "CodeSystem");

        // Test names with spaces
        assert_eq!(
            Naming::to_rust_identifier("Relative Date Criteria"),
            "RelativeDateCriteria"
        );
        assert_eq!(Naming::to_rust_identifier("Care Plan"), "CarePlan");

        // Test names with dashes and underscores
        assert_eq!(Naming::to_rust_identifier("patient-name"), "PatientName");
        assert_eq!(Naming::to_rust_identifier("patient_name"), "patient_name");

        // Test mixed separators
        assert_eq!(
            Naming::to_rust_identifier("some-complex_name with.spaces"),
            "SomeComplexNameWithSpaces"
        );

        // Test empty and edge cases
        assert_eq!(Naming::to_rust_identifier(""), "_");
        assert_eq!(Naming::to_rust_identifier("   "), "_");
        assert_eq!(Naming::to_rust_identifier("a"), "a");
    }

    #[test]
    fn test_type_suffix() {
        assert_eq!(Naming::type_suffix("string"), "string");
        assert_eq!(Naming::type_suffix("DateTime"), "date_time");
        assert_eq!(Naming::type_suffix("CodeableConcept"), "codeable_concept");
    }

    #[test]
    fn test_enum_filename() {
        assert_eq!(Naming::enum_filename("PatientStatus"), "patient_status.rs");
        assert_eq!(Naming::enum_filename("HTTPMethod"), "http_method.rs");
    }

    #[test]
    fn test_trait_module_name() {
        assert_eq!(Naming::trait_module_name("Patient"), "patient");
        assert_eq!(
            Naming::trait_module_name("Relative Date Criteria"),
            "relative_date_criteria"
        );
        assert_eq!(Naming::trait_module_name("patient-name"), "patient_name");
    }
}
