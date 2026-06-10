use crate::bindings::examplescenario_actor_type::ExamplescenarioActorType;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ActorDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The ActorDefinition resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ActorDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ActorDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ActorDefinitionAccessors: DomainResourceAccessors {
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
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the experimental field.
    fn experimental(&self) -> Option<BooleanType>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
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
    /// Returns a reference to the copyrightLabel field.
    fn copyright_label(&self) -> Option<StringType>;
    /// Returns a reference to the type field.
    fn type_(&self) -> ExamplescenarioActorType;
    /// Returns a reference to the documentation field.
    fn documentation(&self) -> Option<StringType>;
    /// Returns a reference to the reference field.
    fn reference(&self) -> &[StringType];
    /// Returns a reference to the capabilities field.
    fn capabilities(&self) -> Option<StringType>;
    /// Returns a reference to the derivedFrom field.
    fn derived_from(&self) -> &[StringType];
}
/// ActorDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The ActorDefinition resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ActorDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ActorDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ActorDefinitionMutators: DomainResourceMutators {
    /// Create a new ActorDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::actor_definition::ActorDefinition;
    /// use rh_hl7_fhir_r5_core::traits::actor_definition::ActorDefinitionMutators;
    ///
    /// let resource = ActorDefinition::new();
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
    /// Sets the copyrightLabel field and returns self for chaining.
    fn set_copyright_label(self, value: String) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: ExamplescenarioActorType) -> Self;
    /// Sets the documentation field and returns self for chaining.
    fn set_documentation(self, value: String) -> Self;
    /// Sets the reference field and returns self for chaining.
    fn set_reference(self, value: Vec<String>) -> Self;
    /// Adds an item to the reference field and returns self for chaining.
    fn add_reference(self, item: String) -> Self;
    /// Sets the capabilities field and returns self for chaining.
    fn set_capabilities(self, value: String) -> Self;
    /// Sets the derivedFrom field and returns self for chaining.
    fn set_derived_from(self, value: Vec<String>) -> Self;
    /// Adds an item to the derivedFrom field and returns self for chaining.
    fn add_derived_from(self, item: String) -> Self;
}
/// ActorDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The ActorDefinition resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ActorDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ActorDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ActorDefinitionExistence: DomainResourceExistence {
    /// Returns true if the url field is present (Some).
    fn has_url(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the version_algorithm field is present (Some).
    fn has_version_algorithm(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
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
    /// Returns true if the copyright_label field is present (Some).
    fn has_copyright_label(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the documentation field is present (Some).
    fn has_documentation(&self) -> bool;
    /// Returns true if the reference field is not empty.
    fn has_reference(&self) -> bool;
    /// Returns true if the capabilities field is present (Some).
    fn has_capabilities(&self) -> bool;
    /// Returns true if the derived_from field is not empty.
    fn has_derived_from(&self) -> bool;
}
