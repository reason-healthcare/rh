use crate::bindings::message_significance_category::MessageSignificanceCategory;
use crate::bindings::messageheader_response_request::MessageheaderResponseRequest;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::message_definition::MessageDefinitionAllowedresponse;
use crate::resources::message_definition::MessageDefinitionFocus;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MessageDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines the characteristics of a message that can be shared between systems, including the type of event that initiates the message, the content to be transmitted and what response(s), if any, are permitted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MessageDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MessageDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MessageDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> Option<StringType>;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the replaces field.
    fn replaces(&self) -> &[StringType];
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the experimental field.
    fn experimental(&self) -> Option<BooleanType>;
    /// Returns a reference to the date field.
    fn date(&self) -> DateTimeType;
    /// Returns a reference to the publisher field.
    fn publisher(&self) -> Option<StringType>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactDetail];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the useContext field.
    fn use_context(&self) -> &[UsageContext];
    /// Returns a reference to the jurisdiction field.
    fn jurisdiction(&self) -> &[CodeableConcept];
    /// Returns a reference to the purpose field.
    fn purpose(&self) -> Option<StringType>;
    /// Returns a reference to the copyright field.
    fn copyright(&self) -> Option<StringType>;
    /// Returns a reference to the base field.
    fn base_definition(&self) -> Option<StringType>;
    /// Returns a reference to the parent field.
    fn parent(&self) -> &[StringType];
    /// Returns a reference to the category field.
    fn category(&self) -> Option<MessageSignificanceCategory>;
    /// Returns a reference to the focus field.
    fn focus(&self) -> &[MessageDefinitionFocus];
    /// Returns a reference to the responseRequired field.
    fn response_required(&self) -> Option<MessageheaderResponseRequest>;
    /// Returns a reference to the allowedResponse field.
    fn allowed_response(&self) -> &[MessageDefinitionAllowedresponse];
    /// Returns a reference to the graph field.
    fn graph(&self) -> &[StringType];
}
/// MessageDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines the characteristics of a message that can be shared between systems, including the type of event that initiates the message, the content to be transmitted and what response(s), if any, are permitted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MessageDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MessageDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MessageDefinitionMutators: DomainResourceMutators {
    /// Create a new MessageDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::message_definition::MessageDefinition;
    /// use hl7_fhir_r4_core::traits::message_definition::MessageDefinitionMutators;
    ///
    /// let resource = MessageDefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the url field and returns self for chaining.
    fn set_url(self, value: String) -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: String) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the replaces field and returns self for chaining.
    fn set_replaces(self, value: Vec<String>) -> Self;
    /// Adds an item to the replaces field and returns self for chaining.
    fn add_replaces(self, item: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the experimental field and returns self for chaining.
    fn set_experimental(self, value: bool) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the publisher field and returns self for chaining.
    fn set_publisher(self, value: String) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactDetail) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the useContext field and returns self for chaining.
    fn set_use_context(self, value: Vec<UsageContext>) -> Self;
    /// Adds an item to the useContext field and returns self for chaining.
    fn add_use_context(self, item: UsageContext) -> Self;
    /// Sets the jurisdiction field and returns self for chaining.
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the jurisdiction field and returns self for chaining.
    fn add_jurisdiction(self, item: CodeableConcept) -> Self;
    /// Sets the purpose field and returns self for chaining.
    fn set_purpose(self, value: String) -> Self;
    /// Sets the copyright field and returns self for chaining.
    fn set_copyright(self, value: String) -> Self;
    /// Sets the base field and returns self for chaining.
    fn set_base_definition(self, value: String) -> Self;
    /// Sets the parent field and returns self for chaining.
    fn set_parent(self, value: Vec<String>) -> Self;
    /// Adds an item to the parent field and returns self for chaining.
    fn add_parent(self, item: String) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: MessageSignificanceCategory) -> Self;
    /// Sets the focus field and returns self for chaining.
    fn set_focus(self, value: Vec<MessageDefinitionFocus>) -> Self;
    /// Adds an item to the focus field and returns self for chaining.
    fn add_focus(self, item: MessageDefinitionFocus) -> Self;
    /// Sets the responseRequired field and returns self for chaining.
    fn set_response_required(self, value: MessageheaderResponseRequest) -> Self;
    /// Sets the allowedResponse field and returns self for chaining.
    fn set_allowed_response(self, value: Vec<MessageDefinitionAllowedresponse>) -> Self;
    /// Adds an item to the allowedResponse field and returns self for chaining.
    fn add_allowed_response(self, item: MessageDefinitionAllowedresponse) -> Self;
    /// Sets the graph field and returns self for chaining.
    fn set_graph(self, value: Vec<String>) -> Self;
    /// Adds an item to the graph field and returns self for chaining.
    fn add_graph(self, item: String) -> Self;
}
/// MessageDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines the characteristics of a message that can be shared between systems, including the type of event that initiates the message, the content to be transmitted and what response(s), if any, are permitted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MessageDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MessageDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MessageDefinitionExistence: DomainResourceExistence {
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
    /// Returns true if the url field is present (Some).
    fn has_url(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the replaces field is not empty.
    fn has_replaces(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the experimental field is present (Some).
    fn has_experimental(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the publisher field is present (Some).
    fn has_publisher(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the use_context field is not empty.
    fn has_use_context(&self) -> bool;
    /// Returns true if the jurisdiction field is not empty.
    fn has_jurisdiction(&self) -> bool;
    /// Returns true if the purpose field is present (Some).
    fn has_purpose(&self) -> bool;
    /// Returns true if the copyright field is present (Some).
    fn has_copyright(&self) -> bool;
    /// Returns true if the base_definition field is present (Some).
    fn has_base_definition(&self) -> bool;
    /// Returns true if the parent field is not empty.
    fn has_parent(&self) -> bool;
    /// Returns true if the event field is present (Some).
    fn has_event(&self) -> bool;
    /// Returns true if the category field is present (Some).
    fn has_category(&self) -> bool;
    /// Returns true if the focus field is not empty.
    fn has_focus(&self) -> bool;
    /// Returns true if the response_required field is present (Some).
    fn has_response_required(&self) -> bool;
    /// Returns true if the allowed_response field is not empty.
    fn has_allowed_response(&self) -> bool;
    /// Returns true if the graph field is not empty.
    fn has_graph(&self) -> bool;
}
