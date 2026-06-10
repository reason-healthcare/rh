use crate::bindings::medicationknowledge_status::MedicationknowledgeStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
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
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicationKnowledge
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledge {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for this medication
    pub identifier: Option<Vec<Identifier>>,
    /// Code that identifies this medication
    ///
    /// Binding: example (A coded concept that defines the type of a medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-codes
    pub code: Option<CodeableConcept>,
    /// active | entered-in-error | inactive
    pub status: Option<MedicationknowledgeStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Creator or owner of the knowledge or information about the medication
    pub author: Option<Reference>,
    /// Codes that identify the different jurisdictions for which the information of this resource was created
    #[serde(rename = "intendedJurisdiction")]
    pub intended_jurisdiction: Option<Vec<CodeableConcept>>,
    /// A name associated with the medication being described
    pub name: Option<Vec<StringType>>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Associated or related medication information
    #[serde(rename = "relatedMedicationKnowledge")]
    pub related_medication_knowledge: Option<Vec<MedicationKnowledgeRelatedmedicationknowledge>>,
    /// The set of medication resources that are associated with this medication
    #[serde(rename = "associatedMedication")]
    pub associated_medication: Option<Vec<Reference>>,
    /// Category of the medication or product
    #[serde(rename = "productType")]
    pub product_type: Option<Vec<CodeableConcept>>,
    /// Associated documentation about the medication
    pub monograph: Option<Vec<MedicationKnowledgeMonograph>>,
    /// The instructions for preparing the medication
    #[serde(rename = "preparationInstruction")]
    pub preparation_instruction: Option<StringType>,
    /// Extension element for the 'preparationInstruction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_preparationInstruction")]
    pub _preparation_instruction: Option<Element>,
    /// The pricing of the medication
    pub cost: Option<Vec<MedicationKnowledgeCost>>,
    /// Program under which a medication is reviewed
    #[serde(rename = "monitoringProgram")]
    pub monitoring_program: Option<Vec<MedicationKnowledgeMonitoringprogram>>,
    /// Guidelines or protocols for administration of the medication for an indication
    #[serde(rename = "indicationGuideline")]
    pub indication_guideline: Option<Vec<MedicationKnowledgeIndicationguideline>>,
    /// Categorization of the medication within a formulary or classification system
    #[serde(rename = "medicineClassification")]
    pub medicine_classification: Option<Vec<MedicationKnowledgeMedicineclassification>>,
    /// Details about packaged medications
    pub packaging: Option<Vec<MedicationKnowledgePackaging>>,
    /// Potential clinical issue with or between medication(s)
    #[serde(rename = "clinicalUseIssue")]
    pub clinical_use_issue: Option<Vec<Reference>>,
    /// How the medication should be stored
    #[serde(rename = "storageGuideline")]
    pub storage_guideline: Option<Vec<MedicationKnowledgeStorageguideline>>,
    /// Regulatory information about a medication
    pub regulatory: Option<Vec<MedicationKnowledgeRegulatory>>,
    /// Minimal definition information about the medication
    pub definitional: Option<MedicationKnowledgeDefinitional>,
}
/// MedicationKnowledgeIndicationguideline nested structure for the 'dosingGuideline' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeIndicationguidelineDosingguideline {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Intention of the treatment
    #[serde(rename = "treatmentIntent")]
    pub treatment_intent: Option<CodeableConcept>,
    /// Type of treatment the guideline applies to
    #[serde(rename = "administrationTreatment")]
    pub administration_treatment: Option<CodeableConcept>,
}
/// MedicationKnowledge nested structure for the 'definitional' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeDefinitional {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Specifies descriptive properties of the medicine
    #[serde(rename = "drugCharacteristic")]
    pub drug_characteristic: Option<Vec<MedicationKnowledgeDefinitionalDrugcharacteristic>>,
    /// Active or inactive ingredient
    pub ingredient: Option<Vec<MedicationKnowledgeDefinitionalIngredient>>,
    /// Definitional resources that provide more information about this medication
    pub definition: Option<Vec<Reference>>,
    /// powder | tablets | capsule +
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-form-codes
    #[serde(rename = "doseForm")]
    pub dose_form: Option<CodeableConcept>,
    /// The intended or approved route of administration
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/route-codes
    #[serde(rename = "intendedRoute")]
    pub intended_route: Option<Vec<CodeableConcept>>,
}
/// MedicationKnowledge nested structure for the 'storageGuideline' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeStorageguideline {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Setting or value of environment for adequate storage
    #[serde(rename = "environmentalSetting")]
    pub environmental_setting: Option<Vec<MedicationKnowledgeStorageguidelineEnvironmentalsetting>>,
    /// Reference to additional information
    pub reference: Option<StringType>,
    /// Extension element for the 'reference' primitive field. Contains metadata and extensions.
    pub _reference: Option<Element>,
    /// Additional storage notes
    pub note: Option<Vec<Annotation>>,
    /// Duration remains stable
    #[serde(rename = "stabilityDuration")]
    pub stability_duration: Option<Duration>,
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
/// MedicationKnowledge nested structure for the 'packaging' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgePackaging {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Cost of the packaged medication
    pub cost: Option<Vec<StringType>>,
    /// The packaged medication that is being priced
    #[serde(rename = "packagedProduct")]
    pub packaged_product: Option<Reference>,
}
/// MedicationKnowledge nested structure for the 'regulatory' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeRegulatory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The maximum number of units of the medication that can be dispensed in a period
    #[serde(rename = "maxDispense")]
    pub max_dispense: Option<MedicationKnowledgeRegulatoryMaxdispense>,
    /// Specifies if changes are allowed when dispensing a medication from a regulatory perspective
    pub substitution: Option<Vec<MedicationKnowledgeRegulatorySubstitution>>,
    /// Specifies the authority of the regulation
    #[serde(rename = "regulatoryAuthority")]
    pub regulatory_authority: Reference,
    /// Specifies the schedule of a medication in jurisdiction
    pub schedule: Option<Vec<CodeableConcept>>,
}
/// MedicationKnowledgeDefinitional nested structure for the 'ingredient' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeDefinitionalIngredient {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Substances contained in the medication
    pub item: CodeableReference,
    /// A code that defines the type of ingredient, active, base, etc
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-RoleClassIngredientEntity
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Quantity of ingredient present (Ratio)
    #[serde(rename = "strengthRatio")]
    pub strength_ratio: Option<Ratio>,
    /// Quantity of ingredient present (CodeableConcept)
    #[serde(rename = "strengthCodeableConcept")]
    pub strength_codeable_concept: Option<CodeableConcept>,
    /// Quantity of ingredient present (Quantity)
    #[serde(rename = "strengthQuantity")]
    pub strength_quantity: Option<Quantity>,
}
/// MedicationKnowledgeIndicationguidelineDosingguideline nested structure for the 'patientCharacteristic' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeIndicationguidelineDosingguidelinePatientcharacteristic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Categorization of specific characteristic that is relevant to the administration guideline
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// The specific characteristic (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// The specific characteristic (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// The specific characteristic (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Option<Range>,
}
/// MedicationKnowledgeStorageguideline nested structure for the 'environmentalSetting' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeStorageguidelineEnvironmentalsetting {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Categorization of the setting
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Value of the setting (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Value of the setting (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// Value of the setting (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
}
/// MedicationKnowledgeIndicationguidelineDosingguideline nested structure for the 'dosage' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeIndicationguidelineDosingguidelineDosage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Category of dosage for a medication
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Dosage for the medication for the specific guidelines
    pub dosage: Vec<Dosage>,
}
/// MedicationKnowledge nested structure for the 'indicationGuideline' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeIndicationguideline {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Guidelines for dosage of the medication
    #[serde(rename = "dosingGuideline")]
    pub dosing_guideline: Option<Vec<MedicationKnowledgeIndicationguidelineDosingguideline>>,
    /// Indication for use that applies to the specific administration guideline
    pub indication: Option<Vec<CodeableReference>>,
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
/// MedicationKnowledge nested structure for the 'cost' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeCost {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The date range for which the cost is effective
    #[serde(rename = "effectiveDate")]
    pub effective_date: Option<Vec<Period>>,
    /// The category of the cost information
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// The source or owner for the price information
    pub source: Option<StringType>,
    /// Extension element for the 'source' primitive field. Contains metadata and extensions.
    pub _source: Option<Element>,
    /// The price or category of the cost of the medication (Money)
    #[serde(rename = "costMoney")]
    pub cost_money: Money,
    /// The price or category of the cost of the medication (CodeableConcept)
    #[serde(rename = "costCodeableConcept")]
    pub cost_codeable_concept: CodeableConcept,
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
/// MedicationKnowledgeDefinitional nested structure for the 'drugCharacteristic' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationKnowledgeDefinitionalDrugcharacteristic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code specifying the type of characteristic of medication
    ///
    /// Binding: example (No description)
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
    /// Description of the characteristic (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
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
    /// The source of the classification (string)
    #[serde(rename = "sourceString")]
    pub source_string: Option<StringType>,
    /// The source of the classification (uri)
    #[serde(rename = "sourceUri")]
    pub source_uri: Option<StringType>,
    /// Specific category assigned to the medication
    pub classification: Option<Vec<CodeableConcept>>,
}

impl Default for MedicationKnowledge {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            code: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            author: Default::default(),
            intended_jurisdiction: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            related_medication_knowledge: Default::default(),
            associated_medication: Default::default(),
            product_type: Default::default(),
            monograph: Default::default(),
            preparation_instruction: Default::default(),
            _preparation_instruction: Default::default(),
            cost: Default::default(),
            monitoring_program: Default::default(),
            indication_guideline: Default::default(),
            medicine_classification: Default::default(),
            packaging: Default::default(),
            clinical_use_issue: Default::default(),
            storage_guideline: Default::default(),
            regulatory: Default::default(),
            definitional: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeIndicationguidelineDosingguideline {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            treatment_intent: Default::default(),
            administration_treatment: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeDefinitional {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            drug_characteristic: Default::default(),
            ingredient: Default::default(),
            definition: Default::default(),
            dose_form: Default::default(),
            intended_route: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeStorageguideline {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            environmental_setting: Default::default(),
            reference: Default::default(),
            _reference: Default::default(),
            note: Default::default(),
            stability_duration: Default::default(),
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

impl Default for MedicationKnowledgePackaging {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            cost: Default::default(),
            packaged_product: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeRegulatory {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            max_dispense: Default::default(),
            substitution: Default::default(),
            regulatory_authority: Reference::default(),
            schedule: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeDefinitionalIngredient {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item: Default::default(),
            type_: Default::default(),
            strength_ratio: Default::default(),
            strength_codeable_concept: Default::default(),
            strength_quantity: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeIndicationguidelineDosingguidelinePatientcharacteristic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_codeable_concept: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeStorageguidelineEnvironmentalsetting {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
            value_codeable_concept: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeIndicationguidelineDosingguidelineDosage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            dosage: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeIndicationguideline {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            dosing_guideline: Default::default(),
            indication: Default::default(),
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

impl Default for MedicationKnowledgeCost {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            effective_date: Default::default(),
            type_: Default::default(),
            source: Default::default(),
            _source: Default::default(),
            cost_money: Default::default(),
            cost_codeable_concept: Default::default(),
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

impl Default for MedicationKnowledgeDefinitionalDrugcharacteristic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_codeable_concept: Default::default(),
            value_string: Default::default(),
            value_quantity: Default::default(),
            value_base64_binary: Default::default(),
            value_attachment: Default::default(),
        }
    }
}

impl Default for MedicationKnowledgeMedicineclassification {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            source_string: Default::default(),
            source_uri: Default::default(),
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
            rh_foundation::Invariant::new(
                "ele-1",
                rh_foundation::Severity::Error,
                "All FHIR elements must have a @value or children",
                "hasValue() or (children().count() > id.count())",
            ),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            ),
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
                "MedicationKnowledge.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "MedicationKnowledge.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/medicationknowledge-status|5.0.0",
            )
            .with_description("A coded concept defining if the medication is in active use."),
        ]
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
    rh_foundation::ElementCardinality::new("MedicationKnowledge.identifier", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.code", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.status", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.author", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.intendedJurisdiction", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.name", 0, None),
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
    rh_foundation::ElementCardinality::new("MedicationKnowledge.preparationInstruction", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.effectiveDate", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.source", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.cost.cost[x]", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monitoringProgram", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monitoringProgram.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monitoringProgram.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monitoringProgram.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monitoringProgram.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.monitoringProgram.name", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.indication", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.treatmentIntent", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.dosage", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.dosage.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.dosage.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.dosage.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.dosage.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.dosage.dosage", 1, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.administrationTreatment", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.patientCharacteristic", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.patientCharacteristic.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.patientCharacteristic.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.patientCharacteristic.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.patientCharacteristic.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.indicationGuideline.dosingGuideline.patientCharacteristic.value[x]", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification.source[x]", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.medicineClassification.classification", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.packaging", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.packaging.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.packaging.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.packaging.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.packaging.cost", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.packaging.packagedProduct", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.clinicalUseIssue", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline.reference", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline.note", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline.stabilityDuration", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline.environmentalSetting", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline.environmentalSetting.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline.environmentalSetting.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline.environmentalSetting.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline.environmentalSetting.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.storageGuideline.environmentalSetting.value[x]", 1, Some(1)),
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
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.maxDispense", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.maxDispense.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.maxDispense.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.maxDispense.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.maxDispense.quantity", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.regulatory.maxDispense.period", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.definition", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.doseForm", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.intendedRoute", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.ingredient", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.ingredient.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.ingredient.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.ingredient.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.ingredient.item", 1, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.ingredient.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.ingredient.strength[x]", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.drugCharacteristic", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.drugCharacteristic.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.drugCharacteristic.extension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.drugCharacteristic.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.drugCharacteristic.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("MedicationKnowledge.definitional.drugCharacteristic.value[x]", 0, Some(1)),
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
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn status(&self) -> Option<MedicationknowledgeStatus> {
        self.status.clone()
    }
    fn author(&self) -> Option<Reference> {
        self.author.clone()
    }
    fn intended_jurisdiction(&self) -> &[CodeableConcept] {
        self.intended_jurisdiction.as_deref().unwrap_or(&[])
    }
    fn name(&self) -> &[StringType] {
        self.name.as_deref().unwrap_or(&[])
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
    fn preparation_instruction(&self) -> Option<StringType> {
        self.preparation_instruction.clone()
    }
    fn cost(&self) -> &[MedicationKnowledgeCost] {
        self.cost.as_deref().unwrap_or(&[])
    }
    fn monitoring_program(&self) -> &[MedicationKnowledgeMonitoringprogram] {
        self.monitoring_program.as_deref().unwrap_or(&[])
    }
    fn indication_guideline(&self) -> &[MedicationKnowledgeIndicationguideline] {
        self.indication_guideline.as_deref().unwrap_or(&[])
    }
    fn medicine_classification(&self) -> &[MedicationKnowledgeMedicineclassification] {
        self.medicine_classification.as_deref().unwrap_or(&[])
    }
    fn packaging(&self) -> &[MedicationKnowledgePackaging] {
        self.packaging.as_deref().unwrap_or(&[])
    }
    fn clinical_use_issue(&self) -> &[Reference] {
        self.clinical_use_issue.as_deref().unwrap_or(&[])
    }
    fn storage_guideline(&self) -> &[MedicationKnowledgeStorageguideline] {
        self.storage_guideline.as_deref().unwrap_or(&[])
    }
    fn regulatory(&self) -> &[MedicationKnowledgeRegulatory] {
        self.regulatory.as_deref().unwrap_or(&[])
    }
    fn definitional(&self) -> Option<MedicationKnowledgeDefinitional> {
        self.definitional.clone()
    }
}

impl crate::traits::medication_knowledge::MedicationKnowledgeMutators for MedicationKnowledge {
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
    fn set_author(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.author = Some(value);
        resource
    }
    fn set_intended_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.intended_jurisdiction = Some(value);
        resource
    }
    fn add_intended_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .intended_jurisdiction
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_name(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn add_name(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.name.get_or_insert_with(Vec::new).push(item);
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
    fn set_preparation_instruction(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.preparation_instruction = Some(value);
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
    fn set_indication_guideline(self, value: Vec<MedicationKnowledgeIndicationguideline>) -> Self {
        let mut resource = self.clone();
        resource.indication_guideline = Some(value);
        resource
    }
    fn add_indication_guideline(self, item: MedicationKnowledgeIndicationguideline) -> Self {
        let mut resource = self.clone();
        resource
            .indication_guideline
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
    fn set_packaging(self, value: Vec<MedicationKnowledgePackaging>) -> Self {
        let mut resource = self.clone();
        resource.packaging = Some(value);
        resource
    }
    fn add_packaging(self, item: MedicationKnowledgePackaging) -> Self {
        let mut resource = self.clone();
        resource.packaging.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_clinical_use_issue(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.clinical_use_issue = Some(value);
        resource
    }
    fn add_clinical_use_issue(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .clinical_use_issue
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_storage_guideline(self, value: Vec<MedicationKnowledgeStorageguideline>) -> Self {
        let mut resource = self.clone();
        resource.storage_guideline = Some(value);
        resource
    }
    fn add_storage_guideline(self, item: MedicationKnowledgeStorageguideline) -> Self {
        let mut resource = self.clone();
        resource
            .storage_guideline
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
    fn set_definitional(self, value: MedicationKnowledgeDefinitional) -> Self {
        let mut resource = self.clone();
        resource.definitional = Some(value);
        resource
    }
}

impl crate::traits::medication_knowledge::MedicationKnowledgeExistence for MedicationKnowledge {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_author(&self) -> bool {
        self.author.is_some()
    }
    fn has_intended_jurisdiction(&self) -> bool {
        self.intended_jurisdiction
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_name(&self) -> bool {
        self.name.as_ref().is_some_and(|v| !v.is_empty())
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
    fn has_preparation_instruction(&self) -> bool {
        self.preparation_instruction.is_some()
    }
    fn has_cost(&self) -> bool {
        self.cost.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_monitoring_program(&self) -> bool {
        self.monitoring_program
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_indication_guideline(&self) -> bool {
        self.indication_guideline
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_medicine_classification(&self) -> bool {
        self.medicine_classification
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_packaging(&self) -> bool {
        self.packaging.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_clinical_use_issue(&self) -> bool {
        self.clinical_use_issue
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_storage_guideline(&self) -> bool {
        self.storage_guideline
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_regulatory(&self) -> bool {
        self.regulatory.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_definitional(&self) -> bool {
        self.definitional.is_some()
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
