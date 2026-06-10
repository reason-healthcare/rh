use crate::bindings::endpoint_status::EndpointStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::endpoint::EndpointPayload;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Endpoint Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The technical details of an endpoint that can be used for electronic services, such as for web services providing XDS.b, a REST endpoint for another FHIR server, or a s/Mime email address. This may include any security context information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Endpoint
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Endpoint
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EndpointAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> EndpointStatus;
    /// Returns a reference to the connectionType field.
    fn connection_type(&self) -> &[CodeableConcept];
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the environmentType field.
    fn environment_type(&self) -> &[CodeableConcept];
    /// Returns a reference to the managingOrganization field.
    fn managing_organization(&self) -> Option<Reference>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactPoint];
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the payload field.
    fn payload(&self) -> &[EndpointPayload];
    /// Returns a reference to the address field.
    fn address(&self) -> StringType;
    /// Returns a reference to the header field.
    fn header(&self) -> &[StringType];
}
/// Endpoint Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The technical details of an endpoint that can be used for electronic services, such as for web services providing XDS.b, a REST endpoint for another FHIR server, or a s/Mime email address. This may include any security context information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Endpoint
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Endpoint
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EndpointMutators: DomainResourceMutators {
    /// Create a new Endpoint with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::endpoint::Endpoint;
    /// use rh_hl7_fhir_r5_core::traits::endpoint::EndpointMutators;
    ///
    /// let resource = Endpoint::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: EndpointStatus) -> Self;
    /// Sets the connectionType field and returns self for chaining.
    fn set_connection_type(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the connectionType field and returns self for chaining.
    fn add_connection_type(self, item: CodeableConcept) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the environmentType field and returns self for chaining.
    fn set_environment_type(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the environmentType field and returns self for chaining.
    fn add_environment_type(self, item: CodeableConcept) -> Self;
    /// Sets the managingOrganization field and returns self for chaining.
    fn set_managing_organization(self, value: Reference) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactPoint>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactPoint) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the payload field and returns self for chaining.
    fn set_payload(self, value: Vec<EndpointPayload>) -> Self;
    /// Adds an item to the payload field and returns self for chaining.
    fn add_payload(self, item: EndpointPayload) -> Self;
    /// Sets the address field and returns self for chaining.
    fn set_address(self, value: String) -> Self;
    /// Sets the header field and returns self for chaining.
    fn set_header(self, value: Vec<String>) -> Self;
    /// Adds an item to the header field and returns self for chaining.
    fn add_header(self, item: String) -> Self;
}
/// Endpoint Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The technical details of an endpoint that can be used for electronic services, such as for web services providing XDS.b, a REST endpoint for another FHIR server, or a s/Mime email address. This may include any security context information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Endpoint
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Endpoint
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EndpointExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the connection_type field is not empty.
    fn has_connection_type(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the environment_type field is not empty.
    fn has_environment_type(&self) -> bool;
    /// Returns true if the managing_organization field is present (Some).
    fn has_managing_organization(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the payload field is not empty.
    fn has_payload(&self) -> bool;
    /// Returns true if the address field is present (Some).
    fn has_address(&self) -> bool;
    /// Returns true if the header field is not empty.
    fn has_header(&self) -> bool;
}
