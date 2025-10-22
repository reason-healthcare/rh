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
use crate::resources::measure::MeasureGroup;
use crate::resources::measure::MeasureSupplementaldata;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Measure Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The Measure resource provides the definition of a quality measure.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Measure
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Measure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MeasureAccessors: DomainResourceAccessors {
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
    /// Returns a reference to the subtitle field.
    fn subtitle(&self) -> Option<StringType>;
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
    /// Returns a reference to the usage field.
    fn usage(&self) -> Option<StringType>;
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
    /// Returns a reference to the library field.
    fn library(&self) -> &[StringType];
    /// Returns a reference to the disclaimer field.
    fn disclaimer(&self) -> Option<StringType>;
    /// Returns a reference to the scoring field.
    fn scoring(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the compositeScoring field.
    fn composite_scoring(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the type field.
    fn type_(&self) -> &[CodeableConcept];
    /// Returns a reference to the riskAdjustment field.
    fn risk_adjustment(&self) -> Option<StringType>;
    /// Returns a reference to the rateAggregation field.
    fn rate_aggregation(&self) -> Option<StringType>;
    /// Returns a reference to the rationale field.
    fn rationale(&self) -> Option<StringType>;
    /// Returns a reference to the clinicalRecommendationStatement field.
    fn clinical_recommendation_statement(&self) -> Option<StringType>;
    /// Returns a reference to the improvementNotation field.
    fn improvement_notation(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the definition field.
    fn definition(&self) -> &[StringType];
    /// Returns a reference to the guidance field.
    fn guidance(&self) -> Option<StringType>;
    /// Returns a reference to the group field.
    fn group(&self) -> &[MeasureGroup];
    /// Returns a reference to the supplementalData field.
    fn supplemental_data(&self) -> &[MeasureSupplementaldata];
}
/// Measure Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The Measure resource provides the definition of a quality measure.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Measure
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Measure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MeasureMutators: DomainResourceMutators {
    /// Create a new Measure with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::measure::Measure;
    /// use hl7_fhir_r4_core::traits::measure::MeasureMutators;
    ///
    /// let resource = Measure::new();
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
    /// Sets the subtitle field and returns self for chaining.
    fn set_subtitle(self, value: String) -> Self;
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
    /// Sets the usage field and returns self for chaining.
    fn set_usage(self, value: String) -> Self;
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
    /// Sets the library field and returns self for chaining.
    fn set_library(self, value: Vec<String>) -> Self;
    /// Adds an item to the library field and returns self for chaining.
    fn add_library(self, item: String) -> Self;
    /// Sets the disclaimer field and returns self for chaining.
    fn set_disclaimer(self, value: String) -> Self;
    /// Sets the scoring field and returns self for chaining.
    fn set_scoring(self, value: CodeableConcept) -> Self;
    /// Sets the compositeScoring field and returns self for chaining.
    fn set_composite_scoring(self, value: CodeableConcept) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the type field and returns self for chaining.
    fn add_type_(self, item: CodeableConcept) -> Self;
    /// Sets the riskAdjustment field and returns self for chaining.
    fn set_risk_adjustment(self, value: String) -> Self;
    /// Sets the rateAggregation field and returns self for chaining.
    fn set_rate_aggregation(self, value: String) -> Self;
    /// Sets the rationale field and returns self for chaining.
    fn set_rationale(self, value: String) -> Self;
    /// Sets the clinicalRecommendationStatement field and returns self for chaining.
    fn set_clinical_recommendation_statement(self, value: String) -> Self;
    /// Sets the improvementNotation field and returns self for chaining.
    fn set_improvement_notation(self, value: CodeableConcept) -> Self;
    /// Sets the definition field and returns self for chaining.
    fn set_definition(self, value: Vec<String>) -> Self;
    /// Adds an item to the definition field and returns self for chaining.
    fn add_definition(self, item: String) -> Self;
    /// Sets the guidance field and returns self for chaining.
    fn set_guidance(self, value: String) -> Self;
    /// Sets the group field and returns self for chaining.
    fn set_group(self, value: Vec<MeasureGroup>) -> Self;
    /// Adds an item to the group field and returns self for chaining.
    fn add_group(self, item: MeasureGroup) -> Self;
    /// Sets the supplementalData field and returns self for chaining.
    fn set_supplemental_data(self, value: Vec<MeasureSupplementaldata>) -> Self;
    /// Adds an item to the supplementalData field and returns self for chaining.
    fn add_supplemental_data(self, item: MeasureSupplementaldata) -> Self;
}
/// Measure Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The Measure resource provides the definition of a quality measure.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Measure
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Measure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MeasureExistence: DomainResourceExistence {
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
    /// Returns true if the subtitle field is present (Some).
    fn has_subtitle(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the experimental field is present (Some).
    fn has_experimental(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
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
    /// Returns true if the usage field is present (Some).
    fn has_usage(&self) -> bool;
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
    /// Returns true if the library field is not empty.
    fn has_library(&self) -> bool;
    /// Returns true if the disclaimer field is present (Some).
    fn has_disclaimer(&self) -> bool;
    /// Returns true if the scoring field is present (Some).
    fn has_scoring(&self) -> bool;
    /// Returns true if the composite_scoring field is present (Some).
    fn has_composite_scoring(&self) -> bool;
    /// Returns true if the type_ field is not empty.
    fn has_type_(&self) -> bool;
    /// Returns true if the risk_adjustment field is present (Some).
    fn has_risk_adjustment(&self) -> bool;
    /// Returns true if the rate_aggregation field is present (Some).
    fn has_rate_aggregation(&self) -> bool;
    /// Returns true if the rationale field is present (Some).
    fn has_rationale(&self) -> bool;
    /// Returns true if the clinical_recommendation_statement field is present (Some).
    fn has_clinical_recommendation_statement(&self) -> bool;
    /// Returns true if the improvement_notation field is present (Some).
    fn has_improvement_notation(&self) -> bool;
    /// Returns true if the definition field is not empty.
    fn has_definition(&self) -> bool;
    /// Returns true if the guidance field is present (Some).
    fn has_guidance(&self) -> bool;
    /// Returns true if the group field is not empty.
    fn has_group(&self) -> bool;
    /// Returns true if the supplemental_data field is not empty.
    fn has_supplemental_data(&self) -> bool;
}
