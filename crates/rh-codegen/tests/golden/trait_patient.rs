use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Patient Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A resource for golden testing
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Patient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Patient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PatientAccessors: DomainResourceAccessors {}
/// Patient Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A resource for golden testing
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Patient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Patient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PatientMutators: DomainResourceMutators {
    /// Create a new Patient with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::patient::Patient;
    /// use hl7_fhir_r4_core::traits::patient::PatientMutators;
    ///
    /// let resource = Patient::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// Patient Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A resource for golden testing
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Patient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Patient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PatientExistence: DomainResourceExistence {}
