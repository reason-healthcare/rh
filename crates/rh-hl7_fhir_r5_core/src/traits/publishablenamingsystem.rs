use crate::traits::resource::ResourceExistence;
/// Publishable NamingSystem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines and enforces the minimum expectations for publication and distribution of a naming system, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablenamingsystem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NamingSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablenamingsystem
pub trait PublishablenamingsystemAccessors {}
/// Publishable NamingSystem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines and enforces the minimum expectations for publication and distribution of a naming system, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablenamingsystem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NamingSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablenamingsystem
pub trait PublishablenamingsystemMutators {
    /// Create a new Publishablenamingsystem with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::publishablenamingsystem::Publishablenamingsystem;
    /// use rh_hl7_fhir_r5_core::traits::publishablenamingsystem::PublishablenamingsystemMutators;
    ///
    /// let resource = Publishablenamingsystem::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// publishablenamingsystem Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines and enforces the minimum expectations for publication and distribution of a naming system, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablenamingsystem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NamingSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablenamingsystem
pub trait PublishablenamingsystemExistence: ResourceExistence {}
