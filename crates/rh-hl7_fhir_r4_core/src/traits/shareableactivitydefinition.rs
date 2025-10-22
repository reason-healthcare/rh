use crate::traits::resource::ResourceExistence;
/// Shareable ActivityDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the activity definition metadata required by HL7 and other organizations that share and publish activity definitions
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareableactivitydefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ActivityDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ActivityDefinition
pub trait ShareableactivitydefinitionAccessors {}
/// Shareable ActivityDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the activity definition metadata required by HL7 and other organizations that share and publish activity definitions
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareableactivitydefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ActivityDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ActivityDefinition
pub trait ShareableactivitydefinitionMutators {
    /// Create a new Shareableactivitydefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::shareableactivitydefinition::Shareableactivitydefinition;
    /// use hl7_fhir_r4_core::traits::shareableactivitydefinition::ShareableactivitydefinitionMutators;
    ///
    /// let resource = Shareableactivitydefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// shareableactivitydefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Enforces the minimum information set for the activity definition metadata required by HL7 and other organizations that share and publish activity definitions
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareableactivitydefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ActivityDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ActivityDefinition
pub trait ShareableactivitydefinitionExistence: ResourceExistence {}
