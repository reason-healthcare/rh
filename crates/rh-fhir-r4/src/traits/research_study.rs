use crate::bindings::research_study_status::ResearchStudyStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::primitives::string::StringType;
use crate::resources::research_study::ResearchStudyArm;
use crate::resources::research_study::ResearchStudyObjective;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ResearchStudy Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A process where a researcher or organization plans and then executes a series of steps intended to increase the field of healthcare-related knowledge.  This includes studies of safety, efficacy, comparative effectiveness and other information about medications, devices, therapies and other interventional and investigative techniques.  A ResearchStudy involves the gathering of information about human or animal subjects.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ResearchStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ResearchStudyAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the protocol field.
    fn protocol(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> ResearchStudyStatus;
    /// Returns a reference to the primaryPurposeType field.
    fn primary_purpose_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the phase field.
    fn phase(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the focus field.
    fn focus(&self) -> &[CodeableConcept];
    /// Returns a reference to the condition field.
    fn condition(&self) -> &[CodeableConcept];
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactDetail];
    /// Returns a reference to the relatedArtifact field.
    fn related_artifact(&self) -> &[RelatedArtifact];
    /// Returns a reference to the keyword field.
    fn keyword(&self) -> &[CodeableConcept];
    /// Returns a reference to the location field.
    fn location(&self) -> &[CodeableConcept];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the enrollment field.
    fn enrollment(&self) -> &[Reference];
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the sponsor field.
    fn sponsor(&self) -> Option<Reference>;
    /// Returns a reference to the principalInvestigator field.
    fn principal_investigator(&self) -> Option<Reference>;
    /// Returns a reference to the site field.
    fn site(&self) -> &[Reference];
    /// Returns a reference to the reasonStopped field.
    fn reason_stopped(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the arm field.
    fn arm(&self) -> &[ResearchStudyArm];
    /// Returns a reference to the objective field.
    fn objective(&self) -> &[ResearchStudyObjective];
}
/// ResearchStudy Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A process where a researcher or organization plans and then executes a series of steps intended to increase the field of healthcare-related knowledge.  This includes studies of safety, efficacy, comparative effectiveness and other information about medications, devices, therapies and other interventional and investigative techniques.  A ResearchStudy involves the gathering of information about human or animal subjects.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
/// - Version: 4.0.1
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
    /// use hl7_fhir_r4_core::resources::research_study::ResearchStudy;
    /// use hl7_fhir_r4_core::traits::research_study::ResearchStudyMutators;
    ///
    /// let resource = ResearchStudy::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the protocol field and returns self for chaining.
    fn set_protocol(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the protocol field and returns self for chaining.
    fn add_protocol(self, item: Reference) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ResearchStudyStatus) -> Self;
    /// Sets the primaryPurposeType field and returns self for chaining.
    fn set_primary_purpose_type(self, value: CodeableConcept) -> Self;
    /// Sets the phase field and returns self for chaining.
    fn set_phase(self, value: CodeableConcept) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the focus field and returns self for chaining.
    fn set_focus(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the focus field and returns self for chaining.
    fn add_focus(self, item: CodeableConcept) -> Self;
    /// Sets the condition field and returns self for chaining.
    fn set_condition(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the condition field and returns self for chaining.
    fn add_condition(self, item: CodeableConcept) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactDetail) -> Self;
    /// Sets the relatedArtifact field and returns self for chaining.
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self;
    /// Adds an item to the relatedArtifact field and returns self for chaining.
    fn add_related_artifact(self, item: RelatedArtifact) -> Self;
    /// Sets the keyword field and returns self for chaining.
    fn set_keyword(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the keyword field and returns self for chaining.
    fn add_keyword(self, item: CodeableConcept) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the location field and returns self for chaining.
    fn add_location(self, item: CodeableConcept) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the enrollment field and returns self for chaining.
    fn set_enrollment(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the enrollment field and returns self for chaining.
    fn add_enrollment(self, item: Reference) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the sponsor field and returns self for chaining.
    fn set_sponsor(self, value: Reference) -> Self;
    /// Sets the principalInvestigator field and returns self for chaining.
    fn set_principal_investigator(self, value: Reference) -> Self;
    /// Sets the site field and returns self for chaining.
    fn set_site(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the site field and returns self for chaining.
    fn add_site(self, item: Reference) -> Self;
    /// Sets the reasonStopped field and returns self for chaining.
    fn set_reason_stopped(self, value: CodeableConcept) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the arm field and returns self for chaining.
    fn set_arm(self, value: Vec<ResearchStudyArm>) -> Self;
    /// Adds an item to the arm field and returns self for chaining.
    fn add_arm(self, item: ResearchStudyArm) -> Self;
    /// Sets the objective field and returns self for chaining.
    fn set_objective(self, value: Vec<ResearchStudyObjective>) -> Self;
    /// Adds an item to the objective field and returns self for chaining.
    fn add_objective(self, item: ResearchStudyObjective) -> Self;
}
/// ResearchStudy Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A process where a researcher or organization plans and then executes a series of steps intended to increase the field of healthcare-related knowledge.  This includes studies of safety, efficacy, comparative effectiveness and other information about medications, devices, therapies and other interventional and investigative techniques.  A ResearchStudy involves the gathering of information about human or animal subjects.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
/// - Version: 4.0.1
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
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the protocol field is not empty.
    fn has_protocol(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the primary_purpose_type field is present (Some).
    fn has_primary_purpose_type(&self) -> bool;
    /// Returns true if the phase field is present (Some).
    fn has_phase(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the focus field is not empty.
    fn has_focus(&self) -> bool;
    /// Returns true if the condition field is not empty.
    fn has_condition(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the related_artifact field is not empty.
    fn has_related_artifact(&self) -> bool;
    /// Returns true if the keyword field is not empty.
    fn has_keyword(&self) -> bool;
    /// Returns true if the location field is not empty.
    fn has_location(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the enrollment field is not empty.
    fn has_enrollment(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the sponsor field is present (Some).
    fn has_sponsor(&self) -> bool;
    /// Returns true if the principal_investigator field is present (Some).
    fn has_principal_investigator(&self) -> bool;
    /// Returns true if the site field is not empty.
    fn has_site(&self) -> bool;
    /// Returns true if the reason_stopped field is present (Some).
    fn has_reason_stopped(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the arm field is not empty.
    fn has_arm(&self) -> bool;
    /// Returns true if the objective field is not empty.
    fn has_objective(&self) -> bool;
}
