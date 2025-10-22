use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::timing::Timing;
use crate::primitives::boolean::BooleanType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Dosage
///
/// Base StructureDefinition for Dosage Type: Indicates how the medication is/was taken or should be taken by the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Dosage
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Dosage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/BackboneElement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dosage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The order of the dosage instructions
    pub sequence: Option<IntegerType>,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Free text dosage instructions e.g. SIG
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
    /// Supplemental instruction or warnings to the patient - e.g. "with meals", "may cause drowsiness"
    ///
    /// Binding: example (A coded concept identifying additional instructions such as "take with water" or "avoid operating heavy machinery".)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/additional-instruction-codes
    #[serde(rename = "additionalInstruction")]
    pub additional_instruction: Option<Vec<CodeableConcept>>,
    /// Patient or consumer oriented instructions
    #[serde(rename = "patientInstruction")]
    pub patient_instruction: Option<StringType>,
    /// Extension element for the 'patientInstruction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_patientInstruction")]
    pub _patient_instruction: Option<Element>,
    /// When medication should be administered
    pub timing: Option<Timing>,
    /// Take "as needed" (for x) (boolean)
    #[serde(rename = "asNeededBoolean")]
    pub as_needed_boolean: Option<BooleanType>,
    /// Take "as needed" (for x) (CodeableConcept)
    #[serde(rename = "asNeededCodeableConcept")]
    pub as_needed_codeable_concept: Option<CodeableConcept>,
    /// Body site to administer to
    ///
    /// Binding: example (A coded concept describing the site location the medicine enters into or onto the body.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/approach-site-codes
    pub site: Option<CodeableConcept>,
    /// How drug should enter body
    ///
    /// Binding: example (A coded concept describing the route or physiological path of administration of a therapeutic agent into or onto the body of a subject.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/route-codes
    pub route: Option<CodeableConcept>,
    /// Technique for administering medication
    ///
    /// Binding: example (A coded concept describing the technique by which the medicine is administered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/administration-method-codes
    pub method: Option<CodeableConcept>,
    /// Amount of medication administered
    #[serde(rename = "doseAndRate")]
    pub dose_and_rate: Option<Vec<Element>>,
    /// Upper limit on medication per unit of time
    #[serde(rename = "maxDosePerPeriod")]
    pub max_dose_per_period: Option<Ratio>,
    /// Upper limit on medication per administration
    #[serde(rename = "maxDosePerAdministration")]
    pub max_dose_per_administration: Option<Quantity>,
    /// Upper limit on medication per lifetime of the patient
    #[serde(rename = "maxDosePerLifetime")]
    pub max_dose_per_lifetime: Option<Quantity>,
}
/// Dosage nested structure for the 'doseAndRate' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DosageDoseandrate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The kind of dose or rate specified
    ///
    /// Binding: example (The kind of dose or rate specified.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/dose-rate-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Amount of medication per dose (Range)
    #[serde(rename = "doseRange")]
    pub dose_range: Option<Range>,
    /// Amount of medication per dose (Quantity)
    #[serde(rename = "doseQuantity")]
    pub dose_quantity: Option<Quantity>,
    /// Amount of medication per unit of time (Ratio)
    #[serde(rename = "rateRatio")]
    pub rate_ratio: Option<Ratio>,
    /// Amount of medication per unit of time (Range)
    #[serde(rename = "rateRange")]
    pub rate_range: Option<Range>,
    /// Amount of medication per unit of time (Quantity)
    #[serde(rename = "rateQuantity")]
    pub rate_quantity: Option<Quantity>,
}

impl Default for Dosage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sequence: Default::default(),
            _sequence: Default::default(),
            text: Default::default(),
            _text: Default::default(),
            additional_instruction: Default::default(),
            patient_instruction: Default::default(),
            _patient_instruction: Default::default(),
            timing: Default::default(),
            as_needed_boolean: Default::default(),
            as_needed_codeable_concept: Default::default(),
            site: Default::default(),
            route: Default::default(),
            method: Default::default(),
            dose_and_rate: Default::default(),
            max_dose_per_period: Default::default(),
            max_dose_per_administration: Default::default(),
            max_dose_per_lifetime: Default::default(),
        }
    }
}

impl Default for DosageDoseandrate {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            dose_range: Default::default(),
            dose_quantity: Default::default(),
            rate_ratio: Default::default(),
            rate_range: Default::default(),
            rate_quantity: Default::default(),
        }
    }
}
