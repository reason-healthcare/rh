use crate::bindings::medication_statement_status::MedicationStatementStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicationStatement
///
/// A record of a medication that is being consumed by a patient.   A MedicationStatement may indicate that the patient may be taking the medication now or has taken the medication in the past or will be taking the medication in the future.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay.   The medication information may come from sources such as the patient's memory, from a prescription bottle,  or from a list of medications the patient, clinician or other party maintains.   The primary difference between a medication statement and a medication administration is that the medication administration has complete administration information and is based on actual administration information from the person who administered the medication.  A medication statement is often, if not always, less specific.  There is no required date/time when the medication was administered, in fact we only know that a source has reported the patient is taking this medication, where details such as time, quantity, or rate or even medication product may be incomplete or missing or less precise.  As stated earlier, the medication statement information may come from the patient's memory, from a prescription bottle or from a list of medications the patient, clinician or other party maintains.  Medication administration is more formal and is not missing detailed information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationStatement
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationStatement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationStatement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Fulfils plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// active | completed | entered-in-error | intended | stopped | on-hold | unknown | not-taken
    pub status: MedicationStatementStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason for current status
    ///
    /// Binding: example (A coded concept indicating the reason for the status of the statement.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/reason-medication-status-codes
    #[serde(rename = "statusReason")]
    pub status_reason: Option<Vec<CodeableConcept>>,
    /// Type of medication usage
    ///
    /// Binding: preferred (A coded concept identifying where the medication included in the MedicationStatement is expected to be consumed or administered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-statement-category
    pub category: Option<CodeableConcept>,
    /// What medication was taken (CodeableConcept)
    #[serde(rename = "medicationCodeableConcept")]
    pub medication_codeable_concept: CodeableConcept,
    /// What medication was taken (Reference)
    #[serde(rename = "medicationReference")]
    pub medication_reference: Reference,
    /// Who is/was taking  the medication
    pub subject: Reference,
    /// Encounter / Episode associated with MedicationStatement
    pub context: Option<Reference>,
    /// The date/time or interval when the medication is/was/will be taken (dateTime)
    #[serde(rename = "effectiveDateTime")]
    pub effective_date_time: Option<DateTimeType>,
    /// The date/time or interval when the medication is/was/will be taken (Period)
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// When the statement was asserted?
    #[serde(rename = "dateAsserted")]
    pub date_asserted: Option<DateTimeType>,
    /// Extension element for the 'dateAsserted' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dateAsserted")]
    pub _date_asserted: Option<Element>,
    /// Person or organization that provided the information about the taking of this medication
    #[serde(rename = "informationSource")]
    pub information_source: Option<Reference>,
    /// Additional supporting information
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Reference>>,
    /// Reason for why the medication is being/was taken
    ///
    /// Binding: example (A coded concept identifying why the medication is being taken.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Condition or observation that supports why the medication is being/was taken
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Further information about the statement
    pub note: Option<Vec<Annotation>>,
    /// Details of how medication is/was taken or should be taken
    pub dosage: Option<Vec<Dosage>>,
}

impl Default for MedicationStatement {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            part_of: Default::default(),
            status: MedicationStatementStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            category: Default::default(),
            medication_codeable_concept: Default::default(),
            medication_reference: Default::default(),
            subject: Reference::default(),
            context: Default::default(),
            effective_date_time: Default::default(),
            effective_period: Default::default(),
            date_asserted: Default::default(),
            _date_asserted: Default::default(),
            information_source: Default::default(),
            derived_from: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            note: Default::default(),
            dosage: Default::default(),
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
            "MedicationStatement.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/medication-statement-status|4.0.1",
        )
        .with_description(
            "A coded concept indicating the current status of a MedicationStatement.",
        )]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("MedicationStatement.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationStatement.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationStatement.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationStatement.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationStatement.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationStatement.contained", 0, None),
            rh_foundation::ElementCardinality::new("MedicationStatement.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicationStatement.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MedicationStatement.identifier", 0, None),
            rh_foundation::ElementCardinality::new("MedicationStatement.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("MedicationStatement.partOf", 0, None),
            rh_foundation::ElementCardinality::new("MedicationStatement.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationStatement.statusReason", 0, None),
            rh_foundation::ElementCardinality::new("MedicationStatement.category", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationStatement.medication[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationStatement.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationStatement.context", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationStatement.effective[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationStatement.dateAsserted", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationStatement.informationSource",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationStatement.derivedFrom", 0, None),
            rh_foundation::ElementCardinality::new("MedicationStatement.reasonCode", 0, None),
            rh_foundation::ElementCardinality::new("MedicationStatement.reasonReference", 0, None),
            rh_foundation::ElementCardinality::new("MedicationStatement.note", 0, None),
            rh_foundation::ElementCardinality::new("MedicationStatement.dosage", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicationStatement {
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

impl crate::traits::resource::ResourceMutators for MedicationStatement {
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

impl crate::traits::resource::ResourceExistence for MedicationStatement {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicationStatement {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicationStatement {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicationStatement {
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

impl crate::traits::medication_statement::MedicationStatementAccessors for MedicationStatement {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> MedicationStatementStatus {
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
    fn date_asserted(&self) -> Option<DateTimeType> {
        self.date_asserted.clone()
    }
    fn information_source(&self) -> Option<Reference> {
        self.information_source.clone()
    }
    fn derived_from(&self) -> &[Reference] {
        self.derived_from.as_deref().unwrap_or(&[])
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn dosage(&self) -> &[Dosage] {
        self.dosage.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::medication_statement::MedicationStatementMutators for MedicationStatement {
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
    fn set_status(self, value: MedicationStatementStatus) -> Self {
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
    fn set_date_asserted(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date_asserted = Some(value);
        resource
    }
    fn set_information_source(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.information_source = Some(value);
        resource
    }
    fn set_derived_from(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.derived_from = Some(value);
        resource
    }
    fn add_derived_from(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .derived_from
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_dosage(self, value: Vec<Dosage>) -> Self {
        let mut resource = self.clone();
        resource.dosage = Some(value);
        resource
    }
    fn add_dosage(self, item: Dosage) -> Self {
        let mut resource = self.clone();
        resource.dosage.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::medication_statement::MedicationStatementExistence for MedicationStatement {
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
        self.effective_date_time.is_some() || self.effective_period.is_some()
    }
    fn has_medication(&self) -> bool {
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
        self.category.is_some()
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_context(&self) -> bool {
        self.context.is_some()
    }
    fn has_date_asserted(&self) -> bool {
        self.date_asserted.is_some()
    }
    fn has_information_source(&self) -> bool {
        self.information_source.is_some()
    }
    fn has_derived_from(&self) -> bool {
        self.derived_from.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_dosage(&self) -> bool {
        self.dosage.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for MedicationStatement {
    fn resource_type(&self) -> &'static str {
        "MedicationStatement"
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
        Some("http://hl7.org/fhir/StructureDefinition/MedicationStatement")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::medication_statement::{
    MedicationStatementAccessors, MedicationStatementExistence, MedicationStatementMutators,
};
