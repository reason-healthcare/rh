use crate::bindings::metric_calibration_state::MetricCalibrationState;
use crate::bindings::metric_calibration_type::MetricCalibrationType;
use crate::bindings::metric_category::MetricCategory;
use crate::bindings::metric_color::MetricColor;
use crate::bindings::metric_operational_status::MetricOperationalStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::instant::InstantType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// DeviceMetric
///
/// Describes a measurement, calculation or setting capability of a medical device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceMetric
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DeviceMetric
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMetric {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Instance identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Identity of metric, for example Heart Rate or PEEP Setting
    ///
    /// Binding: preferred (Describes the metric type.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/devicemetric-type
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Unit of Measure for the Metric
    ///
    /// Binding: preferred (Describes the unit of the metric.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/devicemetric-type
    pub unit: Option<CodeableConcept>,
    /// Describes the link to the source Device
    pub source: Option<Reference>,
    /// Describes the link to the parent Device
    pub parent: Option<Reference>,
    /// on | off | standby | entered-in-error
    #[serde(rename = "operationalStatus")]
    pub operational_status: Option<MetricOperationalStatus>,
    /// Extension element for the 'operationalStatus' primitive field. Contains metadata and extensions.
    #[serde(rename = "_operationalStatus")]
    pub _operational_status: Option<Element>,
    /// black | red | green | yellow | blue | magenta | cyan | white
    pub color: Option<MetricColor>,
    /// Extension element for the 'color' primitive field. Contains metadata and extensions.
    pub _color: Option<Element>,
    /// measurement | setting | calculation | unspecified
    pub category: MetricCategory,
    /// Extension element for the 'category' primitive field. Contains metadata and extensions.
    pub _category: Option<Element>,
    /// Describes the measurement repetition time
    #[serde(rename = "measurementPeriod")]
    pub measurement_period: Option<Timing>,
    /// Describes the calibrations that have been performed or that are required to be performed
    pub calibration: Option<Vec<DeviceMetricCalibration>>,
}
/// DeviceMetric nested structure for the 'calibration' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMetricCalibration {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// unspecified | offset | gain | two-point
    #[serde(rename = "type")]
    pub type_: Option<MetricCalibrationType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// not-calibrated | calibration-required | calibrated | unspecified
    pub state: Option<MetricCalibrationState>,
    /// Extension element for the 'state' primitive field. Contains metadata and extensions.
    pub _state: Option<Element>,
    /// Describes the time last calibration has been performed
    pub time: Option<InstantType>,
    /// Extension element for the 'time' primitive field. Contains metadata and extensions.
    pub _time: Option<Element>,
}

impl Default for DeviceMetric {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            type_: Default::default(),
            unit: Default::default(),
            source: Default::default(),
            parent: Default::default(),
            operational_status: Default::default(),
            _operational_status: Default::default(),
            color: Default::default(),
            _color: Default::default(),
            category: MetricCategory::default(),
            _category: Default::default(),
            measurement_period: Default::default(),
            calibration: Default::default(),
        }
    }
}

impl Default for DeviceMetricCalibration {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            state: Default::default(),
            _state: Default::default(),
            time: Default::default(),
            _time: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DeviceMetric {
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

impl crate::traits::resource::ResourceMutators for DeviceMetric {
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

impl crate::traits::resource::ResourceExistence for DeviceMetric {
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

impl crate::traits::domain_resource::DomainResourceAccessors for DeviceMetric {
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

impl crate::traits::domain_resource::DomainResourceMutators for DeviceMetric {
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

impl crate::traits::domain_resource::DomainResourceExistence for DeviceMetric {
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

impl crate::traits::device_metric::DeviceMetricAccessors for DeviceMetric {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> CodeableConcept {
        self.type_.clone()
    }
    fn unit(&self) -> Option<CodeableConcept> {
        self.unit.clone()
    }
    fn source(&self) -> Option<Reference> {
        self.source.clone()
    }
    fn parent(&self) -> Option<Reference> {
        self.parent.clone()
    }
    fn operational_status(&self) -> Option<MetricOperationalStatus> {
        self.operational_status.clone()
    }
    fn color(&self) -> Option<MetricColor> {
        self.color.clone()
    }
    fn category(&self) -> MetricCategory {
        self.category.clone()
    }
    fn measurement_period(&self) -> Option<Timing> {
        self.measurement_period.clone()
    }
    fn calibration(&self) -> &[DeviceMetricCalibration] {
        self.calibration.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::device_metric::DeviceMetricMutators for DeviceMetric {
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
        resource.type_ = value;
        resource
    }
    fn set_unit(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.unit = Some(value);
        resource
    }
    fn set_source(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.source = Some(value);
        resource
    }
    fn set_parent(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.parent = Some(value);
        resource
    }
    fn set_operational_status(self, value: MetricOperationalStatus) -> Self {
        let mut resource = self.clone();
        resource.operational_status = Some(value);
        resource
    }
    fn set_color(self, value: MetricColor) -> Self {
        let mut resource = self.clone();
        resource.color = Some(value);
        resource
    }
    fn set_category(self, value: MetricCategory) -> Self {
        let mut resource = self.clone();
        resource.category = value;
        resource
    }
    fn set_measurement_period(self, value: Timing) -> Self {
        let mut resource = self.clone();
        resource.measurement_period = Some(value);
        resource
    }
    fn set_calibration(self, value: Vec<DeviceMetricCalibration>) -> Self {
        let mut resource = self.clone();
        resource.calibration = Some(value);
        resource
    }
    fn add_calibration(self, item: DeviceMetricCalibration) -> Self {
        let mut resource = self.clone();
        resource.calibration.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::device_metric::DeviceMetricExistence for DeviceMetric {
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
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_unit(&self) -> bool {
        self.unit.is_some()
    }
    fn has_source(&self) -> bool {
        self.source.is_some()
    }
    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
    fn has_operational_status(&self) -> bool {
        self.operational_status.is_some()
    }
    fn has_color(&self) -> bool {
        self.color.is_some()
    }
    fn has_category(&self) -> bool {
        true
    }
    fn has_measurement_period(&self) -> bool {
        self.measurement_period.is_some()
    }
    fn has_calibration(&self) -> bool {
        self.calibration.as_ref().is_some_and(|v| !v.is_empty())
    }
}
