use crate::bindings::fm_status::FmStatus;
use crate::bindings::vision_base_codes::VisionBaseCodes;
use crate::bindings::vision_eye_codes::VisionEyeCodes;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// VisionPrescription
///
/// An authorization for the provision of glasses and/or contact lenses to a patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/VisionPrescription
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: VisionPrescription
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionPrescription {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for vision prescription
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    pub status: FmStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Response creation date
    pub created: DateTimeType,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Who prescription is for
    pub patient: Reference,
    /// Created during encounter / admission / stay
    pub encounter: Option<Reference>,
    /// When prescription was authorized
    #[serde(rename = "dateWritten")]
    pub date_written: DateTimeType,
    /// Extension element for the 'dateWritten' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dateWritten")]
    pub _date_written: Option<Element>,
    /// Who authorized the vision prescription
    pub prescriber: Reference,
    /// Vision lens authorization
    #[serde(rename = "lensSpecification")]
    pub lens_specification: Vec<VisionPrescriptionLensspecification>,
}
/// VisionPrescriptionLensspecification nested structure for the 'prism' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionPrescriptionLensspecificationPrism {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Amount of adjustment
    pub amount: DecimalType,
    /// Extension element for the 'amount' primitive field. Contains metadata and extensions.
    pub _amount: Option<Element>,
    /// up | down | in | out
    #[serde(rename = "base")]
    pub base_definition: VisionBaseCodes,
    /// Extension element for the 'base' primitive field. Contains metadata and extensions.
    pub _base: Option<Element>,
}
/// VisionPrescription nested structure for the 'lensSpecification' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionPrescriptionLensspecification {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Eye alignment compensation
    pub prism: Option<Vec<VisionPrescriptionLensspecificationPrism>>,
    /// Product to be supplied
    ///
    /// Binding: example (A coded concept describing the vision products.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/vision-product
    pub product: CodeableConcept,
    /// right | left
    pub eye: VisionEyeCodes,
    /// Extension element for the 'eye' primitive field. Contains metadata and extensions.
    pub _eye: Option<Element>,
    /// Power of the lens
    pub sphere: Option<DecimalType>,
    /// Extension element for the 'sphere' primitive field. Contains metadata and extensions.
    pub _sphere: Option<Element>,
    /// Lens power for astigmatism
    pub cylinder: Option<DecimalType>,
    /// Extension element for the 'cylinder' primitive field. Contains metadata and extensions.
    pub _cylinder: Option<Element>,
    /// Lens meridian which contain no power for astigmatism
    pub axis: Option<IntegerType>,
    /// Extension element for the 'axis' primitive field. Contains metadata and extensions.
    pub _axis: Option<Element>,
    /// Added power for multifocal levels
    pub add: Option<DecimalType>,
    /// Extension element for the 'add' primitive field. Contains metadata and extensions.
    pub _add: Option<Element>,
    /// Contact lens power
    pub power: Option<DecimalType>,
    /// Extension element for the 'power' primitive field. Contains metadata and extensions.
    pub _power: Option<Element>,
    /// Contact lens back curvature
    #[serde(rename = "backCurve")]
    pub back_curve: Option<DecimalType>,
    /// Extension element for the 'backCurve' primitive field. Contains metadata and extensions.
    #[serde(rename = "_backCurve")]
    pub _back_curve: Option<Element>,
    /// Contact lens diameter
    pub diameter: Option<DecimalType>,
    /// Extension element for the 'diameter' primitive field. Contains metadata and extensions.
    pub _diameter: Option<Element>,
    /// Lens wear duration
    pub duration: Option<Quantity>,
    /// Color required
    pub color: Option<StringType>,
    /// Extension element for the 'color' primitive field. Contains metadata and extensions.
    pub _color: Option<Element>,
    /// Brand required
    pub brand: Option<StringType>,
    /// Extension element for the 'brand' primitive field. Contains metadata and extensions.
    pub _brand: Option<Element>,
    /// Notes for coatings
    pub note: Option<Vec<Annotation>>,
}

impl Default for VisionPrescription {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: FmStatus::default(),
            _status: Default::default(),
            created: DateTimeType::default(),
            _created: Default::default(),
            patient: Reference::default(),
            encounter: Default::default(),
            date_written: DateTimeType::default(),
            _date_written: Default::default(),
            prescriber: Reference::default(),
            lens_specification: Vec::new(),
        }
    }
}

impl Default for VisionPrescriptionLensspecificationPrism {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            amount: Default::default(),
            _amount: Default::default(),
            base_definition: Default::default(),
            _base: Default::default(),
        }
    }
}

impl Default for VisionPrescriptionLensspecification {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            prism: Default::default(),
            product: Default::default(),
            eye: Default::default(),
            _eye: Default::default(),
            sphere: Default::default(),
            _sphere: Default::default(),
            cylinder: Default::default(),
            _cylinder: Default::default(),
            axis: Default::default(),
            _axis: Default::default(),
            add: Default::default(),
            _add: Default::default(),
            power: Default::default(),
            _power: Default::default(),
            back_curve: Default::default(),
            _back_curve: Default::default(),
            diameter: Default::default(),
            _diameter: Default::default(),
            duration: Default::default(),
            color: Default::default(),
            _color: Default::default(),
            brand: Default::default(),
            _brand: Default::default(),
            note: Default::default(),
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

// Trait implementations
impl crate::traits::resource::ResourceAccessors for VisionPrescription {
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

impl crate::traits::resource::ResourceMutators for VisionPrescription {
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

impl crate::traits::resource::ResourceExistence for VisionPrescription {
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

impl crate::traits::domain_resource::DomainResourceAccessors for VisionPrescription {
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

impl crate::traits::domain_resource::DomainResourceMutators for VisionPrescription {
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

impl crate::traits::domain_resource::DomainResourceExistence for VisionPrescription {
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

impl crate::traits::vision_prescription::VisionPrescriptionAccessors for VisionPrescription {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> FmStatus {
        self.status.clone()
    }
    fn created(&self) -> DateTimeType {
        self.created.clone()
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn date_written(&self) -> DateTimeType {
        self.date_written.clone()
    }
    fn prescriber(&self) -> Reference {
        self.prescriber.clone()
    }
    fn lens_specification(&self) -> &[VisionPrescriptionLensspecification] {
        &self.lens_specification
    }
}

impl crate::traits::vision_prescription::VisionPrescriptionMutators for VisionPrescription {
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
    fn set_status(self, value: FmStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_created(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.created = value;
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_date_written(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date_written = value;
        resource
    }
    fn set_prescriber(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.prescriber = value;
        resource
    }
    fn set_lens_specification(self, value: Vec<VisionPrescriptionLensspecification>) -> Self {
        let mut resource = self.clone();
        resource.lens_specification = value;
        resource
    }
    fn add_lens_specification(self, item: VisionPrescriptionLensspecification) -> Self {
        let mut resource = self.clone();
        resource.lens_specification.push(item);
        resource
    }
}

impl crate::traits::vision_prescription::VisionPrescriptionExistence for VisionPrescription {
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
    fn has_status(&self) -> bool {
        true
    }
    fn has_created(&self) -> bool {
        true
    }
    fn has_patient(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_date_written(&self) -> bool {
        true
    }
    fn has_prescriber(&self) -> bool {
        true
    }
    fn has_lens_specification(&self) -> bool {
        !self.lens_specification.is_empty()
    }
}

impl crate::validation::ValidatableResource for VisionPrescription {
    fn resource_type(&self) -> &'static str {
        "VisionPrescription"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/VisionPrescription")
    }
}
