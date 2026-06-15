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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Name for this study
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Steps followed in executing study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol: Vec<Reference>,
    /// Part of larger study
    #[serde(rename = "partOf")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<Reference>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<CodeableConcept>,
    /// Drugs, devices, etc. under study
    ///
    /// Binding: example (Codes for medications, devices and other interventions.)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<CodeableConcept>,
    /// Condition being studied
    ///
    /// Binding: example (Identification of the condition or diagnosis.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<CodeableConcept>,
    /// Contact details for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactDetail>,
    /// References and dependencies
    #[serde(rename = "relatedArtifact")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<RelatedArtifact>,
    /// Used to search for the study
    ///
    /// Binding: example (Words associated with the study that may be useful in discovery.)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keyword: Vec<CodeableConcept>,
    /// Geographic region(s) for study
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<CodeableConcept>,
    /// What this is study doing
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Inclusion & exclusion criteria
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enrollment: Vec<Reference>,
    /// When the study began and ended
    pub period: Option<Period>,
    /// Organization that initiates and is legally responsible for the study
    pub sponsor: Option<Reference>,
    /// Researcher who oversees multiple aspects of the study
    #[serde(rename = "principalInvestigator")]
    pub principal_investigator: Option<Reference>,
    /// Facility where study activities are conducted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub site: Vec<Reference>,
    /// accrual-goal-met | closed-due-to-toxicity | closed-due-to-lack-of-study-progress | temporarily-closed-per-study-design
    ///
    /// Binding: example (Codes for why the study ended prematurely.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-reason-stopped
    #[serde(rename = "reasonStopped")]
    pub reason_stopped: Option<CodeableConcept>,
    /// Comments made about the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// Defined path through the study for a subject
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arm: Vec<ResearchStudyArm>,
    /// A goal for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub objective: Vec<ResearchStudyObjective>,
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

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "ResearchStudy.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/research-study-status|4.0.1",
        )
        .with_description("Codes that convey the current status of the research study.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ResearchStudy.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.contained", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.extension", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.protocol", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.partOf", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.primaryPurposeType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.phase", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.category", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.focus", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.condition", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.contact", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.keyword", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.location", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.enrollment", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.sponsor", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.principalInvestigator",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchStudy.site", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.reasonStopped", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.note", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.arm", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.arm.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.arm.extension", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.arm.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.arm.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.arm.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.arm.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.objective", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.objective.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.objective.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.objective.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ResearchStudy.objective.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.objective.type", 0, Some(1)),
        ]
    });

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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for ResearchStudy {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::research_study::ResearchStudyAccessors for ResearchStudy {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn protocol(&self) -> &[Reference] {
        self.protocol.as_slice()
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_slice()
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
        self.category.as_slice()
    }
    fn focus(&self) -> &[CodeableConcept] {
        self.focus.as_slice()
    }
    fn condition(&self) -> &[CodeableConcept] {
        self.condition.as_slice()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_slice()
    }
    fn related_artifact(&self) -> &[RelatedArtifact] {
        self.related_artifact.as_slice()
    }
    fn keyword(&self) -> &[CodeableConcept] {
        self.keyword.as_slice()
    }
    fn location(&self) -> &[CodeableConcept] {
        self.location.as_slice()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn enrollment(&self) -> &[Reference] {
        self.enrollment.as_slice()
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
        self.site.as_slice()
    }
    fn reason_stopped(&self) -> Option<CodeableConcept> {
        self.reason_stopped.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
    fn arm(&self) -> &[ResearchStudyArm] {
        self.arm.as_slice()
    }
    fn objective(&self) -> &[ResearchStudyObjective] {
        self.objective.as_slice()
    }
}

impl crate::traits::research_study::ResearchStudyMutators for ResearchStudy {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_protocol(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.protocol = value;
        resource
    }
    fn add_protocol(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.protocol.push(item);
        resource
    }
    fn set_part_of(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.part_of = value;
        resource
    }
    fn add_part_of(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of.push(item);
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
        resource.category = value;
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.push(item);
        resource
    }
    fn set_focus(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.focus = value;
        resource
    }
    fn add_focus(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.focus.push(item);
        resource
    }
    fn set_condition(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.condition = value;
        resource
    }
    fn add_condition(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.condition.push(item);
        resource
    }
    fn set_contact(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = value;
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.push(item);
        resource
    }
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self {
        let mut resource = self.clone();
        resource.related_artifact = value;
        resource
    }
    fn add_related_artifact(self, item: RelatedArtifact) -> Self {
        let mut resource = self.clone();
        resource.related_artifact.push(item);
        resource
    }
    fn set_keyword(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.keyword = value;
        resource
    }
    fn add_keyword(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.keyword.push(item);
        resource
    }
    fn set_location(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.location = value;
        resource
    }
    fn add_location(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.location.push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_enrollment(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.enrollment = value;
        resource
    }
    fn add_enrollment(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.enrollment.push(item);
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
        resource.site = value;
        resource
    }
    fn add_site(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.site.push(item);
        resource
    }
    fn set_reason_stopped(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason_stopped = Some(value);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = value;
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.push(item);
        resource
    }
    fn set_arm(self, value: Vec<ResearchStudyArm>) -> Self {
        let mut resource = self.clone();
        resource.arm = value;
        resource
    }
    fn add_arm(self, item: ResearchStudyArm) -> Self {
        let mut resource = self.clone();
        resource.arm.push(item);
        resource
    }
    fn set_objective(self, value: Vec<ResearchStudyObjective>) -> Self {
        let mut resource = self.clone();
        resource.objective = value;
        resource
    }
    fn add_objective(self, item: ResearchStudyObjective) -> Self {
        let mut resource = self.clone();
        resource.objective.push(item);
        resource
    }
}

impl crate::traits::research_study::ResearchStudyExistence for ResearchStudy {
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_protocol(&self) -> bool {
        !self.protocol.is_empty()
    }
    fn has_part_of(&self) -> bool {
        !self.part_of.is_empty()
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
        !self.category.is_empty()
    }
    fn has_focus(&self) -> bool {
        !self.focus.is_empty()
    }
    fn has_condition(&self) -> bool {
        !self.condition.is_empty()
    }
    fn has_contact(&self) -> bool {
        !self.contact.is_empty()
    }
    fn has_related_artifact(&self) -> bool {
        !self.related_artifact.is_empty()
    }
    fn has_keyword(&self) -> bool {
        !self.keyword.is_empty()
    }
    fn has_location(&self) -> bool {
        !self.location.is_empty()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_enrollment(&self) -> bool {
        !self.enrollment.is_empty()
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
        !self.site.is_empty()
    }
    fn has_reason_stopped(&self) -> bool {
        self.reason_stopped.is_some()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
    }
    fn has_arm(&self) -> bool {
        !self.arm.is_empty()
    }
    fn has_objective(&self) -> bool {
        !self.objective.is_empty()
    }
}

impl crate::validation::ValidatableResource for ResearchStudy {
    fn resource_type(&self) -> &'static str {
        "ResearchStudy"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ResearchStudy")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::research_study::{
    ResearchStudyAccessors, ResearchStudyExistence, ResearchStudyMutators,
};
