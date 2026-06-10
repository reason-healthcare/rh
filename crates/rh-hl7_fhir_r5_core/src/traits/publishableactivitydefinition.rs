use crate::traits::resource::ResourceExistence;
/// Publishable ActivityDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Supports declaration of the ActivityDefinition metadata required by HL7 and other organizations that share and publish activity definitions with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishableactivitydefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ActivityDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareableactivitydefinition
pub trait PublishableactivitydefinitionAccessors {}
/// Publishable ActivityDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Supports declaration of the ActivityDefinition metadata required by HL7 and other organizations that share and publish activity definitions with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishableactivitydefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ActivityDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareableactivitydefinition
pub trait PublishableactivitydefinitionMutators {
    /// Create a new Publishableactivitydefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::publishableactivitydefinition::Publishableactivitydefinition;
    /// use rh_hl7_fhir_r5_core::traits::publishableactivitydefinition::PublishableactivitydefinitionMutators;
    ///
    /// let resource = Publishableactivitydefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// publishableactivitydefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Supports declaration of the ActivityDefinition metadata required by HL7 and other organizations that share and publish activity definitions with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishableactivitydefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ActivityDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareableactivitydefinition
pub trait PublishableactivitydefinitionExistence: ResourceExistence {}
