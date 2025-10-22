use crate::bindings::supplydelivery_status::SupplydeliveryStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::resources::supply_delivery::SupplyDeliverySupplieditem;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SupplyDelivery Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Record of delivery of what is supplied.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SupplyDelivery
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SupplyDelivery
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SupplyDeliveryAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<SupplydeliveryStatus>;
    /// Returns a reference to the patient field.
    fn patient(&self) -> Option<Reference>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the suppliedItem field.
    fn supplied_item(&self) -> Option<SupplyDeliverySupplieditem>;
    /// Returns a reference to the supplier field.
    fn supplier(&self) -> Option<Reference>;
    /// Returns a reference to the destination field.
    fn destination(&self) -> Option<Reference>;
    /// Returns a reference to the receiver field.
    fn receiver(&self) -> &[Reference];
}
/// SupplyDelivery Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Record of delivery of what is supplied.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SupplyDelivery
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SupplyDelivery
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SupplyDeliveryMutators: DomainResourceMutators {
    /// Create a new SupplyDelivery with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::supply_delivery::SupplyDelivery;
    /// use hl7_fhir_r4_core::traits::supply_delivery::SupplyDeliveryMutators;
    ///
    /// let resource = SupplyDelivery::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: SupplydeliveryStatus) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the suppliedItem field and returns self for chaining.
    fn set_supplied_item(self, value: SupplyDeliverySupplieditem) -> Self;
    /// Sets the supplier field and returns self for chaining.
    fn set_supplier(self, value: Reference) -> Self;
    /// Sets the destination field and returns self for chaining.
    fn set_destination(self, value: Reference) -> Self;
    /// Sets the receiver field and returns self for chaining.
    fn set_receiver(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the receiver field and returns self for chaining.
    fn add_receiver(self, item: Reference) -> Self;
}
/// SupplyDelivery Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Record of delivery of what is supplied.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SupplyDelivery
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SupplyDelivery
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SupplyDeliveryExistence: DomainResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the text field is present (Some).
    fn has_text(&self) -> bool;
    /// Returns true if the contained field is not empty.
    fn has_contained(&self) -> bool;
    /// Returns true if the extension field is not empty.
    fn has_extension(&self) -> bool;
    /// Returns true if the modifier_extension field is not empty.
    fn has_modifier_extension(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the supplied_item field is present (Some).
    fn has_supplied_item(&self) -> bool;
    /// Returns true if the occurrence field is present (Some).
    fn has_occurrence(&self) -> bool;
    /// Returns true if the supplier field is present (Some).
    fn has_supplier(&self) -> bool;
    /// Returns true if the destination field is present (Some).
    fn has_destination(&self) -> bool;
    /// Returns true if the receiver field is not empty.
    fn has_receiver(&self) -> bool;
}
