use crate::bindings::event_status::EventStatus;
use crate::datatypes::age::Age;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Procedure
///
/// An action that is or was performed on or for a patient, practitioner, device, organization, or location. For example, this can be a physical intervention on a patient like an operation, or less invasive like long term services, counseling, or hypnotherapy.  This can be a quality or safety inspection for a location, organization, or device.  This can be an accreditation procedure on a practitioner for licensing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Procedure
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Procedure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External Identifiers for this procedure
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<StringType>>,
    /// Extension element for the 'instantiatesCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesCanonical")]
    pub _instantiates_canonical: Option<Element>,
    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<StringType>>,
    /// Extension element for the 'instantiatesUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesUri")]
    pub _instantiates_uri: Option<Element>,
    /// A request for this procedure
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: EventStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason for current status
    ///
    /// Binding: example (A code that identifies the reason a procedure was not performed.)
    ///
    /// Available values:
    /// - `410536001`
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,
    /// Classification of the procedure
    ///
    /// Binding: example (A code that classifies a procedure for searching, sorting and display purposes.)
    ///
    /// Available values:
    /// - `24642003`
    /// - `409063005`
    /// - `409073007`
    /// - `387713003`
    /// - `103693007`
    /// - `46947000`
    /// - `410606002`
    pub category: Option<Vec<CodeableConcept>>,
    /// Identification of the procedure
    ///
    /// Binding: example (A code to identify a specific procedure .)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-code
    pub code: Option<CodeableConcept>,
    /// Individual or entity the procedure was performed on
    pub subject: Reference,
    /// Who is the target of the procedure when it is not the subject of record only
    pub focus: Option<Reference>,
    /// The Encounter during which this Procedure was created
    pub encounter: Option<Reference>,
    /// When the procedure occurred or is occurring (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// When the procedure occurred or is occurring (Period)
    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,
    /// When the procedure occurred or is occurring (string)
    #[serde(rename = "occurrenceString")]
    pub occurrence_string: Option<StringType>,
    /// When the procedure occurred or is occurring (Age)
    #[serde(rename = "occurrenceAge")]
    pub occurrence_age: Option<Age>,
    /// When the procedure occurred or is occurring (Range)
    #[serde(rename = "occurrenceRange")]
    pub occurrence_range: Option<Range>,
    /// When the procedure occurred or is occurring (Timing)
    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,
    /// When the procedure was first captured in the subject's record
    pub recorded: Option<DateTimeType>,
    /// Extension element for the 'recorded' primitive field. Contains metadata and extensions.
    pub _recorded: Option<Element>,
    /// Who recorded the procedure
    pub recorder: Option<Reference>,
    /// Reported rather than primary record (boolean)
    #[serde(rename = "reportedBoolean")]
    pub reported_boolean: Option<BooleanType>,
    /// Reported rather than primary record (Reference)
    #[serde(rename = "reportedReference")]
    pub reported_reference: Option<Reference>,
    /// Who performed the procedure and what they did
    pub performer: Option<Vec<ProcedurePerformer>>,
    /// Where the procedure happened
    pub location: Option<Reference>,
    /// The justification that the procedure was performed
    ///
    /// Binding: example (A code that identifies the reason a procedure is  required.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-reason
    pub reason: Option<Vec<CodeableReference>>,
    /// Target body sites
    ///
    /// Binding: example (SNOMED CT Body site concepts)
    ///
    /// Available values:
    /// - `53075003`: Distal phalanx of hallux
    /// - `76986006`: Distal phalanx of second toe
    /// - `65258003`: Distal phalanx of third toe
    /// - `54333003`: Distal phalanx of fourth toe
    /// - `10770001`: Distal phalanx of fifth toe
    /// - `363670009`: Interphalangeal joint structure of great toe
    /// - `371216008`: Distal interphalangeal joint of second toe
    /// - `371219001`: Distal interphalangeal joint of third toe
    /// - `371205001`: Distal interphalangeal joint of fourth toe
    /// - `371203008`: Distal interphalangeal joint of fifth toe
    /// - ... and 30 more values
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<CodeableConcept>>,
    /// The result of procedure
    ///
    /// Binding: example (An outcome of a procedure - whether it was resolved or otherwise.)
    ///
    /// Available values:
    /// - `385669000`
    /// - `385671000`
    /// - `385670004`
    pub outcome: Option<CodeableConcept>,
    /// Any report resulting from the procedure
    pub report: Option<Vec<Reference>>,
    /// Complication following the procedure
    ///
    /// Binding: example (Codes describing complications that resulted from a procedure.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    pub complication: Option<Vec<CodeableReference>>,
    /// Instructions for follow up
    ///
    /// Binding: example (Specific follow up required for a procedure e.g. removal of sutures.)
    ///
    /// Available values:
    /// - `18949003`
    /// - `30549001`
    /// - `241031001`
    /// - `35963001`
    /// - `225164002`
    /// - `447346005`
    /// - `229506003`
    /// - `274441001`
    /// - `394725008`
    /// - `359825008`
    #[serde(rename = "followUp")]
    pub follow_up: Option<Vec<CodeableConcept>>,
    /// Additional information about the procedure
    pub note: Option<Vec<Annotation>>,
    /// Manipulated, implanted, or removed device
    #[serde(rename = "focalDevice")]
    pub focal_device: Option<Vec<ProcedureFocaldevice>>,
    /// Items used during procedure
    ///
    /// Binding: example (Codes describing items used during a procedure.)
    ///
    /// Available values:
    /// - `528391`: Blood Pressure Cuff
    /// - `528404`: Body Composition Analyzer
    /// - `528425`: Cardiovascular Device
    /// - `528402`: Coagulation meter
    /// - `528409`: Continuous Glucose Monitor
    /// - `528390`: Electro cardiograph
    /// - `528457`: Generic 20601 Device
    /// - `528401`: Glucose Monitor
    /// - `528455`: Independent Activity/Living Hub
    /// - `528403`: Insulin Pump
    /// - ... and 18 more values
    pub used: Option<Vec<CodeableReference>>,
    /// Extra information relevant to the procedure
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Reference>>,
}
/// Procedure nested structure for the 'focalDevice' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureFocaldevice {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Kind of change to device
    ///
    /// Binding: preferred (A kind of change that happened to the device during the procedure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/device-action
    pub action: Option<CodeableConcept>,
    /// Device that was changed
    pub manipulated: Reference,
}
/// Procedure nested structure for the 'performer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedurePerformer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of performance
    ///
    /// Binding: example (A code that identifies the role of a performer of the procedure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/performer-role
    pub function: Option<CodeableConcept>,
    /// Who performed the procedure
    pub actor: Reference,
    /// Organization the device or practitioner was acting for
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Reference>,
    /// When the performer performed the procedure
    pub period: Option<Period>,
}

impl Default for Procedure {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            based_on: Default::default(),
            part_of: Default::default(),
            status: EventStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            category: Default::default(),
            code: Default::default(),
            subject: Reference::default(),
            focus: Default::default(),
            encounter: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            occurrence_string: Default::default(),
            occurrence_age: Default::default(),
            occurrence_range: Default::default(),
            occurrence_timing: Default::default(),
            recorded: Default::default(),
            _recorded: Default::default(),
            recorder: Default::default(),
            reported_boolean: Default::default(),
            reported_reference: Default::default(),
            performer: Default::default(),
            location: Default::default(),
            reason: Default::default(),
            body_site: Default::default(),
            outcome: Default::default(),
            report: Default::default(),
            complication: Default::default(),
            follow_up: Default::default(),
            note: Default::default(),
            focal_device: Default::default(),
            used: Default::default(),
            supporting_info: Default::default(),
        }
    }
}

impl Default for ProcedureFocaldevice {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            action: Default::default(),
            manipulated: Default::default(),
        }
    }
}

impl Default for ProcedurePerformer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: Reference::default(),
            on_behalf_of: Default::default(),
            period: Default::default(),
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
    rh_foundation::Invariant::new("prc-1", rh_foundation::Severity::Error, "Procedure.performer.onBehalfOf can only be populated when performer.actor isn't Practitioner or PractitionerRole", "onBehalfOf.exists() and actor.resolve().exists() implies actor.resolve().where($this is Practitioner or $this is PractitionerRole).empty()"),
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
                "Procedure.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Procedure.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/event-status|5.0.0",
            )
            .with_description("A code specifying the state of the procedure."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Procedure.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.contained", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.extension", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.instantiatesCanonical", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.instantiatesUri", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.partOf", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.statusReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.category", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.focus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.occurrence[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.recorded", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.recorder", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.reported[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.performer", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.performer.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.performer.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Procedure.performer.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Procedure.performer.function", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.performer.actor", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.performer.onBehalfOf", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.performer.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.reason", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.bodySite", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.report", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.complication", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.followUp", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.note", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.focalDevice", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.focalDevice.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.focalDevice.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Procedure.focalDevice.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Procedure.focalDevice.action", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.focalDevice.manipulated", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Procedure.used", 0, None),
            rh_foundation::ElementCardinality::new("Procedure.supportingInfo", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Procedure {
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

impl crate::traits::resource::ResourceMutators for Procedure {
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

impl crate::traits::resource::ResourceExistence for Procedure {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Procedure {
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

impl crate::traits::domain_resource::DomainResourceMutators for Procedure {
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

impl crate::traits::domain_resource::DomainResourceExistence for Procedure {
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

impl crate::traits::procedure::ProcedureAccessors for Procedure {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn instantiates_canonical(&self) -> &[StringType] {
        self.instantiates_canonical.as_deref().unwrap_or(&[])
    }
    fn instantiates_uri(&self) -> &[StringType] {
        self.instantiates_uri.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> EventStatus {
        self.status.clone()
    }
    fn status_reason(&self) -> Option<CodeableConcept> {
        self.status_reason.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn focus(&self) -> Option<Reference> {
        self.focus.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn recorded(&self) -> Option<DateTimeType> {
        self.recorded.clone()
    }
    fn recorder(&self) -> Option<Reference> {
        self.recorder.clone()
    }
    fn performer(&self) -> &[ProcedurePerformer] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn body_site(&self) -> &[CodeableConcept] {
        self.body_site.as_deref().unwrap_or(&[])
    }
    fn outcome(&self) -> Option<CodeableConcept> {
        self.outcome.clone()
    }
    fn report(&self) -> &[Reference] {
        self.report.as_deref().unwrap_or(&[])
    }
    fn complication(&self) -> &[CodeableReference] {
        self.complication.as_deref().unwrap_or(&[])
    }
    fn follow_up(&self) -> &[CodeableConcept] {
        self.follow_up.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn focal_device(&self) -> &[ProcedureFocaldevice] {
        self.focal_device.as_deref().unwrap_or(&[])
    }
    fn used(&self) -> &[CodeableReference] {
        self.used.as_deref().unwrap_or(&[])
    }
    fn supporting_info(&self) -> &[Reference] {
        self.supporting_info.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::procedure::ProcedureMutators for Procedure {
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
    fn set_instantiates_canonical(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_canonical = Some(value);
        resource
    }
    fn add_instantiates_canonical(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates_canonical
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_instantiates_uri(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_uri = Some(value);
        resource
    }
    fn add_instantiates_uri(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates_uri
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
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
    fn set_status(self, value: EventStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status_reason = Some(value);
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
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_focus(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.focus = Some(value);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_recorded(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.recorded = Some(value);
        resource
    }
    fn set_recorder(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.recorder = Some(value);
        resource
    }
    fn set_performer(self, value: Vec<ProcedurePerformer>) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: ProcedurePerformer) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_reason(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.reason = Some(value);
        resource
    }
    fn add_reason(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.reason.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_body_site(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.body_site = Some(value);
        resource
    }
    fn add_body_site(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.body_site.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_outcome(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.outcome = Some(value);
        resource
    }
    fn set_report(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.report = Some(value);
        resource
    }
    fn add_report(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.report.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_complication(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.complication = Some(value);
        resource
    }
    fn add_complication(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource
            .complication
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_follow_up(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.follow_up = Some(value);
        resource
    }
    fn add_follow_up(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.follow_up.get_or_insert_with(Vec::new).push(item);
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
    fn set_focal_device(self, value: Vec<ProcedureFocaldevice>) -> Self {
        let mut resource = self.clone();
        resource.focal_device = Some(value);
        resource
    }
    fn add_focal_device(self, item: ProcedureFocaldevice) -> Self {
        let mut resource = self.clone();
        resource
            .focal_device
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_used(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.used = Some(value);
        resource
    }
    fn add_used(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.used.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_supporting_info(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_info = Some(value);
        resource
    }
    fn add_supporting_info(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_info
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::procedure::ProcedureExistence for Procedure {
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
    fn has_occurrence(&self) -> bool {
        self.occurrence_date_time.is_some()
            || self.occurrence_period.is_some()
            || self.occurrence_string.is_some()
            || self.occurrence_age.is_some()
            || self.occurrence_range.is_some()
            || self.occurrence_timing.is_some()
    }
    fn has_reported(&self) -> bool {
        self.reported_boolean.is_some() || self.reported_reference.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates_canonical(&self) -> bool {
        self.instantiates_canonical
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates_uri(&self) -> bool {
        self.instantiates_uri
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_status_reason(&self) -> bool {
        self.status_reason.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_focus(&self) -> bool {
        self.focus.is_some()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_recorded(&self) -> bool {
        self.recorded.is_some()
    }
    fn has_recorder(&self) -> bool {
        self.recorder.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_body_site(&self) -> bool {
        self.body_site.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_outcome(&self) -> bool {
        self.outcome.is_some()
    }
    fn has_report(&self) -> bool {
        self.report.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_complication(&self) -> bool {
        self.complication.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_follow_up(&self) -> bool {
        self.follow_up.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_focal_device(&self) -> bool {
        self.focal_device.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_used(&self) -> bool {
        self.used.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_supporting_info(&self) -> bool {
        self.supporting_info.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Procedure {
    fn resource_type(&self) -> &'static str {
        "Procedure"
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
        Some("http://hl7.org/fhir/StructureDefinition/Procedure")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::procedure::{ProcedureAccessors, ProcedureExistence, ProcedureMutators};
