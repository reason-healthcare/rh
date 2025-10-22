use crate::traits::resource::ResourceExistence;
/// Shareable CodeSystem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the value set metadata required by HL7 and other organizations that share and publish value sets
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareablecodesystem
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CodeSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/CodeSystem
pub trait ShareablecodesystemAccessors {}
/// Shareable CodeSystem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the value set metadata required by HL7 and other organizations that share and publish value sets
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareablecodesystem
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CodeSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/CodeSystem
pub trait ShareablecodesystemMutators {
    /// Create a new Shareablecodesystem with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::shareablecodesystem::Shareablecodesystem;
    /// use hl7_fhir_r4_core::traits::shareablecodesystem::ShareablecodesystemMutators;
    ///
    /// let resource = Shareablecodesystem::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// shareablecodesystem Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Enforces the minimum information set for the value set metadata required by HL7 and other organizations that share and publish value sets
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareablecodesystem
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CodeSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/CodeSystem
pub trait ShareablecodesystemExistence: ResourceExistence {}
