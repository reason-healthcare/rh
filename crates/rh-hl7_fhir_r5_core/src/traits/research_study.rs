use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::research_study::ResearchStudyAssociatedparty;
use crate::resources::research_study::ResearchStudyComparisongroup;
use crate::resources::research_study::ResearchStudyLabel;
use crate::resources::research_study::ResearchStudyObjective;
use crate::resources::research_study::ResearchStudyOutcomemeasure;
use crate::resources::research_study::ResearchStudyProgressstatus;
use crate::resources::research_study::ResearchStudyRecruitment;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ResearchStudy Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A scientific study of nature that sometimes includes processes involved in health and disease. For example, clinical trials are research studies that involve people. These studies may be related to new ways to screen, prevent, diagnose, and treat disease. They may also study certain outcomes and certain groups of people by looking at data collected in the past or future.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ResearchStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ResearchStudyAccessors: DomainResourceAccessors {
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
    /// Returns a reference to the label field.
    fn label(&self) -> &[ResearchStudyLabel];
    /// Returns a reference to the protocol field.
    fn protocol(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the relatedArtifact field.
    fn related_artifact(&self) -> &[RelatedArtifact];
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the primaryPurposeType field.
    fn primary_purpose_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the phase field.
    fn phase(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the studyDesign field.
    fn study_design(&self) -> &[CodeableConcept];
    /// Returns a reference to the focus field.
    fn focus(&self) -> &[CodeableReference];
    /// Returns a reference to the condition field.
    fn condition(&self) -> &[CodeableConcept];
    /// Returns a reference to the keyword field.
    fn keyword(&self) -> &[CodeableConcept];
    /// Returns a reference to the region field.
    fn region(&self) -> &[CodeableConcept];
    /// Returns a reference to the descriptionSummary field.
    fn description_summary(&self) -> Option<StringType>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the site field.
    fn site(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the classifier field.
    fn classifier(&self) -> &[CodeableConcept];
    /// Returns a reference to the associatedParty field.
    fn associated_party(&self) -> &[ResearchStudyAssociatedparty];
    /// Returns a reference to the progressStatus field.
    fn progress_status(&self) -> &[ResearchStudyProgressstatus];
    /// Returns a reference to the whyStopped field.
    fn why_stopped(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the recruitment field.
    fn recruitment(&self) -> Option<ResearchStudyRecruitment>;
    /// Returns a reference to the comparisonGroup field.
    fn comparison_group(&self) -> &[ResearchStudyComparisongroup];
    /// Returns a reference to the objective field.
    fn objective(&self) -> &[ResearchStudyObjective];
    /// Returns a reference to the outcomeMeasure field.
    fn outcome_measure(&self) -> &[ResearchStudyOutcomemeasure];
    /// Returns a reference to the result field.
    fn result(&self) -> &[Reference];
}
/// ResearchStudy Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A scientific study of nature that sometimes includes processes involved in health and disease. For example, clinical trials are research studies that involve people. These studies may be related to new ways to screen, prevent, diagnose, and treat disease. They may also study certain outcomes and certain groups of people by looking at data collected in the past or future.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ResearchStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ResearchStudyMutators: DomainResourceMutators {
    /// Create a new ResearchStudy with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::research_study::ResearchStudy;
    /// use rh_hl7_fhir_r5_core::traits::research_study::ResearchStudyMutators;
    ///
    /// let resource = ResearchStudy::new();
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
    /// Sets the label field and returns self for chaining.
    fn set_label(self, value: Vec<ResearchStudyLabel>) -> Self;
    /// Adds an item to the label field and returns self for chaining.
    fn add_label(self, item: ResearchStudyLabel) -> Self;
    /// Sets the protocol field and returns self for chaining.
    fn set_protocol(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the protocol field and returns self for chaining.
    fn add_protocol(self, item: Reference) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the relatedArtifact field and returns self for chaining.
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self;
    /// Adds an item to the relatedArtifact field and returns self for chaining.
    fn add_related_artifact(self, item: RelatedArtifact) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the primaryPurposeType field and returns self for chaining.
    fn set_primary_purpose_type(self, value: CodeableConcept) -> Self;
    /// Sets the phase field and returns self for chaining.
    fn set_phase(self, value: CodeableConcept) -> Self;
    /// Sets the studyDesign field and returns self for chaining.
    fn set_study_design(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the studyDesign field and returns self for chaining.
    fn add_study_design(self, item: CodeableConcept) -> Self;
    /// Sets the focus field and returns self for chaining.
    fn set_focus(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the focus field and returns self for chaining.
    fn add_focus(self, item: CodeableReference) -> Self;
    /// Sets the condition field and returns self for chaining.
    fn set_condition(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the condition field and returns self for chaining.
    fn add_condition(self, item: CodeableConcept) -> Self;
    /// Sets the keyword field and returns self for chaining.
    fn set_keyword(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the keyword field and returns self for chaining.
    fn add_keyword(self, item: CodeableConcept) -> Self;
    /// Sets the region field and returns self for chaining.
    fn set_region(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the region field and returns self for chaining.
    fn add_region(self, item: CodeableConcept) -> Self;
    /// Sets the descriptionSummary field and returns self for chaining.
    fn set_description_summary(self, value: String) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the site field and returns self for chaining.
    fn set_site(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the site field and returns self for chaining.
    fn add_site(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the classifier field and returns self for chaining.
    fn set_classifier(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the classifier field and returns self for chaining.
    fn add_classifier(self, item: CodeableConcept) -> Self;
    /// Sets the associatedParty field and returns self for chaining.
    fn set_associated_party(self, value: Vec<ResearchStudyAssociatedparty>) -> Self;
    /// Adds an item to the associatedParty field and returns self for chaining.
    fn add_associated_party(self, item: ResearchStudyAssociatedparty) -> Self;
    /// Sets the progressStatus field and returns self for chaining.
    fn set_progress_status(self, value: Vec<ResearchStudyProgressstatus>) -> Self;
    /// Adds an item to the progressStatus field and returns self for chaining.
    fn add_progress_status(self, item: ResearchStudyProgressstatus) -> Self;
    /// Sets the whyStopped field and returns self for chaining.
    fn set_why_stopped(self, value: CodeableConcept) -> Self;
    /// Sets the recruitment field and returns self for chaining.
    fn set_recruitment(self, value: ResearchStudyRecruitment) -> Self;
    /// Sets the comparisonGroup field and returns self for chaining.
    fn set_comparison_group(self, value: Vec<ResearchStudyComparisongroup>) -> Self;
    /// Adds an item to the comparisonGroup field and returns self for chaining.
    fn add_comparison_group(self, item: ResearchStudyComparisongroup) -> Self;
    /// Sets the objective field and returns self for chaining.
    fn set_objective(self, value: Vec<ResearchStudyObjective>) -> Self;
    /// Adds an item to the objective field and returns self for chaining.
    fn add_objective(self, item: ResearchStudyObjective) -> Self;
    /// Sets the outcomeMeasure field and returns self for chaining.
    fn set_outcome_measure(self, value: Vec<ResearchStudyOutcomemeasure>) -> Self;
    /// Adds an item to the outcomeMeasure field and returns self for chaining.
    fn add_outcome_measure(self, item: ResearchStudyOutcomemeasure) -> Self;
    /// Sets the result field and returns self for chaining.
    fn set_result(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the result field and returns self for chaining.
    fn add_result(self, item: Reference) -> Self;
}
/// ResearchStudy Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A scientific study of nature that sometimes includes processes involved in health and disease. For example, clinical trials are research studies that involve people. These studies may be related to new ways to screen, prevent, diagnose, and treat disease. They may also study certain outcomes and certain groups of people by looking at data collected in the past or future.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ResearchStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ResearchStudyExistence: DomainResourceExistence {
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
    /// Returns true if the label field is not empty.
    fn has_label(&self) -> bool;
    /// Returns true if the protocol field is not empty.
    fn has_protocol(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the related_artifact field is not empty.
    fn has_related_artifact(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the primary_purpose_type field is present (Some).
    fn has_primary_purpose_type(&self) -> bool;
    /// Returns true if the phase field is present (Some).
    fn has_phase(&self) -> bool;
    /// Returns true if the study_design field is not empty.
    fn has_study_design(&self) -> bool;
    /// Returns true if the focus field is not empty.
    fn has_focus(&self) -> bool;
    /// Returns true if the condition field is not empty.
    fn has_condition(&self) -> bool;
    /// Returns true if the keyword field is not empty.
    fn has_keyword(&self) -> bool;
    /// Returns true if the region field is not empty.
    fn has_region(&self) -> bool;
    /// Returns true if the description_summary field is present (Some).
    fn has_description_summary(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the site field is not empty.
    fn has_site(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the classifier field is not empty.
    fn has_classifier(&self) -> bool;
    /// Returns true if the associated_party field is not empty.
    fn has_associated_party(&self) -> bool;
    /// Returns true if the progress_status field is not empty.
    fn has_progress_status(&self) -> bool;
    /// Returns true if the why_stopped field is present (Some).
    fn has_why_stopped(&self) -> bool;
    /// Returns true if the recruitment field is present (Some).
    fn has_recruitment(&self) -> bool;
    /// Returns true if the comparison_group field is not empty.
    fn has_comparison_group(&self) -> bool;
    /// Returns true if the objective field is not empty.
    fn has_objective(&self) -> bool;
    /// Returns true if the outcome_measure field is not empty.
    fn has_outcome_measure(&self) -> bool;
    /// Returns true if the result field is not empty.
    fn has_result(&self) -> bool;
}
