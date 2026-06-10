use crate::bindings::observation_status::ObservationStatus;
use crate::bindings::observation_triggeredbytype::ObservationTriggeredbytype;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::datatypes::sampled_data::SampledData;
use crate::datatypes::timing::Timing;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::instant::InstantType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::primitives::time::TimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Observation
///
/// Measurements and simple assertions made about a patient, device or other subject.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Observation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for observation
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR ObservationDefinition (canonical)
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<StringType>,
    /// Instantiates FHIR ObservationDefinition (Reference)
    #[serde(rename = "instantiatesReference")]
    pub instantiates_reference: Option<Reference>,
    /// Fulfills plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Triggering observation(s)
    #[serde(rename = "triggeredBy")]
    pub triggered_by: Option<Vec<ObservationTriggeredby>>,
    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// registered | preliminary | final | amended +
    pub status: ObservationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Classification of  type of observation
    ///
    /// Binding: preferred (Codes for high level observation categories.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-category
    pub category: Option<Vec<CodeableConcept>>,
    /// Type of observation (code / type)
    ///
    /// Binding: example (Codes identifying names of simple observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-codes
    pub code: CodeableConcept,
    /// Who and/or what the observation is about
    pub subject: Option<Reference>,
    /// What the observation is about, when it is not about the subject of record
    pub focus: Option<Vec<Reference>>,
    /// Healthcare event during which this observation is made
    pub encounter: Option<Reference>,
    /// Clinically relevant time/time-period for observation (dateTime)
    #[serde(rename = "effectiveDateTime")]
    pub effective_date_time: Option<DateTimeType>,
    /// Clinically relevant time/time-period for observation (Period)
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// Clinically relevant time/time-period for observation (Timing)
    #[serde(rename = "effectiveTiming")]
    pub effective_timing: Option<Timing>,
    /// Clinically relevant time/time-period for observation (instant)
    #[serde(rename = "effectiveInstant")]
    pub effective_instant: Option<InstantType>,
    /// Date/Time this version was made available
    pub issued: Option<InstantType>,
    /// Extension element for the 'issued' primitive field. Contains metadata and extensions.
    pub _issued: Option<Element>,
    /// Who is responsible for the observation
    pub performer: Option<Vec<Reference>>,
    /// Actual result (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// Actual result (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// Actual result (string)
    #[serde(rename = "valueString")]
    pub value_string: Option<StringType>,
    /// Actual result (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
    /// Actual result (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: Option<IntegerType>,
    /// Actual result (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Option<Range>,
    /// Actual result (Ratio)
    #[serde(rename = "valueRatio")]
    pub value_ratio: Option<Ratio>,
    /// Actual result (SampledData)
    #[serde(rename = "valueSampledData")]
    pub value_sampled_data: Option<SampledData>,
    /// Actual result (time)
    #[serde(rename = "valueTime")]
    pub value_time: Option<TimeType>,
    /// Actual result (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: Option<DateTimeType>,
    /// Actual result (Period)
    #[serde(rename = "valuePeriod")]
    pub value_period: Option<Period>,
    /// Actual result (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
    /// Actual result (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Option<Reference>,
    /// Why the result is missing
    ///
    /// Binding: extensible (Codes specifying why the result (`Observation.value[x]`) is missing.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/data-absent-reason
    #[serde(rename = "dataAbsentReason")]
    pub data_absent_reason: Option<CodeableConcept>,
    /// High, low, normal, etc
    ///
    /// Binding: extensible (Codes identifying interpretations of observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-interpretation
    pub interpretation: Option<Vec<CodeableConcept>>,
    /// Comments about the observation
    pub note: Option<Vec<Annotation>>,
    /// Observed body part
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
    pub body_site: Option<CodeableConcept>,
    /// Observed body structure
    #[serde(rename = "bodyStructure")]
    pub body_structure: Option<Reference>,
    /// How it was done
    ///
    /// Binding: example (Methods for simple observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-methods
    pub method: Option<CodeableConcept>,
    /// Specimen used for this observation
    pub specimen: Option<Reference>,
    /// A reference to the device that generates the measurements or the device settings for the device
    pub device: Option<Reference>,
    /// Provides guide for interpretation
    #[serde(rename = "referenceRange")]
    pub reference_range: Option<Vec<ObservationReferencerange>>,
    /// Related resource that belongs to the Observation group
    #[serde(rename = "hasMember")]
    pub has_member: Option<Vec<Reference>>,
    /// Related resource from which the observation is made
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Reference>>,
    /// Component results
    pub component: Option<Vec<ObservationComponent>>,
}
/// Observation nested structure for the 'triggeredBy' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationTriggeredby {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Triggering observation
    pub observation: Reference,
    /// reflex | repeat | re-run
    #[serde(rename = "type")]
    pub type_: ObservationTriggeredbytype,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Reason that the observation was triggered
    pub reason: Option<StringType>,
    /// Extension element for the 'reason' primitive field. Contains metadata and extensions.
    pub _reason: Option<Element>,
}
/// Observation nested structure for the 'referenceRange' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationReferencerange {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Low Range, if relevant
    pub low: Option<Quantity>,
    /// High Range, if relevant
    pub high: Option<Quantity>,
    /// Normal value, if relevant
    ///
    /// Binding: extensible (Codes identifying the normal value of the observation.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-referencerange-normalvalue
    #[serde(rename = "normalValue")]
    pub normal_value: Option<CodeableConcept>,
    /// Reference range qualifier
    ///
    /// Binding: preferred (Code for the meaning of a reference range.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/referencerange-meaning
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Reference range population
    ///
    /// Binding: example (Codes identifying the population the reference range applies to.)
    ///
    /// Available values:
    /// - `248153007`
    /// - `248152002`
    /// - `77386006`
    #[serde(rename = "appliesTo")]
    pub applies_to: Option<Vec<CodeableConcept>>,
    /// Applicable age range, if relevant
    pub age: Option<Range>,
    /// Text based reference range in an observation
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
}
/// Observation nested structure for the 'component' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationComponent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of component observation (code / type)
    ///
    /// Binding: example (Codes identifying names of simple observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-codes
    pub code: CodeableConcept,
    /// Actual component result (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// Actual component result (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// Actual component result (string)
    #[serde(rename = "valueString")]
    pub value_string: Option<StringType>,
    /// Actual component result (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
    /// Actual component result (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: Option<IntegerType>,
    /// Actual component result (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Option<Range>,
    /// Actual component result (Ratio)
    #[serde(rename = "valueRatio")]
    pub value_ratio: Option<Ratio>,
    /// Actual component result (SampledData)
    #[serde(rename = "valueSampledData")]
    pub value_sampled_data: Option<SampledData>,
    /// Actual component result (time)
    #[serde(rename = "valueTime")]
    pub value_time: Option<TimeType>,
    /// Actual component result (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: Option<DateTimeType>,
    /// Actual component result (Period)
    #[serde(rename = "valuePeriod")]
    pub value_period: Option<Period>,
    /// Actual component result (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
    /// Actual component result (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Option<Reference>,
    /// Why the component result is missing
    ///
    /// Binding: extensible (Codes specifying why the result (`Observation.value[x]`) is missing.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/data-absent-reason
    #[serde(rename = "dataAbsentReason")]
    pub data_absent_reason: Option<CodeableConcept>,
    /// High, low, normal, etc
    ///
    /// Binding: extensible (Codes identifying interpretations of observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-interpretation
    pub interpretation: Option<Vec<CodeableConcept>>,
    /// Provides guide for interpretation of component result
    #[serde(rename = "referenceRange")]
    pub reference_range: Option<Vec<StringType>>,
}

impl Default for Observation {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            instantiates_canonical: Default::default(),
            instantiates_reference: Default::default(),
            based_on: Default::default(),
            triggered_by: Default::default(),
            part_of: Default::default(),
            status: ObservationStatus::default(),
            _status: Default::default(),
            category: Default::default(),
            code: CodeableConcept::default(),
            subject: Default::default(),
            focus: Default::default(),
            encounter: Default::default(),
            effective_date_time: Default::default(),
            effective_period: Default::default(),
            effective_timing: Default::default(),
            effective_instant: Default::default(),
            issued: Default::default(),
            _issued: Default::default(),
            performer: Default::default(),
            value_quantity: Default::default(),
            value_codeable_concept: Default::default(),
            value_string: Default::default(),
            value_boolean: Default::default(),
            value_integer: Default::default(),
            value_range: Default::default(),
            value_ratio: Default::default(),
            value_sampled_data: Default::default(),
            value_time: Default::default(),
            value_date_time: Default::default(),
            value_period: Default::default(),
            value_attachment: Default::default(),
            value_reference: Default::default(),
            data_absent_reason: Default::default(),
            interpretation: Default::default(),
            note: Default::default(),
            body_site: Default::default(),
            body_structure: Default::default(),
            method: Default::default(),
            specimen: Default::default(),
            device: Default::default(),
            reference_range: Default::default(),
            has_member: Default::default(),
            derived_from: Default::default(),
            component: Default::default(),
        }
    }
}

impl Default for ObservationTriggeredby {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            observation: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            reason: Default::default(),
            _reason: Default::default(),
        }
    }
}

impl Default for ObservationReferencerange {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            low: Default::default(),
            high: Default::default(),
            normal_value: Default::default(),
            type_: Default::default(),
            applies_to: Default::default(),
            age: Default::default(),
            text: Default::default(),
            _text: Default::default(),
        }
    }
}

impl Default for ObservationComponent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: CodeableConcept::default(),
            value_quantity: Default::default(),
            value_codeable_concept: Default::default(),
            value_string: Default::default(),
            value_boolean: Default::default(),
            value_integer: Default::default(),
            value_range: Default::default(),
            value_ratio: Default::default(),
            value_sampled_data: Default::default(),
            value_time: Default::default(),
            value_date_time: Default::default(),
            value_period: Default::default(),
            value_attachment: Default::default(),
            value_reference: Default::default(),
            data_absent_reason: Default::default(),
            interpretation: Default::default(),
            reference_range: Default::default(),
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
    rh_foundation::Invariant::new("obs-3", rh_foundation::Severity::Error, "Must have at least a low or a high or text", "low.exists() or high.exists() or text.exists()"),
    rh_foundation::Invariant::new("obs-6", rh_foundation::Severity::Error, "dataAbsentReason SHALL only be present if Observation.value[x] is not present", "dataAbsentReason.empty() or value.empty()"),
    rh_foundation::Invariant::new("obs-7", rh_foundation::Severity::Error, "If Observation.component.code is the same as Observation.code, then Observation.value SHALL NOT be present (the Observation.component.value[x] holds the value).", "value.empty() or component.code.where(coding.intersect(%resource.code.coding).exists()).empty()"),
    rh_foundation::Invariant::new("obs-8", rh_foundation::Severity::Error, "bodyStructure SHALL only be present if Observation.bodySite is not present", "bodySite.exists() implies bodyStructure.empty()"),
    rh_foundation::Invariant::new("obs-9", rh_foundation::Severity::Error, "If Observation.specimen is a reference to Group, the group can only have specimens", "(reference.resolve().exists() and reference.resolve() is Group) implies reference.resolve().member.entity.resolve().all($this is Specimen)"),
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
                "Observation.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Observation.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/observation-status|5.0.0",
            )
            .with_description("Codes providing the status of an observation."),
            rh_foundation::ElementBinding::new(
                "Observation.triggeredBy.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/observation-triggeredbytype|5.0.0",
            )
            .with_description("The type of TriggeredBy Observation."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Observation.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.contained", 0, None),
            rh_foundation::ElementCardinality::new("Observation.extension", 0, None),
            rh_foundation::ElementCardinality::new("Observation.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Observation.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Observation.instantiates[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("Observation.triggeredBy", 0, None),
            rh_foundation::ElementCardinality::new("Observation.triggeredBy.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.triggeredBy.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Observation.triggeredBy.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Observation.triggeredBy.observation",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Observation.triggeredBy.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.triggeredBy.reason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.partOf", 0, None),
            rh_foundation::ElementCardinality::new("Observation.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.category", 0, None),
            rh_foundation::ElementCardinality::new("Observation.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.focus", 0, None),
            rh_foundation::ElementCardinality::new("Observation.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.effective[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.issued", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.performer", 0, None),
            rh_foundation::ElementCardinality::new("Observation.value[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.dataAbsentReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.interpretation", 0, None),
            rh_foundation::ElementCardinality::new("Observation.note", 0, None),
            rh_foundation::ElementCardinality::new("Observation.bodySite", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.bodyStructure", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.method", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.specimen", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.device", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.referenceRange", 0, None),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Observation.referenceRange.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.low", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.high", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Observation.referenceRange.normalValue",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.appliesTo", 0, None),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.age", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.hasMember", 0, None),
            rh_foundation::ElementCardinality::new("Observation.derivedFrom", 0, None),
            rh_foundation::ElementCardinality::new("Observation.component", 0, None),
            rh_foundation::ElementCardinality::new("Observation.component.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.component.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Observation.component.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Observation.component.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.component.value[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Observation.component.dataAbsentReason",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Observation.component.interpretation", 0, None),
            rh_foundation::ElementCardinality::new("Observation.component.referenceRange", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Observation {
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

impl crate::traits::resource::ResourceMutators for Observation {
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

impl crate::traits::resource::ResourceExistence for Observation {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Observation {
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

impl crate::traits::domain_resource::DomainResourceMutators for Observation {
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

impl crate::traits::domain_resource::DomainResourceExistence for Observation {
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

impl crate::traits::observation::ObservationAccessors for Observation {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn triggered_by(&self) -> &[ObservationTriggeredby] {
        self.triggered_by.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ObservationStatus {
        self.status.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> CodeableConcept {
        self.code.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn focus(&self) -> &[Reference] {
        self.focus.as_deref().unwrap_or(&[])
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn issued(&self) -> Option<InstantType> {
        self.issued.clone()
    }
    fn performer(&self) -> &[Reference] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn data_absent_reason(&self) -> Option<CodeableConcept> {
        self.data_absent_reason.clone()
    }
    fn interpretation(&self) -> &[CodeableConcept] {
        self.interpretation.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn body_site(&self) -> Option<CodeableConcept> {
        self.body_site.clone()
    }
    fn body_structure(&self) -> Option<Reference> {
        self.body_structure.clone()
    }
    fn method(&self) -> Option<CodeableConcept> {
        self.method.clone()
    }
    fn specimen(&self) -> Option<Reference> {
        self.specimen.clone()
    }
    fn device(&self) -> Option<Reference> {
        self.device.clone()
    }
    fn reference_range(&self) -> &[ObservationReferencerange] {
        self.reference_range.as_deref().unwrap_or(&[])
    }
    fn has_member(&self) -> &[Reference] {
        self.has_member.as_deref().unwrap_or(&[])
    }
    fn derived_from(&self) -> &[Reference] {
        self.derived_from.as_deref().unwrap_or(&[])
    }
    fn component(&self) -> &[ObservationComponent] {
        self.component.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::observation::ObservationMutators for Observation {
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
    fn set_triggered_by(self, value: Vec<ObservationTriggeredby>) -> Self {
        let mut resource = self.clone();
        resource.triggered_by = Some(value);
        resource
    }
    fn add_triggered_by(self, item: ObservationTriggeredby) -> Self {
        let mut resource = self.clone();
        resource
            .triggered_by
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
    fn set_status(self, value: ObservationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
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
        resource.code = value;
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_focus(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.focus = Some(value);
        resource
    }
    fn add_focus(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.focus.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_issued(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.issued = Some(value);
        resource
    }
    fn set_performer(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_data_absent_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.data_absent_reason = Some(value);
        resource
    }
    fn set_interpretation(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.interpretation = Some(value);
        resource
    }
    fn add_interpretation(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .interpretation
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
    fn set_body_site(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.body_site = Some(value);
        resource
    }
    fn set_body_structure(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.body_structure = Some(value);
        resource
    }
    fn set_method(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.method = Some(value);
        resource
    }
    fn set_specimen(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.specimen = Some(value);
        resource
    }
    fn set_device(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.device = Some(value);
        resource
    }
    fn set_reference_range(self, value: Vec<ObservationReferencerange>) -> Self {
        let mut resource = self.clone();
        resource.reference_range = Some(value);
        resource
    }
    fn add_reference_range(self, item: ObservationReferencerange) -> Self {
        let mut resource = self.clone();
        resource
            .reference_range
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_has_member(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.has_member = Some(value);
        resource
    }
    fn add_has_member(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.has_member.get_or_insert_with(Vec::new).push(item);
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
    fn set_component(self, value: Vec<ObservationComponent>) -> Self {
        let mut resource = self.clone();
        resource.component = Some(value);
        resource
    }
    fn add_component(self, item: ObservationComponent) -> Self {
        let mut resource = self.clone();
        resource.component.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::observation::ObservationExistence for Observation {
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
    fn has_value(&self) -> bool {
        self.value_quantity.is_some()
            || self.value_codeable_concept.is_some()
            || self.value_string.is_some()
            || self.value_boolean.is_some()
            || self.value_integer.is_some()
            || self.value_range.is_some()
            || self.value_ratio.is_some()
            || self.value_sampled_data.is_some()
            || self.value_time.is_some()
            || self.value_date_time.is_some()
            || self.value_period.is_some()
            || self.value_attachment.is_some()
            || self.value_reference.is_some()
    }
    fn has_instantiates(&self) -> bool {
        self.instantiates_canonical.is_some() || self.instantiates_reference.is_some()
    }
    fn has_effective(&self) -> bool {
        self.effective_date_time.is_some()
            || self.effective_period.is_some()
            || self.effective_timing.is_some()
            || self.effective_instant.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_triggered_by(&self) -> bool {
        self.triggered_by.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_focus(&self) -> bool {
        self.focus.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_issued(&self) -> bool {
        self.issued.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_data_absent_reason(&self) -> bool {
        self.data_absent_reason.is_some()
    }
    fn has_interpretation(&self) -> bool {
        self.interpretation.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_body_site(&self) -> bool {
        self.body_site.is_some()
    }
    fn has_body_structure(&self) -> bool {
        self.body_structure.is_some()
    }
    fn has_method(&self) -> bool {
        self.method.is_some()
    }
    fn has_specimen(&self) -> bool {
        self.specimen.is_some()
    }
    fn has_device(&self) -> bool {
        self.device.is_some()
    }
    fn has_reference_range(&self) -> bool {
        self.reference_range.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_has_member(&self) -> bool {
        self.has_member.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_derived_from(&self) -> bool {
        self.derived_from.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_component(&self) -> bool {
        self.component.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Observation {
    fn resource_type(&self) -> &'static str {
        "Observation"
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
        Some("http://hl7.org/fhir/StructureDefinition/Observation")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::observation::{
    ObservationAccessors, ObservationExistence, ObservationMutators,
};
