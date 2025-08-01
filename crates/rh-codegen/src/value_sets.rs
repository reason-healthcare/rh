//! ValueSet management and code generation utilities
//!
//! This module handles FHIR ValueSets, including generation of Rust enums
//! from ValueSet codes and management of code system mappings.

use crate::fhir_types::{CodeSystem, ValueSet, ValueSetComposeConcept, ValueSetExpansionContains};
use crate::rust_types::{RustEnum, RustEnumVariant};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Manages FHIR ValueSets and their conversion to Rust enums
#[derive(Debug, Clone)]
pub struct ValueSetManager {
    /// Directory containing ValueSet JSON files
    value_set_dir: Option<PathBuf>,
    /// Cache of ValueSet URLs to generated enum names
    value_set_cache: HashMap<String, String>,
    /// Cache of generated enums by name
    enum_cache: HashMap<String, RustEnum>,
}

impl ValueSetManager {
    pub fn new() -> Self {
        Self {
            value_set_dir: None,
            value_set_cache: HashMap::new(),
            enum_cache: HashMap::new(),
        }
    }

    pub fn new_with_directory<P: AsRef<Path>>(value_set_dir: P) -> Self {
        Self {
            value_set_dir: Some(value_set_dir.as_ref().to_path_buf()),
            value_set_cache: HashMap::new(),
            enum_cache: HashMap::new(),
        }
    }

    /// Generate a Rust enum name from a ValueSet URL
    pub fn generate_enum_name(&self, value_set_url: &str) -> String {
        // Extract the last part of the URL and convert to PascalCase
        let name = value_set_url
            .split('/')
            .next_back()
            .unwrap_or("UnknownValueSet")
            .split(&['-', '.'][..])
            .filter(|part| !part.is_empty())
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>();

        // Ensure it's a valid Rust identifier
        if name.chars().next().unwrap_or('0').is_ascii_digit() {
            format!("ValueSet{name}")
        } else {
            name
        }
    }

    /// Check if a ValueSet is already cached
    pub fn is_cached(&self, value_set_url: &str) -> bool {
        self.value_set_cache.contains_key(value_set_url)
    }

    /// Get the enum name for a cached ValueSet
    pub fn get_enum_name(&self, value_set_url: &str) -> Option<&String> {
        self.value_set_cache.get(value_set_url)
    }

    /// Cache a ValueSet with its generated enum
    pub fn cache_value_set(
        &mut self,
        value_set_url: String,
        enum_name: String,
        rust_enum: RustEnum,
    ) {
        self.value_set_cache
            .insert(value_set_url, enum_name.clone());
        self.enum_cache.insert(enum_name, rust_enum);
    }

    /// Get all cached enums
    pub fn get_cached_enums(&self) -> &HashMap<String, RustEnum> {
        &self.enum_cache
    }

    /// Generate enum from ValueSet, trying expansion first, then compose
    pub fn generate_enum_from_value_set(
        &mut self,
        value_set_url: &str,
        version: Option<&str>,
    ) -> Result<String, String> {
        let enum_name = self.generate_enum_name(value_set_url);

        if self.is_cached(value_set_url) {
            return Ok(enum_name);
        }

        // Try to find and load the ValueSet file
        let value_set = match self.load_value_set(value_set_url, version) {
            Ok(vs) => vs,
            Err(err) => {
                eprintln!("Warning: Could not load ValueSet '{value_set_url}': {err}");
                return Err(format!("ValueSet not found: {value_set_url}"));
            }
        };

        // Try to generate enum from expansion first
        if let Some(expansion) = &value_set.expansion {
            if let Some(contains) = &expansion.contains {
                if !contains.is_empty() {
                    let rust_enum =
                        self.create_enum_from_expansion(&enum_name, contains, value_set_url);
                    self.cache_value_set(value_set_url.to_string(), enum_name.clone(), rust_enum);
                    return Ok(enum_name);
                }
            }
        }

        // Fallback to compose if no expansion or expansion is empty
        if let Some(compose) = &value_set.compose {
            if let Some(includes) = &compose.include {
                // Check if there are any filters - if so, we can't generate enum
                for include in includes {
                    if include.filter.is_some() && !include.filter.as_ref().unwrap().is_empty() {
                        eprintln!("Warning: ValueSet '{value_set_url}' has filters, cannot generate enum. Falling back to String.");
                        return Err("ValueSet has filters".to_string());
                    }
                }

                // Try to generate from explicit concepts
                let mut all_concepts = Vec::new();
                for include in includes {
                    if let Some(concepts) = &include.concept {
                        all_concepts.extend(concepts.iter().cloned());
                    } else if let Some(system) = &include.system {
                        // Try to load the entire code system
                        if let Ok(code_system) = self.load_code_system(system) {
                            if let Some(cs_concepts) = &code_system.concept {
                                for cs_concept in cs_concepts {
                                    let compose_concept = ValueSetComposeConcept {
                                        code: cs_concept.code.clone(),
                                        display: cs_concept.display.clone(),
                                    };
                                    all_concepts.push(compose_concept);
                                }
                            }
                        }
                    }
                }

                if !all_concepts.is_empty() {
                    let rust_enum =
                        self.create_enum_from_concepts(&enum_name, &all_concepts, value_set_url);
                    self.cache_value_set(value_set_url.to_string(), enum_name.clone(), rust_enum);
                    return Ok(enum_name);
                }
            }
        }

        // If all methods fail, return error
        eprintln!("Warning: Could not generate enum for ValueSet '{value_set_url}', no expansion or compose concepts found. Falling back to String.");
        Err("No concepts found in ValueSet".to_string())
    }

    /// Load a ValueSet from file
    fn load_value_set(
        &self,
        value_set_url: &str,
        _version: Option<&str>,
    ) -> Result<ValueSet, String> {
        let value_set_dir = self
            .value_set_dir
            .as_ref()
            .ok_or("No ValueSet directory configured")?;

        // Extract the ID from the URL (last part after '/')
        let id = value_set_url
            .split('/')
            .next_back()
            .ok_or("Invalid ValueSet URL")?;

        // Try common filename patterns
        let filenames = vec![
            format!("ValueSet-{}.json", id),
            format!("valueset-{}.json", id),
            format!("{}.json", id),
        ];

        for filename in filenames {
            let file_path = value_set_dir.join(&filename);
            if file_path.exists() {
                let content = fs::read_to_string(&file_path)
                    .map_err(|e| format!("Failed to read file '{}': {}", file_path.display(), e))?;

                let value_set: ValueSet = serde_json::from_str(&content)
                    .map_err(|e| format!("Failed to parse ValueSet JSON: {e}"))?;
                return Ok(value_set);
            }
        }

        Err(format!("ValueSet file not found for ID: {id}"))
    }

    /// Load a CodeSystem from file
    fn load_code_system(&self, system_url: &str) -> Result<CodeSystem, String> {
        let value_set_dir = self
            .value_set_dir
            .as_ref()
            .ok_or("No ValueSet directory configured")?;

        // Extract the ID from the URL (last part after '/')
        let id = system_url
            .split('/')
            .next_back()
            .ok_or("Invalid CodeSystem URL")?;

        // Try common filename patterns
        let filenames = vec![
            format!("CodeSystem-{}.json", id),
            format!("codesystem-{}.json", id),
            format!("{}.json", id),
        ];

        for filename in filenames {
            let file_path = value_set_dir.join(&filename);
            if file_path.exists() {
                let content = fs::read_to_string(&file_path)
                    .map_err(|e| format!("Failed to read file '{}': {}", file_path.display(), e))?;

                let code_system: CodeSystem = serde_json::from_str(&content)
                    .map_err(|e| format!("Failed to parse CodeSystem JSON: {e}"))?;
                return Ok(code_system);
            }
        }

        Err(format!("CodeSystem file not found for ID: {id}"))
    }

    /// Create enum from ValueSet expansion
    fn create_enum_from_expansion(
        &self,
        enum_name: &str,
        contains: &[ValueSetExpansionContains],
        value_set_url: &str,
    ) -> RustEnum {
        let mut rust_enum = RustEnum::new(enum_name.to_string());
        rust_enum.doc_comment = Some(format!("Generated enum for ValueSet: {value_set_url}"));

        for concept in contains {
            let variant_name = ValueSetConcept::new(concept.code.clone()).to_variant_name();
            let mut variant = RustEnumVariant::new(variant_name);

            if let Some(display) = &concept.display {
                variant.doc_comment = Some(display.clone());
            }

            // Add serde annotation to map to the original code
            variant.serde_rename = Some(concept.code.clone());

            rust_enum.add_variant(variant);
        }

        rust_enum
    }

    /// Create enum from ValueSet compose concepts
    fn create_enum_from_concepts(
        &self,
        enum_name: &str,
        concepts: &[ValueSetComposeConcept],
        value_set_url: &str,
    ) -> RustEnum {
        let mut rust_enum = RustEnum::new(enum_name.to_string());
        rust_enum.doc_comment = Some(format!("Generated enum for ValueSet: {value_set_url}"));

        for concept in concepts {
            let variant_name = ValueSetConcept::new(concept.code.clone()).to_variant_name();
            let mut variant = RustEnumVariant::new(variant_name);

            if let Some(display) = &concept.display {
                variant.doc_comment = Some(display.clone());
            }

            // Add serde annotation to map to the original code
            variant.serde_rename = Some(concept.code.clone());

            rust_enum.add_variant(variant);
        }

        rust_enum
    }

    /// Generate a basic enum for unknown ValueSets (fallback)
    pub fn generate_placeholder_enum(&mut self, value_set_url: &str) -> String {
        let enum_name = self.generate_enum_name(value_set_url);

        if !self.is_cached(value_set_url) {
            let mut rust_enum = RustEnum::new(enum_name.clone());
            rust_enum.doc_comment = Some(format!("Generated enum for ValueSet: {value_set_url}"));

            // Add a placeholder variant
            rust_enum.add_variant(RustEnumVariant::new("Unknown".to_string()));

            self.cache_value_set(value_set_url.to_string(), enum_name.clone(), rust_enum);
        }

        enum_name
    }
}

impl Default for ValueSetManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a FHIR ValueSet concept
#[derive(Debug, Clone)]
pub struct ValueSetConcept {
    pub code: String,
    pub display: Option<String>,
    pub definition: Option<String>,
    pub system: Option<String>,
}

impl ValueSetConcept {
    pub fn new(code: String) -> Self {
        Self {
            code,
            display: None,
            definition: None,
            system: None,
        }
    }

    /// Convert the concept code to a valid Rust enum variant name
    pub fn to_variant_name(&self) -> String {
        // Handle special cases first
        let sanitized_code = match self.code.as_str() {
            "=" => "Equal".to_string(),
            "!=" => "NotEqual".to_string(),
            "<" => "LessThan".to_string(),
            "<=" => "LessThanOrEqual".to_string(),
            ">" => "GreaterThan".to_string(),
            ">=" => "GreaterThanOrEqual".to_string(),
            "+" => "Plus".to_string(),
            "-" => "Minus".to_string(),
            "*" => "Star".to_string(),
            "/" => "Slash".to_string(),
            "&" => "Ampersand".to_string(),
            "|" => "Pipe".to_string(),
            "%" => "Percent".to_string(),
            "#" => "Hash".to_string(),
            "@" => "At".to_string(),
            "!" => "Exclamation".to_string(),
            "?" => "Question".to_string(),
            "^" => "Caret".to_string(),
            "~" => "Tilde".to_string(),
            "(" => "LeftParen".to_string(),
            ")" => "RightParen".to_string(),
            "[" => "LeftBracket".to_string(),
            "]" => "RightBracket".to_string(),
            "{" => "LeftBrace".to_string(),
            "}" => "RightBrace".to_string(),
            "'" => "SingleQuote".to_string(),
            "\"" => "DoubleQuote".to_string(),
            "`" => "Backtick".to_string(),
            "$" => "Dollar".to_string(),
            ";" => "Semicolon".to_string(),
            ":" => "Colon".to_string(),
            "," => "Comma".to_string(),
            _ => {
                // For other codes, sanitize by removing/replacing invalid characters
                self.code
                    .chars()
                    .map(|c| match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' => c.to_string(),
                        '-' | '_' | '.' | ' ' => "-".to_string(), // Convert to dash for splitting
                        _ => format!("_{:02x}", c as u32),        // Convert other characters to hex
                    })
                    .collect::<String>()
            }
        };

        // Convert kebab-case, snake_case, or other formats to PascalCase
        let name = sanitized_code
            .split(&['-', '_', '.', ' '][..])
            .filter(|part| !part.is_empty()) // Filter out empty parts
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>();

        // Ensure it starts with a letter and is not empty
        if name.is_empty() {
            "Unknown".to_string()
        } else if name.chars().next().unwrap_or('0').is_ascii_digit() {
            format!("Code{name}")
        } else {
            name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_enum_name() {
        let manager = ValueSetManager::new();

        assert_eq!(
            manager.generate_enum_name("http://hl7.org/fhir/ValueSet/administrative-gender"),
            "AdministrativeGender"
        );

        assert_eq!(
            manager.generate_enum_name("http://hl7.org/fhir/ValueSet/123-test"),
            "ValueSet123Test"
        );
    }

    #[test]
    fn test_concept_variant_name() {
        let concept = ValueSetConcept::new("male".to_string());
        assert_eq!(concept.to_variant_name(), "Male");

        let concept = ValueSetConcept::new("unknown-gender".to_string());
        assert_eq!(concept.to_variant_name(), "UnknownGender");

        let concept = ValueSetConcept::new("123-code".to_string());
        assert_eq!(concept.to_variant_name(), "Code123Code");
    }
}
