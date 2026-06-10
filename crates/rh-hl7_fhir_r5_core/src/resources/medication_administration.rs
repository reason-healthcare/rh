use crate::bindings::medication_admin_status::MedicationAdminStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicationAdministration
///
/// Describes the event of a patient consuming or otherwise being administered a medication.  This may be as simple as swallowing a tablet or it may be a long running infusion. Related resources tie this event to the authorizing prescription, and the specific encounter between patient and health care practitioner. This event can also be used to record waste using a status of not-done and the appropriate statusReason.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationAdministration
/// - Version: 5.0.0
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
    /// Plan this is fulfilled by this administration
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
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
    /// Type of medication administration
    ///
    /// Binding: example (A coded concept describing where the medication administered is expected to occur.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-admin-location
    pub category: Option<Vec<CodeableConcept>>,
    /// What was administered
    ///
    /// Binding: example (Codes identifying substance or product that can be administered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-codes
    pub medication: CodeableReference,
    /// Who received medication
    pub subject: Reference,
    /// Encounter administered as part of
    pub encounter: Option<Reference>,
    /// Additional information to support administration
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Reference>>,
    /// Specific date/time or interval of time during which the administration took place (or did not take place) (dateTime)
    #[serde(rename = "occurenceDateTime")]
    pub occurence_date_time: DateTimeType,
    /// Specific date/time or interval of time during which the administration took place (or did not take place) (Period)
    #[serde(rename = "occurencePeriod")]
    pub occurence_period: Period,
    /// Specific date/time or interval of time during which the administration took place (or did not take place) (Timing)
    #[serde(rename = "occurenceTiming")]
    pub occurence_timing: Timing,
    /// When the MedicationAdministration was first captured in the subject's record
    pub recorded: Option<DateTimeType>,
    /// Extension element for the 'recorded' primitive field. Contains metadata and extensions.
    pub _recorded: Option<Element>,
    /// Full dose was not administered
    #[serde(rename = "isSubPotent")]
    pub is_sub_potent: Option<BooleanType>,
    /// Extension element for the 'isSubPotent' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isSubPotent")]
    pub _is_sub_potent: Option<Element>,
    /// Reason full dose was not administered
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/administration-subpotent-reason
    #[serde(rename = "subPotentReason")]
    pub sub_potent_reason: Option<Vec<CodeableConcept>>,
    /// Who or what performed the medication administration and what type of performance they did
    pub performer: Option<Vec<MedicationAdministrationPerformer>>,
    /// Concept, condition or observation that supports why the medication was administered
    ///
    /// Binding: example (A set of codes indicating the reason why the MedicationAdministration was made.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/reason-medication-given-codes
    pub reason: Option<Vec<CodeableReference>>,
    /// Request administration performed against
    pub request: Option<Reference>,
    /// Device used to administer
    pub device: Option<Vec<CodeableReference>>,
    /// Information about the administration
    pub note: Option<Vec<Annotation>>,
    /// Details of how medication was taken
    pub dosage: Option<MedicationAdministrationDosage>,
    /// A list of events of interest in the lifecycle
    #[serde(rename = "eventHistory")]
    pub event_history: Option<Vec<Reference>>,
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
    /// Who or what performed the medication administration
    pub actor: CodeableReference,
}

impl Default for MedicationAdministration {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            part_of: Default::default(),
            status: MedicationAdminStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            category: Default::default(),
            medication: CodeableReference::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            supporting_information: Default::default(),
            occurence_date_time: Default::default(),
            occurence_period: Default::default(),
            occurence_timing: Default::default(),
            recorded: Default::default(),
            _recorded: Default::default(),
            is_sub_potent: Default::default(),
            _is_sub_potent: Default::default(),
            sub_potent_reason: Default::default(),
            performer: Default::default(),
            reason: Default::default(),
            request: Default::default(),
            device: Default::default(),
            note: Default::default(),
            dosage: Default::default(),
            event_history: Default::default(),
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

impl Default for MedicationAdministrationPerformer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: CodeableReference::default(),
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
    rh_foundation::Invariant::new("mad-1", rh_foundation::Severity::Error, "If dosage attribute is present then SHALL have at least one of dosage.text or dosage.dose or dosage.rate[x]", "(dose.exists() or rate.exists() or text.exists())"),
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
                "MedicationAdministration.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "MedicationAdministration.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/medication-admin-status|5.0.0",
            )
            .with_description(
                "A set of codes indicating the current status of a MedicationAdministration.",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("MedicationAdministration.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationAdministration.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationAdministration.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationAdministration.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationAdministration.contained", 0, None),
            rh_foundation::ElementCardinality::new("MedicationAdministration.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MedicationAdministration.identifier", 0, None),
            rh_foundation::ElementCardinality::new("MedicationAdministration.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("MedicationAdministration.partOf", 0, None),
            rh_foundation::ElementCardinality::new("MedicationAdministration.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.statusReason",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MedicationAdministration.category", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.medication",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationAdministration.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.encounter",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.supportingInformation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.occurence[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationAdministration.recorded", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.isSubPotent",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.subPotentReason",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MedicationAdministration.performer", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.performer.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.performer.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.performer.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.performer.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.performer.actor",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationAdministration.reason", 0, None),
            rh_foundation::ElementCardinality::new("MedicationAdministration.request", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationAdministration.device", 0, None),
            rh_foundation::ElementCardinality::new("MedicationAdministration.note", 0, None),
            rh_foundation::ElementCardinality::new("MedicationAdministration.dosage", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.dosage.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.dosage.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.dosage.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.dosage.text",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.dosage.site",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.dosage.route",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.dosage.method",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.dosage.dose",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.dosage.rate[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationAdministration.eventHistory",
                0,
                None,
            ),
        ]
    });

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
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
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
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn medication(&self) -> CodeableReference {
        self.medication.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn supporting_information(&self) -> &[Reference] {
        self.supporting_information.as_deref().unwrap_or(&[])
    }
    fn recorded(&self) -> Option<DateTimeType> {
        self.recorded.clone()
    }
    fn is_sub_potent(&self) -> Option<BooleanType> {
        self.is_sub_potent
    }
    fn sub_potent_reason(&self) -> &[CodeableConcept] {
        self.sub_potent_reason.as_deref().unwrap_or(&[])
    }
    fn performer(&self) -> &[MedicationAdministrationPerformer] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn request(&self) -> Option<Reference> {
        self.request.clone()
    }
    fn device(&self) -> &[CodeableReference] {
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
    fn set_medication(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.medication = value;
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
    fn set_recorded(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.recorded = Some(value);
        resource
    }
    fn set_is_sub_potent(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.is_sub_potent = Some(value);
        resource
    }
    fn set_sub_potent_reason(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.sub_potent_reason = Some(value);
        resource
    }
    fn add_sub_potent_reason(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .sub_potent_reason
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
    fn set_request(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.request = Some(value);
        resource
    }
    fn set_device(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.device = Some(value);
        resource
    }
    fn add_device(self, item: CodeableReference) -> Self {
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
    fn has_occurence(&self) -> bool {
        true
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
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
        self.status_reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_medication(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_supporting_information(&self) -> bool {
        self.supporting_information
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_recorded(&self) -> bool {
        self.recorded.is_some()
    }
    fn has_is_sub_potent(&self) -> bool {
        self.is_sub_potent.is_some()
    }
    fn has_sub_potent_reason(&self) -> bool {
        self.sub_potent_reason
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
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

impl crate::validation::ValidatableResource for MedicationAdministration {
    fn resource_type(&self) -> &'static str {
        "MedicationAdministration"
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
        Some("http://hl7.org/fhir/StructureDefinition/MedicationAdministration")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::medication_administration::{
    MedicationAdministrationAccessors, MedicationAdministrationExistence,
    MedicationAdministrationMutators,
};
