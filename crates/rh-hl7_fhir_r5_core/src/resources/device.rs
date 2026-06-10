use crate::bindings::device_nametype::DeviceNametype;
use crate::bindings::device_status::DeviceStatus;
use crate::bindings::udi_entry_type::UdiEntryType;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::count::Count;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::primitives::base64binary::Base64BinaryType;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Device
///
/// This resource describes the properties (regulated, has real time clock, etc.), adminstrative (manufacturer name, model number, serial number, firmware, etc.), and type (knee replacement, blood pressure cuff, MRI, etc.) of a physical unit (these values do not change much within a given module, for example the serail number, manufacturer name, and model number). An actual unit may consist of several modules in a distinct hierarchy and these are represented by multiple Device resources and bound through the 'parent' element.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Device
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Device
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Instance identifier
    pub identifier: Option<Vec<Identifier>>,
    /// The name used to display by default when the device is referenced
    #[serde(rename = "displayName")]
    pub display_name: Option<StringType>,
    /// Extension element for the 'displayName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_displayName")]
    pub _display_name: Option<Element>,
    /// The reference to the definition for the device
    pub definition: Option<CodeableReference>,
    /// Unique Device Identifier (UDI) Barcode string
    #[serde(rename = "udiCarrier")]
    pub udi_carrier: Option<Vec<DeviceUdicarrier>>,
    /// active | inactive | entered-in-error
    pub status: Option<DeviceStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// lost | damaged | destroyed | available
    ///
    /// Binding: extensible (The availability status reason of the device.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/device-availability-status
    #[serde(rename = "availabilityStatus")]
    pub availability_status: Option<CodeableConcept>,
    /// An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled
    #[serde(rename = "biologicalSourceEvent")]
    pub biological_source_event: Option<Identifier>,
    /// Name of device manufacturer
    pub manufacturer: Option<StringType>,
    /// Extension element for the 'manufacturer' primitive field. Contains metadata and extensions.
    pub _manufacturer: Option<Element>,
    /// Date when the device was made
    #[serde(rename = "manufactureDate")]
    pub manufacture_date: Option<DateTimeType>,
    /// Extension element for the 'manufactureDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_manufactureDate")]
    pub _manufacture_date: Option<Element>,
    /// Date and time of expiry of this device (if applicable)
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<DateTimeType>,
    /// Extension element for the 'expirationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_expirationDate")]
    pub _expiration_date: Option<Element>,
    /// Lot number of manufacture
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<StringType>,
    /// Extension element for the 'lotNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lotNumber")]
    pub _lot_number: Option<Element>,
    /// Serial number assigned by the manufacturer
    #[serde(rename = "serialNumber")]
    pub serial_number: Option<StringType>,
    /// Extension element for the 'serialNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_serialNumber")]
    pub _serial_number: Option<Element>,
    /// The name or names of the device as known to the manufacturer and/or patient
    pub name: Option<Vec<DeviceName>>,
    /// The manufacturer's model number for the device
    #[serde(rename = "modelNumber")]
    pub model_number: Option<StringType>,
    /// Extension element for the 'modelNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_modelNumber")]
    pub _model_number: Option<Element>,
    /// The part number or catalog number of the device
    #[serde(rename = "partNumber")]
    pub part_number: Option<StringType>,
    /// Extension element for the 'partNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_partNumber")]
    pub _part_number: Option<Element>,
    /// Indicates a high-level grouping of the device
    ///
    /// Binding: example (Categories of medical devices.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/device-category
    pub category: Option<Vec<CodeableConcept>>,
    /// The kind or type of device
    ///
    /// Binding: example (Codes to identify medical devices.)
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
    pub type_: Option<Vec<CodeableConcept>>,
    /// The actual design of the device or software version running on the device
    pub version: Option<Vec<DeviceVersion>>,
    /// Identifies the standards, specifications, or formal guidances for the capabilities supported by the device
    #[serde(rename = "conformsTo")]
    pub conforms_to: Option<Vec<DeviceConformsto>>,
    /// Inherent, essentially fixed, characteristics of the device.  e.g., time properties, size, material, etc.
    pub property: Option<Vec<DeviceProperty>>,
    /// The designated condition for performing a task
    ///
    /// Binding: example (Operational mode of a device.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/device-operation-mode
    pub mode: Option<CodeableConcept>,
    /// The series of occurrences that repeats during the operation of the device
    pub cycle: Option<Count>,
    /// A measurement of time during the device's operation (e.g., days, hours, mins, etc.)
    pub duration: Option<Duration>,
    /// Organization responsible for device
    pub owner: Option<Reference>,
    /// Details for human/organization for support
    pub contact: Option<Vec<ContactPoint>>,
    /// Where the device is found
    pub location: Option<Reference>,
    /// Network address to contact device
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Technical endpoints providing access to electronic services provided by the device
    pub endpoint: Option<Vec<Reference>>,
    /// Linked device acting as a communication/data collector, translator or controller
    pub gateway: Option<Vec<CodeableReference>>,
    /// Device notes and comments
    pub note: Option<Vec<Annotation>>,
    /// Safety Characteristics of Device
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
    /// The higher level or encompassing device that this device is a logical part of
    pub parent: Option<Reference>,
}
/// Device nested structure for the 'version' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceVersion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of the device version, e.g. manufacturer, approved, internal
    ///
    /// Binding: example (The type of version indicated for the device.)
    ///
    /// Available values:
    /// - `531974`: Hardware revision
    /// - `531975`: Software revision
    /// - `531976`: Firmware revision
    /// - `531977`: Protocol revision
    /// - `532352`: Continua version
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The hardware or software module of the device to which the version applies
    pub component: Option<Identifier>,
    /// The date the version was installed on the device
    #[serde(rename = "installDate")]
    pub install_date: Option<DateTimeType>,
    /// Extension element for the 'installDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_installDate")]
    pub _install_date: Option<Element>,
    /// The version text
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}
/// Device nested structure for the 'conformsTo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConformsto {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Describes the common type of the standard, specification, or formal guidance.  communication | performance | measurement
    ///
    /// Binding: example (The kind of standards used by the device.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/device-specification-category
    pub category: Option<CodeableConcept>,
    /// Identifies the standard, specification, or formal guidance that the device adheres to
    ///
    /// Binding: example (The type of version indicated for the device.)
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
    /// Specific form or variant of the standard
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
}
/// Device nested structure for the 'name' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The term that names the device
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// registered-name | user-friendly-name | patient-reported-name
    #[serde(rename = "type")]
    pub type_: DeviceNametype,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// The preferred device name
    pub display: Option<BooleanType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
}
/// Device nested structure for the 'udiCarrier' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceUdicarrier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Mandatory fixed portion of UDI
    #[serde(rename = "deviceIdentifier")]
    pub device_identifier: StringType,
    /// Extension element for the 'deviceIdentifier' primitive field. Contains metadata and extensions.
    #[serde(rename = "_deviceIdentifier")]
    pub _device_identifier: Option<Element>,
    /// UDI Issuing Organization
    pub issuer: StringType,
    /// Extension element for the 'issuer' primitive field. Contains metadata and extensions.
    pub _issuer: Option<Element>,
    /// Regional UDI authority
    pub jurisdiction: Option<StringType>,
    /// Extension element for the 'jurisdiction' primitive field. Contains metadata and extensions.
    pub _jurisdiction: Option<Element>,
    /// UDI Machine Readable Barcode String
    #[serde(rename = "carrierAIDC")]
    pub carrier_a_i_d_c: Option<Base64BinaryType>,
    /// Extension element for the 'carrierAIDC' primitive field. Contains metadata and extensions.
    #[serde(rename = "_carrierAIDC")]
    pub _carrier_a_i_d_c: Option<Element>,
    /// UDI Human Readable Barcode String
    #[serde(rename = "carrierHRF")]
    pub carrier_h_r_f: Option<StringType>,
    /// Extension element for the 'carrierHRF' primitive field. Contains metadata and extensions.
    #[serde(rename = "_carrierHRF")]
    pub _carrier_h_r_f: Option<Element>,
    /// barcode | rfid | manual | card | self-reported | electronic-transmission | unknown
    #[serde(rename = "entryType")]
    pub entry_type: Option<UdiEntryType>,
    /// Extension element for the 'entryType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_entryType")]
    pub _entry_type: Option<Element>,
}
/// Device nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProperty {
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

impl Default for Device {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            display_name: Default::default(),
            _display_name: Default::default(),
            definition: Default::default(),
            udi_carrier: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            availability_status: Default::default(),
            biological_source_event: Default::default(),
            manufacturer: Default::default(),
            _manufacturer: Default::default(),
            manufacture_date: Default::default(),
            _manufacture_date: Default::default(),
            expiration_date: Default::default(),
            _expiration_date: Default::default(),
            lot_number: Default::default(),
            _lot_number: Default::default(),
            serial_number: Default::default(),
            _serial_number: Default::default(),
            name: Default::default(),
            model_number: Default::default(),
            _model_number: Default::default(),
            part_number: Default::default(),
            _part_number: Default::default(),
            category: Default::default(),
            type_: Default::default(),
            version: Default::default(),
            conforms_to: Default::default(),
            property: Default::default(),
            mode: Default::default(),
            cycle: Default::default(),
            duration: Default::default(),
            owner: Default::default(),
            contact: Default::default(),
            location: Default::default(),
            url: Default::default(),
            _url: Default::default(),
            endpoint: Default::default(),
            gateway: Default::default(),
            note: Default::default(),
            safety: Default::default(),
            parent: Default::default(),
        }
    }
}

impl Default for DeviceVersion {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            component: Default::default(),
            install_date: Default::default(),
            _install_date: Default::default(),
            value: StringType::default(),
            _value: Default::default(),
        }
    }
}

impl Default for DeviceConformsto {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            category: Default::default(),
            specification: Default::default(),
            version: Default::default(),
            _version: Default::default(),
        }
    }
}

impl Default for DeviceName {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            value: StringType::default(),
            _value: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            display: Default::default(),
            _display: Default::default(),
        }
    }
}

impl Default for DeviceUdicarrier {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            device_identifier: Default::default(),
            _device_identifier: Default::default(),
            issuer: Default::default(),
            _issuer: Default::default(),
            jurisdiction: Default::default(),
            _jurisdiction: Default::default(),
            carrier_a_i_d_c: Default::default(),
            _carrier_a_i_d_c: Default::default(),
            carrier_h_r_f: Default::default(),
            _carrier_h_r_f: Default::default(),
            entry_type: Default::default(),
            _entry_type: Default::default(),
        }
    }
}

impl Default for DeviceProperty {
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

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("dev-1", rh_foundation::Severity::Error, "only one Device.name.display SHALL be true when there is more than one Device.name", "name.where(display=true).count() <= 1"),
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
                "Device.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Device.name.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/device-nametype|5.0.0",
            )
            .with_description("The type of name the device is referred by."),
            rh_foundation::ElementBinding::new(
                "Device.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/device-status|5.0.0",
            )
            .with_description("The record status of the device."),
            rh_foundation::ElementBinding::new(
                "Device.udiCarrier.entryType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/udi-entry-type|5.0.0",
            )
            .with_description("Codes to identify how UDI data was entered."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Device.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.contained", 0, None),
            rh_foundation::ElementCardinality::new("Device.extension", 0, None),
            rh_foundation::ElementCardinality::new("Device.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Device.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Device.displayName", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.definition", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.udiCarrier", 0, None),
            rh_foundation::ElementCardinality::new("Device.udiCarrier.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.udiCarrier.extension", 0, None),
            rh_foundation::ElementCardinality::new("Device.udiCarrier.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Device.udiCarrier.deviceIdentifier",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Device.udiCarrier.issuer", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Device.udiCarrier.jurisdiction", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.udiCarrier.carrierAIDC", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.udiCarrier.carrierHRF", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.udiCarrier.entryType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.availabilityStatus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.biologicalSourceEvent", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.manufacturer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.manufactureDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.expirationDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.lotNumber", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.serialNumber", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.name", 0, None),
            rh_foundation::ElementCardinality::new("Device.name.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.name.extension", 0, None),
            rh_foundation::ElementCardinality::new("Device.name.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Device.name.value", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Device.name.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Device.name.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.modelNumber", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.partNumber", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.category", 0, None),
            rh_foundation::ElementCardinality::new("Device.type", 0, None),
            rh_foundation::ElementCardinality::new("Device.version", 0, None),
            rh_foundation::ElementCardinality::new("Device.version.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.version.extension", 0, None),
            rh_foundation::ElementCardinality::new("Device.version.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Device.version.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.version.component", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.version.installDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.version.value", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Device.conformsTo", 0, None),
            rh_foundation::ElementCardinality::new("Device.conformsTo.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.conformsTo.extension", 0, None),
            rh_foundation::ElementCardinality::new("Device.conformsTo.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Device.conformsTo.category", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.conformsTo.specification", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Device.conformsTo.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.property", 0, None),
            rh_foundation::ElementCardinality::new("Device.property.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.property.extension", 0, None),
            rh_foundation::ElementCardinality::new("Device.property.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Device.property.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Device.property.value[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Device.mode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.cycle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.duration", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.owner", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.contact", 0, None),
            rh_foundation::ElementCardinality::new("Device.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Device.endpoint", 0, None),
            rh_foundation::ElementCardinality::new("Device.gateway", 0, None),
            rh_foundation::ElementCardinality::new("Device.note", 0, None),
            rh_foundation::ElementCardinality::new("Device.safety", 0, None),
            rh_foundation::ElementCardinality::new("Device.parent", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Device {
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

impl crate::traits::resource::ResourceMutators for Device {
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

impl crate::traits::resource::ResourceExistence for Device {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Device {
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

impl crate::traits::domain_resource::DomainResourceMutators for Device {
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

impl crate::traits::domain_resource::DomainResourceExistence for Device {
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

impl crate::traits::device::DeviceAccessors for Device {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn display_name(&self) -> Option<StringType> {
        self.display_name.clone()
    }
    fn definition(&self) -> Option<CodeableReference> {
        self.definition.clone()
    }
    fn udi_carrier(&self) -> &[DeviceUdicarrier] {
        self.udi_carrier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Option<DeviceStatus> {
        self.status.clone()
    }
    fn availability_status(&self) -> Option<CodeableConcept> {
        self.availability_status.clone()
    }
    fn biological_source_event(&self) -> Option<Identifier> {
        self.biological_source_event.clone()
    }
    fn manufacturer(&self) -> Option<StringType> {
        self.manufacturer.clone()
    }
    fn manufacture_date(&self) -> Option<DateTimeType> {
        self.manufacture_date.clone()
    }
    fn expiration_date(&self) -> Option<DateTimeType> {
        self.expiration_date.clone()
    }
    fn lot_number(&self) -> Option<StringType> {
        self.lot_number.clone()
    }
    fn serial_number(&self) -> Option<StringType> {
        self.serial_number.clone()
    }
    fn name(&self) -> &[DeviceName] {
        self.name.as_deref().unwrap_or(&[])
    }
    fn model_number(&self) -> Option<StringType> {
        self.model_number.clone()
    }
    fn part_number(&self) -> Option<StringType> {
        self.part_number.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> &[CodeableConcept] {
        self.type_.as_deref().unwrap_or(&[])
    }
    fn version(&self) -> &[DeviceVersion] {
        self.version.as_deref().unwrap_or(&[])
    }
    fn conforms_to(&self) -> &[DeviceConformsto] {
        self.conforms_to.as_deref().unwrap_or(&[])
    }
    fn property(&self) -> &[DeviceProperty] {
        self.property.as_deref().unwrap_or(&[])
    }
    fn mode(&self) -> Option<CodeableConcept> {
        self.mode.clone()
    }
    fn cycle(&self) -> Option<Count> {
        self.cycle.clone()
    }
    fn duration(&self) -> Option<Duration> {
        self.duration.clone()
    }
    fn owner(&self) -> Option<Reference> {
        self.owner.clone()
    }
    fn contact(&self) -> &[ContactPoint] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn endpoint(&self) -> &[Reference] {
        self.endpoint.as_deref().unwrap_or(&[])
    }
    fn gateway(&self) -> &[CodeableReference] {
        self.gateway.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn safety(&self) -> &[CodeableConcept] {
        self.safety.as_deref().unwrap_or(&[])
    }
    fn parent(&self) -> Option<Reference> {
        self.parent.clone()
    }
}

impl crate::traits::device::DeviceMutators for Device {
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
    fn set_display_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.display_name = Some(value);
        resource
    }
    fn set_definition(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.definition = Some(value);
        resource
    }
    fn set_udi_carrier(self, value: Vec<DeviceUdicarrier>) -> Self {
        let mut resource = self.clone();
        resource.udi_carrier = Some(value);
        resource
    }
    fn add_udi_carrier(self, item: DeviceUdicarrier) -> Self {
        let mut resource = self.clone();
        resource.udi_carrier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: DeviceStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_availability_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.availability_status = Some(value);
        resource
    }
    fn set_biological_source_event(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.biological_source_event = Some(value);
        resource
    }
    fn set_manufacturer(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.manufacturer = Some(value);
        resource
    }
    fn set_manufacture_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.manufacture_date = Some(value);
        resource
    }
    fn set_expiration_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.expiration_date = Some(value);
        resource
    }
    fn set_lot_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.lot_number = Some(value);
        resource
    }
    fn set_serial_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.serial_number = Some(value);
        resource
    }
    fn set_name(self, value: Vec<DeviceName>) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn add_name(self, item: DeviceName) -> Self {
        let mut resource = self.clone();
        resource.name.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_model_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.model_number = Some(value);
        resource
    }
    fn set_part_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.part_number = Some(value);
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
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn add_type_(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_version(self, value: Vec<DeviceVersion>) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn add_version(self, item: DeviceVersion) -> Self {
        let mut resource = self.clone();
        resource.version.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_conforms_to(self, value: Vec<DeviceConformsto>) -> Self {
        let mut resource = self.clone();
        resource.conforms_to = Some(value);
        resource
    }
    fn add_conforms_to(self, item: DeviceConformsto) -> Self {
        let mut resource = self.clone();
        resource.conforms_to.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_property(self, value: Vec<DeviceProperty>) -> Self {
        let mut resource = self.clone();
        resource.property = Some(value);
        resource
    }
    fn add_property(self, item: DeviceProperty) -> Self {
        let mut resource = self.clone();
        resource.property.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_mode(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.mode = Some(value);
        resource
    }
    fn set_cycle(self, value: Count) -> Self {
        let mut resource = self.clone();
        resource.cycle = Some(value);
        resource
    }
    fn set_duration(self, value: Duration) -> Self {
        let mut resource = self.clone();
        resource.duration = Some(value);
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
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
    }
    fn set_endpoint(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.endpoint = Some(value);
        resource
    }
    fn add_endpoint(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.endpoint.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_gateway(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.gateway = Some(value);
        resource
    }
    fn add_gateway(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.gateway.get_or_insert_with(Vec::new).push(item);
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
    fn set_parent(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.parent = Some(value);
        resource
    }
}

impl crate::traits::device::DeviceExistence for Device {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_display_name(&self) -> bool {
        self.display_name.is_some()
    }
    fn has_definition(&self) -> bool {
        self.definition.is_some()
    }
    fn has_udi_carrier(&self) -> bool {
        self.udi_carrier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_availability_status(&self) -> bool {
        self.availability_status.is_some()
    }
    fn has_biological_source_event(&self) -> bool {
        self.biological_source_event.is_some()
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.is_some()
    }
    fn has_manufacture_date(&self) -> bool {
        self.manufacture_date.is_some()
    }
    fn has_expiration_date(&self) -> bool {
        self.expiration_date.is_some()
    }
    fn has_lot_number(&self) -> bool {
        self.lot_number.is_some()
    }
    fn has_serial_number(&self) -> bool {
        self.serial_number.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_model_number(&self) -> bool {
        self.model_number.is_some()
    }
    fn has_part_number(&self) -> bool {
        self.part_number.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_type_(&self) -> bool {
        self.type_.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_version(&self) -> bool {
        self.version.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_conforms_to(&self) -> bool {
        self.conforms_to.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_property(&self) -> bool {
        self.property.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_mode(&self) -> bool {
        self.mode.is_some()
    }
    fn has_cycle(&self) -> bool {
        self.cycle.is_some()
    }
    fn has_duration(&self) -> bool {
        self.duration.is_some()
    }
    fn has_owner(&self) -> bool {
        self.owner.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_endpoint(&self) -> bool {
        self.endpoint.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_gateway(&self) -> bool {
        self.gateway.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_safety(&self) -> bool {
        self.safety.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}

impl crate::validation::ValidatableResource for Device {
    fn resource_type(&self) -> &'static str {
        "Device"
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
        Some("http://hl7.org/fhir/StructureDefinition/Device")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::device::{DeviceAccessors, DeviceExistence, DeviceMutators};
