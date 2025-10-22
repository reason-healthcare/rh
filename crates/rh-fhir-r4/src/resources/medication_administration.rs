use crate::bindings::medication_admin_status::MedicationAdminStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicationAdministration
///
/// Describes the event of a patient consuming or otherwise being administered a medication.  This may be as simple as swallowing a tablet or it may be a long running infusion.  Related resources tie this event to the authorizing prescription, and the specific encounter between patient and health care practitioner.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationAdministration
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationAdministration
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationAdministration {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates protocol or definition
    pub instantiates: Option<Vec<StringType>>,
    /// Extension element for the 'instantiates' primitive field. Contains metadata and extensions.
    pub _instantiates: Option<Element>,
    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// in-progress | not-done | on-hold | completed | entered-in-error | stopped | unknown
    pub status: MedicationAdminStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason administration not performed
    ///
    /// Binding: example (A set of codes indicating the reason why the MedicationAdministration is negated.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/reason-medication-not-given-codes
    #[serde(rename = "statusReason")]
    pub status_reason: Option<Vec<CodeableConcept>>,
    /// Type of medication usage
    ///
    /// Binding: preferred (A coded concept describing where the medication administered is expected to occur.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-admin-category
    pub category: Option<CodeableConcept>,
    /// What was administered (CodeableConcept)
    #[serde(rename = "medicationCodeableConcept")]
    pub medication_codeable_concept: CodeableConcept,
    /// What was administered (Reference)
    #[serde(rename = "medicationReference")]
    pub medication_reference: Reference,
    /// Who received medication
    pub subject: Reference,
    /// Encounter or Episode of Care administered as part of
    pub context: Option<Reference>,
    /// Additional information to support administration
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Reference>>,
    /// Start and end time of administration (dateTime)
    #[serde(rename = "effectiveDateTime")]
    pub effective_date_time: DateTimeType,
    /// Start and end time of administration (Period)
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Period,
    /// Who performed the medication administration and what they did
    pub performer: Option<Vec<MedicationAdministrationPerformer>>,
    /// Reason administration performed
    ///
    /// Binding: example (A set of codes indicating the reason why the MedicationAdministration was made.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/reason-medication-given-codes
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Condition or observation that supports why the medication was administered
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Request administration performed against
    pub request: Option<Reference>,
    /// Device used to administer
    pub device: Option<Vec<Reference>>,
    /// Information about the administration
    pub note: Option<Vec<Annotation>>,
    /// Details of how medication was taken
    pub dosage: Option<MedicationAdministrationDosage>,
    /// A list of events of interest in the lifecycle
    #[serde(rename = "eventHistory")]
    pub event_history: Option<Vec<Reference>>,
}
/// MedicationAdministration nested structure for the 'performer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationAdministrationPerformer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of performance
    ///
    /// Binding: example (A code describing the role an individual played in administering the medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/med-admin-perform-function
    pub function: Option<CodeableConcept>,
    /// Who performed the medication administration
    pub actor: Reference,
}
/// MedicationAdministration nested structure for the 'dosage' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationAdministrationDosage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Free text dosage instructions e.g. SIG
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
    /// Body site administered to
    ///
    /// Binding: example (A coded concept describing the site location the medicine enters into or onto the body.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/approach-site-codes
    pub site: Option<CodeableConcept>,
    /// Path of substance into body
    ///
    /// Binding: example (A coded concept describing the route or physiological path of administration of a therapeutic agent into or onto the body of a subject.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/route-codes
    pub route: Option<CodeableConcept>,
    /// How drug was administered
    ///
    /// Binding: example (A coded concept describing the technique by which the medicine is administered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/administration-method-codes
    pub method: Option<CodeableConcept>,
    /// Amount of medication per dose
    pub dose: Option<Quantity>,
    /// Dose quantity per unit of time (Ratio)
    #[serde(rename = "rateRatio")]
    pub rate_ratio: Option<Ratio>,
    /// Dose quantity per unit of time (Quantity)
    #[serde(rename = "rateQuantity")]
    pub rate_quantity: Option<Quantity>,
}

impl Default for MedicationAdministration {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            instantiates: Default::default(),
            _instantiates: Default::default(),
            part_of: Default::default(),
            status: MedicationAdminStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            category: Default::default(),
            medication_codeable_concept: Default::default(),
            medication_reference: Default::default(),
            subject: Reference::default(),
            context: Default::default(),
            supporting_information: Default::default(),
            effective_date_time: Default::default(),
            effective_period: Default::default(),
            performer: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            request: Default::default(),
            device: Default::default(),
            note: Default::default(),
            dosage: Default::default(),
            event_history: Default::default(),
        }
    }
}

impl Default for MedicationAdministrationPerformer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: Reference::default(),
        }
    }
}

impl Default for MedicationAdministrationDosage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            text: Default::default(),
            _text: Default::default(),
            site: Default::default(),
            route: Default::default(),
            method: Default::default(),
            dose: Default::default(),
            rate_ratio: Default::default(),
            rate_quantity: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicationAdministration {
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

impl crate::traits::resource::ResourceMutators for MedicationAdministration {
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

impl crate::traits::resource::ResourceExistence for MedicationAdministration {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicationAdministration {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicationAdministration {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicationAdministration {
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

impl crate::traits::medication_administration::MedicationAdministrationAccessors
    for MedicationAdministration
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn instantiates(&self) -> &[StringType] {
        self.instantiates.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> MedicationAdminStatus {
        self.status.clone()
    }
    fn status_reason(&self) -> &[CodeableConcept] {
        self.status_reason.as_deref().unwrap_or(&[])
    }
    fn category(&self) -> Option<CodeableConcept> {
        self.category.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn context(&self) -> Option<Reference> {
        self.context.clone()
    }
    fn supporting_information(&self) -> &[Reference] {
        self.supporting_information.as_deref().unwrap_or(&[])
    }
    fn performer(&self) -> &[MedicationAdministrationPerformer] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
    }
    fn request(&self) -> Option<Reference> {
        self.request.clone()
    }
    fn device(&self) -> &[Reference] {
        self.device.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn dosage(&self) -> Option<MedicationAdministrationDosage> {
        self.dosage.clone()
    }
    fn event_history(&self) -> &[Reference] {
        self.event_history.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::medication_administration::MedicationAdministrationMutators
    for MedicationAdministration
{
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
    fn set_instantiates(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates = Some(value);
        resource
    }
    fn add_instantiates(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_status(self, value: MedicationAdminStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_reason(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.status_reason = Some(value);
        resource
    }
    fn add_status_reason(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .status_reason
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_category(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_context(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.context = Some(value);
        resource
    }
    fn set_supporting_information(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_information = Some(value);
        resource
    }
    fn add_supporting_information(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_information
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_performer(self, value: Vec<MedicationAdministrationPerformer>) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: MedicationAdministrationPerformer) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
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
    fn set_request(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.request = Some(value);
        resource
    }
    fn set_device(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.device = Some(value);
        resource
    }
    fn add_device(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.device.get_or_insert_with(Vec::new).push(item);
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
    fn set_dosage(self, value: MedicationAdministrationDosage) -> Self {
        let mut resource = self.clone();
        resource.dosage = Some(value);
        resource
    }
    fn set_event_history(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.event_history = Some(value);
        resource
    }
    fn add_event_history(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .event_history
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::medication_administration::MedicationAdministrationExistence
    for MedicationAdministration
{
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
    fn has_effective(&self) -> bool {
        true
    }
    fn has_medication(&self) -> bool {
        true
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates(&self) -> bool {
        self.instantiates.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_status_reason(&self) -> bool {
        self.status_reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_category(&self) -> bool {
        self.category.is_some()
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_context(&self) -> bool {
        self.context.is_some()
    }
    fn has_supporting_information(&self) -> bool {
        self.supporting_information
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_request(&self) -> bool {
        self.request.is_some()
    }
    fn has_device(&self) -> bool {
        self.device.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_dosage(&self) -> bool {
        self.dosage.is_some()
    }
    fn has_event_history(&self) -> bool {
        self.event_history.as_ref().is_some_and(|v| !v.is_empty())
    }
}
