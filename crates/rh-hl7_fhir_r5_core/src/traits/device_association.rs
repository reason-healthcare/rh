use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::resources::device_association::DeviceAssociationOperation;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DeviceAssociation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of association of a device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceAssociation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceAssociation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceAssociationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the device field.
    fn device(&self) -> Reference;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the status field.
    fn status(&self) -> CodeableConcept;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> &[CodeableConcept];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the bodyStructure field.
    fn body_structure(&self) -> Option<Reference>;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the operation field.
    fn operation(&self) -> &[DeviceAssociationOperation];
}
/// DeviceAssociation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of association of a device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceAssociation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceAssociation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceAssociationMutators: DomainResourceMutators {
    /// Create a new DeviceAssociation with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::device_association::DeviceAssociation;
    /// use rh_hl7_fhir_r5_core::traits::device_association::DeviceAssociationMutators;
    ///
    /// let resource = DeviceAssociation::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: Reference) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: CodeableConcept) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the statusReason field and returns self for chaining.
    fn add_status_reason(self, item: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the bodyStructure field and returns self for chaining.
    fn set_body_structure(self, value: Reference) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the operation field and returns self for chaining.
    fn set_operation(self, value: Vec<DeviceAssociationOperation>) -> Self;
    /// Adds an item to the operation field and returns self for chaining.
    fn add_operation(self, item: DeviceAssociationOperation) -> Self;
}
/// DeviceAssociation Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of association of a device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceAssociation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceAssociation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceAssociationExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the device field is present (Some).
    fn has_device(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is not empty.
    fn has_status_reason(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the body_structure field is present (Some).
    fn has_body_structure(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the operation field is not empty.
    fn has_operation(&self) -> bool;
}
