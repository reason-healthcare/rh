use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ResearchStudy
///
/// A scientific study of nature that sometimes includes processes involved in health and disease. For example, clinical trials are research studies that involve people. These studies may be related to new ways to screen, prevent, diagnose, and treat disease. They may also study certain outcomes and certain groups of people by looking at data collected in the past or future.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ResearchStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchStudy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this study resource
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Business Identifier for study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// The business version for the study record
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this study (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Human readable name of the study
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Additional names for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub label: Vec<ResearchStudyLabel>,
    /// Steps followed in executing study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol: Vec<Reference>,
    /// Part of larger study
    #[serde(rename = "partOf")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<Reference>,
    /// References, URLs, and attachments
    #[serde(rename = "relatedArtifact")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<RelatedArtifact>,
    /// Date the resource last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// treatment | prevention | diagnostic | supportive-care | screening | health-services-research | basic-science | device-feasibility
    ///
    /// Binding: preferred (Codes for the main intent of the study.)
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
    /// Classifications of the study design characteristics
    ///
    /// Binding: preferred (This is a set of terms for study design characteristics.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/study-design
    #[serde(rename = "studyDesign")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub study_design: Vec<CodeableConcept>,
    /// Drugs, devices, etc. under study
    ///
    /// Binding: example (Common codes of research study focus)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-focus-type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<CodeableReference>,
    /// Condition being studied
    ///
    /// Binding: example (Identification of the condition or diagnosis.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<CodeableConcept>,
    /// Used to search for the study
    ///
    /// Binding: example (Words associated with the study that may be useful in discovery.)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keyword: Vec<CodeableConcept>,
    /// Geographic area for the study
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub region: Vec<CodeableConcept>,
    /// Brief text explaining the study
    #[serde(rename = "descriptionSummary")]
    pub description_summary: Option<StringType>,
    /// Extension element for the 'descriptionSummary' primitive field. Contains metadata and extensions.
    #[serde(rename = "_descriptionSummary")]
    pub _description_summary: Option<Element>,
    /// Detailed narrative of the study
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// When the study began and ended
    pub period: Option<Period>,
    /// Facility where study activities are conducted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub site: Vec<Reference>,
    /// Comments made about the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// Classification for the study
    ///
    /// Binding: example (desc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-classifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<CodeableConcept>,
    /// Sponsors, collaborators, and other parties
    #[serde(rename = "associatedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub associated_party: Vec<ResearchStudyAssociatedparty>,
    /// Status of study with time for that status
    #[serde(rename = "progressStatus")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub progress_status: Vec<ResearchStudyProgressstatus>,
    /// accrual-goal-met | closed-due-to-toxicity | closed-due-to-lack-of-study-progress | temporarily-closed-per-study-design
    ///
    /// Binding: example (Codes for why the study ended prematurely.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-reason-stopped
    #[serde(rename = "whyStopped")]
    pub why_stopped: Option<CodeableConcept>,
    /// Target or actual group of participants enrolled in study
    pub recruitment: Option<ResearchStudyRecruitment>,
    /// Defined path through the study for a subject
    #[serde(rename = "comparisonGroup")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comparison_group: Vec<ResearchStudyComparisongroup>,
    /// A goal for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub objective: Vec<ResearchStudyObjective>,
    /// A variable measured during the study
    #[serde(rename = "outcomeMeasure")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome_measure: Vec<ResearchStudyOutcomemeasure>,
    /// Link to results generated during the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<Reference>,
}
/// ResearchStudy nested structure for the 'recruitment' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchStudyRecruitment {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Estimated total number of participants to be enrolled
    #[serde(rename = "targetNumber")]
    pub target_number: Option<UnsignedIntType>,
    /// Extension element for the 'targetNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_targetNumber")]
    pub _target_number: Option<Element>,
    /// Actual total number of participants enrolled in study
    #[serde(rename = "actualNumber")]
    pub actual_number: Option<UnsignedIntType>,
    /// Extension element for the 'actualNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_actualNumber")]
    pub _actual_number: Option<Element>,
    /// Inclusion and exclusion criteria
    pub eligibility: Option<Reference>,
    /// Group of participants who were enrolled in study
    #[serde(rename = "actualGroup")]
    pub actual_group: Option<Reference>,
}
/// ResearchStudy nested structure for the 'associatedParty' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchStudyAssociatedparty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name of associated party
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// sponsor | lead-sponsor | sponsor-investigator | primary-investigator | collaborator | funding-source | general-contact | recruitment-contact | sub-investigator | study-director | study-chair
    ///
    /// Binding: extensible (desc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-party-role
    pub role: CodeableConcept,
    /// When active in the role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub period: Vec<Period>,
    /// nih | fda | government | nonprofit | academic | industry
    ///
    /// Binding: example (A characterization or type of the entity.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-party-organization-type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<CodeableConcept>,
    /// Individual or organization associated with study (use practitionerRole to specify their organisation)
    pub party: Option<Reference>,
}
/// ResearchStudy nested structure for the 'progressStatus' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchStudyProgressstatus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Label for status or state (e.g. recruitment status)
    ///
    /// Binding: extensible (defn.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-status
    pub state: CodeableConcept,
    /// Actual if true else anticipated
    pub actual: Option<BooleanType>,
    /// Extension element for the 'actual' primitive field. Contains metadata and extensions.
    pub _actual: Option<Element>,
    /// Date range
    pub period: Option<Period>,
}
/// ResearchStudy nested structure for the 'outcomeMeasure' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchStudyOutcomemeasure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Label for the outcome
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// primary | secondary | exploratory
    ///
    /// Binding: preferred (defn.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-objective-type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_: Vec<CodeableConcept>,
    /// Description of the outcome
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Structured outcome definition
    pub reference: Option<Reference>,
}
/// ResearchStudy nested structure for the 'comparisonGroup' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchStudyComparisongroup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Allows the comparisonGroup for the study and the comparisonGroup for the subject to be linked easily
    #[serde(rename = "linkId")]
    pub link_id: Option<StringType>,
    /// Extension element for the 'linkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_linkId")]
    pub _link_id: Option<Element>,
    /// Label for study comparisonGroup
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Categorization of study comparisonGroup
    ///
    /// Binding: extensible (desc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-study-arm-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Short explanation of study path
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Interventions or exposures in this comparisonGroup or cohort
    #[serde(rename = "intendedExposure")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intended_exposure: Vec<Reference>,
    /// Group of participants who were enrolled in study comparisonGroup
    #[serde(rename = "observedGroup")]
    pub observed_group: Option<Reference>,
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
    /// Description of the objective
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}
/// ResearchStudy nested structure for the 'label' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchStudyLabel {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// primary | official | scientific | plain-language | subtitle | short-title | acronym | earlier-title | language | auto-translated | human-use | machine-use | duplicate-uid
    ///
    /// Binding: extensible (desc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/title-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The name
    pub value: Option<StringType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}

impl Default for ResearchStudy {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            label: Default::default(),
            protocol: Default::default(),
            part_of: Default::default(),
            related_artifact: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            primary_purpose_type: Default::default(),
            phase: Default::default(),
            study_design: Default::default(),
            focus: Default::default(),
            condition: Default::default(),
            keyword: Default::default(),
            region: Default::default(),
            description_summary: Default::default(),
            _description_summary: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            period: Default::default(),
            site: Default::default(),
            note: Default::default(),
            classifier: Default::default(),
            associated_party: Default::default(),
            progress_status: Default::default(),
            why_stopped: Default::default(),
            recruitment: Default::default(),
            comparison_group: Default::default(),
            objective: Default::default(),
            outcome_measure: Default::default(),
            result: Default::default(),
        }
    }
}

impl Default for ResearchStudyRecruitment {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            target_number: Default::default(),
            _target_number: Default::default(),
            actual_number: Default::default(),
            _actual_number: Default::default(),
            eligibility: Default::default(),
            actual_group: Default::default(),
        }
    }
}

impl Default for ResearchStudyAssociatedparty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            role: Default::default(),
            period: Default::default(),
            classifier: Default::default(),
            party: Default::default(),
        }
    }
}

impl Default for ResearchStudyProgressstatus {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            state: Default::default(),
            actual: Default::default(),
            _actual: Default::default(),
            period: Default::default(),
        }
    }
}

impl Default for ResearchStudyOutcomemeasure {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            type_: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            reference: Default::default(),
        }
    }
}

impl Default for ResearchStudyComparisongroup {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            link_id: Default::default(),
            _link_id: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            type_: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            intended_exposure: Default::default(),
            observed_group: Default::default(),
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
            description: Default::default(),
            _description: Default::default(),
        }
    }
}

impl Default for ResearchStudyLabel {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value: Default::default(),
            _value: Default::default(),
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "ResearchStudy.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ResearchStudy.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description(
                "Codes that convey the current publication status of the research study resource.",
            ),
        ]
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
            rh_foundation::ElementCardinality::new("ResearchStudy.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.label", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.label.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.label.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.label.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ResearchStudy.label.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.label.value", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.protocol", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.partOf", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.primaryPurposeType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.phase", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.studyDesign", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.focus", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.condition", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.keyword", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.region", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.descriptionSummary", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.site", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.note", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.classifier", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.associatedParty", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.associatedParty.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.associatedParty.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.associatedParty.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.associatedParty.name",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.associatedParty.role",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchStudy.associatedParty.period", 0, None),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.associatedParty.classifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.associatedParty.party",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchStudy.progressStatus", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.progressStatus.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.progressStatus.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.progressStatus.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.progressStatus.state",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.progressStatus.actual",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.progressStatus.period",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchStudy.whyStopped", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.recruitment", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.recruitment.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.recruitment.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.recruitment.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.recruitment.targetNumber",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.recruitment.actualNumber",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.recruitment.eligibility",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.recruitment.actualGroup",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchStudy.comparisonGroup", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.comparisonGroup.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.comparisonGroup.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.comparisonGroup.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.comparisonGroup.linkId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.comparisonGroup.name",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.comparisonGroup.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.comparisonGroup.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.comparisonGroup.intendedExposure",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.comparisonGroup.observedGroup",
                0,
                Some(1),
            ),
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
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.objective.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchStudy.outcomeMeasure", 0, None),
            rh_foundation::ElementCardinality::new("ResearchStudy.outcomeMeasure.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.outcomeMeasure.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.outcomeMeasure.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ResearchStudy.outcomeMeasure.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchStudy.outcomeMeasure.type", 0, None),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.outcomeMeasure.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchStudy.outcomeMeasure.reference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchStudy.result", 0, None),
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
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn label(&self) -> &[ResearchStudyLabel] {
        self.label.as_slice()
    }
    fn protocol(&self) -> &[Reference] {
        self.protocol.as_slice()
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_slice()
    }
    fn related_artifact(&self) -> &[RelatedArtifact] {
        self.related_artifact.as_slice()
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn primary_purpose_type(&self) -> Option<CodeableConcept> {
        self.primary_purpose_type.clone()
    }
    fn phase(&self) -> Option<CodeableConcept> {
        self.phase.clone()
    }
    fn study_design(&self) -> &[CodeableConcept] {
        self.study_design.as_slice()
    }
    fn focus(&self) -> &[CodeableReference] {
        self.focus.as_slice()
    }
    fn condition(&self) -> &[CodeableConcept] {
        self.condition.as_slice()
    }
    fn keyword(&self) -> &[CodeableConcept] {
        self.keyword.as_slice()
    }
    fn region(&self) -> &[CodeableConcept] {
        self.region.as_slice()
    }
    fn description_summary(&self) -> Option<StringType> {
        self.description_summary.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn site(&self) -> &[Reference] {
        self.site.as_slice()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
    fn classifier(&self) -> &[CodeableConcept] {
        self.classifier.as_slice()
    }
    fn associated_party(&self) -> &[ResearchStudyAssociatedparty] {
        self.associated_party.as_slice()
    }
    fn progress_status(&self) -> &[ResearchStudyProgressstatus] {
        self.progress_status.as_slice()
    }
    fn why_stopped(&self) -> Option<CodeableConcept> {
        self.why_stopped.clone()
    }
    fn recruitment(&self) -> Option<ResearchStudyRecruitment> {
        self.recruitment.clone()
    }
    fn comparison_group(&self) -> &[ResearchStudyComparisongroup] {
        self.comparison_group.as_slice()
    }
    fn objective(&self) -> &[ResearchStudyObjective] {
        self.objective.as_slice()
    }
    fn outcome_measure(&self) -> &[ResearchStudyOutcomemeasure] {
        self.outcome_measure.as_slice()
    }
    fn result(&self) -> &[Reference] {
        self.result.as_slice()
    }
}

impl crate::traits::research_study::ResearchStudyMutators for ResearchStudy {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
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
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_label(self, value: Vec<ResearchStudyLabel>) -> Self {
        let mut resource = self.clone();
        resource.label = value;
        resource
    }
    fn add_label(self, item: ResearchStudyLabel) -> Self {
        let mut resource = self.clone();
        resource.label.push(item);
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
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
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
    fn set_study_design(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.study_design = value;
        resource
    }
    fn add_study_design(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.study_design.push(item);
        resource
    }
    fn set_focus(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.focus = value;
        resource
    }
    fn add_focus(self, item: CodeableReference) -> Self {
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
    fn set_region(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.region = value;
        resource
    }
    fn add_region(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.region.push(item);
        resource
    }
    fn set_description_summary(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description_summary = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
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
    fn set_classifier(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.classifier = value;
        resource
    }
    fn add_classifier(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.classifier.push(item);
        resource
    }
    fn set_associated_party(self, value: Vec<ResearchStudyAssociatedparty>) -> Self {
        let mut resource = self.clone();
        resource.associated_party = value;
        resource
    }
    fn add_associated_party(self, item: ResearchStudyAssociatedparty) -> Self {
        let mut resource = self.clone();
        resource.associated_party.push(item);
        resource
    }
    fn set_progress_status(self, value: Vec<ResearchStudyProgressstatus>) -> Self {
        let mut resource = self.clone();
        resource.progress_status = value;
        resource
    }
    fn add_progress_status(self, item: ResearchStudyProgressstatus) -> Self {
        let mut resource = self.clone();
        resource.progress_status.push(item);
        resource
    }
    fn set_why_stopped(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.why_stopped = Some(value);
        resource
    }
    fn set_recruitment(self, value: ResearchStudyRecruitment) -> Self {
        let mut resource = self.clone();
        resource.recruitment = Some(value);
        resource
    }
    fn set_comparison_group(self, value: Vec<ResearchStudyComparisongroup>) -> Self {
        let mut resource = self.clone();
        resource.comparison_group = value;
        resource
    }
    fn add_comparison_group(self, item: ResearchStudyComparisongroup) -> Self {
        let mut resource = self.clone();
        resource.comparison_group.push(item);
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
    fn set_outcome_measure(self, value: Vec<ResearchStudyOutcomemeasure>) -> Self {
        let mut resource = self.clone();
        resource.outcome_measure = value;
        resource
    }
    fn add_outcome_measure(self, item: ResearchStudyOutcomemeasure) -> Self {
        let mut resource = self.clone();
        resource.outcome_measure.push(item);
        resource
    }
    fn set_result(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.result = value;
        resource
    }
    fn add_result(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.result.push(item);
        resource
    }
}

impl crate::traits::research_study::ResearchStudyExistence for ResearchStudy {
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_label(&self) -> bool {
        !self.label.is_empty()
    }
    fn has_protocol(&self) -> bool {
        !self.protocol.is_empty()
    }
    fn has_part_of(&self) -> bool {
        !self.part_of.is_empty()
    }
    fn has_related_artifact(&self) -> bool {
        !self.related_artifact.is_empty()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
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
    fn has_study_design(&self) -> bool {
        !self.study_design.is_empty()
    }
    fn has_focus(&self) -> bool {
        !self.focus.is_empty()
    }
    fn has_condition(&self) -> bool {
        !self.condition.is_empty()
    }
    fn has_keyword(&self) -> bool {
        !self.keyword.is_empty()
    }
    fn has_region(&self) -> bool {
        !self.region.is_empty()
    }
    fn has_description_summary(&self) -> bool {
        self.description_summary.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_site(&self) -> bool {
        !self.site.is_empty()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
    }
    fn has_classifier(&self) -> bool {
        !self.classifier.is_empty()
    }
    fn has_associated_party(&self) -> bool {
        !self.associated_party.is_empty()
    }
    fn has_progress_status(&self) -> bool {
        !self.progress_status.is_empty()
    }
    fn has_why_stopped(&self) -> bool {
        self.why_stopped.is_some()
    }
    fn has_recruitment(&self) -> bool {
        self.recruitment.is_some()
    }
    fn has_comparison_group(&self) -> bool {
        !self.comparison_group.is_empty()
    }
    fn has_objective(&self) -> bool {
        !self.objective.is_empty()
    }
    fn has_outcome_measure(&self) -> bool {
        !self.outcome_measure.is_empty()
    }
    fn has_result(&self) -> bool {
        !self.result.is_empty()
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
