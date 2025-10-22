use crate::bindings::metric_category::MetricCategory;
use crate::bindings::metric_color::MetricColor;
use crate::bindings::metric_operational_status::MetricOperationalStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::resources::device_metric::DeviceMetricCalibration;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DeviceMetric Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes a measurement, calculation or setting capability of a medical device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceMetric
/// - Version: 4.0.1
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
    /// Returns a reference to the source field.
    fn source(&self) -> Option<Reference>;
    /// Returns a reference to the parent field.
    fn parent(&self) -> Option<Reference>;
    /// Returns a reference to the operationalStatus field.
    fn operational_status(&self) -> Option<MetricOperationalStatus>;
    /// Returns a reference to the color field.
    fn color(&self) -> Option<MetricColor>;
    /// Returns a reference to the category field.
    fn category(&self) -> MetricCategory;
    /// Returns a reference to the measurementPeriod field.
    fn measurement_period(&self) -> Option<Timing>;
    /// Returns a reference to the calibration field.
    fn calibration(&self) -> &[DeviceMetricCalibration];
}
/// DeviceMetric Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes a measurement, calculation or setting capability of a medical device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceMetric
/// - Version: 4.0.1
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
    /// use hl7_fhir_r4_core::resources::device_metric::DeviceMetric;
    /// use hl7_fhir_r4_core::traits::device_metric::DeviceMetricMutators;
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
    /// Sets the source field and returns self for chaining.
    fn set_source(self, value: Reference) -> Self;
    /// Sets the parent field and returns self for chaining.
    fn set_parent(self, value: Reference) -> Self;
    /// Sets the operationalStatus field and returns self for chaining.
    fn set_operational_status(self, value: MetricOperationalStatus) -> Self;
    /// Sets the color field and returns self for chaining.
    fn set_color(self, value: MetricColor) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: MetricCategory) -> Self;
    /// Sets the measurementPeriod field and returns self for chaining.
    fn set_measurement_period(self, value: Timing) -> Self;
    /// Sets the calibration field and returns self for chaining.
    fn set_calibration(self, value: Vec<DeviceMetricCalibration>) -> Self;
    /// Adds an item to the calibration field and returns self for chaining.
    fn add_calibration(self, item: DeviceMetricCalibration) -> Self;
}
/// DeviceMetric Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes a measurement, calculation or setting capability of a medical device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceMetric
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DeviceMetric
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceMetricExistence: DomainResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the text field is present (Some).
    fn has_text(&self) -> bool;
    /// Returns true if the contained field is not empty.
    fn has_contained(&self) -> bool;
    /// Returns true if the extension field is not empty.
    fn has_extension(&self) -> bool;
    /// Returns true if the modifier_extension field is not empty.
    fn has_modifier_extension(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the unit field is present (Some).
    fn has_unit(&self) -> bool;
    /// Returns true if the source field is present (Some).
    fn has_source(&self) -> bool;
    /// Returns true if the parent field is present (Some).
    fn has_parent(&self) -> bool;
    /// Returns true if the operational_status field is present (Some).
    fn has_operational_status(&self) -> bool;
    /// Returns true if the color field is present (Some).
    fn has_color(&self) -> bool;
    /// Returns true if the category field is present (Some).
    fn has_category(&self) -> bool;
    /// Returns true if the measurement_period field is present (Some).
    fn has_measurement_period(&self) -> bool;
    /// Returns true if the calibration field is not empty.
    fn has_calibration(&self) -> bool;
}
