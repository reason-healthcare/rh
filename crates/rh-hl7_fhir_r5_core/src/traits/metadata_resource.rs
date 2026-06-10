use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::period::Period;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::primitives::date::DateType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MetadataResource Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Common Interface declaration for conformance and knowledge artifact resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MetadataResource
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MetadataResource
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MetadataResourceAccessors: DomainResourceAccessors {
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
}
/// MetadataResource Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Common Interface declaration for conformance and knowledge artifact resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MetadataResource
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MetadataResource
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MetadataResourceMutators: DomainResourceMutators {
    /// Create a new MetadataResource with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::metadata_resource::MetadataResource;
    /// use rh_hl7_fhir_r5_core::traits::metadata_resource::MetadataResourceMutators;
    ///
    /// let resource = MetadataResource::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
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
}
/// MetadataResource Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Common Interface declaration for conformance and knowledge artifact resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MetadataResource
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MetadataResource
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MetadataResourceExistence: DomainResourceExistence {
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
}
