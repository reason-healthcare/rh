use crate::bindings::allergy_intolerance_category::AllergyIntoleranceCategory;
use crate::bindings::allergy_intolerance_criticality::AllergyIntoleranceCriticality;
use crate::bindings::reaction_event_severity::ReactionEventSeverity;
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
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: AllergyIntolerance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyIntolerance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External ids for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// active | inactive | resolved
    #[serde(rename = "clinicalStatus")]
    pub clinical_status: Option<CodeableConcept>,
    /// unconfirmed | presumed | confirmed | refuted | entered-in-error
    #[serde(rename = "verificationStatus")]
    pub verification_status: Option<CodeableConcept>,
    /// allergy | intolerance - Underlying mechanism (if known)
    ///
    /// Binding: preferred (Identification of the underlying physiological mechanism for a Reaction Risk.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/allergy-intolerance-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// food | medication | environment | biologic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<AllergyIntoleranceCategory>,
    /// Extension element for the 'category' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _category: Vec<Element>,
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
    /// Who the allergy or intolerance is for
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
    /// Date allergy or intolerance was first recorded
    #[serde(rename = "recordedDate")]
    pub recorded_date: Option<DateTimeType>,
    /// Extension element for the 'recordedDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_recordedDate")]
    pub _recorded_date: Option<Element>,
    /// Who or what participated in the activities related to the allergy or intolerance and how they were involved
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<AllergyIntoleranceParticipant>,
    /// Date(/time) of last known occurrence of a reaction
    #[serde(rename = "lastOccurrence")]
    pub last_occurrence: Option<DateTimeType>,
    /// Extension element for the 'lastOccurrence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastOccurrence")]
    pub _last_occurrence: Option<Element>,
    /// Additional text not captured in other fields
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// Adverse Reaction Events linked to exposure to substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reaction: Vec<AllergyIntoleranceReaction>,
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
    pub manifestation: Vec<CodeableReference>,
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
    /// Binding: example (A coded concept describing the route or physiological path of exposure to a substance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/route-codes
    #[serde(rename = "exposureRoute")]
    pub exposure_route: Option<CodeableConcept>,
    /// Text about event not captured in other fields
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
}
/// AllergyIntolerance nested structure for the 'participant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyIntoleranceParticipant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of involvement
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/participation-role-type
    pub function: Option<CodeableConcept>,
    /// Who or what participated in the activities related to the allergy or intolerance
    pub actor: Reference,
}

impl Default for AllergyIntolerance {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            clinical_status: Default::default(),
            verification_status: Default::default(),
            type_: Default::default(),
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
            participant: Default::default(),
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

impl Default for AllergyIntoleranceParticipant {
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
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("AllergyIntolerance.category", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/allergy-intolerance-category|5.0.0").with_description("Category of an identified substance associated with allergies or intolerances."),
    rh_foundation::ElementBinding::new("AllergyIntolerance.clinicalStatus", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/allergyintolerance-clinical|5.0.0").with_description("The clinical status of the allergy or intolerance."),
    rh_foundation::ElementBinding::new("AllergyIntolerance.criticality", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/allergy-intolerance-criticality|5.0.0").with_description("Estimate of the potential clinical harm, or seriousness, of a reaction to an identified substance."),
    rh_foundation::ElementBinding::new("AllergyIntolerance.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("AllergyIntolerance.reaction.severity", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/reaction-event-severity|5.0.0").with_description("Clinical assessment of the severity of a reaction event as a whole, potentially considering multiple different manifestations."),
    rh_foundation::ElementBinding::new("AllergyIntolerance.verificationStatus", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/allergyintolerance-verification|5.0.0").with_description("Assertion about certainty associated with a propensity, or potential risk, of a reaction to the identified substance."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("AllergyIntolerance.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.contained", 0, None),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.extension", 0, None),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.identifier", 0, None),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.clinicalStatus", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "AllergyIntolerance.verificationStatus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.category", 0, None),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.criticality", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.patient", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.onset[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.recordedDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.participant", 0, None),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.participant.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "AllergyIntolerance.participant.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AllergyIntolerance.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AllergyIntolerance.participant.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "AllergyIntolerance.participant.actor",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.lastOccurrence", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.note", 0, None),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.reaction", 0, None),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.reaction.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "AllergyIntolerance.reaction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AllergyIntolerance.reaction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AllergyIntolerance.reaction.substance",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "AllergyIntolerance.reaction.manifestation",
                1,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AllergyIntolerance.reaction.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.reaction.onset", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "AllergyIntolerance.reaction.severity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "AllergyIntolerance.reaction.exposureRoute",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("AllergyIntolerance.reaction.note", 0, None),
        ]
    });

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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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

impl crate::traits::domain_resource::DomainResourceExistence for AllergyIntolerance {
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

impl crate::traits::allergy_intolerance::AllergyIntoleranceAccessors for AllergyIntolerance {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn clinical_status(&self) -> Option<CodeableConcept> {
        self.clinical_status.clone()
    }
    fn verification_status(&self) -> Option<CodeableConcept> {
        self.verification_status.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn category(&self) -> &[AllergyIntoleranceCategory] {
        self.category.as_slice()
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
    fn participant(&self) -> &[AllergyIntoleranceParticipant] {
        self.participant.as_slice()
    }
    fn last_occurrence(&self) -> Option<DateTimeType> {
        self.last_occurrence.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
    fn reaction(&self) -> &[AllergyIntoleranceReaction] {
        self.reaction.as_slice()
    }
}

impl crate::traits::allergy_intolerance::AllergyIntoleranceMutators for AllergyIntolerance {
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
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_category(self, value: Vec<AllergyIntoleranceCategory>) -> Self {
        let mut resource = self.clone();
        resource.category = value;
        resource
    }
    fn add_category(self, item: AllergyIntoleranceCategory) -> Self {
        let mut resource = self.clone();
        resource.category.push(item);
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
    fn set_participant(self, value: Vec<AllergyIntoleranceParticipant>) -> Self {
        let mut resource = self.clone();
        resource.participant = value;
        resource
    }
    fn add_participant(self, item: AllergyIntoleranceParticipant) -> Self {
        let mut resource = self.clone();
        resource.participant.push(item);
        resource
    }
    fn set_last_occurrence(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.last_occurrence = Some(value);
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
    fn set_reaction(self, value: Vec<AllergyIntoleranceReaction>) -> Self {
        let mut resource = self.clone();
        resource.reaction = value;
        resource
    }
    fn add_reaction(self, item: AllergyIntoleranceReaction) -> Self {
        let mut resource = self.clone();
        resource.reaction.push(item);
        resource
    }
}

impl crate::traits::allergy_intolerance::AllergyIntoleranceExistence for AllergyIntolerance {
    fn has_onset(&self) -> bool {
        self.onset_date_time.is_some()
            || self.onset_age.is_some()
            || self.onset_period.is_some()
            || self.onset_range.is_some()
            || self.onset_string.is_some()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
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
        !self.category.is_empty()
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
    fn has_participant(&self) -> bool {
        !self.participant.is_empty()
    }
    fn has_last_occurrence(&self) -> bool {
        self.last_occurrence.is_some()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
    }
    fn has_reaction(&self) -> bool {
        !self.reaction.is_empty()
    }
}

impl crate::validation::ValidatableResource for AllergyIntolerance {
    fn resource_type(&self) -> &'static str {
        "AllergyIntolerance"
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
        Some("http://hl7.org/fhir/StructureDefinition/AllergyIntolerance")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::allergy_intolerance::{
    AllergyIntoleranceAccessors, AllergyIntoleranceExistence, AllergyIntoleranceMutators,
};
