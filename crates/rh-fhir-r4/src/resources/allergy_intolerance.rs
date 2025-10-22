use crate::bindings::allergy_intolerance_category::AllergyIntoleranceCategory;
use crate::bindings::allergy_intolerance_criticality::AllergyIntoleranceCriticality;
use crate::bindings::allergy_intolerance_type::AllergyIntoleranceType;
use crate::bindings::reaction_event_severity::ReactionEventSeverity;
use crate::datatypes::age::Age;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// AllergyIntolerance
///
/// Risk of harmful or undesirable, physiological response which is unique to an individual and associated with exposure to a substance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AllergyIntolerance
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AllergyIntolerance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyIntolerance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External ids for this item
    pub identifier: Option<Vec<Identifier>>,
    /// active | inactive | resolved
    #[serde(rename = "clinicalStatus")]
    pub clinical_status: Option<CodeableConcept>,
    /// unconfirmed | confirmed | refuted | entered-in-error
    #[serde(rename = "verificationStatus")]
    pub verification_status: Option<CodeableConcept>,
    /// allergy | intolerance - Underlying mechanism (if known)
    #[serde(rename = "type")]
    pub type_: Option<AllergyIntoleranceType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// food | medication | environment | biologic
    pub category: Option<Vec<AllergyIntoleranceCategory>>,
    /// Extension element for the 'category' primitive field. Contains metadata and extensions.
    pub _category: Option<Element>,
    /// low | high | unable-to-assess
    pub criticality: Option<AllergyIntoleranceCriticality>,
    /// Extension element for the 'criticality' primitive field. Contains metadata and extensions.
    pub _criticality: Option<Element>,
    /// Code that identifies the allergy or intolerance
    ///
    /// Binding: example (Type of the substance/product, allergy or intolerance condition, or negation/exclusion codes for reporting no known allergies.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/allergyintolerance-code
    pub code: Option<CodeableConcept>,
    /// Who the sensitivity is for
    pub patient: Reference,
    /// Encounter when the allergy or intolerance was asserted
    pub encounter: Option<Reference>,
    /// When allergy or intolerance was identified (dateTime)
    #[serde(rename = "onsetDateTime")]
    pub onset_date_time: Option<DateTimeType>,
    /// When allergy or intolerance was identified (Age)
    #[serde(rename = "onsetAge")]
    pub onset_age: Option<Age>,
    /// When allergy or intolerance was identified (Period)
    #[serde(rename = "onsetPeriod")]
    pub onset_period: Option<Period>,
    /// When allergy or intolerance was identified (Range)
    #[serde(rename = "onsetRange")]
    pub onset_range: Option<Range>,
    /// When allergy or intolerance was identified (string)
    #[serde(rename = "onsetString")]
    pub onset_string: Option<StringType>,
    /// Date first version of the resource instance was recorded
    #[serde(rename = "recordedDate")]
    pub recorded_date: Option<DateTimeType>,
    /// Extension element for the 'recordedDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_recordedDate")]
    pub _recorded_date: Option<Element>,
    /// Who recorded the sensitivity
    pub recorder: Option<Reference>,
    /// Source of the information about the allergy
    pub asserter: Option<Reference>,
    /// Date(/time) of last known occurrence of a reaction
    #[serde(rename = "lastOccurrence")]
    pub last_occurrence: Option<DateTimeType>,
    /// Extension element for the 'lastOccurrence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastOccurrence")]
    pub _last_occurrence: Option<Element>,
    /// Additional text not captured in other fields
    pub note: Option<Vec<Annotation>>,
    /// Adverse Reaction Events linked to exposure to substance
    pub reaction: Option<Vec<AllergyIntoleranceReaction>>,
}
/// AllergyIntolerance nested structure for the 'reaction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyIntoleranceReaction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Specific substance or pharmaceutical product considered to be responsible for event
    ///
    /// Binding: example (Codes defining the type of the substance (including pharmaceutical products).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-code
    pub substance: Option<CodeableConcept>,
    /// Clinical symptoms/signs associated with the Event
    ///
    /// Binding: example (Clinical symptoms and/or signs that are observed or associated with an Adverse Reaction Event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinical-findings
    pub manifestation: Vec<CodeableConcept>,
    /// Description of the event as a whole
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Date(/time) when manifestations showed
    pub onset: Option<DateTimeType>,
    /// Extension element for the 'onset' primitive field. Contains metadata and extensions.
    pub _onset: Option<Element>,
    /// mild | moderate | severe (of event as a whole)
    pub severity: Option<ReactionEventSeverity>,
    /// Extension element for the 'severity' primitive field. Contains metadata and extensions.
    pub _severity: Option<Element>,
    /// How the subject was exposed to the substance
    ///
    /// Binding: example (A coded concept describing the route or physiological path of administration of a therapeutic agent into or onto the body of a subject.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/route-codes
    #[serde(rename = "exposureRoute")]
    pub exposure_route: Option<CodeableConcept>,
    /// Text about event not captured in other fields
    pub note: Option<Vec<Annotation>>,
}

impl Default for AllergyIntolerance {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            clinical_status: Default::default(),
            verification_status: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            category: Default::default(),
            _category: Default::default(),
            criticality: Default::default(),
            _criticality: Default::default(),
            code: Default::default(),
            patient: Reference::default(),
            encounter: Default::default(),
            onset_date_time: Default::default(),
            onset_age: Default::default(),
            onset_period: Default::default(),
            onset_range: Default::default(),
            onset_string: Default::default(),
            recorded_date: Default::default(),
            _recorded_date: Default::default(),
            recorder: Default::default(),
            asserter: Default::default(),
            last_occurrence: Default::default(),
            _last_occurrence: Default::default(),
            note: Default::default(),
            reaction: Default::default(),
        }
    }
}

impl Default for AllergyIntoleranceReaction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            substance: Default::default(),
            manifestation: Vec::new(),
            description: Default::default(),
            _description: Default::default(),
            onset: Default::default(),
            _onset: Default::default(),
            severity: Default::default(),
            _severity: Default::default(),
            exposure_route: Default::default(),
            note: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for AllergyIntolerance {
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

impl crate::traits::resource::ResourceMutators for AllergyIntolerance {
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

impl crate::traits::resource::ResourceExistence for AllergyIntolerance {
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

impl crate::traits::domain_resource::DomainResourceAccessors for AllergyIntolerance {
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

impl crate::traits::domain_resource::DomainResourceMutators for AllergyIntolerance {
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

impl crate::traits::domain_resource::DomainResourceExistence for AllergyIntolerance {
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

impl crate::traits::allergy_intolerance::AllergyIntoleranceAccessors for AllergyIntolerance {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn clinical_status(&self) -> Option<CodeableConcept> {
        self.clinical_status.clone()
    }
    fn verification_status(&self) -> Option<CodeableConcept> {
        self.verification_status.clone()
    }
    fn type_(&self) -> Option<AllergyIntoleranceType> {
        self.type_.clone()
    }
    fn category(&self) -> &[AllergyIntoleranceCategory] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn criticality(&self) -> Option<AllergyIntoleranceCriticality> {
        self.criticality.clone()
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn recorded_date(&self) -> Option<DateTimeType> {
        self.recorded_date.clone()
    }
    fn recorder(&self) -> Option<Reference> {
        self.recorder.clone()
    }
    fn asserter(&self) -> Option<Reference> {
        self.asserter.clone()
    }
    fn last_occurrence(&self) -> Option<DateTimeType> {
        self.last_occurrence.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn reaction(&self) -> &[AllergyIntoleranceReaction] {
        self.reaction.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::allergy_intolerance::AllergyIntoleranceMutators for AllergyIntolerance {
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
    fn set_clinical_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.clinical_status = Some(value);
        resource
    }
    fn set_verification_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.verification_status = Some(value);
        resource
    }
    fn set_type_(self, value: AllergyIntoleranceType) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_category(self, value: Vec<AllergyIntoleranceCategory>) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn add_category(self, item: AllergyIntoleranceCategory) -> Self {
        let mut resource = self.clone();
        resource.category.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_criticality(self, value: AllergyIntoleranceCriticality) -> Self {
        let mut resource = self.clone();
        resource.criticality = Some(value);
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_recorded_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.recorded_date = Some(value);
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
    fn set_last_occurrence(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.last_occurrence = Some(value);
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
    fn set_reaction(self, value: Vec<AllergyIntoleranceReaction>) -> Self {
        let mut resource = self.clone();
        resource.reaction = Some(value);
        resource
    }
    fn add_reaction(self, item: AllergyIntoleranceReaction) -> Self {
        let mut resource = self.clone();
        resource.reaction.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::allergy_intolerance::AllergyIntoleranceExistence for AllergyIntolerance {
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
    fn has_onset(&self) -> bool {
        self.onset_date_time.is_some()
            || self.onset_age.is_some()
            || self.onset_period.is_some()
            || self.onset_range.is_some()
            || self.onset_string.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_clinical_status(&self) -> bool {
        self.clinical_status.is_some()
    }
    fn has_verification_status(&self) -> bool {
        self.verification_status.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_criticality(&self) -> bool {
        self.criticality.is_some()
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_patient(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_recorded_date(&self) -> bool {
        self.recorded_date.is_some()
    }
    fn has_recorder(&self) -> bool {
        self.recorder.is_some()
    }
    fn has_asserter(&self) -> bool {
        self.asserter.is_some()
    }
    fn has_last_occurrence(&self) -> bool {
        self.last_occurrence.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reaction(&self) -> bool {
        self.reaction.as_ref().is_some_and(|v| !v.is_empty())
    }
}
