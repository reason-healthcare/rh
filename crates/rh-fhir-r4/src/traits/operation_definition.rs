use crate::bindings::operation_kind::OperationKind;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::resource_types::ResourceTypes;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::operation_definition::OperationDefinitionOverload;
use crate::resources::operation_definition::OperationDefinitionParameter;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// OperationDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A formal computable definition of an operation (on the RESTful interface) or a named query (using the search interaction).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/OperationDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: OperationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait OperationDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> Option<StringType>;
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> StringType;
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the kind field.
    fn kind(&self) -> OperationKind;
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
    /// Returns a reference to the affectsState field.
    fn affects_state(&self) -> Option<BooleanType>;
    /// Returns a reference to the code field.
    fn code(&self) -> StringType;
    /// Returns a reference to the comment field.
    fn comment(&self) -> Option<StringType>;
    /// Returns a reference to the base field.
    fn base_definition(&self) -> Option<StringType>;
    /// Returns a reference to the resource field.
    fn resource(&self) -> &[ResourceTypes];
    /// Returns a reference to the system field.
    fn system(&self) -> BooleanType;
    /// Returns a reference to the type field.
    fn type_(&self) -> BooleanType;
    /// Returns a reference to the instance field.
    fn instance(&self) -> BooleanType;
    /// Returns a reference to the inputProfile field.
    fn input_profile(&self) -> Option<StringType>;
    /// Returns a reference to the outputProfile field.
    fn output_profile(&self) -> Option<StringType>;
    /// Returns a reference to the parameter field.
    fn parameter(&self) -> &[OperationDefinitionParameter];
    /// Returns a reference to the overload field.
    fn overload(&self) -> &[OperationDefinitionOverload];
}
/// OperationDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A formal computable definition of an operation (on the RESTful interface) or a named query (using the search interaction).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/OperationDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: OperationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait OperationDefinitionMutators: DomainResourceMutators {
    /// Create a new OperationDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::operation_definition::OperationDefinition;
    /// use hl7_fhir_r4_core::traits::operation_definition::OperationDefinitionMutators;
    ///
    /// let resource = OperationDefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the url field and returns self for chaining.
    fn set_url(self, value: String) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: String) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the kind field and returns self for chaining.
    fn set_kind(self, value: OperationKind) -> Self;
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
    /// Sets the affectsState field and returns self for chaining.
    fn set_affects_state(self, value: bool) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: String) -> Self;
    /// Sets the comment field and returns self for chaining.
    fn set_comment(self, value: String) -> Self;
    /// Sets the base field and returns self for chaining.
    fn set_base_definition(self, value: String) -> Self;
    /// Sets the resource field and returns self for chaining.
    fn set_resource(self, value: Vec<ResourceTypes>) -> Self;
    /// Adds an item to the resource field and returns self for chaining.
    fn add_resource(self, item: ResourceTypes) -> Self;
    /// Sets the system field and returns self for chaining.
    fn set_system(self, value: bool) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: bool) -> Self;
    /// Sets the instance field and returns self for chaining.
    fn set_instance(self, value: bool) -> Self;
    /// Sets the inputProfile field and returns self for chaining.
    fn set_input_profile(self, value: String) -> Self;
    /// Sets the outputProfile field and returns self for chaining.
    fn set_output_profile(self, value: String) -> Self;
    /// Sets the parameter field and returns self for chaining.
    fn set_parameter(self, value: Vec<OperationDefinitionParameter>) -> Self;
    /// Adds an item to the parameter field and returns self for chaining.
    fn add_parameter(self, item: OperationDefinitionParameter) -> Self;
    /// Sets the overload field and returns self for chaining.
    fn set_overload(self, value: Vec<OperationDefinitionOverload>) -> Self;
    /// Adds an item to the overload field and returns self for chaining.
    fn add_overload(self, item: OperationDefinitionOverload) -> Self;
}
/// OperationDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A formal computable definition of an operation (on the RESTful interface) or a named query (using the search interaction).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/OperationDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: OperationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait OperationDefinitionExistence: DomainResourceExistence {
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
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the kind field is present (Some).
    fn has_kind(&self) -> bool;
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
    /// Returns true if the affects_state field is present (Some).
    fn has_affects_state(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the comment field is present (Some).
    fn has_comment(&self) -> bool;
    /// Returns true if the base_definition field is present (Some).
    fn has_base_definition(&self) -> bool;
    /// Returns true if the resource field is not empty.
    fn has_resource(&self) -> bool;
    /// Returns true if the system field is present (Some).
    fn has_system(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the instance field is present (Some).
    fn has_instance(&self) -> bool;
    /// Returns true if the input_profile field is present (Some).
    fn has_input_profile(&self) -> bool;
    /// Returns true if the output_profile field is present (Some).
    fn has_output_profile(&self) -> bool;
    /// Returns true if the parameter field is not empty.
    fn has_parameter(&self) -> bool;
    /// Returns true if the overload field is not empty.
    fn has_overload(&self) -> bool;
}
