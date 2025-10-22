use crate::bindings::request_priority::RequestPriority;
use crate::bindings::supplyrequest_status::SupplyrequestStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::supply_request::SupplyRequestParameter;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SupplyRequest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a request for a medication, substance or device used in the healthcare setting.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SupplyRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SupplyRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SupplyRequestAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<SupplyrequestStatus>;
    /// Returns a reference to the category field.
    fn category(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<RequestPriority>;
    /// Returns a reference to the quantity field.
    fn quantity(&self) -> Quantity;
    /// Returns a reference to the parameter field.
    fn parameter(&self) -> &[SupplyRequestParameter];
    /// Returns a reference to the authoredOn field.
    fn authored_on(&self) -> Option<DateTimeType>;
    /// Returns a reference to the requester field.
    fn requester(&self) -> Option<Reference>;
    /// Returns a reference to the supplier field.
    fn supplier(&self) -> &[Reference];
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the deliverFrom field.
    fn deliver_from(&self) -> Option<Reference>;
    /// Returns a reference to the deliverTo field.
    fn deliver_to(&self) -> Option<Reference>;
}
/// SupplyRequest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a request for a medication, substance or device used in the healthcare setting.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SupplyRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SupplyRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SupplyRequestMutators: DomainResourceMutators {
    /// Create a new SupplyRequest with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::supply_request::SupplyRequest;
    /// use hl7_fhir_r4_core::traits::supply_request::SupplyRequestMutators;
    ///
    /// let resource = SupplyRequest::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: SupplyrequestStatus) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: CodeableConcept) -> Self;
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: RequestPriority) -> Self;
    /// Sets the quantity field and returns self for chaining.
    fn set_quantity(self, value: Quantity) -> Self;
    /// Sets the parameter field and returns self for chaining.
    fn set_parameter(self, value: Vec<SupplyRequestParameter>) -> Self;
    /// Adds an item to the parameter field and returns self for chaining.
    fn add_parameter(self, item: SupplyRequestParameter) -> Self;
    /// Sets the authoredOn field and returns self for chaining.
    fn set_authored_on(self, value: String) -> Self;
    /// Sets the requester field and returns self for chaining.
    fn set_requester(self, value: Reference) -> Self;
    /// Sets the supplier field and returns self for chaining.
    fn set_supplier(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supplier field and returns self for chaining.
    fn add_supplier(self, item: Reference) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the deliverFrom field and returns self for chaining.
    fn set_deliver_from(self, value: Reference) -> Self;
    /// Sets the deliverTo field and returns self for chaining.
    fn set_deliver_to(self, value: Reference) -> Self;
}
/// SupplyRequest Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of a request for a medication, substance or device used in the healthcare setting.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SupplyRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SupplyRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SupplyRequestExistence: DomainResourceExistence {
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
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the category field is present (Some).
    fn has_category(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the item field is present (Some).
    fn has_item(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the parameter field is not empty.
    fn has_parameter(&self) -> bool;
    /// Returns true if the occurrence field is present (Some).
    fn has_occurrence(&self) -> bool;
    /// Returns true if the authored_on field is present (Some).
    fn has_authored_on(&self) -> bool;
    /// Returns true if the requester field is present (Some).
    fn has_requester(&self) -> bool;
    /// Returns true if the supplier field is not empty.
    fn has_supplier(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the deliver_from field is present (Some).
    fn has_deliver_from(&self) -> bool;
    /// Returns true if the deliver_to field is present (Some).
    fn has_deliver_to(&self) -> bool;
}
