use crate::bindings::color_codes::ColorCodes;
use crate::bindings::metric_category::MetricCategory;
use crate::bindings::metric_operational_status::MetricOperationalStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::resources::device_metric::DeviceMetricCalibration;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DeviceMetric Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes a measurement, calculation or setting capability of a device.  The DeviceMetric resource is derived from the ISO/IEEE 11073-10201 Domain Information Model standard, but is more widely applicable.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceMetric
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceMetric
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceMetricAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the type field.
    fn type_(&self) -> CodeableConcept;
    /// Returns a reference to the unit field.
    fn unit(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the device field.
    fn device(&self) -> Reference;
    /// Returns a reference to the operationalStatus field.
    fn operational_status(&self) -> Option<MetricOperationalStatus>;
    /// Returns a reference to the color field.
    fn color(&self) -> Option<ColorCodes>;
    /// Returns a reference to the category field.
    fn category(&self) -> MetricCategory;
    /// Returns a reference to the measurementFrequency field.
    fn measurement_frequency(&self) -> Option<Quantity>;
    /// Returns a reference to the calibration field.
    fn calibration(&self) -> &[DeviceMetricCalibration];
}
/// DeviceMetric Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes a measurement, calculation or setting capability of a device.  The DeviceMetric resource is derived from the ISO/IEEE 11073-10201 Domain Information Model standard, but is more widely applicable.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceMetric
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceMetric
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceMetricMutators: DomainResourceMutators {
    /// Create a new DeviceMetric with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::device_metric::DeviceMetric;
    /// use rh_hl7_fhir_r5_core::traits::device_metric::DeviceMetricMutators;
    ///
    /// let resource = DeviceMetric::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the unit field and returns self for chaining.
    fn set_unit(self, value: CodeableConcept) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: Reference) -> Self;
    /// Sets the operationalStatus field and returns self for chaining.
    fn set_operational_status(self, value: MetricOperationalStatus) -> Self;
    /// Sets the color field and returns self for chaining.
    fn set_color(self, value: ColorCodes) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: MetricCategory) -> Self;
    /// Sets the measurementFrequency field and returns self for chaining.
    fn set_measurement_frequency(self, value: Quantity) -> Self;
    /// Sets the calibration field and returns self for chaining.
    fn set_calibration(self, value: Vec<DeviceMetricCalibration>) -> Self;
    /// Adds an item to the calibration field and returns self for chaining.
    fn add_calibration(self, item: DeviceMetricCalibration) -> Self;
}
/// DeviceMetric Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes a measurement, calculation or setting capability of a device.  The DeviceMetric resource is derived from the ISO/IEEE 11073-10201 Domain Information Model standard, but is more widely applicable.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceMetric
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceMetric
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceMetricExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the unit field is present (Some).
    fn has_unit(&self) -> bool;
    /// Returns true if the device field is present (Some).
    fn has_device(&self) -> bool;
    /// Returns true if the operational_status field is present (Some).
    fn has_operational_status(&self) -> bool;
    /// Returns true if the color field is present (Some).
    fn has_color(&self) -> bool;
    /// Returns true if the category field is present (Some).
    fn has_category(&self) -> bool;
    /// Returns true if the measurement_frequency field is present (Some).
    fn has_measurement_frequency(&self) -> bool;
    /// Returns true if the calibration field is not empty.
    fn has_calibration(&self) -> bool;
}
