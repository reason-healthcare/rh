use crate::bindings::location_mode::LocationMode;
use crate::bindings::location_status::LocationStatus;
use crate::datatypes::address::Address;
use crate::datatypes::availability::Availability;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::extended_contact_detail::ExtendedContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::datatypes::virtual_service_detail::VirtualServiceDetail;
use crate::primitives::string::StringType;
use crate::resources::location::LocationPosition;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Location Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Details and position information for a place where services are provided and resources and participants may be stored, found, contained, or accommodated.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Location
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Location
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait LocationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<LocationStatus>;
    /// Returns a reference to the operationalStatus field.
    fn operational_status(&self) -> Option<Coding>;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the alias field.
    fn alias(&self) -> &[StringType];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the mode field.
    fn mode(&self) -> Option<LocationMode>;
    /// Returns a reference to the type field.
    fn type_(&self) -> &[CodeableConcept];
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ExtendedContactDetail];
    /// Returns a reference to the address field.
    fn address(&self) -> Option<Address>;
    /// Returns a reference to the form field.
    fn form(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the position field.
    fn position(&self) -> Option<LocationPosition>;
    /// Returns a reference to the managingOrganization field.
    fn managing_organization(&self) -> Option<Reference>;
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> Option<Reference>;
    /// Returns a reference to the characteristic field.
    fn characteristic(&self) -> &[CodeableConcept];
    /// Returns a reference to the hoursOfOperation field.
    fn hours_of_operation(&self) -> &[Availability];
    /// Returns a reference to the virtualService field.
    fn virtual_service(&self) -> &[VirtualServiceDetail];
    /// Returns a reference to the endpoint field.
    fn endpoint(&self) -> &[Reference];
}
/// Location Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Details and position information for a place where services are provided and resources and participants may be stored, found, contained, or accommodated.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Location
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Location
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait LocationMutators: DomainResourceMutators {
    /// Create a new Location with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::location::Location;
    /// use rh_hl7_fhir_r5_core::traits::location::LocationMutators;
    ///
    /// let resource = Location::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: LocationStatus) -> Self;
    /// Sets the operationalStatus field and returns self for chaining.
    fn set_operational_status(self, value: Coding) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the alias field and returns self for chaining.
    fn set_alias(self, value: Vec<String>) -> Self;
    /// Adds an item to the alias field and returns self for chaining.
    fn add_alias(self, item: String) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the mode field and returns self for chaining.
    fn set_mode(self, value: LocationMode) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the type field and returns self for chaining.
    fn add_type_(self, item: CodeableConcept) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ExtendedContactDetail>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ExtendedContactDetail) -> Self;
    /// Sets the address field and returns self for chaining.
    fn set_address(self, value: Address) -> Self;
    /// Sets the form field and returns self for chaining.
    fn set_form(self, value: CodeableConcept) -> Self;
    /// Sets the position field and returns self for chaining.
    fn set_position(self, value: LocationPosition) -> Self;
    /// Sets the managingOrganization field and returns self for chaining.
    fn set_managing_organization(self, value: Reference) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Reference) -> Self;
    /// Sets the characteristic field and returns self for chaining.
    fn set_characteristic(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the characteristic field and returns self for chaining.
    fn add_characteristic(self, item: CodeableConcept) -> Self;
    /// Sets the hoursOfOperation field and returns self for chaining.
    fn set_hours_of_operation(self, value: Vec<Availability>) -> Self;
    /// Adds an item to the hoursOfOperation field and returns self for chaining.
    fn add_hours_of_operation(self, item: Availability) -> Self;
    /// Sets the virtualService field and returns self for chaining.
    fn set_virtual_service(self, value: Vec<VirtualServiceDetail>) -> Self;
    /// Adds an item to the virtualService field and returns self for chaining.
    fn add_virtual_service(self, item: VirtualServiceDetail) -> Self;
    /// Sets the endpoint field and returns self for chaining.
    fn set_endpoint(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the endpoint field and returns self for chaining.
    fn add_endpoint(self, item: Reference) -> Self;
}
/// Location Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Details and position information for a place where services are provided and resources and participants may be stored, found, contained, or accommodated.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Location
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Location
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait LocationExistence: DomainResourceExistence {
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
    /// Returns true if the operational_status field is present (Some).
    fn has_operational_status(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the alias field is not empty.
    fn has_alias(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the mode field is present (Some).
    fn has_mode(&self) -> bool;
    /// Returns true if the type_ field is not empty.
    fn has_type_(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the address field is present (Some).
    fn has_address(&self) -> bool;
    /// Returns true if the form field is present (Some).
    fn has_form(&self) -> bool;
    /// Returns true if the position field is present (Some).
    fn has_position(&self) -> bool;
    /// Returns true if the managing_organization field is present (Some).
    fn has_managing_organization(&self) -> bool;
    /// Returns true if the part_of field is present (Some).
    fn has_part_of(&self) -> bool;
    /// Returns true if the characteristic field is not empty.
    fn has_characteristic(&self) -> bool;
    /// Returns true if the hours_of_operation field is not empty.
    fn has_hours_of_operation(&self) -> bool;
    /// Returns true if the virtual_service field is not empty.
    fn has_virtual_service(&self) -> bool;
    /// Returns true if the endpoint field is not empty.
    fn has_endpoint(&self) -> bool;
}
