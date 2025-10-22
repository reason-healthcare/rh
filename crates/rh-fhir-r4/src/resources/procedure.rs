use crate::bindings::event_status::EventStatus;
use crate::datatypes::age::Age;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Procedure
///
/// An action that is or was performed on or for a patient. This can be a physical intervention like an operation, or less invasive like long term services, counseling, or hypnotherapy.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Procedure
/// - Version: 4.0.1
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
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-not-performed-reason
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
    pub category: Option<CodeableConcept>,
    /// Identification of the procedure
    ///
    /// Binding: example (A code to identify a specific procedure .)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-code
    pub code: Option<CodeableConcept>,
    /// Who the procedure was performed on
    pub subject: Reference,
    /// Encounter created as part of
    pub encounter: Option<Reference>,
    /// When the procedure was performed (dateTime)
    #[serde(rename = "performedDateTime")]
    pub performed_date_time: Option<DateTimeType>,
    /// When the procedure was performed (Period)
    #[serde(rename = "performedPeriod")]
    pub performed_period: Option<Period>,
    /// When the procedure was performed (string)
    #[serde(rename = "performedString")]
    pub performed_string: Option<StringType>,
    /// When the procedure was performed (Age)
    #[serde(rename = "performedAge")]
    pub performed_age: Option<Age>,
    /// When the procedure was performed (Range)
    #[serde(rename = "performedRange")]
    pub performed_range: Option<Range>,
    /// Who recorded the procedure
    pub recorder: Option<Reference>,
    /// Person who asserts this procedure
    pub asserter: Option<Reference>,
    /// The people who performed the procedure
    pub performer: Option<Vec<ProcedurePerformer>>,
    /// Where the procedure happened
    pub location: Option<Reference>,
    /// Coded reason procedure performed
    ///
    /// Binding: example (A code that identifies the reason a procedure is  required.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-reason
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// The justification that the procedure was performed
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Target body sites
    ///
    /// Binding: example (Codes describing anatomical locations. May include laterality.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/body-site
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
    pub complication: Option<Vec<CodeableConcept>>,
    /// A condition that is a result of the procedure
    #[serde(rename = "complicationDetail")]
    pub complication_detail: Option<Vec<Reference>>,
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
    #[serde(rename = "usedReference")]
    pub used_reference: Option<Vec<Reference>>,
    /// Coded items used during the procedure
    ///
    /// Binding: example (Codes describing items used during a procedure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/device-kind
    #[serde(rename = "usedCode")]
    pub used_code: Option<Vec<CodeableConcept>>,
}
/// schedule
///
/// The schedule that was followed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-schedule
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureSchedule {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// directedBy
///
/// The target of the extension is a distinct actor from the requester and has decision-making authority over the service and takes direct responsibility to manage the service.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-directedBy
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureDirectedBy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// targetBodyStructure
///
/// The target body site used for this procedure.  Multiple locations are allowed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-targetBodyStructure
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureTargetBodyStructure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// causedBy
///
/// This procedure is because of the related item.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-causedBy
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureCausedBy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// approachBodyStructure
///
/// The approach body site used for this procedure.  Multiple locations are allowed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-approachBodyStructure
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureApproachBodyStructure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// method
///
/// The method used to perform this procedure.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-method
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureMethod {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// incisionDateTime
///
/// The time of the first incision.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-incisionDateTime
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureIncisionDateTime {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// progressStatus
///
/// A code to track a detailed progress of  a procedure (e.g. In Recovery, Prepared for Surgery).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-progressStatus
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureProgressStatus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
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
    /// The reference to the practitioner
    pub actor: Reference,
    /// Organization the device or practitioner was acting for
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Reference>,
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
            encounter: Default::default(),
            performed_date_time: Default::default(),
            performed_period: Default::default(),
            performed_string: Default::default(),
            performed_age: Default::default(),
            performed_range: Default::default(),
            recorder: Default::default(),
            asserter: Default::default(),
            performer: Default::default(),
            location: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            body_site: Default::default(),
            outcome: Default::default(),
            report: Default::default(),
            complication: Default::default(),
            complication_detail: Default::default(),
            follow_up: Default::default(),
            note: Default::default(),
            focal_device: Default::default(),
            used_reference: Default::default(),
            used_code: Default::default(),
        }
    }
}

impl Default for ProcedureSchedule {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ProcedureDirectedBy {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ProcedureTargetBodyStructure {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ProcedureCausedBy {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ProcedureApproachBodyStructure {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ProcedureMethod {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ProcedureIncisionDateTime {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ProcedureProgressStatus {
    fn default() -> Self {
        Self {
            base: Extension::default(),
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
    fn category(&self) -> Option<CodeableConcept> {
        self.category.clone()
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn recorder(&self) -> Option<Reference> {
        self.recorder.clone()
    }
    fn asserter(&self) -> Option<Reference> {
        self.asserter.clone()
    }
    fn performer(&self) -> &[ProcedurePerformer] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
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
    fn complication(&self) -> &[CodeableConcept] {
        self.complication.as_deref().unwrap_or(&[])
    }
    fn complication_detail(&self) -> &[Reference] {
        self.complication_detail.as_deref().unwrap_or(&[])
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
    fn used_reference(&self) -> &[Reference] {
        self.used_reference.as_deref().unwrap_or(&[])
    }
    fn used_code(&self) -> &[CodeableConcept] {
        self.used_code.as_deref().unwrap_or(&[])
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
    fn set_category(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
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
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_recorder(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.recorder = Some(value);
        resource
    }
    fn set_asserter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.asserter = Some(value);
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
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.reason_code = Some(value);
        resource
    }
    fn add_reason_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason_code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reason_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.reason_reference = Some(value);
        resource
    }
    fn add_reason_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .reason_reference
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_complication(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.complication = Some(value);
        resource
    }
    fn add_complication(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .complication
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_complication_detail(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.complication_detail = Some(value);
        resource
    }
    fn add_complication_detail(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .complication_detail
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
    fn set_used_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.used_reference = Some(value);
        resource
    }
    fn add_used_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .used_reference
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_used_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.used_code = Some(value);
        resource
    }
    fn add_used_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.used_code.get_or_insert_with(Vec::new).push(item);
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
    fn has_performed(&self) -> bool {
        self.performed_date_time.is_some()
            || self.performed_period.is_some()
            || self.performed_string.is_some()
            || self.performed_age.is_some()
            || self.performed_range.is_some()
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
        self.category.is_some()
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_recorder(&self) -> bool {
        self.recorder.is_some()
    }
    fn has_asserter(&self) -> bool {
        self.asserter.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
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
    fn has_complication_detail(&self) -> bool {
        self.complication_detail
            .as_ref()
            .is_some_and(|v| !v.is_empty())
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
    fn has_used_reference(&self) -> bool {
        self.used_reference.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_used_code(&self) -> bool {
        self.used_code.as_ref().is_some_and(|v| !v.is_empty())
    }
}
