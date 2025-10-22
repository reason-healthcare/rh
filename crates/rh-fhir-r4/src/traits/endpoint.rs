use crate::bindings::endpoint_status::EndpointStatus;
use crate::bindings::mimetypes::Mimetypes;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Endpoint Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The technical details of an endpoint that can be used for electronic services, such as for web services providing XDS.b or a REST endpoint for another FHIR server. This may include any security context information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Endpoint
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Endpoint
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EndpointAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> EndpointStatus;
    /// Returns a reference to the connectionType field.
    fn connection_type(&self) -> Coding;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the managingOrganization field.
    fn managing_organization(&self) -> Option<Reference>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactPoint];
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the payloadType field.
    fn payload_type(&self) -> &[CodeableConcept];
    /// Returns a reference to the payloadMimeType field.
    fn payload_mime_type(&self) -> &[Mimetypes];
    /// Returns a reference to the address field.
    fn address(&self) -> StringType;
    /// Returns a reference to the header field.
    fn header(&self) -> &[StringType];
}
/// Endpoint Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The technical details of an endpoint that can be used for electronic services, such as for web services providing XDS.b or a REST endpoint for another FHIR server. This may include any security context information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Endpoint
/// - Version: 4.0.1
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
    /// use hl7_fhir_r4_core::resources::endpoint::Endpoint;
    /// use hl7_fhir_r4_core::traits::endpoint::EndpointMutators;
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
    fn set_connection_type(self, value: Coding) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the managingOrganization field and returns self for chaining.
    fn set_managing_organization(self, value: Reference) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactPoint>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactPoint) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the payloadType field and returns self for chaining.
    fn set_payload_type(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the payloadType field and returns self for chaining.
    fn add_payload_type(self, item: CodeableConcept) -> Self;
    /// Sets the payloadMimeType field and returns self for chaining.
    fn set_payload_mime_type(self, value: Vec<Mimetypes>) -> Self;
    /// Adds an item to the payloadMimeType field and returns self for chaining.
    fn add_payload_mime_type(self, item: Mimetypes) -> Self;
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
/// The technical details of an endpoint that can be used for electronic services, such as for web services providing XDS.b or a REST endpoint for another FHIR server. This may include any security context information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Endpoint
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Endpoint
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EndpointExistence: DomainResourceExistence {
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
    /// Returns true if the connection_type field is present (Some).
    fn has_connection_type(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the managing_organization field is present (Some).
    fn has_managing_organization(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the payload_type field is not empty.
    fn has_payload_type(&self) -> bool;
    /// Returns true if the payload_mime_type field is not empty.
    fn has_payload_mime_type(&self) -> bool;
    /// Returns true if the address field is present (Some).
    fn has_address(&self) -> bool;
    /// Returns true if the header field is not empty.
    fn has_header(&self) -> bool;
}
