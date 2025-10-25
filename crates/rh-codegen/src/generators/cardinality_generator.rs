//! Cardinality constant generation
//!
//! This module generates constant declarations for FHIR element cardinalities that can be
//! embedded in generated resource and datatype structs.

use crate::fhir_types::StructureDefinition;

/// Generator for cardinality constants
pub struct CardinalityGenerator;

impl CardinalityGenerator {
    /// Generate a CARDINALITIES constant for a StructureDefinition
    ///
    /// Returns Rust code as a string containing the static declaration.
    ///
    /// Uses `once_cell::sync::Lazy` for runtime initialization.
    ///
    /// Example output:
    /// ```rust,ignore
    /// pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    ///     once_cell::sync::Lazy::new(|| vec![
    ///         rh_foundation::ElementCardinality::new("Patient.identifier", 0, None),
    ///         rh_foundation::ElementCardinality::new("Patient.active", 0, Some(1)),
    ///     ]);
    /// ```
    pub fn generate_cardinalities_constant(structure_def: &StructureDefinition) -> String {
        let cardinalities = Self::extract_cardinalities(structure_def);

        if cardinalities.is_empty() {
            return String::new();
        }

        let mut code = String::new();
        code.push_str("/// FHIR cardinality constraints for this resource/datatype\n");
        code.push_str("///\n");
        code.push_str(
            "/// These define the minimum and maximum occurrences allowed for each element.\n",
        );
        code.push_str("pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> = once_cell::sync::Lazy::new(|| vec![\n");

        for card in &cardinalities {
            let path = escape_rust_string(&card.path);
            let max_str = match card.max {
                None => "None".to_string(),
                Some(n) => format!("Some({n})"),
            };

            code.push_str(&format!(
                "    rh_foundation::ElementCardinality::new(\"{path}\", {}, {max_str}),\n",
                card.min
            ));
        }

        code.push_str("]);\n");
        code
    }

    /// Extract cardinality constraints from a StructureDefinition
    ///
    /// Returns a vector of ElementCardinality structs for all elements in the resource.
    fn extract_cardinalities(
        structure_def: &StructureDefinition,
    ) -> Vec<rh_foundation::validation::ElementCardinality> {
        let mut cardinalities = Vec::new();

        // Get the base type name (e.g., "Patient" from "Patient" or "http://hl7.org/fhir/StructureDefinition/Patient")
        let base_type = structure_def.name.as_str();

        // Process snapshot elements if available
        if let Some(snapshot) = &structure_def.snapshot {
            for element in &snapshot.element {
                // Skip the root element (e.g., just "Patient")
                if element.path == base_type {
                    continue;
                }

                // Only process direct child elements (not nested BackboneElement sub-elements for now)
                // We can enhance this later to handle nested elements
                if element.path.matches('.').count() > 1 {
                    // This is a nested element like "Patient.contact.name"
                    // For Phase 9, we'll handle these but need to be careful about the path
                    // For now, include them all
                }

                let min = element.min.unwrap_or(0) as usize;
                let max = if let Some(max_str) = &element.max {
                    if max_str == "*" {
                        None // Unbounded
                    } else {
                        max_str.parse::<usize>().ok()
                    }
                } else {
                    Some(1) // Default to 1 if not specified
                };

                cardinalities.push(rh_foundation::validation::ElementCardinality::new(
                    element.path.clone(),
                    min,
                    max,
                ));
            }
        }

        cardinalities
    }
}

/// Escape a string for use in a Rust string literal
fn escape_rust_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_rust_string() {
        assert_eq!(escape_rust_string("hello"), "hello");
        assert_eq!(escape_rust_string("hello \"world\""), "hello \\\"world\\\"");
        assert_eq!(escape_rust_string("path\\to\\file"), "path\\\\to\\\\file");
    }
}
