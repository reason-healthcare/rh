use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::risk_evidence_synthesis::RiskEvidenceSynthesisCertainty;
use crate::resources::risk_evidence_synthesis::RiskEvidenceSynthesisRiskestimate;
use crate::resources::risk_evidence_synthesis::RiskEvidenceSynthesisSamplesize;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// RiskEvidenceSynthesis Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a population plus exposure state where the risk estimate is derived from a combination of research studies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RiskEvidenceSynthesis
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RiskEvidenceSynthesis
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RiskEvidenceSynthesisAccessors: DomainResourceAccessors {
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
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the publisher field.
    fn publisher(&self) -> Option<StringType>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactDetail];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the useContext field.
    fn use_context(&self) -> &[UsageContext];
    /// Returns a reference to the jurisdiction field.
    fn jurisdiction(&self) -> &[CodeableConcept];
    /// Returns a reference to the copyright field.
    fn copyright(&self) -> Option<StringType>;
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
    /// Returns a reference to the synthesisType field.
    fn synthesis_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the studyType field.
    fn study_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the population field.
    fn population(&self) -> Reference;
    /// Returns a reference to the exposure field.
    fn exposure(&self) -> Option<Reference>;
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> Reference;
    /// Returns a reference to the sampleSize field.
    fn sample_size(&self) -> Option<RiskEvidenceSynthesisSamplesize>;
    /// Returns a reference to the riskEstimate field.
    fn risk_estimate(&self) -> Option<RiskEvidenceSynthesisRiskestimate>;
    /// Returns a reference to the certainty field.
    fn certainty(&self) -> &[RiskEvidenceSynthesisCertainty];
}
/// RiskEvidenceSynthesis Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a population plus exposure state where the risk estimate is derived from a combination of research studies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RiskEvidenceSynthesis
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RiskEvidenceSynthesis
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RiskEvidenceSynthesisMutators: DomainResourceMutators {
    /// Create a new RiskEvidenceSynthesis with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::risk_evidence_synthesis::RiskEvidenceSynthesis;
    /// use hl7_fhir_r4_core::traits::risk_evidence_synthesis::RiskEvidenceSynthesisMutators;
    ///
    /// let resource = RiskEvidenceSynthesis::new();
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
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the useContext field and returns self for chaining.
    fn set_use_context(self, value: Vec<UsageContext>) -> Self;
    /// Adds an item to the useContext field and returns self for chaining.
    fn add_use_context(self, item: UsageContext) -> Self;
    /// Sets the jurisdiction field and returns self for chaining.
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the jurisdiction field and returns self for chaining.
    fn add_jurisdiction(self, item: CodeableConcept) -> Self;
    /// Sets the copyright field and returns self for chaining.
    fn set_copyright(self, value: String) -> Self;
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
    /// Sets the synthesisType field and returns self for chaining.
    fn set_synthesis_type(self, value: CodeableConcept) -> Self;
    /// Sets the studyType field and returns self for chaining.
    fn set_study_type(self, value: CodeableConcept) -> Self;
    /// Sets the population field and returns self for chaining.
    fn set_population(self, value: Reference) -> Self;
    /// Sets the exposure field and returns self for chaining.
    fn set_exposure(self, value: Reference) -> Self;
    /// Sets the outcome field and returns self for chaining.
    fn set_outcome(self, value: Reference) -> Self;
    /// Sets the sampleSize field and returns self for chaining.
    fn set_sample_size(self, value: RiskEvidenceSynthesisSamplesize) -> Self;
    /// Sets the riskEstimate field and returns self for chaining.
    fn set_risk_estimate(self, value: RiskEvidenceSynthesisRiskestimate) -> Self;
    /// Sets the certainty field and returns self for chaining.
    fn set_certainty(self, value: Vec<RiskEvidenceSynthesisCertainty>) -> Self;
    /// Adds an item to the certainty field and returns self for chaining.
    fn add_certainty(self, item: RiskEvidenceSynthesisCertainty) -> Self;
}
/// RiskEvidenceSynthesis Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a population plus exposure state where the risk estimate is derived from a combination of research studies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RiskEvidenceSynthesis
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RiskEvidenceSynthesis
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RiskEvidenceSynthesisExistence: DomainResourceExistence {
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
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the publisher field is present (Some).
    fn has_publisher(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the use_context field is not empty.
    fn has_use_context(&self) -> bool;
    /// Returns true if the jurisdiction field is not empty.
    fn has_jurisdiction(&self) -> bool;
    /// Returns true if the copyright field is present (Some).
    fn has_copyright(&self) -> bool;
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
    /// Returns true if the synthesis_type field is present (Some).
    fn has_synthesis_type(&self) -> bool;
    /// Returns true if the study_type field is present (Some).
    fn has_study_type(&self) -> bool;
    /// Returns true if the population field is present (Some).
    fn has_population(&self) -> bool;
    /// Returns true if the exposure field is present (Some).
    fn has_exposure(&self) -> bool;
    /// Returns true if the outcome field is present (Some).
    fn has_outcome(&self) -> bool;
    /// Returns true if the sample_size field is present (Some).
    fn has_sample_size(&self) -> bool;
    /// Returns true if the risk_estimate field is present (Some).
    fn has_risk_estimate(&self) -> bool;
    /// Returns true if the certainty field is not empty.
    fn has_certainty(&self) -> bool;
}
