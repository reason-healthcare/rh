use crate::bindings::namingsystem_type::NamingsystemType;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::naming_system::NamingSystemUniqueid;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// NamingSystem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc.  Represents a "System" used within the Identifier and Coding data types.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NamingSystem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NamingSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NamingSystemAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> Option<StringType>;
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
    /// Returns a reference to the kind field.
    fn kind(&self) -> NamingsystemType;
    /// Returns a reference to the experimental field.
    fn experimental(&self) -> Option<BooleanType>;
    /// Returns a reference to the date field.
    fn date(&self) -> DateTimeType;
    /// Returns a reference to the publisher field.
    fn publisher(&self) -> Option<StringType>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactDetail];
    /// Returns a reference to the responsible field.
    fn responsible(&self) -> Option<StringType>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
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
    /// Returns a reference to the approvalDate field.
    fn approval_date(&self) -> Option<DateType>;
    /// Returns a reference to the lastReviewDate field.
    fn last_review_date(&self) -> Option<DateType>;
    /// Returns a reference to the effectivePeriod field.
    fn effective_period(&self) -> Option<Period>;
    /// Returns a reference to the topic field.
    fn topic(&self) -> &[CodeableConcept];
    /// Returns a reference to the author field.
    fn author(&self) -> &[ContactDetail];
    /// Returns a reference to the editor field.
    fn editor(&self) -> &[ContactDetail];
    /// Returns a reference to the reviewer field.
    fn reviewer(&self) -> &[ContactDetail];
    /// Returns a reference to the endorser field.
    fn endorser(&self) -> &[ContactDetail];
    /// Returns a reference to the relatedArtifact field.
    fn related_artifact(&self) -> &[RelatedArtifact];
    /// Returns a reference to the usage field.
    fn usage(&self) -> Option<StringType>;
    /// Returns a reference to the uniqueId field.
    fn unique_id(&self) -> &[NamingSystemUniqueid];
}
/// NamingSystem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc.  Represents a "System" used within the Identifier and Coding data types.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NamingSystem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NamingSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NamingSystemMutators: DomainResourceMutators {
    /// Create a new NamingSystem with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::naming_system::NamingSystem;
    /// use rh_hl7_fhir_r5_core::traits::naming_system::NamingSystemMutators;
    ///
    /// let resource = NamingSystem::new();
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
    /// Sets the kind field and returns self for chaining.
    fn set_kind(self, value: NamingsystemType) -> Self;
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
    /// Sets the responsible field and returns self for chaining.
    fn set_responsible(self, value: String) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
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
    /// Sets the approvalDate field and returns self for chaining.
    fn set_approval_date(self, value: String) -> Self;
    /// Sets the lastReviewDate field and returns self for chaining.
    fn set_last_review_date(self, value: String) -> Self;
    /// Sets the effectivePeriod field and returns self for chaining.
    fn set_effective_period(self, value: Period) -> Self;
    /// Sets the topic field and returns self for chaining.
    fn set_topic(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the topic field and returns self for chaining.
    fn add_topic(self, item: CodeableConcept) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the author field and returns self for chaining.
    fn add_author(self, item: ContactDetail) -> Self;
    /// Sets the editor field and returns self for chaining.
    fn set_editor(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the editor field and returns self for chaining.
    fn add_editor(self, item: ContactDetail) -> Self;
    /// Sets the reviewer field and returns self for chaining.
    fn set_reviewer(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the reviewer field and returns self for chaining.
    fn add_reviewer(self, item: ContactDetail) -> Self;
    /// Sets the endorser field and returns self for chaining.
    fn set_endorser(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the endorser field and returns self for chaining.
    fn add_endorser(self, item: ContactDetail) -> Self;
    /// Sets the relatedArtifact field and returns self for chaining.
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self;
    /// Adds an item to the relatedArtifact field and returns self for chaining.
    fn add_related_artifact(self, item: RelatedArtifact) -> Self;
    /// Sets the usage field and returns self for chaining.
    fn set_usage(self, value: String) -> Self;
    /// Sets the uniqueId field and returns self for chaining.
    fn set_unique_id(self, value: Vec<NamingSystemUniqueid>) -> Self;
    /// Adds an item to the uniqueId field and returns self for chaining.
    fn add_unique_id(self, item: NamingSystemUniqueid) -> Self;
}
/// NamingSystem Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc.  Represents a "System" used within the Identifier and Coding data types.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NamingSystem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NamingSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NamingSystemExistence: DomainResourceExistence {
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
    /// Returns true if the responsible field is present (Some).
    fn has_responsible(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
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
    /// Returns true if the approval_date field is present (Some).
    fn has_approval_date(&self) -> bool;
    /// Returns true if the last_review_date field is present (Some).
    fn has_last_review_date(&self) -> bool;
    /// Returns true if the effective_period field is present (Some).
    fn has_effective_period(&self) -> bool;
    /// Returns true if the topic field is not empty.
    fn has_topic(&self) -> bool;
    /// Returns true if the author field is not empty.
    fn has_author(&self) -> bool;
    /// Returns true if the editor field is not empty.
    fn has_editor(&self) -> bool;
    /// Returns true if the reviewer field is not empty.
    fn has_reviewer(&self) -> bool;
    /// Returns true if the endorser field is not empty.
    fn has_endorser(&self) -> bool;
    /// Returns true if the related_artifact field is not empty.
    fn has_related_artifact(&self) -> bool;
    /// Returns true if the usage field is present (Some).
    fn has_usage(&self) -> bool;
    /// Returns true if the unique_id field is not empty.
    fn has_unique_id(&self) -> bool;
}
