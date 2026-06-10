use crate::bindings::history_status::HistoryStatus;
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
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// FamilyMemberHistory
///
/// Significant health conditions for a person related to the patient relevant in the context of care for the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: FamilyMemberHistory
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMemberHistory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External Id(s) for this record
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
    /// partial | completed | entered-in-error | health-unknown
    pub status: HistoryStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// subject-unknown | withheld | unable-to-obtain | deferred
    ///
    /// Binding: example (Codes describing the reason why a family member's history is not available.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/history-absent-reason
    #[serde(rename = "dataAbsentReason")]
    pub data_absent_reason: Option<CodeableConcept>,
    /// Patient history is about
    pub patient: Reference,
    /// When history was recorded or last updated
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Who or what participated in the activities related to the family member history and how they were involved
    pub participant: Option<Vec<FamilyMemberHistoryParticipant>>,
    /// The family member described
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Relationship to the subject
    ///
    /// Binding: example (The nature of the relationship between the patient and the related person being described in the family member history.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-FamilyMember
    pub relationship: CodeableConcept,
    /// male | female | other | unknown
    ///
    /// Binding: extensible (Codes describing the sex assigned at birth as documented on the birth registration.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/administrative-gender
    pub sex: Option<CodeableConcept>,
    /// (approximate) date of birth (Period)
    #[serde(rename = "bornPeriod")]
    pub born_period: Option<Period>,
    /// (approximate) date of birth (date)
    #[serde(rename = "bornDate")]
    pub born_date: Option<DateType>,
    /// (approximate) date of birth (string)
    #[serde(rename = "bornString")]
    pub born_string: Option<StringType>,
    /// (approximate) age (Age)
    #[serde(rename = "ageAge")]
    pub age_age: Option<Age>,
    /// (approximate) age (Range)
    #[serde(rename = "ageRange")]
    pub age_range: Option<Range>,
    /// (approximate) age (string)
    #[serde(rename = "ageString")]
    pub age_string: Option<StringType>,
    /// Age is estimated?
    #[serde(rename = "estimatedAge")]
    pub estimated_age: Option<BooleanType>,
    /// Extension element for the 'estimatedAge' primitive field. Contains metadata and extensions.
    #[serde(rename = "_estimatedAge")]
    pub _estimated_age: Option<Element>,
    /// Dead? How old/when? (boolean)
    #[serde(rename = "deceasedBoolean")]
    pub deceased_boolean: Option<BooleanType>,
    /// Dead? How old/when? (Age)
    #[serde(rename = "deceasedAge")]
    pub deceased_age: Option<Age>,
    /// Dead? How old/when? (Range)
    #[serde(rename = "deceasedRange")]
    pub deceased_range: Option<Range>,
    /// Dead? How old/when? (date)
    #[serde(rename = "deceasedDate")]
    pub deceased_date: Option<DateType>,
    /// Dead? How old/when? (string)
    #[serde(rename = "deceasedString")]
    pub deceased_string: Option<StringType>,
    /// Why was family member history performed?
    ///
    /// Binding: example (Codes indicating why the family member history was done.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinical-findings
    pub reason: Option<Vec<CodeableReference>>,
    /// General note about related person
    pub note: Option<Vec<Annotation>>,
    /// Condition that the related person had
    pub condition: Option<Vec<FamilyMemberHistoryCondition>>,
    /// Procedures that the related person had
    pub procedure: Option<Vec<FamilyMemberHistoryProcedure>>,
}
/// FamilyMemberHistory nested structure for the 'procedure' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMemberHistoryProcedure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Procedures performed on the related person
    ///
    /// Binding: example (A code to identify a specific procedure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-code
    pub code: CodeableConcept,
    /// What happened following the procedure
    ///
    /// Binding: example (The result of the procedure; e.g. death, permanent disability, temporary disability, etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinical-findings
    pub outcome: Option<CodeableConcept>,
    /// Whether the procedure contributed to the cause of death
    #[serde(rename = "contributedToDeath")]
    pub contributed_to_death: Option<BooleanType>,
    /// Extension element for the 'contributedToDeath' primitive field. Contains metadata and extensions.
    #[serde(rename = "_contributedToDeath")]
    pub _contributed_to_death: Option<Element>,
    /// When the procedure was performed (Age)
    #[serde(rename = "performedAge")]
    pub performed_age: Option<Age>,
    /// When the procedure was performed (Range)
    #[serde(rename = "performedRange")]
    pub performed_range: Option<Range>,
    /// When the procedure was performed (Period)
    #[serde(rename = "performedPeriod")]
    pub performed_period: Option<Period>,
    /// When the procedure was performed (string)
    #[serde(rename = "performedString")]
    pub performed_string: Option<StringType>,
    /// When the procedure was performed (dateTime)
    #[serde(rename = "performedDateTime")]
    pub performed_date_time: Option<DateTimeType>,
    /// Extra information about the procedure
    pub note: Option<Vec<Annotation>>,
}
/// FamilyMemberHistory nested structure for the 'condition' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMemberHistoryCondition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Condition suffered by relation
    ///
    /// Binding: example (Identification of the Condition or diagnosis.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    pub code: CodeableConcept,
    /// deceased | permanent disability | etc
    ///
    /// Binding: example (The result of the condition for the patient; e.g. death, permanent disability, temporary disability, etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/condition-outcome
    pub outcome: Option<CodeableConcept>,
    /// Whether the condition contributed to the cause of death
    #[serde(rename = "contributedToDeath")]
    pub contributed_to_death: Option<BooleanType>,
    /// Extension element for the 'contributedToDeath' primitive field. Contains metadata and extensions.
    #[serde(rename = "_contributedToDeath")]
    pub _contributed_to_death: Option<Element>,
    /// When condition first manifested (Age)
    #[serde(rename = "onsetAge")]
    pub onset_age: Option<Age>,
    /// When condition first manifested (Range)
    #[serde(rename = "onsetRange")]
    pub onset_range: Option<Range>,
    /// When condition first manifested (Period)
    #[serde(rename = "onsetPeriod")]
    pub onset_period: Option<Period>,
    /// When condition first manifested (string)
    #[serde(rename = "onsetString")]
    pub onset_string: Option<StringType>,
    /// Extra information about condition
    pub note: Option<Vec<Annotation>>,
}
/// FamilyMemberHistory nested structure for the 'participant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMemberHistoryParticipant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of involvement
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/participation-role-type
    pub function: Option<CodeableConcept>,
    /// Who or what participated in the activities related to the family member history
    pub actor: Reference,
}

impl Default for FamilyMemberHistory {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            status: HistoryStatus::default(),
            _status: Default::default(),
            data_absent_reason: Default::default(),
            patient: Reference::default(),
            date: Default::default(),
            _date: Default::default(),
            participant: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            relationship: CodeableConcept::default(),
            sex: Default::default(),
            born_period: Default::default(),
            born_date: Default::default(),
            born_string: Default::default(),
            age_age: Default::default(),
            age_range: Default::default(),
            age_string: Default::default(),
            estimated_age: Default::default(),
            _estimated_age: Default::default(),
            deceased_boolean: Default::default(),
            deceased_age: Default::default(),
            deceased_range: Default::default(),
            deceased_date: Default::default(),
            deceased_string: Default::default(),
            reason: Default::default(),
            note: Default::default(),
            condition: Default::default(),
            procedure: Default::default(),
        }
    }
}

impl Default for FamilyMemberHistoryProcedure {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: CodeableConcept::default(),
            outcome: Default::default(),
            contributed_to_death: Default::default(),
            _contributed_to_death: Default::default(),
            performed_age: Default::default(),
            performed_range: Default::default(),
            performed_period: Default::default(),
            performed_string: Default::default(),
            performed_date_time: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for FamilyMemberHistoryCondition {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: CodeableConcept::default(),
            outcome: Default::default(),
            contributed_to_death: Default::default(),
            _contributed_to_death: Default::default(),
            onset_age: Default::default(),
            onset_range: Default::default(),
            onset_period: Default::default(),
            onset_string: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for FamilyMemberHistoryParticipant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: Reference::default(),
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
    rh_foundation::Invariant::new("fhs-1", rh_foundation::Severity::Error, "Can have age[x] or born[x], but not both", "age.empty() or born.empty()"),
    rh_foundation::Invariant::new("fhs-2", rh_foundation::Severity::Error, "Can only have estimatedAge if age[x] is present", "age.exists() or estimatedAge.empty()"),
    rh_foundation::Invariant::new("fhs-3", rh_foundation::Severity::Error, "Can have age[x] or deceased[x], but not both", "age.empty() or deceased.empty()"),
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
                "FamilyMemberHistory.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "FamilyMemberHistory.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/history-status|5.0.0",
            )
            .with_description("A code that identifies the status of the family history record."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.contained", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.identifier", 0, None),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.instantiatesCanonical",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.instantiatesUri", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.dataAbsentReason",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.patient", 1, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.participant", 0, None),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.participant.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.participant.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.participant.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.participant.actor",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.relationship", 1, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.sex", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.born[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.age[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.estimatedAge", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.deceased[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.reason", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.note", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.condition", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.condition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.outcome",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.contributedToDeath",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.onset[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.condition.note", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.procedure", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.procedure.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.procedure.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.procedure.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.procedure.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.procedure.outcome",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.procedure.contributedToDeath",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.procedure.performed[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.procedure.note", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for FamilyMemberHistory {
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

impl crate::traits::resource::ResourceMutators for FamilyMemberHistory {
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

impl crate::traits::resource::ResourceExistence for FamilyMemberHistory {
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

impl crate::traits::domain_resource::DomainResourceAccessors for FamilyMemberHistory {
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

impl crate::traits::domain_resource::DomainResourceMutators for FamilyMemberHistory {
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

impl crate::traits::domain_resource::DomainResourceExistence for FamilyMemberHistory {
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

impl crate::traits::family_member_history::FamilyMemberHistoryAccessors for FamilyMemberHistory {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn instantiates_canonical(&self) -> &[StringType] {
        self.instantiates_canonical.as_deref().unwrap_or(&[])
    }
    fn instantiates_uri(&self) -> &[StringType] {
        self.instantiates_uri.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> HistoryStatus {
        self.status.clone()
    }
    fn data_absent_reason(&self) -> Option<CodeableConcept> {
        self.data_absent_reason.clone()
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn participant(&self) -> &[FamilyMemberHistoryParticipant] {
        self.participant.as_deref().unwrap_or(&[])
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn relationship(&self) -> CodeableConcept {
        self.relationship.clone()
    }
    fn sex(&self) -> Option<CodeableConcept> {
        self.sex.clone()
    }
    fn estimated_age(&self) -> Option<BooleanType> {
        self.estimated_age
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn condition(&self) -> &[FamilyMemberHistoryCondition] {
        self.condition.as_deref().unwrap_or(&[])
    }
    fn procedure(&self) -> &[FamilyMemberHistoryProcedure] {
        self.procedure.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::family_member_history::FamilyMemberHistoryMutators for FamilyMemberHistory {
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
    fn set_status(self, value: HistoryStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_data_absent_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.data_absent_reason = Some(value);
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_participant(self, value: Vec<FamilyMemberHistoryParticipant>) -> Self {
        let mut resource = self.clone();
        resource.participant = Some(value);
        resource
    }
    fn add_participant(self, item: FamilyMemberHistoryParticipant) -> Self {
        let mut resource = self.clone();
        resource.participant.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_relationship(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.relationship = value;
        resource
    }
    fn set_sex(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.sex = Some(value);
        resource
    }
    fn set_estimated_age(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.estimated_age = Some(value);
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
    fn set_condition(self, value: Vec<FamilyMemberHistoryCondition>) -> Self {
        let mut resource = self.clone();
        resource.condition = Some(value);
        resource
    }
    fn add_condition(self, item: FamilyMemberHistoryCondition) -> Self {
        let mut resource = self.clone();
        resource.condition.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_procedure(self, value: Vec<FamilyMemberHistoryProcedure>) -> Self {
        let mut resource = self.clone();
        resource.procedure = Some(value);
        resource
    }
    fn add_procedure(self, item: FamilyMemberHistoryProcedure) -> Self {
        let mut resource = self.clone();
        resource.procedure.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::family_member_history::FamilyMemberHistoryExistence for FamilyMemberHistory {
    fn has_age(&self) -> bool {
        self.age_age.is_some() || self.age_range.is_some() || self.age_string.is_some()
    }
    fn has_deceased(&self) -> bool {
        self.deceased_boolean.is_some()
            || self.deceased_age.is_some()
            || self.deceased_range.is_some()
            || self.deceased_date.is_some()
            || self.deceased_string.is_some()
    }
    fn has_born(&self) -> bool {
        self.born_period.is_some() || self.born_date.is_some() || self.born_string.is_some()
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
    fn has_status(&self) -> bool {
        true
    }
    fn has_data_absent_reason(&self) -> bool {
        self.data_absent_reason.is_some()
    }
    fn has_patient(&self) -> bool {
        true
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_participant(&self) -> bool {
        self.participant.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_relationship(&self) -> bool {
        true
    }
    fn has_sex(&self) -> bool {
        self.sex.is_some()
    }
    fn has_estimated_age(&self) -> bool {
        self.estimated_age.is_some()
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_condition(&self) -> bool {
        self.condition.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_procedure(&self) -> bool {
        self.procedure.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for FamilyMemberHistory {
    fn resource_type(&self) -> &'static str {
        "FamilyMemberHistory"
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
        Some("http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::family_member_history::{
    FamilyMemberHistoryAccessors, FamilyMemberHistoryExistence, FamilyMemberHistoryMutators,
};
