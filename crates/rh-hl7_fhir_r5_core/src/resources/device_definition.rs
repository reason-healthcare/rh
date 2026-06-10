use crate::bindings::device_correctiveactionscope::DeviceCorrectiveactionscope;
use crate::bindings::device_nametype::DeviceNametype;
use crate::bindings::device_productidentifierinudi::DeviceProductidentifierinudi;
use crate::bindings::devicedefinition_regulatory_identifier_type::DevicedefinitionRegulatoryIdentifierType;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::product_shelf_life::ProductShelfLife;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// DeviceDefinition
///
/// This is a specialized resource that defines the characteristics and capabilities of a device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Additional information to describe the device
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Instance identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Unique Device Identifier (UDI) Barcode string
    #[serde(rename = "udiDeviceIdentifier")]
    pub udi_device_identifier: Option<Vec<DeviceDefinitionUdideviceidentifier>>,
    /// Regulatory identifier(s) associated with this device
    #[serde(rename = "regulatoryIdentifier")]
    pub regulatory_identifier: Option<Vec<DeviceDefinitionRegulatoryidentifier>>,
    /// The part number or catalog number of the device
    #[serde(rename = "partNumber")]
    pub part_number: Option<StringType>,
    /// Extension element for the 'partNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_partNumber")]
    pub _part_number: Option<Element>,
    /// Name of device manufacturer
    pub manufacturer: Option<Reference>,
    /// The name or names of the device as given by the manufacturer
    #[serde(rename = "deviceName")]
    pub device_name: Option<Vec<DeviceDefinitionDevicename>>,
    /// The catalog or model number for the device for example as defined by the manufacturer
    #[serde(rename = "modelNumber")]
    pub model_number: Option<StringType>,
    /// Extension element for the 'modelNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_modelNumber")]
    pub _model_number: Option<Element>,
    /// What kind of device or device system this is
    pub classification: Option<Vec<DeviceDefinitionClassification>>,
    /// Identifies the standards, specifications, or formal guidances for the capabilities supported by the device
    #[serde(rename = "conformsTo")]
    pub conforms_to: Option<Vec<DeviceDefinitionConformsto>>,
    /// A device, part of the current one
    #[serde(rename = "hasPart")]
    pub has_part: Option<Vec<DeviceDefinitionHaspart>>,
    /// Information about the packaging of the device, i.e. how the device is packaged
    pub packaging: Option<Vec<DeviceDefinitionPackaging>>,
    /// The version of the device or software
    pub version: Option<Vec<DeviceDefinitionVersion>>,
    /// Safety characteristics of the device
    ///
    /// Binding: example (No description)
    ///
    /// Available values:
    /// - `C106046`: Magnetic Resonance Conditional
    /// - `C106045`: Magnetic Resonance Safe
    /// - `C106047`: Magnetic Resonance Unsafe
    /// - `C113844`: Labeling does not Contain MRI Safety Information
    /// - `C101673`: Labeled as Containing Natural Rubber Latex
    /// - `C106038`: Not Made with Natural Rubber Latex
    pub safety: Option<Vec<CodeableConcept>>,
    /// Shelf Life and storage information
    #[serde(rename = "shelfLifeStorage")]
    pub shelf_life_storage: Option<Vec<ProductShelfLife>>,
    /// Language code for the human-readable text strings produced by the device (all supported)
    #[serde(rename = "languageCode")]
    pub language_code: Option<Vec<CodeableConcept>>,
    /// Inherent, essentially fixed, characteristics of this kind of device, e.g., time properties, size, etc
    pub property: Option<Vec<DeviceDefinitionProperty>>,
    /// Organization responsible for device
    pub owner: Option<Reference>,
    /// Details for human/organization for support
    pub contact: Option<Vec<ContactPoint>>,
    /// An associated device, attached to, used with, communicating with or linking a previous or new device model to the focal device
    pub link: Option<Vec<DeviceDefinitionLink>>,
    /// Device notes and comments
    pub note: Option<Vec<Annotation>>,
    /// A substance used to create the material(s) of which the device is made
    pub material: Option<Vec<DeviceDefinitionMaterial>>,
    /// lot-number | manufactured-date | serial-number | expiration-date | biological-source | software-version
    #[serde(rename = "productionIdentifierInUDI")]
    pub production_identifier_in_u_d_i: Option<Vec<DeviceProductidentifierinudi>>,
    /// Extension element for the 'productionIdentifierInUDI' primitive field. Contains metadata and extensions.
    #[serde(rename = "_productionIdentifierInUDI")]
    pub _production_identifier_in_u_d_i: Option<Element>,
    /// Information aimed at providing directions for the usage of this model of device
    pub guideline: Option<DeviceDefinitionGuideline>,
    /// Tracking of latest field safety corrective action
    #[serde(rename = "correctiveAction")]
    pub corrective_action: Option<DeviceDefinitionCorrectiveaction>,
    /// Billing code or reference associated with the device
    #[serde(rename = "chargeItem")]
    pub charge_item: Option<Vec<DeviceDefinitionChargeitem>>,
}
/// DeviceDefinition nested structure for the 'chargeItem' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionChargeitem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The code or reference for the charge item
    #[serde(rename = "chargeItemCode")]
    pub charge_item_code: CodeableReference,
    /// Coefficient applicable to the billing code
    pub count: Quantity,
    /// A specific time period in which this charge item applies
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// The context to which this charge item applies
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
}
/// DeviceDefinition nested structure for the 'conformsTo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionConformsto {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Describes the common type of the standard, specification, or formal guidance
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/device-specification-category
    pub category: Option<CodeableConcept>,
    /// Identifies the standard, specification, or formal guidance that the device adheres to the Device Specification type
    ///
    /// Binding: example (No description)
    ///
    /// Available values:
    /// - `528391`: Blood Pressure Cuff
    /// - `528404`: Body Composition Analyzer
    /// - `528425`: Cardiovascular Device
    /// - `528402`: Coagulation meter
    /// - `528409`: Continuous Glucose Monitor
    /// - `528390`: Electro cardiograph
    /// - `528457`: Generic 20601 Device
    /// - `528401`: Glucose Monitor
    /// - `528455`: Independent Activity/Living Hub
    /// - `528403`: Insulin Pump
    /// - ... and 7 more values
    pub specification: CodeableConcept,
    /// The specific form or variant of the standard, specification or formal guidance
    pub version: Option<Vec<StringType>>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Standard, regulation, certification, or guidance website, document, or other publication, or similar, supporting the conformance
    pub source: Option<Vec<RelatedArtifact>>,
}
/// DeviceDefinition nested structure for the 'deviceName' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionDevicename {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A name that is used to refer to the device
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// registered-name | user-friendly-name | patient-reported-name
    #[serde(rename = "type")]
    pub type_: DeviceNametype,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
}
/// DeviceDefinition nested structure for the 'version' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionVersion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of the device version, e.g. manufacturer, approved, internal
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The hardware or software module of the device to which the version applies
    pub component: Option<Identifier>,
    /// The version text
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}
/// DeviceDefinition nested structure for the 'udiDeviceIdentifier' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionUdideviceidentifier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Indicates whether and when the device is available on the market
    #[serde(rename = "marketDistribution")]
    pub market_distribution: Option<Vec<DeviceDefinitionUdideviceidentifierMarketdistribution>>,
    /// The identifier that is to be associated with every Device that references this DeviceDefintiion for the issuer and jurisdiction provided in the DeviceDefinition.udiDeviceIdentifier
    #[serde(rename = "deviceIdentifier")]
    pub device_identifier: StringType,
    /// Extension element for the 'deviceIdentifier' primitive field. Contains metadata and extensions.
    #[serde(rename = "_deviceIdentifier")]
    pub _device_identifier: Option<Element>,
    /// The organization that assigns the identifier algorithm
    pub issuer: StringType,
    /// Extension element for the 'issuer' primitive field. Contains metadata and extensions.
    pub _issuer: Option<Element>,
    /// The jurisdiction to which the deviceIdentifier applies
    pub jurisdiction: StringType,
    /// Extension element for the 'jurisdiction' primitive field. Contains metadata and extensions.
    pub _jurisdiction: Option<Element>,
}
/// DeviceDefinition nested structure for the 'packaging' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionPackaging {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// An organization that distributes the packaged device
    pub distributor: Option<Vec<DeviceDefinitionPackagingDistributor>>,
    /// Business identifier of the packaged medication
    pub identifier: Option<Identifier>,
    /// A code that defines the specific type of packaging
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The number of items contained in the package (devices or sub-packages)
    pub count: Option<IntegerType>,
    /// Extension element for the 'count' primitive field. Contains metadata and extensions.
    pub _count: Option<Element>,
    /// Unique Device Identifier (UDI) Barcode string on the packaging
    #[serde(rename = "udiDeviceIdentifier")]
    pub udi_device_identifier: Option<Vec<StringType>>,
    /// Allows packages within packages
    pub packaging: Option<Vec<StringType>>,
}
/// DeviceDefinition nested structure for the 'guideline' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionGuideline {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The circumstances that form the setting for using the device
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Detailed written and visual directions for the user on how to use the device
    #[serde(rename = "usageInstruction")]
    pub usage_instruction: Option<StringType>,
    /// Extension element for the 'usageInstruction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_usageInstruction")]
    pub _usage_instruction: Option<Element>,
    /// A source of information or reference for this guideline
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// A clinical condition for which the device was designed to be used
    pub indication: Option<Vec<CodeableConcept>>,
    /// A specific situation when a device should not be used because it may cause harm
    pub contraindication: Option<Vec<CodeableConcept>>,
    /// Specific hazard alert information that a user needs to know before using the device
    pub warning: Option<Vec<CodeableConcept>>,
    /// A description of the general purpose or medical use of the device or its function
    #[serde(rename = "intendedUse")]
    pub intended_use: Option<StringType>,
    /// Extension element for the 'intendedUse' primitive field. Contains metadata and extensions.
    #[serde(rename = "_intendedUse")]
    pub _intended_use: Option<Element>,
}
/// DeviceDefinitionUdideviceidentifier nested structure for the 'marketDistribution' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionUdideviceidentifierMarketdistribution {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Begin and end dates for the commercial distribution of the device
    #[serde(rename = "marketPeriod")]
    pub market_period: Period,
    /// National state or territory where the device is commercialized
    #[serde(rename = "subJurisdiction")]
    pub sub_jurisdiction: StringType,
    /// Extension element for the 'subJurisdiction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_subJurisdiction")]
    pub _sub_jurisdiction: Option<Element>,
}
/// DeviceDefinition nested structure for the 'hasPart' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionHaspart {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reference to the part
    pub reference: Reference,
    /// Number of occurrences of the part
    pub count: Option<IntegerType>,
    /// Extension element for the 'count' primitive field. Contains metadata and extensions.
    pub _count: Option<Element>,
}
/// DeviceDefinition nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionProperty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code that specifies the property being represented
    ///
    /// Binding: example (Device property type.)
    ///
    /// Available values:
    /// - `528391`: Blood Pressure Cuff
    /// - `528404`: Body Composition Analyzer
    /// - `528425`: Cardiovascular Device
    /// - `528402`: Coagulation meter
    /// - `528409`: Continuous Glucose Monitor
    /// - `528390`: Electro cardiograph
    /// - `528457`: Generic 20601 Device
    /// - `528401`: Glucose Monitor
    /// - `528455`: Independent Activity/Living Hub
    /// - `528403`: Insulin Pump
    /// - ... and 18 more values
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Value of the property (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Value of the property (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// Value of the property (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// Value of the property (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Value of the property (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// Value of the property (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// Value of the property (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,
}
/// DeviceDefinition nested structure for the 'link' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionLink {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type indicates the relationship of the related device to the device instance
    ///
    /// Binding: extensible (The type of relation between this and the linked device.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/devicedefinition-relationtype
    pub relation: Coding,
    /// A reference to the linked device
    #[serde(rename = "relatedDevice")]
    pub related_device: CodeableReference,
}
/// DeviceDefinition nested structure for the 'material' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionMaterial {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A relevant substance that the device contains, may contain, or is made of
    pub substance: CodeableConcept,
    /// Indicates an alternative material of the device
    pub alternate: Option<BooleanType>,
    /// Extension element for the 'alternate' primitive field. Contains metadata and extensions.
    pub _alternate: Option<Element>,
    /// Whether the substance is a known or suspected allergen
    #[serde(rename = "allergenicIndicator")]
    pub allergenic_indicator: Option<BooleanType>,
    /// Extension element for the 'allergenicIndicator' primitive field. Contains metadata and extensions.
    #[serde(rename = "_allergenicIndicator")]
    pub _allergenic_indicator: Option<Element>,
}
/// DeviceDefinition nested structure for the 'classification' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionClassification {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A classification or risk class of the device model
    ///
    /// Binding: example (Type of device e.g. according to official classification.)
    ///
    /// Available values:
    /// - `528391`: Blood Pressure Cuff
    /// - `528404`: Body Composition Analyzer
    /// - `528425`: Cardiovascular Device
    /// - `528402`: Coagulation meter
    /// - `528409`: Continuous Glucose Monitor
    /// - `528390`: Electro cardiograph
    /// - `528457`: Generic 20601 Device
    /// - `528401`: Glucose Monitor
    /// - `528455`: Independent Activity/Living Hub
    /// - `528403`: Insulin Pump
    /// - ... and 18 more values
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Further information qualifying this classification of the device model
    pub justification: Option<Vec<RelatedArtifact>>,
}
/// DeviceDefinition nested structure for the 'correctiveAction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionCorrectiveaction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Whether the corrective action was a recall
    pub recall: BooleanType,
    /// Extension element for the 'recall' primitive field. Contains metadata and extensions.
    pub _recall: Option<Element>,
    /// model | lot-numbers | serial-numbers
    pub scope: Option<DeviceCorrectiveactionscope>,
    /// Extension element for the 'scope' primitive field. Contains metadata and extensions.
    pub _scope: Option<Element>,
    /// Start and end dates of the  corrective action
    pub period: Period,
}
/// DeviceDefinition nested structure for the 'regulatoryIdentifier' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionRegulatoryidentifier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// basic | master | license
    #[serde(rename = "type")]
    pub type_: DevicedefinitionRegulatoryIdentifierType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// The identifier itself
    #[serde(rename = "deviceIdentifier")]
    pub device_identifier: StringType,
    /// Extension element for the 'deviceIdentifier' primitive field. Contains metadata and extensions.
    #[serde(rename = "_deviceIdentifier")]
    pub _device_identifier: Option<Element>,
    /// The organization that issued this identifier
    pub issuer: StringType,
    /// Extension element for the 'issuer' primitive field. Contains metadata and extensions.
    pub _issuer: Option<Element>,
    /// The jurisdiction to which the deviceIdentifier applies
    pub jurisdiction: StringType,
    /// Extension element for the 'jurisdiction' primitive field. Contains metadata and extensions.
    pub _jurisdiction: Option<Element>,
}
/// DeviceDefinitionPackaging nested structure for the 'distributor' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionPackagingDistributor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Distributor's human-readable name
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Distributor as an Organization resource
    #[serde(rename = "organizationReference")]
    pub organization_reference: Option<Vec<Reference>>,
}

impl Default for DeviceDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            description: Default::default(),
            _description: Default::default(),
            identifier: Default::default(),
            udi_device_identifier: Default::default(),
            regulatory_identifier: Default::default(),
            part_number: Default::default(),
            _part_number: Default::default(),
            manufacturer: Default::default(),
            device_name: Default::default(),
            model_number: Default::default(),
            _model_number: Default::default(),
            classification: Default::default(),
            conforms_to: Default::default(),
            has_part: Default::default(),
            packaging: Default::default(),
            version: Default::default(),
            safety: Default::default(),
            shelf_life_storage: Default::default(),
            language_code: Default::default(),
            property: Default::default(),
            owner: Default::default(),
            contact: Default::default(),
            link: Default::default(),
            note: Default::default(),
            material: Default::default(),
            production_identifier_in_u_d_i: Default::default(),
            _production_identifier_in_u_d_i: Default::default(),
            guideline: Default::default(),
            corrective_action: Default::default(),
            charge_item: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionChargeitem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            charge_item_code: Default::default(),
            count: Default::default(),
            effective_period: Default::default(),
            use_context: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionConformsto {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            category: Default::default(),
            specification: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            source: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionDevicename {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionVersion {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            component: Default::default(),
            value: StringType::default(),
            _value: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionUdideviceidentifier {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            market_distribution: Default::default(),
            device_identifier: Default::default(),
            _device_identifier: Default::default(),
            issuer: Default::default(),
            _issuer: Default::default(),
            jurisdiction: Default::default(),
            _jurisdiction: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionPackaging {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            distributor: Default::default(),
            identifier: Default::default(),
            type_: Default::default(),
            count: Default::default(),
            _count: Default::default(),
            udi_device_identifier: Default::default(),
            packaging: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionGuideline {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            use_context: Default::default(),
            usage_instruction: Default::default(),
            _usage_instruction: Default::default(),
            related_artifact: Default::default(),
            indication: Default::default(),
            contraindication: Default::default(),
            warning: Default::default(),
            intended_use: Default::default(),
            _intended_use: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionUdideviceidentifierMarketdistribution {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            market_period: Default::default(),
            sub_jurisdiction: Default::default(),
            _sub_jurisdiction: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionHaspart {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            reference: Default::default(),
            count: Default::default(),
            _count: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionProperty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_quantity: Default::default(),
            value_codeable_concept: Default::default(),
            value_string: Default::default(),
            value_boolean: Default::default(),
            value_integer: Default::default(),
            value_range: Default::default(),
            value_attachment: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionLink {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            relation: Coding::default(),
            related_device: CodeableReference::default(),
        }
    }
}

impl Default for DeviceDefinitionMaterial {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            substance: CodeableConcept::default(),
            alternate: Default::default(),
            _alternate: Default::default(),
            allergenic_indicator: Default::default(),
            _allergenic_indicator: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionClassification {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            justification: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionCorrectiveaction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            recall: Default::default(),
            _recall: Default::default(),
            scope: Default::default(),
            _scope: Default::default(),
            period: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionRegulatoryidentifier {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            device_identifier: Default::default(),
            _device_identifier: Default::default(),
            issuer: Default::default(),
            _issuer: Default::default(),
            jurisdiction: Default::default(),
            _jurisdiction: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionPackagingDistributor {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            organization_reference: Default::default(),
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
                "DeviceDefinition.correctiveAction.scope",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/device-correctiveactionscope|5.0.0",
            )
            .with_description("The type or scope of the corrective action."),
            rh_foundation::ElementBinding::new(
                "DeviceDefinition.deviceName.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/device-nametype|5.0.0",
            )
            .with_description("The type of name the device is referred by."),
            rh_foundation::ElementBinding::new(
                "DeviceDefinition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "DeviceDefinition.productionIdentifierInUDI",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/device-productidentifierinudi|5.0.0",
            )
            .with_description(
                "The production identifier(s) that are expected to appear in the UDI carrier.",
            ),
            rh_foundation::ElementBinding::new(
                "DeviceDefinition.regulatoryIdentifier.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/devicedefinition-regulatory-identifier-type|5.0.0",
            )
            .with_description("Device regulatory identifier type."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("DeviceDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.udiDeviceIdentifier", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.udiDeviceIdentifier.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.udiDeviceIdentifier.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.udiDeviceIdentifier.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.udiDeviceIdentifier.deviceIdentifier",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.udiDeviceIdentifier.issuer",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.udiDeviceIdentifier.jurisdiction",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.udiDeviceIdentifier.marketDistribution",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.udiDeviceIdentifier.marketDistribution.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.udiDeviceIdentifier.marketDistribution.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.udiDeviceIdentifier.marketDistribution.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.udiDeviceIdentifier.marketDistribution.marketPeriod",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.udiDeviceIdentifier.marketDistribution.subJurisdiction",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.regulatoryIdentifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.regulatoryIdentifier.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.regulatoryIdentifier.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.regulatoryIdentifier.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.regulatoryIdentifier.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.regulatoryIdentifier.deviceIdentifier",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.regulatoryIdentifier.issuer",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.regulatoryIdentifier.jurisdiction",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.partNumber", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.manufacturer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.deviceName", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.deviceName.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.deviceName.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.deviceName.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.deviceName.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.deviceName.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.modelNumber", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.classification", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.classification.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.classification.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.classification.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.classification.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.classification.justification",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.conformsTo", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.conformsTo.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.conformsTo.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.conformsTo.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.conformsTo.category",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.conformsTo.specification",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.conformsTo.version", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.conformsTo.source", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.hasPart", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.hasPart.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.hasPart.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.hasPart.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.hasPart.reference",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.hasPart.count", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.packaging", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.packaging.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.packaging.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.packaging.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.packaging.identifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.packaging.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.packaging.count", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.packaging.distributor",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.packaging.distributor.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.packaging.distributor.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.packaging.distributor.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.packaging.distributor.name",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.packaging.distributor.organizationReference",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.packaging.udiDeviceIdentifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.packaging.packaging", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.version", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.version.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.version.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.version.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.version.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.version.component",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.version.value", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.safety", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.shelfLifeStorage", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.languageCode", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.property", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.property.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.property.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.property.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.property.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.owner", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.link", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.link.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.link.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.link.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.link.relation", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.link.relatedDevice",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.note", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.material", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.material.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.material.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.material.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.material.substance",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.material.alternate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.material.allergenicIndicator",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.productionIdentifierInUDI",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.guideline", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.guideline.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDefinition.guideline.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.guideline.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.guideline.useContext",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.guideline.usageInstruction",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.guideline.relatedArtifact",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.guideline.indication",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.guideline.contraindication",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.guideline.warning", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.guideline.intendedUse",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.correctiveAction", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.correctiveAction.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.correctiveAction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.correctiveAction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.correctiveAction.recall",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.correctiveAction.scope",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.correctiveAction.period",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.chargeItem", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDefinition.chargeItem.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.chargeItem.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.chargeItem.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.chargeItem.chargeItemCode",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DeviceDefinition.chargeItem.count", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.chargeItem.effectivePeriod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceDefinition.chargeItem.useContext",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DeviceDefinition {
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

impl crate::traits::resource::ResourceMutators for DeviceDefinition {
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

impl crate::traits::resource::ResourceExistence for DeviceDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for DeviceDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for DeviceDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for DeviceDefinition {
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

impl crate::traits::device_definition::DeviceDefinitionAccessors for DeviceDefinition {
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn udi_device_identifier(&self) -> &[DeviceDefinitionUdideviceidentifier] {
        self.udi_device_identifier.as_deref().unwrap_or(&[])
    }
    fn regulatory_identifier(&self) -> &[DeviceDefinitionRegulatoryidentifier] {
        self.regulatory_identifier.as_deref().unwrap_or(&[])
    }
    fn part_number(&self) -> Option<StringType> {
        self.part_number.clone()
    }
    fn manufacturer(&self) -> Option<Reference> {
        self.manufacturer.clone()
    }
    fn device_name(&self) -> &[DeviceDefinitionDevicename] {
        self.device_name.as_deref().unwrap_or(&[])
    }
    fn model_number(&self) -> Option<StringType> {
        self.model_number.clone()
    }
    fn classification(&self) -> &[DeviceDefinitionClassification] {
        self.classification.as_deref().unwrap_or(&[])
    }
    fn conforms_to(&self) -> &[DeviceDefinitionConformsto] {
        self.conforms_to.as_deref().unwrap_or(&[])
    }
    fn has_part(&self) -> &[DeviceDefinitionHaspart] {
        self.has_part.as_deref().unwrap_or(&[])
    }
    fn packaging(&self) -> &[DeviceDefinitionPackaging] {
        self.packaging.as_deref().unwrap_or(&[])
    }
    fn version(&self) -> &[DeviceDefinitionVersion] {
        self.version.as_deref().unwrap_or(&[])
    }
    fn safety(&self) -> &[CodeableConcept] {
        self.safety.as_deref().unwrap_or(&[])
    }
    fn shelf_life_storage(&self) -> &[ProductShelfLife] {
        self.shelf_life_storage.as_deref().unwrap_or(&[])
    }
    fn language_code(&self) -> &[CodeableConcept] {
        self.language_code.as_deref().unwrap_or(&[])
    }
    fn property(&self) -> &[DeviceDefinitionProperty] {
        self.property.as_deref().unwrap_or(&[])
    }
    fn owner(&self) -> Option<Reference> {
        self.owner.clone()
    }
    fn contact(&self) -> &[ContactPoint] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn link(&self) -> &[DeviceDefinitionLink] {
        self.link.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn material(&self) -> &[DeviceDefinitionMaterial] {
        self.material.as_deref().unwrap_or(&[])
    }
    fn production_identifier_in_u_d_i(&self) -> &[DeviceProductidentifierinudi] {
        self.production_identifier_in_u_d_i
            .as_deref()
            .unwrap_or(&[])
    }
    fn guideline(&self) -> Option<DeviceDefinitionGuideline> {
        self.guideline.clone()
    }
    fn corrective_action(&self) -> Option<DeviceDefinitionCorrectiveaction> {
        self.corrective_action.clone()
    }
    fn charge_item(&self) -> &[DeviceDefinitionChargeitem] {
        self.charge_item.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::device_definition::DeviceDefinitionMutators for DeviceDefinition {
    fn new() -> Self {
        Self::default()
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
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
    fn set_udi_device_identifier(self, value: Vec<DeviceDefinitionUdideviceidentifier>) -> Self {
        let mut resource = self.clone();
        resource.udi_device_identifier = Some(value);
        resource
    }
    fn add_udi_device_identifier(self, item: DeviceDefinitionUdideviceidentifier) -> Self {
        let mut resource = self.clone();
        resource
            .udi_device_identifier
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_regulatory_identifier(self, value: Vec<DeviceDefinitionRegulatoryidentifier>) -> Self {
        let mut resource = self.clone();
        resource.regulatory_identifier = Some(value);
        resource
    }
    fn add_regulatory_identifier(self, item: DeviceDefinitionRegulatoryidentifier) -> Self {
        let mut resource = self.clone();
        resource
            .regulatory_identifier
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_part_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.part_number = Some(value);
        resource
    }
    fn set_manufacturer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.manufacturer = Some(value);
        resource
    }
    fn set_device_name(self, value: Vec<DeviceDefinitionDevicename>) -> Self {
        let mut resource = self.clone();
        resource.device_name = Some(value);
        resource
    }
    fn add_device_name(self, item: DeviceDefinitionDevicename) -> Self {
        let mut resource = self.clone();
        resource.device_name.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_model_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.model_number = Some(value);
        resource
    }
    fn set_classification(self, value: Vec<DeviceDefinitionClassification>) -> Self {
        let mut resource = self.clone();
        resource.classification = Some(value);
        resource
    }
    fn add_classification(self, item: DeviceDefinitionClassification) -> Self {
        let mut resource = self.clone();
        resource
            .classification
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_conforms_to(self, value: Vec<DeviceDefinitionConformsto>) -> Self {
        let mut resource = self.clone();
        resource.conforms_to = Some(value);
        resource
    }
    fn add_conforms_to(self, item: DeviceDefinitionConformsto) -> Self {
        let mut resource = self.clone();
        resource.conforms_to.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_has_part(self, value: Vec<DeviceDefinitionHaspart>) -> Self {
        let mut resource = self.clone();
        resource.has_part = Some(value);
        resource
    }
    fn add_has_part(self, item: DeviceDefinitionHaspart) -> Self {
        let mut resource = self.clone();
        resource.has_part.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_packaging(self, value: Vec<DeviceDefinitionPackaging>) -> Self {
        let mut resource = self.clone();
        resource.packaging = Some(value);
        resource
    }
    fn add_packaging(self, item: DeviceDefinitionPackaging) -> Self {
        let mut resource = self.clone();
        resource.packaging.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_version(self, value: Vec<DeviceDefinitionVersion>) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn add_version(self, item: DeviceDefinitionVersion) -> Self {
        let mut resource = self.clone();
        resource.version.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_safety(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.safety = Some(value);
        resource
    }
    fn add_safety(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.safety.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_shelf_life_storage(self, value: Vec<ProductShelfLife>) -> Self {
        let mut resource = self.clone();
        resource.shelf_life_storage = Some(value);
        resource
    }
    fn add_shelf_life_storage(self, item: ProductShelfLife) -> Self {
        let mut resource = self.clone();
        resource
            .shelf_life_storage
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_language_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.language_code = Some(value);
        resource
    }
    fn add_language_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .language_code
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_property(self, value: Vec<DeviceDefinitionProperty>) -> Self {
        let mut resource = self.clone();
        resource.property = Some(value);
        resource
    }
    fn add_property(self, item: DeviceDefinitionProperty) -> Self {
        let mut resource = self.clone();
        resource.property.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_owner(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.owner = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactPoint>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ContactPoint) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_link(self, value: Vec<DeviceDefinitionLink>) -> Self {
        let mut resource = self.clone();
        resource.link = Some(value);
        resource
    }
    fn add_link(self, item: DeviceDefinitionLink) -> Self {
        let mut resource = self.clone();
        resource.link.get_or_insert_with(Vec::new).push(item);
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
    fn set_material(self, value: Vec<DeviceDefinitionMaterial>) -> Self {
        let mut resource = self.clone();
        resource.material = Some(value);
        resource
    }
    fn add_material(self, item: DeviceDefinitionMaterial) -> Self {
        let mut resource = self.clone();
        resource.material.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_production_identifier_in_u_d_i(self, value: Vec<DeviceProductidentifierinudi>) -> Self {
        let mut resource = self.clone();
        resource.production_identifier_in_u_d_i = Some(value);
        resource
    }
    fn add_production_identifier_in_u_d_i(self, item: DeviceProductidentifierinudi) -> Self {
        let mut resource = self.clone();
        resource
            .production_identifier_in_u_d_i
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_guideline(self, value: DeviceDefinitionGuideline) -> Self {
        let mut resource = self.clone();
        resource.guideline = Some(value);
        resource
    }
    fn set_corrective_action(self, value: DeviceDefinitionCorrectiveaction) -> Self {
        let mut resource = self.clone();
        resource.corrective_action = Some(value);
        resource
    }
    fn set_charge_item(self, value: Vec<DeviceDefinitionChargeitem>) -> Self {
        let mut resource = self.clone();
        resource.charge_item = Some(value);
        resource
    }
    fn add_charge_item(self, item: DeviceDefinitionChargeitem) -> Self {
        let mut resource = self.clone();
        resource.charge_item.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::device_definition::DeviceDefinitionExistence for DeviceDefinition {
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_udi_device_identifier(&self) -> bool {
        self.udi_device_identifier
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_regulatory_identifier(&self) -> bool {
        self.regulatory_identifier
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_part_number(&self) -> bool {
        self.part_number.is_some()
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.is_some()
    }
    fn has_device_name(&self) -> bool {
        self.device_name.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_model_number(&self) -> bool {
        self.model_number.is_some()
    }
    fn has_classification(&self) -> bool {
        self.classification.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_conforms_to(&self) -> bool {
        self.conforms_to.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_has_part(&self) -> bool {
        self.has_part.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_packaging(&self) -> bool {
        self.packaging.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_version(&self) -> bool {
        self.version.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_safety(&self) -> bool {
        self.safety.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_shelf_life_storage(&self) -> bool {
        self.shelf_life_storage
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_language_code(&self) -> bool {
        self.language_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_property(&self) -> bool {
        self.property.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_owner(&self) -> bool {
        self.owner.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_link(&self) -> bool {
        self.link.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_material(&self) -> bool {
        self.material.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_production_identifier_in_u_d_i(&self) -> bool {
        self.production_identifier_in_u_d_i
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_guideline(&self) -> bool {
        self.guideline.is_some()
    }
    fn has_corrective_action(&self) -> bool {
        self.corrective_action.is_some()
    }
    fn has_charge_item(&self) -> bool {
        self.charge_item.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for DeviceDefinition {
    fn resource_type(&self) -> &'static str {
        "DeviceDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/DeviceDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::device_definition::{
    DeviceDefinitionAccessors, DeviceDefinitionExistence, DeviceDefinitionMutators,
};
