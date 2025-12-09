//! Type conversion support for CQL expressions.
//!
//! This module provides infrastructure for tracking and applying implicit type
//! conversions based on ModelInfo conversion definitions. When the translator
//! encounters a type mismatch (e.g., FHIR.CodeableConcept where System.Concept
//! is expected), it looks up the appropriate converter function and wraps the
//! expression in a FunctionRef.
//!
//! # ModelInfo Conversions
//!
//! ModelInfo (e.g., from fhir.cqf.common) defines conversions like:
//! ```text
//! fromType="FHIR.Coding" toType="System.Code" functionName="FHIRHelpers.ToCode"
//! fromType="FHIR.CodeableConcept" toType="System.Concept" functionName="FHIRHelpers.ToConcept"
//! ```
//!
//! # Usage
//!
//! ```rust,ignore
//! use rh_cql::conversion::ConversionRegistry;
//! use rh_cql::modelinfo::ModelInfo;
//!
//! let model_info = load_modelinfo_from_package(...)?;
//! let registry = ConversionRegistry::from_model_info(&model_info);
//!
//! // Look up a conversion
//! if let Some(conv) = registry.find_conversion("FHIR.Coding", "System.Code") {
//!     println!("Use function: {:?}", conv.function_name);
//! }
//! ```

use crate::datatype::DataType;
use crate::modelinfo::{ConversionInfo, ModelInfo};
use std::collections::HashMap;

/// A registry of type conversions loaded from ModelInfo.
///
/// The registry provides fast lookup of conversions by source and target type.
#[derive(Debug, Clone, Default)]
pub struct ConversionRegistry {
    /// Conversions indexed by (from_type, to_type).
    conversions: HashMap<(String, String), ConversionEntry>,
    /// Conversions indexed by from_type for finding any conversion from a type.
    by_source: HashMap<String, Vec<ConversionEntry>>,
    /// Conversions indexed by to_type for finding any conversion to a type.
    by_target: HashMap<String, Vec<ConversionEntry>>,
}

/// A single conversion entry.
#[derive(Debug, Clone)]
pub struct ConversionEntry {
    /// The source type (e.g., "FHIR.Coding").
    pub from_type: String,
    /// The target type (e.g., "System.Code").
    pub to_type: String,
    /// The function that performs the conversion (e.g., "FHIRHelpers.ToCode").
    pub function_name: String,
    /// The library name parsed from function_name (e.g., "FHIRHelpers").
    pub library_name: Option<String>,
    /// The function name without library prefix (e.g., "ToCode").
    pub function_simple_name: String,
}

impl ConversionEntry {
    /// Create a conversion entry from ModelInfo ConversionInfo.
    pub fn from_conversion_info(info: &ConversionInfo) -> Option<Self> {
        let from_type = info.from_type.clone()?;
        let to_type = info.to_type.clone()?;
        let function_name = info.function_name.clone()?;

        // Parse "FHIRHelpers.ToCode" into library_name and function_simple_name
        let (library_name, function_simple_name) = if let Some(dot_pos) = function_name.find('.') {
            (
                Some(function_name[..dot_pos].to_string()),
                function_name[dot_pos + 1..].to_string(),
            )
        } else {
            (None, function_name.clone())
        };

        Some(Self {
            from_type,
            to_type,
            function_name,
            library_name,
            function_simple_name,
        })
    }
}

impl ConversionRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a registry from ModelInfo conversion definitions.
    pub fn from_model_info(model_info: &ModelInfo) -> Self {
        let mut registry = Self::new();

        for conv_info in &model_info.conversion_info {
            if let Some(entry) = ConversionEntry::from_conversion_info(conv_info) {
                registry.register(entry);
            }
        }

        registry
    }

    /// Register a conversion.
    pub fn register(&mut self, entry: ConversionEntry) {
        let key = (entry.from_type.clone(), entry.to_type.clone());

        // Index by source type
        self.by_source
            .entry(entry.from_type.clone())
            .or_default()
            .push(entry.clone());

        // Index by target type
        self.by_target
            .entry(entry.to_type.clone())
            .or_default()
            .push(entry.clone());

        // Primary index
        self.conversions.insert(key, entry);
    }

    /// Find a conversion from one type to another.
    pub fn find_conversion(&self, from_type: &str, to_type: &str) -> Option<&ConversionEntry> {
        let key = (from_type.to_string(), to_type.to_string());
        self.conversions.get(&key)
    }

    /// Find all conversions from a source type.
    pub fn find_conversions_from(&self, from_type: &str) -> &[ConversionEntry] {
        self.by_source
            .get(from_type)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Find all conversions to a target type.
    pub fn find_conversions_to(&self, to_type: &str) -> &[ConversionEntry] {
        self.by_target
            .get(to_type)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Check if a conversion exists.
    pub fn has_conversion(&self, from_type: &str, to_type: &str) -> bool {
        let key = (from_type.to_string(), to_type.to_string());
        self.conversions.contains_key(&key)
    }

    /// Get the number of registered conversions.
    pub fn len(&self) -> usize {
        self.conversions.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.conversions.is_empty()
    }

    /// Get all unique source types.
    pub fn source_types(&self) -> impl Iterator<Item = &str> {
        self.by_source.keys().map(|s| s.as_str())
    }

    /// Get all unique target types.
    pub fn target_types(&self) -> impl Iterator<Item = &str> {
        self.by_target.keys().map(|s| s.as_str())
    }
}

// =============================================================================
// Type Conversion Context
// =============================================================================

/// Context for tracking type conversions during translation.
///
/// This tracks what conversions are available and whether the required
/// helper libraries (like FHIRHelpers) are included.
#[derive(Debug, Clone, Default)]
pub struct ConversionContext {
    /// The conversion registry with available conversions.
    registry: ConversionRegistry,
    /// Libraries that have been included (for checking FHIRHelpers availability).
    included_libraries: HashMap<String, String>,
}

impl ConversionContext {
    /// Create a new conversion context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a context with a conversion registry.
    pub fn with_registry(registry: ConversionRegistry) -> Self {
        Self {
            registry,
            included_libraries: HashMap::new(),
        }
    }

    /// Register an included library.
    ///
    /// The alias is the local name used to reference the library.
    pub fn add_included_library(&mut self, alias: &str, library_name: &str) {
        self.included_libraries
            .insert(alias.to_string(), library_name.to_string());
    }

    /// Check if a library is included.
    pub fn is_library_included(&self, library_name: &str) -> bool {
        self.included_libraries
            .values()
            .any(|v| v == library_name || v.starts_with(&format!("{library_name}.")))
    }

    /// Get the alias for an included library (if any).
    pub fn get_library_alias(&self, library_name: &str) -> Option<&str> {
        self.included_libraries
            .iter()
            .find(|(_, v)| *v == library_name || v.starts_with(&format!("{library_name}.")))
            .map(|(k, _)| k.as_str())
    }

    /// Try to find a conversion and check if it's usable.
    ///
    /// Returns the conversion entry and whether the required library is included.
    pub fn check_conversion(
        &self,
        from_type: &str,
        to_type: &str,
    ) -> Option<(&ConversionEntry, bool)> {
        let entry = self.registry.find_conversion(from_type, to_type)?;
        let library_available = entry
            .library_name
            .as_ref()
            .map(|lib| self.is_library_included(lib))
            .unwrap_or(true);
        Some((entry, library_available))
    }

    /// Find a conversion that can be applied.
    ///
    /// Returns the conversion entry only if the required library is included.
    pub fn find_applicable_conversion(
        &self,
        from_type: &str,
        to_type: &str,
    ) -> Option<&ConversionEntry> {
        let (entry, available) = self.check_conversion(from_type, to_type)?;
        if available {
            Some(entry)
        } else {
            None
        }
    }

    /// Get the registry.
    pub fn registry(&self) -> &ConversionRegistry {
        &self.registry
    }
}

// =============================================================================
// DataType Conversion Helpers
// =============================================================================

/// Convert a DataType to a type string for conversion lookup.
///
/// This produces strings like "FHIR.Coding", "System.Code", "FHIR.string", etc.
pub fn datatype_to_conversion_key(dtype: &DataType) -> Option<String> {
    match dtype {
        DataType::System(sys) => Some(format!("System.{}", sys.simple_name())),
        DataType::Model { namespace, name } => Some(format!("{namespace}.{name}")),
        DataType::List(elem) => {
            let elem_key = datatype_to_conversion_key(elem)?;
            Some(format!("List<{elem_key}>"))
        }
        DataType::Interval(point) => {
            let point_key = datatype_to_conversion_key(point)?;
            Some(format!("Interval<{point_key}>"))
        }
        _ => None,
    }
}

/// Parse a conversion type string to a DataType.
///
/// Handles strings like "FHIR.Coding", "System.Code", "Interval<System.DateTime>".
pub fn conversion_key_to_datatype(key: &str) -> Option<DataType> {
    // Handle Interval<...>
    if let Some(inner) = key
        .strip_prefix("Interval<")
        .and_then(|s| s.strip_suffix('>'))
    {
        let point_type = conversion_key_to_datatype(inner)?;
        return Some(DataType::interval(point_type));
    }

    // Handle List<...>
    if let Some(inner) = key.strip_prefix("List<").and_then(|s| s.strip_suffix('>')) {
        let elem_type = conversion_key_to_datatype(inner)?;
        return Some(DataType::list(elem_type));
    }

    // Handle qualified names: System.X or FHIR.X
    if let Some(dot_pos) = key.find('.') {
        let namespace = &key[..dot_pos];
        let name = &key[dot_pos + 1..];

        if namespace == "System" {
            if let Some(sys) = crate::datatype::SystemType::from_name(name) {
                return Some(DataType::system(sys));
            }
        }

        return Some(DataType::model(namespace, name));
    }

    None
}

/// Check if a type needs conversion to match an expected type.
///
/// Returns the source and target type keys if conversion is needed.
pub fn needs_conversion(actual: &DataType, expected: &DataType) -> Option<(String, String)> {
    // Same type - no conversion needed
    if actual == expected {
        return None;
    }

    // If actual is a subtype, no conversion needed
    if actual.is_subtype_of(expected) {
        return None;
    }

    // If standard implicit conversion exists, no ModelInfo conversion needed
    if actual.can_convert_to(expected) {
        return None;
    }

    // Get type keys for ModelInfo lookup
    let from_key = datatype_to_conversion_key(actual)?;
    let to_key = datatype_to_conversion_key(expected)?;

    Some((from_key, to_key))
}

// =============================================================================
// Conversion Expression Builder
// =============================================================================

/// Wrap an expression in a FunctionRef conversion call.
///
/// Creates an ELM FunctionRef expression that calls the appropriate converter
/// function (e.g., FHIRHelpers.ToCode) with the operand as its single argument.
///
/// # Arguments
///
/// * `operand` - The expression to convert.
/// * `entry` - The conversion entry describing the conversion.
/// * `element_fields` - The ElementFields to use for the FunctionRef.
///
/// # Returns
///
/// A FunctionRef expression wrapping the operand.
///
/// # Example
///
/// ```rust,ignore
/// use rh_cql::conversion::{wrap_in_conversion, ConversionEntry};
/// use rh_cql::elm::{Expression, Literal, ElementFields};
///
/// let operand = Expression::Literal(Literal { /* ... */ });
/// let entry = ConversionEntry {
///     from_type: "FHIR.Coding".to_string(),
///     to_type: "System.Code".to_string(),
///     function_name: "FHIRHelpers.ToCode".to_string(),
///     library_name: Some("FHIRHelpers".to_string()),
///     function_simple_name: "ToCode".to_string(),
/// };
/// let converted = wrap_in_conversion(operand, &entry, ElementFields::default());
/// ```
pub fn wrap_in_conversion(
    operand: crate::elm::Expression,
    entry: &ConversionEntry,
    element_fields: crate::elm::ElementFields,
) -> crate::elm::Expression {
    crate::elm::Expression::FunctionRef(crate::elm::FunctionRef {
        element: element_fields,
        name: Some(entry.function_simple_name.clone()),
        library_name: entry.library_name.clone(),
        operand: vec![operand],
        signature: Vec::new(),
    })
}

/// Result of attempting to apply a conversion.
#[derive(Debug, Clone)]
pub enum ConversionResult {
    /// No conversion needed - types are compatible.
    NotNeeded,
    /// Conversion applied successfully.
    Applied {
        /// The converted expression (boxed to reduce enum size).
        expression: Box<crate::elm::Expression>,
        /// The conversion entry that was used.
        entry: ConversionEntry,
    },
    /// Conversion needed but the required library is not included.
    LibraryNotIncluded {
        /// The conversion that would be applied.
        entry: ConversionEntry,
        /// The library that needs to be included.
        required_library: String,
    },
    /// Conversion needed but no conversion is defined.
    NoConversionDefined {
        /// The source type.
        from_type: String,
        /// The target type.
        to_type: String,
    },
}

impl ConversionContext {
    /// Try to apply a conversion to an expression if needed.
    ///
    /// This method checks if the actual type needs conversion to the expected type,
    /// and if so, wraps the expression in the appropriate FunctionRef.
    ///
    /// # Arguments
    ///
    /// * `operand` - The expression to potentially convert.
    /// * `actual_type` - The actual type of the operand.
    /// * `expected_type` - The expected/target type.
    /// * `element_fields` - The ElementFields to use if conversion is applied.
    ///
    /// # Returns
    ///
    /// A `ConversionResult` indicating what action was taken or needed.
    pub fn try_convert(
        &self,
        operand: crate::elm::Expression,
        actual_type: &DataType,
        expected_type: &DataType,
        element_fields: crate::elm::ElementFields,
    ) -> ConversionResult {
        // Check if conversion is needed
        let (from_key, to_key) = match needs_conversion(actual_type, expected_type) {
            Some(keys) => keys,
            None => return ConversionResult::NotNeeded,
        };

        // Look up conversion in registry
        let entry = match self.registry.find_conversion(&from_key, &to_key) {
            Some(e) => e,
            None => {
                return ConversionResult::NoConversionDefined {
                    from_type: from_key,
                    to_type: to_key,
                }
            }
        };

        // Check if required library is available
        if let Some(ref lib) = entry.library_name {
            if !self.is_library_included(lib) {
                return ConversionResult::LibraryNotIncluded {
                    entry: entry.clone(),
                    required_library: lib.clone(),
                };
            }
        }

        // Apply the conversion
        let converted = wrap_in_conversion(operand, entry, element_fields);
        ConversionResult::Applied {
            expression: Box::new(converted),
            entry: entry.clone(),
        }
    }

    /// Apply a conversion if needed and available, otherwise return the original.
    ///
    /// This is a convenience method that returns the converted expression if
    /// conversion was successful, or the original expression if not needed
    /// or not possible.
    ///
    /// # Returns
    ///
    /// A tuple of (expression, was_converted).
    pub fn convert_if_needed(
        &self,
        operand: crate::elm::Expression,
        actual_type: &DataType,
        expected_type: &DataType,
        element_fields: crate::elm::ElementFields,
    ) -> (crate::elm::Expression, bool) {
        match self.try_convert(operand.clone(), actual_type, expected_type, element_fields) {
            ConversionResult::Applied { expression, .. } => (*expression, true),
            _ => (operand, false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datatype::SystemType;

    #[test]
    fn test_conversion_entry_from_info() {
        let info = ConversionInfo {
            from_type: Some("FHIR.Coding".to_string()),
            to_type: Some("System.Code".to_string()),
            function_name: Some("FHIRHelpers.ToCode".to_string()),
            ..Default::default()
        };

        let entry = ConversionEntry::from_conversion_info(&info).unwrap();
        assert_eq!(entry.from_type, "FHIR.Coding");
        assert_eq!(entry.to_type, "System.Code");
        assert_eq!(entry.function_name, "FHIRHelpers.ToCode");
        assert_eq!(entry.library_name, Some("FHIRHelpers".to_string()));
        assert_eq!(entry.function_simple_name, "ToCode");
    }

    #[test]
    fn test_registry_lookup() {
        let mut registry = ConversionRegistry::new();

        registry.register(ConversionEntry {
            from_type: "FHIR.Coding".to_string(),
            to_type: "System.Code".to_string(),
            function_name: "FHIRHelpers.ToCode".to_string(),
            library_name: Some("FHIRHelpers".to_string()),
            function_simple_name: "ToCode".to_string(),
        });

        registry.register(ConversionEntry {
            from_type: "FHIR.CodeableConcept".to_string(),
            to_type: "System.Concept".to_string(),
            function_name: "FHIRHelpers.ToConcept".to_string(),
            library_name: Some("FHIRHelpers".to_string()),
            function_simple_name: "ToConcept".to_string(),
        });

        assert_eq!(registry.len(), 2);

        let conv = registry.find_conversion("FHIR.Coding", "System.Code");
        assert!(conv.is_some());
        assert_eq!(conv.unwrap().function_simple_name, "ToCode");

        let conv = registry.find_conversion("FHIR.CodeableConcept", "System.Concept");
        assert!(conv.is_some());

        let conv = registry.find_conversion("FHIR.Coding", "System.String");
        assert!(conv.is_none());
    }

    #[test]
    fn test_registry_from_model_info() {
        let model_info = ModelInfo {
            conversion_info: vec![
                ConversionInfo {
                    from_type: Some("FHIR.Coding".to_string()),
                    to_type: Some("System.Code".to_string()),
                    function_name: Some("FHIRHelpers.ToCode".to_string()),
                    ..Default::default()
                },
                ConversionInfo {
                    from_type: Some("FHIR.string".to_string()),
                    to_type: Some("System.String".to_string()),
                    function_name: Some("FHIRHelpers.ToString".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let registry = ConversionRegistry::from_model_info(&model_info);
        assert_eq!(registry.len(), 2);
        assert!(registry.has_conversion("FHIR.Coding", "System.Code"));
        assert!(registry.has_conversion("FHIR.string", "System.String"));
    }

    #[test]
    fn test_conversion_context() {
        let mut registry = ConversionRegistry::new();
        registry.register(ConversionEntry {
            from_type: "FHIR.Coding".to_string(),
            to_type: "System.Code".to_string(),
            function_name: "FHIRHelpers.ToCode".to_string(),
            library_name: Some("FHIRHelpers".to_string()),
            function_simple_name: "ToCode".to_string(),
        });

        let mut ctx = ConversionContext::with_registry(registry);

        // Without FHIRHelpers included
        let result = ctx.check_conversion("FHIR.Coding", "System.Code");
        assert!(result.is_some());
        let (entry, available) = result.unwrap();
        assert_eq!(entry.function_simple_name, "ToCode");
        assert!(!available);

        // Add FHIRHelpers
        ctx.add_included_library("FHIRHelpers", "FHIRHelpers");
        let result = ctx.check_conversion("FHIR.Coding", "System.Code");
        assert!(result.is_some());
        let (_, available) = result.unwrap();
        assert!(available);

        // Now find_applicable_conversion should work
        let conv = ctx.find_applicable_conversion("FHIR.Coding", "System.Code");
        assert!(conv.is_some());
    }

    #[test]
    fn test_datatype_to_conversion_key() {
        assert_eq!(
            datatype_to_conversion_key(&DataType::system(SystemType::Code)),
            Some("System.Code".to_string())
        );
        assert_eq!(
            datatype_to_conversion_key(&DataType::model("FHIR", "Coding")),
            Some("FHIR.Coding".to_string())
        );
        assert_eq!(
            datatype_to_conversion_key(&DataType::list(DataType::model("FHIR", "Observation"))),
            Some("List<FHIR.Observation>".to_string())
        );
        assert_eq!(
            datatype_to_conversion_key(&DataType::interval(DataType::system(SystemType::DateTime))),
            Some("Interval<System.DateTime>".to_string())
        );
    }

    #[test]
    fn test_conversion_key_to_datatype() {
        assert_eq!(
            conversion_key_to_datatype("System.Code"),
            Some(DataType::system(SystemType::Code))
        );
        assert_eq!(
            conversion_key_to_datatype("FHIR.Coding"),
            Some(DataType::model("FHIR", "Coding"))
        );
        assert_eq!(
            conversion_key_to_datatype("List<FHIR.Observation>"),
            Some(DataType::list(DataType::model("FHIR", "Observation")))
        );
        assert_eq!(
            conversion_key_to_datatype("Interval<System.DateTime>"),
            Some(DataType::interval(DataType::system(SystemType::DateTime)))
        );
    }

    #[test]
    fn test_needs_conversion() {
        // Same type - no conversion
        let coding = DataType::model("FHIR", "Coding");
        assert!(needs_conversion(&coding, &coding).is_none());

        // Subtype - no conversion
        let int = DataType::integer();
        let decimal = DataType::decimal();
        assert!(needs_conversion(&int, &decimal).is_none());

        // Different types - need conversion
        let code = DataType::system(SystemType::Code);
        let result = needs_conversion(&coding, &code);
        assert!(result.is_some());
        let (from, to) = result.unwrap();
        assert_eq!(from, "FHIR.Coding");
        assert_eq!(to, "System.Code");
    }

    #[test]
    fn test_find_conversions_from() {
        let mut registry = ConversionRegistry::new();
        registry.register(ConversionEntry {
            from_type: "FHIR.string".to_string(),
            to_type: "System.String".to_string(),
            function_name: "FHIRHelpers.ToString".to_string(),
            library_name: Some("FHIRHelpers".to_string()),
            function_simple_name: "ToString".to_string(),
        });
        registry.register(ConversionEntry {
            from_type: "FHIR.Coding".to_string(),
            to_type: "System.Code".to_string(),
            function_name: "FHIRHelpers.ToCode".to_string(),
            library_name: Some("FHIRHelpers".to_string()),
            function_simple_name: "ToCode".to_string(),
        });

        let from_string = registry.find_conversions_from("FHIR.string");
        assert_eq!(from_string.len(), 1);

        let from_coding = registry.find_conversions_from("FHIR.Coding");
        assert_eq!(from_coding.len(), 1);

        let from_unknown = registry.find_conversions_from("FHIR.Unknown");
        assert_eq!(from_unknown.len(), 0);
    }

    #[test]
    fn test_load_from_real_modelinfo() {
        // Test with real ModelInfo if available
        if let Ok(model_info) = crate::provider::load_fhir_r4_modelinfo_from_package() {
            let registry = ConversionRegistry::from_model_info(&model_info);

            // Should have many conversions
            assert!(
                registry.len() > 200,
                "Expected > 200 conversions, got {}",
                registry.len()
            );

            // Check key conversions
            assert!(registry.has_conversion("FHIR.Coding", "System.Code"));
            assert!(registry.has_conversion("FHIR.CodeableConcept", "System.Concept"));
            assert!(registry.has_conversion("FHIR.Quantity", "System.Quantity"));
            assert!(registry.has_conversion("FHIR.Period", "Interval<System.DateTime>"));
            assert!(registry.has_conversion("FHIR.Range", "Interval<System.Quantity>"));
            assert!(registry.has_conversion("FHIR.string", "System.String"));
            assert!(registry.has_conversion("FHIR.boolean", "System.Boolean"));
        }
    }

    #[test]
    fn test_wrap_in_conversion() {
        use crate::elm::{ElementFields, Expression, Literal};

        // Create a simple literal to convert
        let operand = Expression::Literal(Literal {
            element: ElementFields::default(),
            value_type: None,
            value: Some("test".to_string()),
        });

        let entry = ConversionEntry {
            from_type: "FHIR.Coding".to_string(),
            to_type: "System.Code".to_string(),
            function_name: "FHIRHelpers.ToCode".to_string(),
            library_name: Some("FHIRHelpers".to_string()),
            function_simple_name: "ToCode".to_string(),
        };

        let converted = wrap_in_conversion(operand, &entry, ElementFields::default());

        // Check that we got a FunctionRef
        if let Expression::FunctionRef(func_ref) = converted {
            assert_eq!(func_ref.name, Some("ToCode".to_string()));
            assert_eq!(func_ref.library_name, Some("FHIRHelpers".to_string()));
            assert_eq!(func_ref.operand.len(), 1);
        } else {
            panic!("Expected FunctionRef, got {converted:?}");
        }
    }

    #[test]
    fn test_try_convert_not_needed() {
        let registry = ConversionRegistry::new();
        let ctx = ConversionContext::with_registry(registry);

        let operand = crate::elm::Expression::Literal(crate::elm::Literal {
            element: crate::elm::ElementFields::default(),
            value_type: None,
            value: Some("42".to_string()),
        });

        // Same type - no conversion needed
        let int_type = DataType::integer();
        let result = ctx.try_convert(
            operand,
            &int_type,
            &int_type,
            crate::elm::ElementFields::default(),
        );

        assert!(matches!(result, ConversionResult::NotNeeded));
    }

    #[test]
    fn test_try_convert_applied() {
        let mut registry = ConversionRegistry::new();
        registry.register(ConversionEntry {
            from_type: "FHIR.Coding".to_string(),
            to_type: "System.Code".to_string(),
            function_name: "FHIRHelpers.ToCode".to_string(),
            library_name: Some("FHIRHelpers".to_string()),
            function_simple_name: "ToCode".to_string(),
        });

        let mut ctx = ConversionContext::with_registry(registry);
        ctx.add_included_library("FHIRHelpers", "FHIRHelpers");

        let operand = crate::elm::Expression::Literal(crate::elm::Literal {
            element: crate::elm::ElementFields::default(),
            value_type: None,
            value: Some("test".to_string()),
        });

        let coding = DataType::model("FHIR", "Coding");
        let code = DataType::system(SystemType::Code);

        let result = ctx.try_convert(
            operand,
            &coding,
            &code,
            crate::elm::ElementFields::default(),
        );

        match result {
            ConversionResult::Applied { expression, entry } => {
                assert_eq!(entry.function_simple_name, "ToCode");
                if let crate::elm::Expression::FunctionRef(func_ref) = *expression {
                    assert_eq!(func_ref.name, Some("ToCode".to_string()));
                    assert_eq!(func_ref.library_name, Some("FHIRHelpers".to_string()));
                } else {
                    panic!("Expected FunctionRef");
                }
            }
            other => panic!("Expected Applied, got {other:?}"),
        }
    }

    #[test]
    fn test_try_convert_library_not_included() {
        let mut registry = ConversionRegistry::new();
        registry.register(ConversionEntry {
            from_type: "FHIR.Coding".to_string(),
            to_type: "System.Code".to_string(),
            function_name: "FHIRHelpers.ToCode".to_string(),
            library_name: Some("FHIRHelpers".to_string()),
            function_simple_name: "ToCode".to_string(),
        });

        // Note: NOT adding FHIRHelpers to included libraries
        let ctx = ConversionContext::with_registry(registry);

        let operand = crate::elm::Expression::Literal(crate::elm::Literal {
            element: crate::elm::ElementFields::default(),
            value_type: None,
            value: Some("test".to_string()),
        });

        let coding = DataType::model("FHIR", "Coding");
        let code = DataType::system(SystemType::Code);

        let result = ctx.try_convert(
            operand,
            &coding,
            &code,
            crate::elm::ElementFields::default(),
        );

        match result {
            ConversionResult::LibraryNotIncluded {
                required_library, ..
            } => {
                assert_eq!(required_library, "FHIRHelpers");
            }
            other => panic!("Expected LibraryNotIncluded, got {other:?}"),
        }
    }

    #[test]
    fn test_try_convert_no_conversion_defined() {
        let registry = ConversionRegistry::new(); // Empty registry
        let ctx = ConversionContext::with_registry(registry);

        let operand = crate::elm::Expression::Literal(crate::elm::Literal {
            element: crate::elm::ElementFields::default(),
            value_type: None,
            value: Some("test".to_string()),
        });

        let coding = DataType::model("FHIR", "Coding");
        let code = DataType::system(SystemType::Code);

        let result = ctx.try_convert(
            operand,
            &coding,
            &code,
            crate::elm::ElementFields::default(),
        );

        match result {
            ConversionResult::NoConversionDefined { from_type, to_type } => {
                assert_eq!(from_type, "FHIR.Coding");
                assert_eq!(to_type, "System.Code");
            }
            other => panic!("Expected NoConversionDefined, got {other:?}"),
        }
    }

    #[test]
    fn test_convert_if_needed() {
        let mut registry = ConversionRegistry::new();
        registry.register(ConversionEntry {
            from_type: "FHIR.Coding".to_string(),
            to_type: "System.Code".to_string(),
            function_name: "FHIRHelpers.ToCode".to_string(),
            library_name: Some("FHIRHelpers".to_string()),
            function_simple_name: "ToCode".to_string(),
        });

        let mut ctx = ConversionContext::with_registry(registry);
        ctx.add_included_library("FHIRHelpers", "FHIRHelpers");

        let operand = crate::elm::Expression::Literal(crate::elm::Literal {
            element: crate::elm::ElementFields::default(),
            value_type: None,
            value: Some("test".to_string()),
        });

        let coding = DataType::model("FHIR", "Coding");
        let code = DataType::system(SystemType::Code);

        let (converted, was_converted) = ctx.convert_if_needed(
            operand.clone(),
            &coding,
            &code,
            crate::elm::ElementFields::default(),
        );

        assert!(was_converted);
        assert!(matches!(converted, crate::elm::Expression::FunctionRef(_)));

        // Test when no conversion needed
        let int_type = DataType::integer();
        let (expr, was_converted) = ctx.convert_if_needed(
            operand,
            &int_type,
            &int_type,
            crate::elm::ElementFields::default(),
        );

        assert!(!was_converted);
        assert!(matches!(expr, crate::elm::Expression::Literal(_)));
    }
}
