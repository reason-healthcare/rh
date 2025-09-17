use crate::traits::domain_resource::DomainResourceMutators;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceAccessors;
/// Patient Resource Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Demographics and other administrative information about an individual.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Patient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DomainResource
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PatientAccessors: DomainResourceAccessors {}
/// Patient Resource Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Demographics and other administrative information about an individual.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Patient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DomainResource
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PatientMutators: DomainResourceMutators {
    /// Create a new Patient with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// let resource = Patient::new()
    ///     .set_status("active".to_string())
    ///     .set_name("Example Name".to_string());
    /// ```
    fn new(&self) -> Self;
}
/// Patient Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Demographics and other administrative information about an individual.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Patient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DomainResource
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PatientExistence: DomainResourceExistence {}
