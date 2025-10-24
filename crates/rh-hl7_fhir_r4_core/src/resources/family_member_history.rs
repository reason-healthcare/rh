use crate::bindings::history_status::HistoryStatus;
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
/// - Version: 4.0.1
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
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Why was family member history performed?
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// General note about related person
    pub note: Option<Vec<Annotation>>,
    /// Condition that the related person had
    pub condition: Option<Vec<FamilyMemberHistoryCondition>>,
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
    /// deceased | permanent disability | etc.
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
/// parent
///
/// Identifies a parent of the relative.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/family-member-history-genetics-parent
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMemberHistoryGeneticsParent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
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
            reason_code: Default::default(),
            reason_reference: Default::default(),
            note: Default::default(),
            condition: Default::default(),
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

impl Default for FamilyMemberHistoryGeneticsParent {
    fn default() -> Self {
        Self {
            base: Extension::default(),
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
    rh_foundation::Invariant::new("fhs-1", rh_foundation::Severity::Error, "Can have age[x] or born[x], but not both", "age.empty() or born.empty()").with_xpath("not (*[starts-with(local-name(.), 'age')] and *[starts-with(local-name(.), 'birth')])"),
    rh_foundation::Invariant::new("fhs-2", rh_foundation::Severity::Error, "Can only have estimatedAge if age[x] is present", "age.exists() or estimatedAge.empty()").with_xpath("exists(*[starts-with(local-name(.), 'age')]) or not(exists(f:estimatedAge))"),
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
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn condition(&self) -> &[FamilyMemberHistoryCondition] {
        self.condition.as_deref().unwrap_or(&[])
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
}

impl crate::traits::family_member_history::FamilyMemberHistoryExistence for FamilyMemberHistory {
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
    fn has_age(&self) -> bool {
        self.age_age.is_some() || self.age_range.is_some() || self.age_string.is_some()
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
    fn has_condition(&self) -> bool {
        self.condition.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for FamilyMemberHistory {
    fn resource_type(&self) -> &'static str {
        "FamilyMemberHistory"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory")
    }
}
