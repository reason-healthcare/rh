use crate::traits::resource::ResourceExistence;
/// Shareable ValueSet Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the value set metadata required by HL7 and other organizations that share and publish value sets
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareablevalueset
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ValueSet
pub trait ShareablevaluesetAccessors {}
/// Shareable ValueSet Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the value set metadata required by HL7 and other organizations that share and publish value sets
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareablevalueset
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ValueSet
pub trait ShareablevaluesetMutators {
    /// Create a new Shareablevalueset with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::shareablevalueset::Shareablevalueset;
    /// use hl7_fhir_r4_core::traits::shareablevalueset::ShareablevaluesetMutators;
    ///
    /// let resource = Shareablevalueset::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// shareablevalueset Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Enforces the minimum information set for the value set metadata required by HL7 and other organizations that share and publish value sets
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareablevalueset
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ValueSet
pub trait ShareablevaluesetExistence: ResourceExistence {}
