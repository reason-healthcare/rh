use crate::bindings::medicationknowledge_status::MedicationknowledgeStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::money::Money;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::base64binary::Base64BinaryType;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicationKnowledge
///
/// Information about a medication that is used to support knowledge.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationKnowledge
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationKnowledge
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledge {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Code that identifies this medication
    ///
    /// Binding: example (A coded concept that defines the type of a medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-codes
    pub code: Option<CodeableConcept>,
    /// active | inactive | entered-in-error
    pub status: Option<MedicationknowledgeStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Manufacturer of the item
    pub manufacturer: Option<Reference>,
    /// powder | tablets | capsule +
    ///
    /// Binding: example (A coded concept defining the form of a medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-form-codes
    #[serde(rename = "doseForm")]
    pub dose_form: Option<CodeableConcept>,
    /// Amount of drug in package
    pub amount: Option<Quantity>,
    /// Additional names for a medication
    pub synonym: Option<Vec<StringType>>,
    /// Extension element for the 'synonym' primitive field. Contains metadata and extensions.
    pub _synonym: Option<Element>,
    /// Associated or related medication information
    #[serde(rename = "relatedMedicationKnowledge")]
    pub related_medication_knowledge: Option<Vec<MedicationKnowledgeRelatedmedicationknowledge>>,
    /// A medication resource that is associated with this medication
    #[serde(rename = "associatedMedication")]
    pub associated_medication: Option<Vec<Reference>>,
    /// Category of the medication or product
    #[serde(rename = "productType")]
    pub product_type: Option<Vec<CodeableConcept>>,
    /// Associated documentation about the medication
    pub monograph: Option<Vec<MedicationKnowledgeMonograph>>,
    /// Active or inactive ingredient
    pub ingredient: Option<Vec<MedicationKnowledgeIngredient>>,
    /// The instructions for preparing the medication
    #[serde(rename = "preparationInstruction")]
    pub preparation_instruction: Option<StringType>,
    /// Extension element for the 'preparationInstruction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_preparationInstruction")]
    pub _preparation_instruction: Option<Element>,
    /// The intended or approved route of administration
    ///
    /// Binding: example (A coded concept defining the intended route of administration.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/route-codes
    #[serde(rename = "intendedRoute")]
    pub intended_route: Option<Vec<CodeableConcept>>,
    /// The pricing of the medication
    pub cost: Option<Vec<MedicationKnowledgeCost>>,
    /// Program under which a medication is reviewed
    #[serde(rename = "monitoringProgram")]
    pub monitoring_program: Option<Vec<MedicationKnowledgeMonitoringprogram>>,
    /// Guidelines for administration of the medication
    #[serde(rename = "administrationGuidelines")]
    pub administration_guidelines: Option<Vec<MedicationKnowledgeAdministrationguidelines>>,
    /// Categorization of the medication within a formulary or classification system
    #[serde(rename = "medicineClassification")]
    pub medicine_classification: Option<Vec<MedicationKnowledgeMedicineclassification>>,
    /// Details about packaged medications
    pub packaging: Option<MedicationKnowledgePackaging>,
    /// Specifies descriptive properties of the medicine
    #[serde(rename = "drugCharacteristic")]
    pub drug_characteristic: Option<Vec<MedicationKnowledgeDrugcharacteristic>>,
    /// Potential clinical issue with or between medication(s)
    pub contraindication: Option<Vec<Reference>>,
    /// Regulatory information about a medication
    pub regulatory: Option<Vec<MedicationKnowledgeRegulatory>>,
    /// The time course of drug absorption, distribution, metabolism and excretion of a medication from the body
    pub kinetics: Option<Vec<MedicationKnowledgeKinetics>>,
}
/// MedicationKnowledge nested structure for the 'kinetics' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeKinetics {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The drug concentration measured at certain discrete points in time
    #[serde(rename = "areaUnderCurve")]
    pub area_under_curve: Option<Vec<Quantity>>,
    /// The median lethal dose of a drug
    #[serde(rename = "lethalDose50")]
    pub lethal_dose50: Option<Vec<Quantity>>,
    /// Time required for concentration in the body to decrease by half
    #[serde(rename = "halfLifePeriod")]
    pub half_life_period: Option<Duration>,
}
/// MedicationKnowledgeAdministrationguidelines nested structure for the 'patientCharacteristics' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeAdministrationguidelinesPatientcharacteristics {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Specific characteristic that is relevant to the administration guideline (CodeableConcept)
    #[serde(rename = "characteristicCodeableConcept")]
    pub characteristic_codeable_concept: CodeableConcept,
    /// Specific characteristic that is relevant to the administration guideline (Quantity)
    #[serde(rename = "characteristicQuantity")]
    pub characteristic_quantity: Quantity,
    /// The specific characteristic
    pub value: Option<Vec<StringType>>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}
/// MedicationKnowledge nested structure for the 'monitoringProgram' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeMonitoringprogram {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of program under which the medication is monitored
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Name of the reviewing program
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
}
/// MedicationKnowledgeRegulatory nested structure for the 'maxDispense' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeRegulatoryMaxdispense {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The maximum number of units of the medication that can be dispensed
    pub quantity: Quantity,
    /// The period that applies to the maximum number of units
    pub period: Option<Duration>,
}
/// MedicationKnowledgeAdministrationguidelines nested structure for the 'dosage' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeAdministrationguidelinesDosage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of dosage
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Dosage for the medication for the specific guidelines
    pub dosage: Vec<Dosage>,
}
/// MedicationKnowledge nested structure for the 'relatedMedicationKnowledge' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeRelatedmedicationknowledge {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Category of medicationKnowledge
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Associated documentation about the associated medication knowledge
    pub reference: Vec<Reference>,
}
/// MedicationKnowledge nested structure for the 'administrationGuidelines' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeAdministrationguidelines {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Dosage for the medication for the specific guidelines
    pub dosage: Option<Vec<MedicationKnowledgeAdministrationguidelinesDosage>>,
    /// Characteristics of the patient that are relevant to the administration guidelines
    #[serde(rename = "patientCharacteristics")]
    pub patient_characteristics:
        Option<Vec<MedicationKnowledgeAdministrationguidelinesPatientcharacteristics>>,
    /// Indication for use that apply to the specific administration guidelines (CodeableConcept)
    #[serde(rename = "indicationCodeableConcept")]
    pub indication_codeable_concept: Option<CodeableConcept>,
    /// Indication for use that apply to the specific administration guidelines (Reference)
    #[serde(rename = "indicationReference")]
    pub indication_reference: Option<Reference>,
}
/// MedicationKnowledgeRegulatory nested structure for the 'schedule' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeRegulatorySchedule {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Specifies the specific drug schedule
    pub schedule: CodeableConcept,
}
/// MedicationKnowledge nested structure for the 'ingredient' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeIngredient {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Medication(s) or substance(s) contained in the medication (CodeableConcept)
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: CodeableConcept,
    /// Medication(s) or substance(s) contained in the medication (Reference)
    #[serde(rename = "itemReference")]
    pub item_reference: Reference,
    /// Active ingredient indicator
    #[serde(rename = "isActive")]
    pub is_active: Option<BooleanType>,
    /// Extension element for the 'isActive' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isActive")]
    pub _is_active: Option<Element>,
    /// Quantity of ingredient present
    pub strength: Option<Ratio>,
}
/// MedicationKnowledge nested structure for the 'packaging' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgePackaging {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A code that defines the specific type of packaging that the medication can be found in
    ///
    /// Binding: example (A coded concept defining the type of packaging of a medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicationknowledge-package-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The number of product units the package would contain if fully loaded
    pub quantity: Option<Quantity>,
}
/// MedicationKnowledge nested structure for the 'monograph' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeMonograph {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The category of medication document
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Associated documentation about the medication
    pub source: Option<Reference>,
}
/// MedicationKnowledge nested structure for the 'cost' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeCost {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The category of the cost information
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// The source or owner for the price information
    pub source: Option<StringType>,
    /// Extension element for the 'source' primitive field. Contains metadata and extensions.
    pub _source: Option<Element>,
    /// The price of the medication
    pub cost: Money,
}
/// MedicationKnowledgeRegulatory nested structure for the 'substitution' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeRegulatorySubstitution {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Specifies the type of substitution allowed
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Specifies if regulation allows for changes in the medication when dispensing
    pub allowed: BooleanType,
    /// Extension element for the 'allowed' primitive field. Contains metadata and extensions.
    pub _allowed: Option<Element>,
}
/// MedicationKnowledge nested structure for the 'regulatory' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeRegulatory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Specifies if changes are allowed when dispensing a medication from a regulatory perspective
    pub substitution: Option<Vec<MedicationKnowledgeRegulatorySubstitution>>,
    /// Specifies the schedule of a medication in jurisdiction
    pub schedule: Option<Vec<MedicationKnowledgeRegulatorySchedule>>,
    /// The maximum number of units of the medication that can be dispensed in a period
    #[serde(rename = "maxDispense")]
    pub max_dispense: Option<MedicationKnowledgeRegulatoryMaxdispense>,
    /// Specifies the authority of the regulation
    #[serde(rename = "regulatoryAuthority")]
    pub regulatory_authority: Reference,
}
/// MedicationKnowledge nested structure for the 'drugCharacteristic' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeDrugcharacteristic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code specifying the type of characteristic of medication
    ///
    /// Binding: example (A coded concept defining the characteristic types of a medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicationknowledge-characteristic
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Description of the characteristic (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// Description of the characteristic (string)
    #[serde(rename = "valueString")]
    pub value_string: Option<StringType>,
    /// Description of the characteristic (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// Description of the characteristic (base64Binary)
    #[serde(rename = "valueBase64Binary")]
    pub value_base64_binary: Option<Base64BinaryType>,
}
/// MedicationKnowledge nested structure for the 'medicineClassification' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeMedicineclassification {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of category for the medication (for example, therapeutic classification, therapeutic sub-classification)
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Specific category assigned to the medication
    pub classification: Option<Vec<CodeableConcept>>,
}

impl Default for MedicationKnowledge {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            code: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            manufacturer: Default::default(),
            dose_form: Default::default(),
            amount: Default::default(),
            synonym: Default::default(),
            _synonym: Default::default(),
            related_medication_knowledge: Default::default(),
            associated_medication: Default::default(),
            product_type: Default::default(),
            monograph: Default::default(),
            ingredient: Default::default(),
            preparation_instruction: Default::default(),
            _preparation_instruction: Default::default(),
            intended_route: Default::default(),
            cost: Default::default(),
            monitoring_program: Default::default(),
            administration_guidelines: Default::default(),
            medicine_classification: Default::default(),
            packaging: Default::default(),
            drug_characteristic: Default::default(),
            contraindication: Default::default(),
            regulatory: Default::default(),
            kinetics: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeKinetics {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            area_under_curve: Default::default(),
            lethal_dose50: Default::default(),
            half_life_period: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeAdministrationguidelinesPatientcharacteristics {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            characteristic_codeable_concept: Default::default(),
            characteristic_quantity: Default::default(),
            value: Default::default(),
            _value: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeMonitoringprogram {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            name: Default::default(),
            _name: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeRegulatoryMaxdispense {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            quantity: Default::default(),
            period: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeAdministrationguidelinesDosage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            dosage: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeRelatedmedicationknowledge {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            reference: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeAdministrationguidelines {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            dosage: Default::default(),
            patient_characteristics: Default::default(),
            indication_codeable_concept: Default::default(),
            indication_reference: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeRegulatorySchedule {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            schedule: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeIngredient {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item_codeable_concept: Default::default(),
            item_reference: Default::default(),
            is_active: Default::default(),
            _is_active: Default::default(),
            strength: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgePackaging {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            quantity: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeMonograph {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            source: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeCost {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            source: Default::default(),
            _source: Default::default(),
            cost: Money::default(),
        }
    }
}

impl Default for MedicationKnowledgeRegulatorySubstitution {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            allowed: Default::default(),
            _allowed: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeRegulatory {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            substitution: Default::default(),
            schedule: Default::default(),
            max_dispense: Default::default(),
            regulatory_authority: Reference::default(),
        }
    }
}

impl Default for MedicationKnowledgeDrugcharacteristic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_codeable_concept: Default::default(),
            value_string: Default::default(),
            value_quantity: Default::default(),
            value_base64_binary: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeMedicineclassification {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            classification: Default::default(),
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
            "MedicationKnowledge.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/medicationknowledge-status|4.0.1",
        )
        .with_description("A coded concept defining if the medication is in active use.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementCardinality::new("MedicationKnowledge.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.meta", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.implicitRules", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.language", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.text", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.contained", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.code", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.status", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.manufacturer", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.doseForm", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.amount", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.synonym", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.relatedMedicationKnowledge", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.relatedMedicationKnowledge.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.relatedMedicationKnowledge.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.relatedMedicationKnowledge.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.relatedMedicationKnowledge.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.relatedMedicationKnowledge.reference", 1, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.associatedMedication", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.productType", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monograph", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monograph.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monograph.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monograph.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monograph.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monograph.source", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.ingredient", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.ingredient.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.ingredient.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.ingredient.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.ingredient.item[x]", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.ingredient.isActive", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.ingredient.strength", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.preparationInstruction", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.intendedRoute", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.source", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.cost", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monitoringProgram", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monitoringProgram.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monitoringProgram.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monitoringProgram.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monitoringProgram.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monitoringProgram.name", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.dosage", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.dosage.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.dosage.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.dosage.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.dosage.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.dosage.dosage", 1, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.indication[x]", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.patientCharacteristics", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.patientCharacteristics.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.patientCharacteristics.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.patientCharacteristics.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.patientCharacteristics.characteristic[x]", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.administrationGuidelines.patientCharacteristics.value", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification.classification", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.packaging", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.packaging.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.packaging.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.packaging.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.packaging.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.packaging.quantity", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.drugCharacteristic", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.drugCharacteristic.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.drugCharacteristic.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.drugCharacteristic.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.drugCharacteristic.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.drugCharacteristic.value[x]", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.contraindication", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.regulatoryAuthority", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.substitution", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.substitution.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.substitution.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.substitution.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.substitution.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.substitution.allowed", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.schedule", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.schedule.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.schedule.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.schedule.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.schedule.schedule", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.maxDispense", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.maxDispense.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.maxDispense.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.maxDispense.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.maxDispense.quantity", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.maxDispense.period", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.kinetics", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.kinetics.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.kinetics.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.kinetics.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.kinetics.areaUnderCurve", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.kinetics.lethalDose50", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.kinetics.halfLifePeriod", 0, Some(1)),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicationKnowledge {
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

impl crate::traits::resource::ResourceMutators for MedicationKnowledge {
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

impl crate::traits::resource::ResourceExistence for MedicationKnowledge {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicationKnowledge {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicationKnowledge {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicationKnowledge {
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

impl crate::traits::medication_knowledge::MedicationKnowledgeAccessors for MedicationKnowledge {
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn status(&self) -> Option<MedicationknowledgeStatus> {
        self.status.clone()
    }
    fn manufacturer(&self) -> Option<Reference> {
        self.manufacturer.clone()
    }
    fn dose_form(&self) -> Option<CodeableConcept> {
        self.dose_form.clone()
    }
    fn amount(&self) -> Option<Quantity> {
        self.amount.clone()
    }
    fn synonym(&self) -> &[StringType] {
        self.synonym.as_deref().unwrap_or(&[])
    }
    fn related_medication_knowledge(&self) -> &[MedicationKnowledgeRelatedmedicationknowledge] {
        self.related_medication_knowledge.as_deref().unwrap_or(&[])
    }
    fn associated_medication(&self) -> &[Reference] {
        self.associated_medication.as_deref().unwrap_or(&[])
    }
    fn product_type(&self) -> &[CodeableConcept] {
        self.product_type.as_deref().unwrap_or(&[])
    }
    fn monograph(&self) -> &[MedicationKnowledgeMonograph] {
        self.monograph.as_deref().unwrap_or(&[])
    }
    fn ingredient(&self) -> &[MedicationKnowledgeIngredient] {
        self.ingredient.as_deref().unwrap_or(&[])
    }
    fn preparation_instruction(&self) -> Option<StringType> {
        self.preparation_instruction.clone()
    }
    fn intended_route(&self) -> &[CodeableConcept] {
        self.intended_route.as_deref().unwrap_or(&[])
    }
    fn cost(&self) -> &[MedicationKnowledgeCost] {
        self.cost.as_deref().unwrap_or(&[])
    }
    fn monitoring_program(&self) -> &[MedicationKnowledgeMonitoringprogram] {
        self.monitoring_program.as_deref().unwrap_or(&[])
    }
    fn administration_guidelines(&self) -> &[MedicationKnowledgeAdministrationguidelines] {
        self.administration_guidelines.as_deref().unwrap_or(&[])
    }
    fn medicine_classification(&self) -> &[MedicationKnowledgeMedicineclassification] {
        self.medicine_classification.as_deref().unwrap_or(&[])
    }
    fn packaging(&self) -> Option<MedicationKnowledgePackaging> {
        self.packaging.clone()
    }
    fn drug_characteristic(&self) -> &[MedicationKnowledgeDrugcharacteristic] {
        self.drug_characteristic.as_deref().unwrap_or(&[])
    }
    fn contraindication(&self) -> &[Reference] {
        self.contraindication.as_deref().unwrap_or(&[])
    }
    fn regulatory(&self) -> &[MedicationKnowledgeRegulatory] {
        self.regulatory.as_deref().unwrap_or(&[])
    }
    fn kinetics(&self) -> &[MedicationKnowledgeKinetics] {
        self.kinetics.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::medication_knowledge::MedicationKnowledgeMutators for MedicationKnowledge {
    fn new() -> Self {
        Self::default()
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_status(self, value: MedicationknowledgeStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_manufacturer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.manufacturer = Some(value);
        resource
    }
    fn set_dose_form(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.dose_form = Some(value);
        resource
    }
    fn set_amount(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.amount = Some(value);
        resource
    }
    fn set_synonym(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.synonym = Some(value);
        resource
    }
    fn add_synonym(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.synonym.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_related_medication_knowledge(
        self,
        value: Vec<MedicationKnowledgeRelatedmedicationknowledge>,
    ) -> Self {
        let mut resource = self.clone();
        resource.related_medication_knowledge = Some(value);
        resource
    }
    fn add_related_medication_knowledge(
        self,
        item: MedicationKnowledgeRelatedmedicationknowledge,
    ) -> Self {
        let mut resource = self.clone();
        resource
            .related_medication_knowledge
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_associated_medication(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.associated_medication = Some(value);
        resource
    }
    fn add_associated_medication(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .associated_medication
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_product_type(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.product_type = Some(value);
        resource
    }
    fn add_product_type(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .product_type
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_monograph(self, value: Vec<MedicationKnowledgeMonograph>) -> Self {
        let mut resource = self.clone();
        resource.monograph = Some(value);
        resource
    }
    fn add_monograph(self, item: MedicationKnowledgeMonograph) -> Self {
        let mut resource = self.clone();
        resource.monograph.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_ingredient(self, value: Vec<MedicationKnowledgeIngredient>) -> Self {
        let mut resource = self.clone();
        resource.ingredient = Some(value);
        resource
    }
    fn add_ingredient(self, item: MedicationKnowledgeIngredient) -> Self {
        let mut resource = self.clone();
        resource.ingredient.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_preparation_instruction(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.preparation_instruction = Some(value);
        resource
    }
    fn set_intended_route(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.intended_route = Some(value);
        resource
    }
    fn add_intended_route(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .intended_route
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_cost(self, value: Vec<MedicationKnowledgeCost>) -> Self {
        let mut resource = self.clone();
        resource.cost = Some(value);
        resource
    }
    fn add_cost(self, item: MedicationKnowledgeCost) -> Self {
        let mut resource = self.clone();
        resource.cost.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_monitoring_program(self, value: Vec<MedicationKnowledgeMonitoringprogram>) -> Self {
        let mut resource = self.clone();
        resource.monitoring_program = Some(value);
        resource
    }
    fn add_monitoring_program(self, item: MedicationKnowledgeMonitoringprogram) -> Self {
        let mut resource = self.clone();
        resource
            .monitoring_program
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_administration_guidelines(
        self,
        value: Vec<MedicationKnowledgeAdministrationguidelines>,
    ) -> Self {
        let mut resource = self.clone();
        resource.administration_guidelines = Some(value);
        resource
    }
    fn add_administration_guidelines(
        self,
        item: MedicationKnowledgeAdministrationguidelines,
    ) -> Self {
        let mut resource = self.clone();
        resource
            .administration_guidelines
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_medicine_classification(
        self,
        value: Vec<MedicationKnowledgeMedicineclassification>,
    ) -> Self {
        let mut resource = self.clone();
        resource.medicine_classification = Some(value);
        resource
    }
    fn add_medicine_classification(self, item: MedicationKnowledgeMedicineclassification) -> Self {
        let mut resource = self.clone();
        resource
            .medicine_classification
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_packaging(self, value: MedicationKnowledgePackaging) -> Self {
        let mut resource = self.clone();
        resource.packaging = Some(value);
        resource
    }
    fn set_drug_characteristic(self, value: Vec<MedicationKnowledgeDrugcharacteristic>) -> Self {
        let mut resource = self.clone();
        resource.drug_characteristic = Some(value);
        resource
    }
    fn add_drug_characteristic(self, item: MedicationKnowledgeDrugcharacteristic) -> Self {
        let mut resource = self.clone();
        resource
            .drug_characteristic
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_contraindication(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.contraindication = Some(value);
        resource
    }
    fn add_contraindication(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .contraindication
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_regulatory(self, value: Vec<MedicationKnowledgeRegulatory>) -> Self {
        let mut resource = self.clone();
        resource.regulatory = Some(value);
        resource
    }
    fn add_regulatory(self, item: MedicationKnowledgeRegulatory) -> Self {
        let mut resource = self.clone();
        resource.regulatory.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_kinetics(self, value: Vec<MedicationKnowledgeKinetics>) -> Self {
        let mut resource = self.clone();
        resource.kinetics = Some(value);
        resource
    }
    fn add_kinetics(self, item: MedicationKnowledgeKinetics) -> Self {
        let mut resource = self.clone();
        resource.kinetics.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::medication_knowledge::MedicationKnowledgeExistence for MedicationKnowledge {
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
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.is_some()
    }
    fn has_dose_form(&self) -> bool {
        self.dose_form.is_some()
    }
    fn has_amount(&self) -> bool {
        self.amount.is_some()
    }
    fn has_synonym(&self) -> bool {
        self.synonym.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_related_medication_knowledge(&self) -> bool {
        self.related_medication_knowledge
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_associated_medication(&self) -> bool {
        self.associated_medication
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_product_type(&self) -> bool {
        self.product_type.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_monograph(&self) -> bool {
        self.monograph.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_ingredient(&self) -> bool {
        self.ingredient.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_preparation_instruction(&self) -> bool {
        self.preparation_instruction.is_some()
    }
    fn has_intended_route(&self) -> bool {
        self.intended_route.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_cost(&self) -> bool {
        self.cost.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_monitoring_program(&self) -> bool {
        self.monitoring_program
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_administration_guidelines(&self) -> bool {
        self.administration_guidelines
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_medicine_classification(&self) -> bool {
        self.medicine_classification
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_packaging(&self) -> bool {
        self.packaging.is_some()
    }
    fn has_drug_characteristic(&self) -> bool {
        self.drug_characteristic
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_contraindication(&self) -> bool {
        self.contraindication
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_regulatory(&self) -> bool {
        self.regulatory.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_kinetics(&self) -> bool {
        self.kinetics.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for MedicationKnowledge {
    fn resource_type(&self) -> &'static str {
        "MedicationKnowledge"
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
        Some("http://hl7.org/fhir/StructureDefinition/MedicationKnowledge")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::medication_knowledge::{
    MedicationKnowledgeAccessors, MedicationKnowledgeExistence, MedicationKnowledgeMutators,
};
