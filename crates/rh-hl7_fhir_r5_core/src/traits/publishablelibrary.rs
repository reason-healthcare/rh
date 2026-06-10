use crate::traits::resource::ResourceExistence;
/// Publishable Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Supports declaration of the library metadata required by HL7 and other organizations that share and publish libraries with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablelibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablelibrary
pub trait PublishablelibraryAccessors {}
/// Publishable Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Supports declaration of the library metadata required by HL7 and other organizations that share and publish libraries with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablelibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablelibrary
pub trait PublishablelibraryMutators {
    /// Create a new Publishablelibrary with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::publishablelibrary::Publishablelibrary;
    /// use rh_hl7_fhir_r5_core::traits::publishablelibrary::PublishablelibraryMutators;
    ///
    /// let resource = Publishablelibrary::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// publishablelibrary Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Supports declaration of the library metadata required by HL7 and other organizations that share and publish libraries with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablelibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablelibrary
pub trait PublishablelibraryExistence: ResourceExistence {}
