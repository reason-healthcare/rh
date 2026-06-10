use crate::traits::resource::ResourceExistence;
/// Shareable  Test Script Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the test script metadata required by HL7 and other organizations that share and publish test scripts
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareabletestscript
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: TestScript
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/TestScript
pub trait ShareabletestscriptAccessors {}
/// Shareable  Test Script Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the test script metadata required by HL7 and other organizations that share and publish test scripts
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareabletestscript
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: TestScript
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/TestScript
pub trait ShareabletestscriptMutators {
    /// Create a new Shareabletestscript with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::shareabletestscript::Shareabletestscript;
    /// use rh_hl7_fhir_r5_core::traits::shareabletestscript::ShareabletestscriptMutators;
    ///
    /// let resource = Shareabletestscript::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// shareabletestscript Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Enforces the minimum information set for the test script metadata required by HL7 and other organizations that share and publish test scripts
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareabletestscript
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: TestScript
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/TestScript
pub trait ShareabletestscriptExistence: ResourceExistence {}
