//! Name generation utilities
//!
//! This module handles the conversion of FHIR names to valid Rust identifiers,
//! including struct names, field names, filenames, and module names.

use crate::fhir_types::StructureDefinition;

/// Utility struct for generating valid Rust names from FHIR structures
pub struct NameGenerator;

impl NameGenerator {
    /// Generate a proper Rust struct name from StructureDefinition URL or ID
    pub fn generate_struct_name(structure_def: &StructureDefinition) -> String {
        let raw_name = if structure_def.name == "alternate" {
            // Special case for "alternate" name - use ID
            Self::to_valid_rust_identifier(&structure_def.id)
        } else if structure_def.name.is_empty() {
            // No name provided - use ID
            Self::to_valid_rust_identifier(&structure_def.id)
        } else if structure_def.name != structure_def.id && !structure_def.id.is_empty() {
            // Name and ID differ - prefer ID for uniqueness, especially for extensions
            // This handles cases like cqf-library where name="library" but id="cqf-library"
            Self::to_valid_rust_identifier(&structure_def.id)
        } else {
            // Use name when it matches ID or ID is empty
            Self::to_valid_rust_identifier(&structure_def.name)
        };

        // FHIR conventions is to have capitalized names for non-primitive types
        if structure_def.kind != "primitive-type" {
            Self::capitalize_first_letter(&raw_name)
        } else {
            raw_name
        }
    }

    /// Convert a FHIR name to a valid Rust identifier while preserving the original as much as possible
    pub fn to_valid_rust_identifier(name: &str) -> String {
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

    /// Fix common FHIR acronyms to maintain proper casing
    pub fn fix_acronyms(name: &str) -> String {
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

    /// Convert a FHIR resource type name to filename using snake_case
    pub fn to_filename(structure_def: &StructureDefinition) -> String {
        // Use the struct name generation and convert to snake_case for filename
        let struct_name = Self::generate_struct_name(structure_def);
        let snake_case_name = Self::to_snake_case(&struct_name);

        format!("{snake_case_name}.rs")
    }

    /// Convert a PascalCase type name to snake_case for module imports
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

    /// Convert a string to PascalCase
    pub fn to_pascal_case(s: &str) -> String {
        s.split('_')
            .map(|word| {
                let mut chars = word.chars();
                #[allow(non_snake_case)]
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect()
    }

    /// Convert a FHIR field name to a valid Rust field name
    pub fn to_rust_field_name(name: &str) -> String {
        // Handle FHIR choice types (fields ending with [x])
        let clean_name = if name.ends_with("[x]") {
            name.strip_suffix("[x]").unwrap_or(name)
        } else {
            name
        };

        // Convert to snake_case and handle Rust keywords
        let snake_case = clean_name
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
        match snake_case.as_str() {
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
            _ => snake_case,
        }
    }

    /// Capitalize the first letter of a string
    pub fn capitalize_first_letter(s: &str) -> String {
        if s.is_empty() {
            return s.to_string();
        }
        s[0..1].to_uppercase() + &s[1..]
    }

    /// Convert an enum name to a filename using snake_case
    pub fn enum_name_to_filename(enum_name: &str) -> String {
        let snake_case_name = Self::to_snake_case(enum_name);
        format!("{snake_case_name}.rs")
    }

    /// Convert an enum name to a module name using snake_case (matching filename)
    pub fn enum_name_to_module_name(enum_name: &str) -> String {
        Self::to_snake_case(enum_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_valid_rust_identifier_conversion() {
        // Test FHIR resource names that should preserve original case
        assert_eq!(
            NameGenerator::to_valid_rust_identifier("StructureDefinition"),
            "StructureDefinition"
        );
        assert_eq!(
            NameGenerator::to_valid_rust_identifier("Patient"),
            "Patient"
        );
        assert_eq!(
            NameGenerator::to_valid_rust_identifier("Observation"),
            "Observation"
        );
        assert_eq!(
            NameGenerator::to_valid_rust_identifier("CodeSystem"),
            "CodeSystem"
        );

        // Test names that need conversion due to special characters
        assert_eq!(
            NameGenerator::to_valid_rust_identifier("patient"),
            "patient"
        );

        // Test names with spaces
        assert_eq!(
            NameGenerator::to_valid_rust_identifier("Relative Date Criteria"),
            "RelativeDateCriteria"
        );
        assert_eq!(
            NameGenerator::to_valid_rust_identifier("Care Plan"),
            "CarePlan"
        );

        // Test names with dashes and underscores
        assert_eq!(
            NameGenerator::to_valid_rust_identifier("patient-name"),
            "PatientName"
        );
        assert_eq!(
            NameGenerator::to_valid_rust_identifier("patient_name"),
            "patient_name"
        );

        // Test mixed separators
        assert_eq!(
            NameGenerator::to_valid_rust_identifier("some-complex_name with.spaces"),
            "SomeComplexNameWithSpaces"
        );

        // Test empty and edge cases
        assert_eq!(NameGenerator::to_valid_rust_identifier(""), "_");
        assert_eq!(NameGenerator::to_valid_rust_identifier("   "), "_");
        assert_eq!(NameGenerator::to_valid_rust_identifier("a"), "a");
    }

    #[test]
    fn test_snake_case_conversion() {
        assert_eq!(NameGenerator::to_snake_case("Patient"), "patient");
        assert_eq!(
            NameGenerator::to_snake_case("StructureDefinition"),
            "structure_definition"
        );
        assert_eq!(NameGenerator::to_snake_case("HTTPRequest"), "http_request");
        assert_eq!(NameGenerator::to_snake_case("someField"), "some_field");
    }

    #[test]
    fn test_pascal_case_conversion() {
        assert_eq!(NameGenerator::to_pascal_case("patient_name"), "PatientName");
        assert_eq!(NameGenerator::to_pascal_case("some_field"), "SomeField");
        assert_eq!(NameGenerator::to_pascal_case("http_request"), "HttpRequest");
    }
}
