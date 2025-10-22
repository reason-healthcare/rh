use crate::bindings::research_study_status::ResearchStudyStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ResearchStudy
///
/// A process where a researcher or organization plans and then executes a series of steps intended to increase the field of healthcare-related knowledge.  This includes studies of safety, efficacy, comparative effectiveness and other information about medications, devices, therapies and other interventional and investigative techniques.  A ResearchStudy involves the gathering of information about human or animal subjects.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ResearchStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchStudy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for study
    pub identifier: Option<Vec<Identifier>>,
    /// Name for this study
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Steps followed in executing study
    pub protocol: Option<Vec<Reference>>,
    /// Part of larger study
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// active | administratively-completed | approved | closed-to-accrual | closed-to-accrual-and-intervention | completed | disapproved | in-review | temporarily-closed-to-accrual | temporarily-closed-to-accrual-and-intervention | withdrawn
    pub status: ResearchStudyStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// treatment | prevention | diagnostic | supportive-care | screening | health-services-research | basic-science | device-feasibility
    ///
    /// Binding: extensible (Codes for the main intent of the study.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-prim-purp-type
    #[serde(rename = "primaryPurposeType")]
    pub primary_purpose_type: Option<CodeableConcept>,
    /// n-a | early-phase-1 | phase-1 | phase-1-phase-2 | phase-2 | phase-2-phase-3 | phase-3 | phase-4
    ///
    /// Binding: example (Codes for the stage in the progression of a therapy from initial experimental use in humans in clinical trials to post-market evaluation.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-phase
    pub phase: Option<CodeableConcept>,
    /// Classifications for the study
    ///
    /// Binding: example (Codes that describe the type of research study.  E.g. Study phase, Interventional/Observational, blinding type, etc.)
    pub category: Option<Vec<CodeableConcept>>,
    /// Drugs, devices, etc. under study
    ///
    /// Binding: example (Codes for medications, devices and other interventions.)
    pub focus: Option<Vec<CodeableConcept>>,
    /// Condition being studied
    ///
    /// Binding: example (Identification of the condition or diagnosis.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    pub condition: Option<Vec<CodeableConcept>>,
    /// Contact details for the study
    pub contact: Option<Vec<ContactDetail>>,
    /// References and dependencies
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Used to search for the study
    ///
    /// Binding: example (Words associated with the study that may be useful in discovery.)
    pub keyword: Option<Vec<CodeableConcept>>,
    /// Geographic region(s) for study
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub location: Option<Vec<CodeableConcept>>,
    /// What this is study doing
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Inclusion & exclusion criteria
    pub enrollment: Option<Vec<Reference>>,
    /// When the study began and ended
    pub period: Option<Period>,
    /// Organization that initiates and is legally responsible for the study
    pub sponsor: Option<Reference>,
    /// Researcher who oversees multiple aspects of the study
    #[serde(rename = "principalInvestigator")]
    pub principal_investigator: Option<Reference>,
    /// Facility where study activities are conducted
    pub site: Option<Vec<Reference>>,
    /// accrual-goal-met | closed-due-to-toxicity | closed-due-to-lack-of-study-progress | temporarily-closed-per-study-design
    ///
    /// Binding: example (Codes for why the study ended prematurely.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-reason-stopped
    #[serde(rename = "reasonStopped")]
    pub reason_stopped: Option<CodeableConcept>,
    /// Comments made about the study
    pub note: Option<Vec<Annotation>>,
    /// Defined path through the study for a subject
    pub arm: Option<Vec<ResearchStudyArm>>,
    /// A goal for the study
    pub objective: Option<Vec<ResearchStudyObjective>>,
}
/// ResearchStudy nested structure for the 'objective' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchStudyObjective {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Label for the objective
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// primary | secondary | exploratory
    ///
    /// Binding: preferred (Codes for the kind of study objective.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-objective-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
}
/// ResearchStudy nested structure for the 'arm' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchStudyArm {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Label for study arm
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Categorization of study arm
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Short explanation of study path
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}

impl Default for ResearchStudy {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            protocol: Default::default(),
            part_of: Default::default(),
            status: ResearchStudyStatus::default(),
            _status: Default::default(),
            primary_purpose_type: Default::default(),
            phase: Default::default(),
            category: Default::default(),
            focus: Default::default(),
            condition: Default::default(),
            contact: Default::default(),
            related_artifact: Default::default(),
            keyword: Default::default(),
            location: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            enrollment: Default::default(),
            period: Default::default(),
            sponsor: Default::default(),
            principal_investigator: Default::default(),
            site: Default::default(),
            reason_stopped: Default::default(),
            note: Default::default(),
            arm: Default::default(),
            objective: Default::default(),
        }
    }
}

impl Default for ResearchStudyObjective {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            type_: Default::default(),
        }
    }
}

impl Default for ResearchStudyArm {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: StringType::default(),
            _name: Default::default(),
            type_: Default::default(),
            description: Default::default(),
            _description: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ResearchStudy {
    fn id(&self) -> Option<String> {
        self.base.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for ResearchStudy {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for ResearchStudy {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for ResearchStudy {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for ResearchStudy {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: crate::datatypes::narrative::Narrative) -> Self {
        let mut resource = self.clone();
        resource.base.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<crate::resources::resource::Resource>) -> Self {
        let mut resource = self.clone();
        resource.base.contained = Some(value);
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .contained
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = Some(value);
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for ResearchStudy {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
}

impl crate::traits::research_study::ResearchStudyAccessors for ResearchStudy {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn protocol(&self) -> &[Reference] {
        self.protocol.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ResearchStudyStatus {
        self.status.clone()
    }
    fn primary_purpose_type(&self) -> Option<CodeableConcept> {
        self.primary_purpose_type.clone()
    }
    fn phase(&self) -> Option<CodeableConcept> {
        self.phase.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn focus(&self) -> &[CodeableConcept] {
        self.focus.as_deref().unwrap_or(&[])
    }
    fn condition(&self) -> &[CodeableConcept] {
        self.condition.as_deref().unwrap_or(&[])
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn related_artifact(&self) -> &[RelatedArtifact] {
        self.related_artifact.as_deref().unwrap_or(&[])
    }
    fn keyword(&self) -> &[CodeableConcept] {
        self.keyword.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> &[CodeableConcept] {
        self.location.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn enrollment(&self) -> &[Reference] {
        self.enrollment.as_deref().unwrap_or(&[])
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn sponsor(&self) -> Option<Reference> {
        self.sponsor.clone()
    }
    fn principal_investigator(&self) -> Option<Reference> {
        self.principal_investigator.clone()
    }
    fn site(&self) -> &[Reference] {
        self.site.as_deref().unwrap_or(&[])
    }
    fn reason_stopped(&self) -> Option<CodeableConcept> {
        self.reason_stopped.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn arm(&self) -> &[ResearchStudyArm] {
        self.arm.as_deref().unwrap_or(&[])
    }
    fn objective(&self) -> &[ResearchStudyObjective] {
        self.objective.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::research_study::ResearchStudyMutators for ResearchStudy {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_protocol(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.protocol = Some(value);
        resource
    }
    fn add_protocol(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.protocol.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_part_of(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
        resource
    }
    fn add_part_of(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: ResearchStudyStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_primary_purpose_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.primary_purpose_type = Some(value);
        resource
    }
    fn set_phase(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.phase = Some(value);
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_focus(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.focus = Some(value);
        resource
    }
    fn add_focus(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.focus.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_condition(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.condition = Some(value);
        resource
    }
    fn add_condition(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.condition.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_contact(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self {
        let mut resource = self.clone();
        resource.related_artifact = Some(value);
        resource
    }
    fn add_related_artifact(self, item: RelatedArtifact) -> Self {
        let mut resource = self.clone();
        resource
            .related_artifact
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_keyword(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.keyword = Some(value);
        resource
    }
    fn add_keyword(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.keyword.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_location(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn add_location(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.location.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_enrollment(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.enrollment = Some(value);
        resource
    }
    fn add_enrollment(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.enrollment.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_sponsor(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.sponsor = Some(value);
        resource
    }
    fn set_principal_investigator(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.principal_investigator = Some(value);
        resource
    }
    fn set_site(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.site = Some(value);
        resource
    }
    fn add_site(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.site.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reason_stopped(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason_stopped = Some(value);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = Some(value);
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_arm(self, value: Vec<ResearchStudyArm>) -> Self {
        let mut resource = self.clone();
        resource.arm = Some(value);
        resource
    }
    fn add_arm(self, item: ResearchStudyArm) -> Self {
        let mut resource = self.clone();
        resource.arm.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_objective(self, value: Vec<ResearchStudyObjective>) -> Self {
        let mut resource = self.clone();
        resource.objective = Some(value);
        resource
    }
    fn add_objective(self, item: ResearchStudyObjective) -> Self {
        let mut resource = self.clone();
        resource.objective.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::research_study::ResearchStudyExistence for ResearchStudy {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_protocol(&self) -> bool {
        self.protocol.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_primary_purpose_type(&self) -> bool {
        self.primary_purpose_type.is_some()
    }
    fn has_phase(&self) -> bool {
        self.phase.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_focus(&self) -> bool {
        self.focus.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_condition(&self) -> bool {
        self.condition.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_related_artifact(&self) -> bool {
        self.related_artifact
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_keyword(&self) -> bool {
        self.keyword.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_enrollment(&self) -> bool {
        self.enrollment.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_sponsor(&self) -> bool {
        self.sponsor.is_some()
    }
    fn has_principal_investigator(&self) -> bool {
        self.principal_investigator.is_some()
    }
    fn has_site(&self) -> bool {
        self.site.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_stopped(&self) -> bool {
        self.reason_stopped.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_arm(&self) -> bool {
        self.arm.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_objective(&self) -> bool {
        self.objective.as_ref().is_some_and(|v| !v.is_empty())
    }
}
