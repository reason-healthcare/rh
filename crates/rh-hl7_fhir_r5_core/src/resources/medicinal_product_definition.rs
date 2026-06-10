use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::marketing_status::MarketingStatus;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicinalProductDefinition
///
/// Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use, drug catalogs, to support prescribing, adverse events management etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicinalProductDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for this product. Could be an MPID
    pub identifier: Option<Vec<Identifier>>,
    /// Regulatory type, e.g. Investigational or Authorized
    ///
    /// Binding: example (Overall defining type of this medicinal product.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// If this medicine applies to human or veterinary uses
    ///
    /// Binding: example (Applicable domain for this product (e.g. human, veterinary).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-domain
    pub domain: Option<CodeableConcept>,
    /// A business identifier relating to a specific version of the product
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// The status within the lifecycle of this product record
    ///
    /// Binding: preferred (The lifecycle status of an artifact.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/publication-status
    pub status: Option<CodeableConcept>,
    /// The date at which the given status became applicable
    #[serde(rename = "statusDate")]
    pub status_date: Option<DateTimeType>,
    /// Extension element for the 'statusDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_statusDate")]
    pub _status_date: Option<Element>,
    /// General description of this product
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The dose form for a single part product, or combined form of a multiple part product
    ///
    /// Binding: example (Dose forms for a product as a whole, considering all individual parts, but before any mixing)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/combined-dose-form
    #[serde(rename = "combinedPharmaceuticalDoseForm")]
    pub combined_pharmaceutical_dose_form: Option<CodeableConcept>,
    /// The path by which the product is taken into or makes contact with the body
    ///
    /// Binding: example (A code specifying the route or physiological path of administration of a therapeutic agent into or onto a patient's body.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/route-codes
    pub route: Option<Vec<CodeableConcept>>,
    /// Description of indication(s) for this product, used when structured indications are not required
    pub indication: Option<StringType>,
    /// Extension element for the 'indication' primitive field. Contains metadata and extensions.
    pub _indication: Option<Element>,
    /// The legal status of supply of the medicinal product as classified by the regulator
    ///
    /// Binding: example (The prescription supply types appropriate to a medicinal product)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/legal-status-of-supply
    #[serde(rename = "legalStatusOfSupply")]
    pub legal_status_of_supply: Option<CodeableConcept>,
    /// Whether the Medicinal Product is subject to additional monitoring for regulatory reasons
    ///
    /// Binding: example (Extra measures defined for a Medicinal Product, such as heightened reporting requirements (e.g. Black Triangle Monitoring).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-additional-monitoring
    #[serde(rename = "additionalMonitoringIndicator")]
    pub additional_monitoring_indicator: Option<CodeableConcept>,
    /// Whether the Medicinal Product is subject to special measures for regulatory reasons
    ///
    /// Binding: example (Extra measures defined for a Medicinal Product, such as a requirement to conduct post-authorization studies.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-special-measures
    #[serde(rename = "specialMeasures")]
    pub special_measures: Option<Vec<CodeableConcept>>,
    /// If authorised for use in children
    ///
    /// Binding: example (Suitability for age groups, in particular children.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-pediatric-use
    #[serde(rename = "pediatricUseIndicator")]
    pub pediatric_use_indicator: Option<CodeableConcept>,
    /// Allows the product to be classified by various systems
    ///
    /// Binding: example (This value set includes codes from the Anatomical Therapeutic Chemical Classification System - provided as an exemplar value set.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-classification
    pub classification: Option<Vec<CodeableConcept>>,
    /// Marketing status of the medicinal product, in contrast to marketing authorization
    #[serde(rename = "marketingStatus")]
    pub marketing_status: Option<Vec<MarketingStatus>>,
    /// Package type for the product
    ///
    /// Binding: example (Types of medicinal product packs)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-package-type
    #[serde(rename = "packagedMedicinalProduct")]
    pub packaged_medicinal_product: Option<Vec<CodeableConcept>>,
    /// Types of medicinal manufactured items and/or devices that this product consists of, such as tablets, capsule, or syringes
    #[serde(rename = "comprisedOf")]
    pub comprised_of: Option<Vec<Reference>>,
    /// The ingredients of this medicinal product - when not detailed in other resources
    ///
    /// Binding: example (This value set includes all substance codes from SNOMED CT - provided as an exemplar value set.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-codes
    pub ingredient: Option<Vec<CodeableConcept>>,
    /// Any component of the drug product which is not the chemical entity defined as the drug substance, or an excipient in the drug product
    ///
    /// Binding: example (This value set includes all substance codes from SNOMED CT - provided as an exemplar value set.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-codes
    pub impurity: Option<Vec<CodeableReference>>,
    /// Additional documentation about the medicinal product
    #[serde(rename = "attachedDocument")]
    pub attached_document: Option<Vec<Reference>>,
    /// A master file for the medicinal product (e.g. Pharmacovigilance System Master File)
    #[serde(rename = "masterFile")]
    pub master_file: Option<Vec<Reference>>,
    /// A product specific contact, person (in a role), or an organization
    pub contact: Option<Vec<MedicinalProductDefinitionContact>>,
    /// Clinical trials or studies that this product is involved in
    #[serde(rename = "clinicalTrial")]
    pub clinical_trial: Option<Vec<Reference>>,
    /// A code that this product is known by, within some formal terminology
    ///
    /// Binding: example (A coded concept that defines the type of a medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-codes
    pub code: Option<Vec<Coding>>,
    /// The product's name, including full name and possibly coded parts
    pub name: Vec<MedicinalProductDefinitionName>,
    /// Reference to another product, e.g. for linking authorised to investigational product
    #[serde(rename = "crossReference")]
    pub cross_reference: Option<Vec<MedicinalProductDefinitionCrossreference>>,
    /// A manufacturing or administrative process for the medicinal product
    pub operation: Option<Vec<MedicinalProductDefinitionOperation>>,
    /// Key product features such as "sugar free", "modified release"
    pub characteristic: Option<Vec<MedicinalProductDefinitionCharacteristic>>,
}
/// MedicinalProductDefinition nested structure for the 'operation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductDefinitionOperation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of manufacturing operation e.g. manufacturing itself, re-packaging
    #[serde(rename = "type")]
    pub type_: Option<CodeableReference>,
    /// Date range of applicability
    #[serde(rename = "effectiveDate")]
    pub effective_date: Option<Period>,
    /// The organization responsible for the particular process, e.g. the manufacturer or importer
    pub organization: Option<Vec<Reference>>,
    /// Specifies whether this process is considered proprietary or confidential
    ///
    /// Binding: example (Confidentiality rating, e.g. commercial sensitivity for a Medicinal Product.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-confidentiality
    #[serde(rename = "confidentialityIndicator")]
    pub confidentiality_indicator: Option<CodeableConcept>,
}
/// MedicinalProductDefinition nested structure for the 'crossReference' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductDefinitionCrossreference {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reference to another product, e.g. for linking authorised to investigational product
    pub product: CodeableReference,
    /// The type of relationship, for instance branded to generic or virtual to actual product
    ///
    /// Binding: example (Extra measures defined for a Medicinal Product, such as heightened reporting requirements.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-cross-reference-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
}
/// MedicinalProductDefinition nested structure for the 'name' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductDefinitionName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Coding words or phrases of the name
    pub part: Option<Vec<MedicinalProductDefinitionNamePart>>,
    /// Country and jurisdiction where the name applies
    pub usage: Option<Vec<MedicinalProductDefinitionNameUsage>>,
    /// The full product name
    #[serde(rename = "productName")]
    pub product_name: StringType,
    /// Extension element for the 'productName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_productName")]
    pub _product_name: Option<Element>,
    /// Type of product name, such as rINN, BAN, Proprietary, Non-Proprietary
    ///
    /// Binding: example (Type of a name for a Medicinal Product.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-name-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
}
/// MedicinalProductDefinitionName nested structure for the 'usage' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductDefinitionNameUsage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Country code for where this name applies
    ///
    /// Binding: example (Jurisdiction codes)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/country
    pub country: CodeableConcept,
    /// Jurisdiction code for where this name applies
    ///
    /// Binding: example (Jurisdiction codes)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<CodeableConcept>,
    /// Language code for this name
    pub language: StringType,
}
/// MedicinalProductDefinition nested structure for the 'contact' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductDefinitionContact {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Allows the contact to be classified, for example QPPV, Pharmacovigilance Enquiry Information
    ///
    /// Binding: example (Extra measures defined for a Medicinal Product, such as heightened reporting requirements.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-contact-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// A product specific contact, person (in a role), or an organization
    pub contact: Reference,
}
/// MedicinalProductDefinitionName nested structure for the 'part' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductDefinitionNamePart {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A fragment of a product name
    pub part: StringType,
    /// Extension element for the 'part' primitive field. Contains metadata and extensions.
    pub _part: Option<Element>,
    /// Identifying type for this part of the name (e.g. strength part)
    ///
    /// Binding: example (Type of part of a name for a Medicinal Product.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-name-part-type
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
}
/// MedicinalProductDefinition nested structure for the 'characteristic' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductDefinitionCharacteristic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A code expressing the type of characteristic
    ///
    /// Binding: example (This value set includes all observable entity codes from SNOMED CT - provided as an exemplar value set.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/product-characteristic-codes
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// A value for the characteristic (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// A value for the characteristic (markdown)
    #[serde(rename = "valueMarkdown")]
    pub value_markdown: Option<StringType>,
    /// A value for the characteristic (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// A value for the characteristic (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: Option<IntegerType>,
    /// A value for the characteristic (date)
    #[serde(rename = "valueDate")]
    pub value_date: Option<DateType>,
    /// A value for the characteristic (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
    /// A value for the characteristic (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
}

impl Default for MedicinalProductDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            type_: Default::default(),
            domain: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            status: Default::default(),
            status_date: Default::default(),
            _status_date: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            combined_pharmaceutical_dose_form: Default::default(),
            route: Default::default(),
            indication: Default::default(),
            _indication: Default::default(),
            legal_status_of_supply: Default::default(),
            additional_monitoring_indicator: Default::default(),
            special_measures: Default::default(),
            pediatric_use_indicator: Default::default(),
            classification: Default::default(),
            marketing_status: Default::default(),
            packaged_medicinal_product: Default::default(),
            comprised_of: Default::default(),
            ingredient: Default::default(),
            impurity: Default::default(),
            attached_document: Default::default(),
            master_file: Default::default(),
            contact: Default::default(),
            clinical_trial: Default::default(),
            code: Default::default(),
            name: Vec::new(),
            cross_reference: Default::default(),
            operation: Default::default(),
            characteristic: Default::default(),
        }
    }
}

impl Default for MedicinalProductDefinitionOperation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            effective_date: Default::default(),
            organization: Default::default(),
            confidentiality_indicator: Default::default(),
        }
    }
}

impl Default for MedicinalProductDefinitionCrossreference {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            product: Default::default(),
            type_: Default::default(),
        }
    }
}

impl Default for MedicinalProductDefinitionName {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            part: Default::default(),
            usage: Default::default(),
            product_name: StringType::default(),
            _product_name: Default::default(),
            type_: Default::default(),
        }
    }
}

impl Default for MedicinalProductDefinitionNameUsage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            country: Default::default(),
            jurisdiction: Default::default(),
            language: Default::default(),
        }
    }
}

impl Default for MedicinalProductDefinitionContact {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            contact: Reference::default(),
        }
    }
}

impl Default for MedicinalProductDefinitionNamePart {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            part: Default::default(),
            _part: Default::default(),
            type_: Default::default(),
        }
    }
}

impl Default for MedicinalProductDefinitionCharacteristic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_codeable_concept: Default::default(),
            value_markdown: Default::default(),
            value_quantity: Default::default(),
            value_integer: Default::default(),
            value_date: Default::default(),
            value_boolean: Default::default(),
            value_attachment: Default::default(),
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
            rh_foundation::ElementBinding::new(
                "MedicinalProductDefinition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "MedicinalProductDefinition.name.usage.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.identifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.domain", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.version",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.statusDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.combinedPharmaceuticalDoseForm",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.route", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.indication",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.legalStatusOfSupply",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.additionalMonitoringIndicator",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.specialMeasures",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.pediatricUseIndicator",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.classification",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.marketingStatus",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.packagedMedicinalProduct",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.comprisedOf",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.ingredient",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.impurity", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.attachedDocument",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.masterFile",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.contact.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.contact.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.contact.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.contact.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.contact.contact",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.clinicalTrial",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.code", 0, None),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.name", 1, None),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.productName",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.name.part", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.part.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.part.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.part.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.part.part",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.part.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.usage",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.usage.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.usage.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.usage.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.usage.country",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.usage.jurisdiction",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.name.usage.language",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.crossReference",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.crossReference.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.crossReference.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.crossReference.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.crossReference.product",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.crossReference.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicinalProductDefinition.operation", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.operation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.operation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.operation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.operation.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.operation.effectiveDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.operation.organization",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.operation.confidentialityIndicator",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.characteristic",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.characteristic.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.characteristic.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.characteristic.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.characteristic.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductDefinition.characteristic.value[x]",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicinalProductDefinition {
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

impl crate::traits::resource::ResourceMutators for MedicinalProductDefinition {
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

impl crate::traits::resource::ResourceExistence for MedicinalProductDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicinalProductDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicinalProductDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicinalProductDefinition {
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

impl crate::traits::medicinal_product_definition::MedicinalProductDefinitionAccessors
    for MedicinalProductDefinition
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn domain(&self) -> Option<CodeableConcept> {
        self.domain.clone()
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn status(&self) -> Option<CodeableConcept> {
        self.status.clone()
    }
    fn status_date(&self) -> Option<DateTimeType> {
        self.status_date.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn combined_pharmaceutical_dose_form(&self) -> Option<CodeableConcept> {
        self.combined_pharmaceutical_dose_form.clone()
    }
    fn route(&self) -> &[CodeableConcept] {
        self.route.as_deref().unwrap_or(&[])
    }
    fn indication(&self) -> Option<StringType> {
        self.indication.clone()
    }
    fn legal_status_of_supply(&self) -> Option<CodeableConcept> {
        self.legal_status_of_supply.clone()
    }
    fn additional_monitoring_indicator(&self) -> Option<CodeableConcept> {
        self.additional_monitoring_indicator.clone()
    }
    fn special_measures(&self) -> &[CodeableConcept] {
        self.special_measures.as_deref().unwrap_or(&[])
    }
    fn pediatric_use_indicator(&self) -> Option<CodeableConcept> {
        self.pediatric_use_indicator.clone()
    }
    fn classification(&self) -> &[CodeableConcept] {
        self.classification.as_deref().unwrap_or(&[])
    }
    fn marketing_status(&self) -> &[MarketingStatus] {
        self.marketing_status.as_deref().unwrap_or(&[])
    }
    fn packaged_medicinal_product(&self) -> &[CodeableConcept] {
        self.packaged_medicinal_product.as_deref().unwrap_or(&[])
    }
    fn comprised_of(&self) -> &[Reference] {
        self.comprised_of.as_deref().unwrap_or(&[])
    }
    fn ingredient(&self) -> &[CodeableConcept] {
        self.ingredient.as_deref().unwrap_or(&[])
    }
    fn impurity(&self) -> &[CodeableReference] {
        self.impurity.as_deref().unwrap_or(&[])
    }
    fn attached_document(&self) -> &[Reference] {
        self.attached_document.as_deref().unwrap_or(&[])
    }
    fn master_file(&self) -> &[Reference] {
        self.master_file.as_deref().unwrap_or(&[])
    }
    fn contact(&self) -> &[MedicinalProductDefinitionContact] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn clinical_trial(&self) -> &[Reference] {
        self.clinical_trial.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> &[Coding] {
        self.code.as_deref().unwrap_or(&[])
    }
    fn name(&self) -> &[MedicinalProductDefinitionName] {
        &self.name
    }
    fn cross_reference(&self) -> &[MedicinalProductDefinitionCrossreference] {
        self.cross_reference.as_deref().unwrap_or(&[])
    }
    fn operation(&self) -> &[MedicinalProductDefinitionOperation] {
        self.operation.as_deref().unwrap_or(&[])
    }
    fn characteristic(&self) -> &[MedicinalProductDefinitionCharacteristic] {
        self.characteristic.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::medicinal_product_definition::MedicinalProductDefinitionMutators
    for MedicinalProductDefinition
{
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
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_domain(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.domain = Some(value);
        resource
    }
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_status_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.status_date = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_combined_pharmaceutical_dose_form(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.combined_pharmaceutical_dose_form = Some(value);
        resource
    }
    fn set_route(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.route = Some(value);
        resource
    }
    fn add_route(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.route.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_indication(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.indication = Some(value);
        resource
    }
    fn set_legal_status_of_supply(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.legal_status_of_supply = Some(value);
        resource
    }
    fn set_additional_monitoring_indicator(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.additional_monitoring_indicator = Some(value);
        resource
    }
    fn set_special_measures(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.special_measures = Some(value);
        resource
    }
    fn add_special_measures(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .special_measures
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_pediatric_use_indicator(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.pediatric_use_indicator = Some(value);
        resource
    }
    fn set_classification(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.classification = Some(value);
        resource
    }
    fn add_classification(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .classification
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_marketing_status(self, value: Vec<MarketingStatus>) -> Self {
        let mut resource = self.clone();
        resource.marketing_status = Some(value);
        resource
    }
    fn add_marketing_status(self, item: MarketingStatus) -> Self {
        let mut resource = self.clone();
        resource
            .marketing_status
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_packaged_medicinal_product(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.packaged_medicinal_product = Some(value);
        resource
    }
    fn add_packaged_medicinal_product(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .packaged_medicinal_product
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_comprised_of(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.comprised_of = Some(value);
        resource
    }
    fn add_comprised_of(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .comprised_of
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_ingredient(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.ingredient = Some(value);
        resource
    }
    fn add_ingredient(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.ingredient.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_impurity(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.impurity = Some(value);
        resource
    }
    fn add_impurity(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.impurity.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_attached_document(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.attached_document = Some(value);
        resource
    }
    fn add_attached_document(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .attached_document
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_master_file(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.master_file = Some(value);
        resource
    }
    fn add_master_file(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.master_file.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_contact(self, value: Vec<MedicinalProductDefinitionContact>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: MedicinalProductDefinitionContact) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_clinical_trial(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.clinical_trial = Some(value);
        resource
    }
    fn add_clinical_trial(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .clinical_trial
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_code(self, value: Vec<Coding>) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn add_code(self, item: Coding) -> Self {
        let mut resource = self.clone();
        resource.code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_name(self, value: Vec<MedicinalProductDefinitionName>) -> Self {
        let mut resource = self.clone();
        resource.name = value;
        resource
    }
    fn add_name(self, item: MedicinalProductDefinitionName) -> Self {
        let mut resource = self.clone();
        resource.name.push(item);
        resource
    }
    fn set_cross_reference(self, value: Vec<MedicinalProductDefinitionCrossreference>) -> Self {
        let mut resource = self.clone();
        resource.cross_reference = Some(value);
        resource
    }
    fn add_cross_reference(self, item: MedicinalProductDefinitionCrossreference) -> Self {
        let mut resource = self.clone();
        resource
            .cross_reference
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_operation(self, value: Vec<MedicinalProductDefinitionOperation>) -> Self {
        let mut resource = self.clone();
        resource.operation = Some(value);
        resource
    }
    fn add_operation(self, item: MedicinalProductDefinitionOperation) -> Self {
        let mut resource = self.clone();
        resource.operation.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_characteristic(self, value: Vec<MedicinalProductDefinitionCharacteristic>) -> Self {
        let mut resource = self.clone();
        resource.characteristic = Some(value);
        resource
    }
    fn add_characteristic(self, item: MedicinalProductDefinitionCharacteristic) -> Self {
        let mut resource = self.clone();
        resource
            .characteristic
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::medicinal_product_definition::MedicinalProductDefinitionExistence
    for MedicinalProductDefinition
{
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_domain(&self) -> bool {
        self.domain.is_some()
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_status_date(&self) -> bool {
        self.status_date.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_combined_pharmaceutical_dose_form(&self) -> bool {
        self.combined_pharmaceutical_dose_form.is_some()
    }
    fn has_route(&self) -> bool {
        self.route.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_indication(&self) -> bool {
        self.indication.is_some()
    }
    fn has_legal_status_of_supply(&self) -> bool {
        self.legal_status_of_supply.is_some()
    }
    fn has_additional_monitoring_indicator(&self) -> bool {
        self.additional_monitoring_indicator.is_some()
    }
    fn has_special_measures(&self) -> bool {
        self.special_measures
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_pediatric_use_indicator(&self) -> bool {
        self.pediatric_use_indicator.is_some()
    }
    fn has_classification(&self) -> bool {
        self.classification.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_marketing_status(&self) -> bool {
        self.marketing_status
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_packaged_medicinal_product(&self) -> bool {
        self.packaged_medicinal_product
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_comprised_of(&self) -> bool {
        self.comprised_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_ingredient(&self) -> bool {
        self.ingredient.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_impurity(&self) -> bool {
        self.impurity.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_attached_document(&self) -> bool {
        self.attached_document
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_master_file(&self) -> bool {
        self.master_file.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_clinical_trial(&self) -> bool {
        self.clinical_trial.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        self.code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_name(&self) -> bool {
        !self.name.is_empty()
    }
    fn has_cross_reference(&self) -> bool {
        self.cross_reference.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_operation(&self) -> bool {
        self.operation.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_characteristic(&self) -> bool {
        self.characteristic.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for MedicinalProductDefinition {
    fn resource_type(&self) -> &'static str {
        "MedicinalProductDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/MedicinalProductDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::medicinal_product_definition::{
    MedicinalProductDefinitionAccessors, MedicinalProductDefinitionExistence,
    MedicinalProductDefinitionMutators,
};
