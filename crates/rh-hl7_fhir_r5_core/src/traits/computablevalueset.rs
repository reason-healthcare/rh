use crate::traits::resource::ResourceExistence;
/// Computable ValueSet Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines a computable value set as one that SHALL have an expression-based definition (i.e. a value set defined intensionally using expressions of the code systems involved) and MAY have an expansion included. The expression-based definition SHALL be represented in only one of three ways; using the compose element, using the expression extension, or using the rules-text extension to provide a step-by-step process for expanding the value set definition
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/computablevalueset
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablevalueset
pub trait ComputablevaluesetAccessors {}
/// Computable ValueSet Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines a computable value set as one that SHALL have an expression-based definition (i.e. a value set defined intensionally using expressions of the code systems involved) and MAY have an expansion included. The expression-based definition SHALL be represented in only one of three ways; using the compose element, using the expression extension, or using the rules-text extension to provide a step-by-step process for expanding the value set definition
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/computablevalueset
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablevalueset
pub trait ComputablevaluesetMutators {
    /// Create a new Computablevalueset with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::computablevalueset::Computablevalueset;
    /// use rh_hl7_fhir_r5_core::traits::computablevalueset::ComputablevaluesetMutators;
    ///
    /// let resource = Computablevalueset::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// computablevalueset Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines a computable value set as one that SHALL have an expression-based definition (i.e. a value set defined intensionally using expressions of the code systems involved) and MAY have an expansion included. The expression-based definition SHALL be represented in only one of three ways; using the compose element, using the expression extension, or using the rules-text extension to provide a step-by-step process for expanding the value set definition
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/computablevalueset
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablevalueset
pub trait ComputablevaluesetExistence: ResourceExistence {}
