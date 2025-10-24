use crate::bindings::observation_status::ObservationStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
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
/// - Version: 4.0.1
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
    /// Fulfills plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
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
    /// Why the result is missing
    ///
    /// Binding: extensible (Codes specifying why the result (`Observation.value[x]`) is missing.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/data-absent-reason
    #[serde(rename = "dataAbsentReason")]
    pub data_absent_reason: Option<CodeableConcept>,
    /// High, low, normal, etc.
    ///
    /// Binding: extensible (Codes identifying interpretations of observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-interpretation
    pub interpretation: Option<Vec<CodeableConcept>>,
    /// Comments about the observation
    pub note: Option<Vec<Annotation>>,
    /// Observed body part
    ///
    /// Binding: example (Codes describing anatomical locations. May include laterality.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/body-site
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,
    /// How it was done
    ///
    /// Binding: example (Methods for simple observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-methods
    pub method: Option<CodeableConcept>,
    /// Specimen used for this observation
    pub specimen: Option<Reference>,
    /// (Measurement) Device
    pub device: Option<Reference>,
    /// Provides guide for interpretation
    #[serde(rename = "referenceRange")]
    pub reference_range: Option<Vec<ObservationReferencerange>>,
    /// Related resource that belongs to the Observation group
    #[serde(rename = "hasMember")]
    pub has_member: Option<Vec<Reference>>,
    /// Related measurements the observation is made from
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Reference>>,
    /// Component results
    pub component: Option<Vec<ObservationComponent>>,
}
/// Sequel To
///
/// This observation follows the target observation (e.g. timed tests such as Glucose Tolerance Test).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-sequelTo
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationSequelTo {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
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
    /// Why the component result is missing
    ///
    /// Binding: extensible (Codes specifying why the result (`Observation.value[x]`) is missing.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/data-absent-reason
    #[serde(rename = "dataAbsentReason")]
    pub data_absent_reason: Option<CodeableConcept>,
    /// High, low, normal, etc.
    ///
    /// Binding: extensible (Codes identifying interpretations of observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-interpretation
    pub interpretation: Option<Vec<CodeableConcept>>,
    /// Provides guide for interpretation of component result
    #[serde(rename = "referenceRange")]
    pub reference_range: Option<Vec<StringType>>,
}
/// Observation-genetics
///
/// Describes how the observation resource is used to report structured genetic test results
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ObservationGenetics {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Observation,
}
/// Replaces
///
/// This observation replaces a previous observation (i.e. a revised value).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-replaces
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationReplaces {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Precondition
///
/// Other preceding or concurrent observations that must be known to correctly interpret the the observation.  For example an fiO2 measure taken alongside of a SpO2 measurement.  See the [Observation notes](observation.html#notes) section for additional guidance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-precondition
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationPrecondition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Device Code
///
/// A code representing the the type of device used for this observation.  Should only be used if not implicit in the code found in `Observation.code`.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-deviceCode
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationDeviceCode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Gateway Device
///
/// The Provenance/AuditEvent resources can represent the same information.  Note that the Provenance/AuditEvent resources can represent the same information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-gatewayDevice
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGatewayDevice {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Variant
///
/// Variant information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsVariant
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsVariant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// AminoAcidChange
///
/// AminoAcidChange information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsAminoAcidChange
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsAminoAcidChange {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Specimen Code
///
/// A code representing the the type of specimen used for this observation.  Should only be used if not implicit in the code found in `Observation.code`.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-specimenCode
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationSpecimenCode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// GenomicSourceClass
///
/// Source of sample used to determine the sequence in sequencing lab -- germline, somatic, prenatal. LOINC Code: ([48002-0](http://loinc.org/48002-0)).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsGenomicSourceClass
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsGenomicSourceClass {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// CopyNumberEvent
///
/// A variation that increases or decreases the copy number of a given region ([SO:0001019](http://www.sequenceontology.org/browser/current_svn/term/SO:0001019)). Values: amplification/deletion/LOH.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsCopyNumberEvent
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsCopyNumberEvent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Time-offset
///
/// A specific offset time in milliseconds from the stated time in the Observation.appliesDateTime to allow for representation of sequential recording  of sampled data from the same lead or data stream.  For example, an ECG recorder may record sequentially 3 leads four time to receive 12-lead ECG, see [ISO 22077](https://www.iso.org/obp/ui/#iso:std:61871:en).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-timeOffset
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationTimeOffset {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// PhaseSet
///
/// Phase set information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsPhaseSet
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsPhaseSet {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// delta
///
/// The qualitative change in the value relative to the previous measurement. Usually only recorded if the change is clinically significant.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-delta
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationDelta {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Secondary Finding
///
/// Secondary findings are genetic test results that provide information about variants in a gene unrelated to the primary purpose for the testing, most often discovered when [Whole Exome Sequencing (WES)](https://en.wikipedia.org/wiki/Exome_sequencing) or [Whole Genome Sequencing (WGS)](https://en.wikipedia.org/wiki/Whole_genome_sequencing) is performed. This extension should be used to denote when a genetic finding is being shared as a secondary finding, and ideally refer to a corresponding guideline or policy statement.  For more detail, please see: https://ghr.nlm.nih.gov/primer/testing/secondaryfindings.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-secondaryFinding
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationSecondaryFinding {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Reagent
///
/// Reference to reagents used to generate this observation.  This is intended for this for in-lab transactions between instruments and Laboratory Information Systems (LIS).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-reagent
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationReagent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// DNARegionName
///
/// A human readable name for the region of interest. Typically Exon #, Intron # or other. NOTE: This is not standardized and is mainly for convenience and display purposes.  LOINC Code: ([47999-8](http://loinc.org/47999-8)).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsDNARegionName
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsDNARegionName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Gene
///
/// A region (or regions) that includes all of the sequence elements necessary to encode a functional transcript. A gene may include regulatory regions, transcribed regions and/or other functional sequence regions ([SO:0000704](http://www.sequenceontology.org/browser/current_svn/term/SO:0000704)). This element is the official gene symbol approved by the HGNC, which is a short abbreviated form of the gene name ([HGNC](http://www.genenames.org)). LOINC Code: ([48018-6](http://loinc.org/48018-6)).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsGene
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsGene {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Interpretation
///
/// Clinical Interpretations for variant. It's a reference to an Observation resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsInterpretation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsInterpretation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
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
/// Allele
///
/// Allele information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsAllele
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsAllele {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Ancestry
///
/// Ancestry information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsAncestry
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsAncestry {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Focal Subject Code
///
/// A code representing the  focus of an observation when the focus is not the patient of record.  In other words, the focus of the observation is different from `Observation.subject`.   An example use case would be using the *Observation* resource to capture whether the mother is trained to change her child's tracheostomy tube.  In this example, the child is the patient of record and the mother is focal subject referenced using this extension.  Other example focal subjects include spouses, related persons, feti, or  donors.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-focusCode
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationFocusCode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// bodyPosition
///
/// The position of the body when the observation was done, e.g. standing, sitting. To be used only when the body position in not precoordinated in the observation code.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-bodyPosition
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationBodyPosition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Observation {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
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
            data_absent_reason: Default::default(),
            interpretation: Default::default(),
            note: Default::default(),
            body_site: Default::default(),
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

impl Default for ObservationSequelTo {
    fn default() -> Self {
        Self {
            base: Extension::default(),
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
            data_absent_reason: Default::default(),
            interpretation: Default::default(),
            reference_range: Default::default(),
        }
    }
}

impl Default for ObservationReplaces {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationPrecondition {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationDeviceCode {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationGatewayDevice {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationGeneticsVariant {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationGeneticsAminoAcidChange {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationSpecimenCode {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationGeneticsGenomicSourceClass {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationGeneticsCopyNumberEvent {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationTimeOffset {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationGeneticsPhaseSet {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationDelta {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationSecondaryFinding {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationReagent {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationGeneticsDNARegionName {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationGeneticsGene {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationGeneticsInterpretation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationReferencerange {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            low: Default::default(),
            high: Default::default(),
            type_: Default::default(),
            applies_to: Default::default(),
            age: Default::default(),
            text: Default::default(),
            _text: Default::default(),
        }
    }
}

impl Default for ObservationGeneticsAllele {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationGeneticsAncestry {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationFocusCode {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ObservationBodyPosition {
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
    rh_foundation::Invariant::new("obs-3", rh_foundation::Severity::Error, "Must have at least a low or a high or text", "low.exists() or high.exists() or text.exists()").with_xpath("(exists(f:low) or exists(f:high)or exists(f:text))"),
    rh_foundation::Invariant::new("obs-6", rh_foundation::Severity::Error, "dataAbsentReason SHALL only be present if Observation.value[x] is not present", "dataAbsentReason.empty() or value.empty()").with_xpath("not(exists(f:dataAbsentReason)) or (not(exists(*[starts-with(local-name(.), 'value')])))"),
    rh_foundation::Invariant::new("obs-7", rh_foundation::Severity::Error, "If Observation.code is the same as an Observation.component.code then the value element associated with the code SHALL NOT be present", "value.empty() or component.code.where(coding.intersect(%resource.code.coding).exists()).empty()").with_xpath("not(f:*[starts-with(local-name(.), 'value')] and (for $coding in f:code/f:coding return f:component/f:code/f:coding[f:code/@value=$coding/f:code/@value] [f:system/@value=$coding/f:system/@value]))"),
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
    fn has_effective(&self) -> bool {
        self.effective_date_time.is_some()
            || self.effective_period.is_some()
            || self.effective_timing.is_some()
            || self.effective_instant.is_some()
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

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Observation")
    }
}
