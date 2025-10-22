use crate::bindings::fhir_version::FHIRVersion;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::structure_definition_kind::StructureDefinitionKind;
use crate::bindings::type_derivation_rule::TypeDerivationRule;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::structure_definition::StructureDefinitionContext;
use crate::resources::structure_definition::StructureDefinitionDifferential;
use crate::resources::structure_definition::StructureDefinitionMapping;
use crate::resources::structure_definition::StructureDefinitionSnapshot;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// StructureDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A definition of a FHIR structure. This resource is used to describe the underlying resources, data types defined in FHIR, and also for describing extensions and constraints on resources and data types.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/StructureDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: StructureDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait StructureDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> StringType;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> StringType;
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
    /// Returns a reference to the keyword field.
    fn keyword(&self) -> &[Coding];
    /// Returns a reference to the fhirVersion field.
    fn fhir_version(&self) -> Option<FHIRVersion>;
    /// Returns a reference to the mapping field.
    fn mapping(&self) -> &[StructureDefinitionMapping];
    /// Returns a reference to the kind field.
    fn kind(&self) -> StructureDefinitionKind;
    /// Returns a reference to the abstract field.
    fn abstract_(&self) -> BooleanType;
    /// Returns a reference to the context field.
    fn context(&self) -> &[StructureDefinitionContext];
    /// Returns a reference to the contextInvariant field.
    fn context_invariant(&self) -> &[StringType];
    /// Returns a reference to the type field.
    fn type_(&self) -> StringType;
    /// Returns a reference to the baseDefinition field.
    fn base_definition(&self) -> Option<StringType>;
    /// Returns a reference to the derivation field.
    fn derivation(&self) -> Option<TypeDerivationRule>;
    /// Returns a reference to the snapshot field.
    fn snapshot(&self) -> Option<StructureDefinitionSnapshot>;
    /// Returns a reference to the differential field.
    fn differential(&self) -> Option<StructureDefinitionDifferential>;
}
/// StructureDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A definition of a FHIR structure. This resource is used to describe the underlying resources, data types defined in FHIR, and also for describing extensions and constraints on resources and data types.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/StructureDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: StructureDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait StructureDefinitionMutators: DomainResourceMutators {
    /// Create a new StructureDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::structure_definition::StructureDefinition;
    /// use hl7_fhir_r4_core::traits::structure_definition::StructureDefinitionMutators;
    ///
    /// let resource = StructureDefinition::new();
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
    /// Sets the keyword field and returns self for chaining.
    fn set_keyword(self, value: Vec<Coding>) -> Self;
    /// Adds an item to the keyword field and returns self for chaining.
    fn add_keyword(self, item: Coding) -> Self;
    /// Sets the fhirVersion field and returns self for chaining.
    fn set_fhir_version(self, value: FHIRVersion) -> Self;
    /// Sets the mapping field and returns self for chaining.
    fn set_mapping(self, value: Vec<StructureDefinitionMapping>) -> Self;
    /// Adds an item to the mapping field and returns self for chaining.
    fn add_mapping(self, item: StructureDefinitionMapping) -> Self;
    /// Sets the kind field and returns self for chaining.
    fn set_kind(self, value: StructureDefinitionKind) -> Self;
    /// Sets the abstract field and returns self for chaining.
    fn set_abstract_(self, value: bool) -> Self;
    /// Sets the context field and returns self for chaining.
    fn set_context(self, value: Vec<StructureDefinitionContext>) -> Self;
    /// Adds an item to the context field and returns self for chaining.
    fn add_context(self, item: StructureDefinitionContext) -> Self;
    /// Sets the contextInvariant field and returns self for chaining.
    fn set_context_invariant(self, value: Vec<String>) -> Self;
    /// Adds an item to the contextInvariant field and returns self for chaining.
    fn add_context_invariant(self, item: String) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: String) -> Self;
    /// Sets the baseDefinition field and returns self for chaining.
    fn set_base_definition(self, value: String) -> Self;
    /// Sets the derivation field and returns self for chaining.
    fn set_derivation(self, value: TypeDerivationRule) -> Self;
    /// Sets the snapshot field and returns self for chaining.
    fn set_snapshot(self, value: StructureDefinitionSnapshot) -> Self;
    /// Sets the differential field and returns self for chaining.
    fn set_differential(self, value: StructureDefinitionDifferential) -> Self;
}
/// StructureDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A definition of a FHIR structure. This resource is used to describe the underlying resources, data types defined in FHIR, and also for describing extensions and constraints on resources and data types.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/StructureDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: StructureDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait StructureDefinitionExistence: DomainResourceExistence {
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
    /// Returns true if the keyword field is not empty.
    fn has_keyword(&self) -> bool;
    /// Returns true if the fhir_version field is present (Some).
    fn has_fhir_version(&self) -> bool;
    /// Returns true if the mapping field is not empty.
    fn has_mapping(&self) -> bool;
    /// Returns true if the kind field is present (Some).
    fn has_kind(&self) -> bool;
    /// Returns true if the abstract_ field is present (Some).
    fn has_abstract_(&self) -> bool;
    /// Returns true if the context field is not empty.
    fn has_context(&self) -> bool;
    /// Returns true if the context_invariant field is not empty.
    fn has_context_invariant(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the base_definition field is present (Some).
    fn has_base_definition(&self) -> bool;
    /// Returns true if the derivation field is present (Some).
    fn has_derivation(&self) -> bool;
    /// Returns true if the snapshot field is present (Some).
    fn has_snapshot(&self) -> bool;
    /// Returns true if the differential field is present (Some).
    fn has_differential(&self) -> bool;
}
