use crate::bindings::event_status::EventStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::instant::InstantType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Media
///
/// A photo, video, or audio recording acquired or used in healthcare. The actual content may be inline or provided by direct reference.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Media
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Media
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifier(s) for the image
    pub identifier: Option<Vec<Identifier>>,
    /// Procedure that caused this media to be created
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: EventStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Classification of media as image, video, or audio
    ///
    /// Binding: extensible (Codes for high level media categories.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/media-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The type of acquisition equipment/process
    ///
    /// Binding: example (Detailed information about the type of the image - its kind, purpose, or the kind of equipment used to generate it.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/media-modality
    pub modality: Option<CodeableConcept>,
    /// Imaging view, e.g. Lateral or Antero-posterior
    ///
    /// Binding: example (Imaging view (projection) used when collecting an image.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/media-view
    pub view: Option<CodeableConcept>,
    /// Who/What this Media is a record of
    pub subject: Option<Reference>,
    /// Encounter associated with media
    pub encounter: Option<Reference>,
    /// When Media was collected (dateTime)
    #[serde(rename = "createdDateTime")]
    pub created_date_time: Option<DateTimeType>,
    /// When Media was collected (Period)
    #[serde(rename = "createdPeriod")]
    pub created_period: Option<Period>,
    /// Date/Time this version was made available
    pub issued: Option<InstantType>,
    /// Extension element for the 'issued' primitive field. Contains metadata and extensions.
    pub _issued: Option<Element>,
    /// The person who generated the image
    pub operator: Option<Reference>,
    /// Why was event performed?
    ///
    /// Binding: example (The reason for the media.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-reason
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Observed body part
    ///
    /// Binding: example (Codes describing anatomical locations. May include laterality.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/body-site
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,
    /// Name of the device/manufacturer
    #[serde(rename = "deviceName")]
    pub device_name: Option<StringType>,
    /// Extension element for the 'deviceName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_deviceName")]
    pub _device_name: Option<Element>,
    /// Observing Device
    pub device: Option<Reference>,
    /// Height of the image in pixels (photo/video)
    pub height: Option<PositiveIntType>,
    /// Extension element for the 'height' primitive field. Contains metadata and extensions.
    pub _height: Option<Element>,
    /// Width of the image in pixels (photo/video)
    pub width: Option<PositiveIntType>,
    /// Extension element for the 'width' primitive field. Contains metadata and extensions.
    pub _width: Option<Element>,
    /// Number of frames if > 1 (photo)
    pub frames: Option<PositiveIntType>,
    /// Extension element for the 'frames' primitive field. Contains metadata and extensions.
    pub _frames: Option<Element>,
    /// Length in seconds (audio / video)
    pub duration: Option<DecimalType>,
    /// Extension element for the 'duration' primitive field. Contains metadata and extensions.
    pub _duration: Option<Element>,
    /// Actual Media - reference or data
    pub content: Attachment,
    /// Comments made about the media
    pub note: Option<Vec<Annotation>>,
}

impl Default for Media {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            part_of: Default::default(),
            status: EventStatus::default(),
            _status: Default::default(),
            type_: Default::default(),
            modality: Default::default(),
            view: Default::default(),
            subject: Default::default(),
            encounter: Default::default(),
            created_date_time: Default::default(),
            created_period: Default::default(),
            issued: Default::default(),
            _issued: Default::default(),
            operator: Default::default(),
            reason_code: Default::default(),
            body_site: Default::default(),
            device_name: Default::default(),
            _device_name: Default::default(),
            device: Default::default(),
            height: Default::default(),
            _height: Default::default(),
            width: Default::default(),
            _width: Default::default(),
            frames: Default::default(),
            _frames: Default::default(),
            duration: Default::default(),
            _duration: Default::default(),
            content: Attachment::default(),
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

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "Media.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/event-status|4.0.1",
        )
        .with_description("Codes identifying the lifecycle stage of an event.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Media.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.contained", 0, None),
            rh_foundation::ElementCardinality::new("Media.extension", 0, None),
            rh_foundation::ElementCardinality::new("Media.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Media.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Media.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("Media.partOf", 0, None),
            rh_foundation::ElementCardinality::new("Media.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Media.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.modality", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.view", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.created[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.issued", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.operator", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.reasonCode", 0, None),
            rh_foundation::ElementCardinality::new("Media.bodySite", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.deviceName", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.device", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.height", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.width", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.frames", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.duration", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Media.content", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Media.note", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Media {
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

impl crate::traits::resource::ResourceMutators for Media {
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

impl crate::traits::resource::ResourceExistence for Media {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Media {
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

impl crate::traits::domain_resource::DomainResourceMutators for Media {
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

impl crate::traits::domain_resource::DomainResourceExistence for Media {
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

impl crate::traits::media::MediaAccessors for Media {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> EventStatus {
        self.status.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn modality(&self) -> Option<CodeableConcept> {
        self.modality.clone()
    }
    fn view(&self) -> Option<CodeableConcept> {
        self.view.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn issued(&self) -> Option<InstantType> {
        self.issued.clone()
    }
    fn operator(&self) -> Option<Reference> {
        self.operator.clone()
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn body_site(&self) -> Option<CodeableConcept> {
        self.body_site.clone()
    }
    fn device_name(&self) -> Option<StringType> {
        self.device_name.clone()
    }
    fn device(&self) -> Option<Reference> {
        self.device.clone()
    }
    fn height(&self) -> Option<PositiveIntType> {
        self.height
    }
    fn width(&self) -> Option<PositiveIntType> {
        self.width
    }
    fn frames(&self) -> Option<PositiveIntType> {
        self.frames
    }
    fn duration(&self) -> Option<DecimalType> {
        self.duration
    }
    fn content(&self) -> Attachment {
        self.content.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::media::MediaMutators for Media {
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
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_part_of(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
        resource
    }
    fn add_part_of(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: EventStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_modality(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.modality = Some(value);
        resource
    }
    fn set_view(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.view = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_issued(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.issued = Some(value);
        resource
    }
    fn set_operator(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.operator = Some(value);
        resource
    }
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.reason_code = Some(value);
        resource
    }
    fn add_reason_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason_code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_body_site(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.body_site = Some(value);
        resource
    }
    fn set_device_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.device_name = Some(value);
        resource
    }
    fn set_device(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.device = Some(value);
        resource
    }
    fn set_height(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.height = Some(value);
        resource
    }
    fn set_width(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.width = Some(value);
        resource
    }
    fn set_frames(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.frames = Some(value);
        resource
    }
    fn set_duration(self, value: f64) -> Self {
        let mut resource = self.clone();
        resource.duration = Some(value);
        resource
    }
    fn set_content(self, value: Attachment) -> Self {
        let mut resource = self.clone();
        resource.content = value;
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
}

impl crate::traits::media::MediaExistence for Media {
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
    fn has_created(&self) -> bool {
        self.created_date_time.is_some() || self.created_period.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_modality(&self) -> bool {
        self.modality.is_some()
    }
    fn has_view(&self) -> bool {
        self.view.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_issued(&self) -> bool {
        self.issued.is_some()
    }
    fn has_operator(&self) -> bool {
        self.operator.is_some()
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_body_site(&self) -> bool {
        self.body_site.is_some()
    }
    fn has_device_name(&self) -> bool {
        self.device_name.is_some()
    }
    fn has_device(&self) -> bool {
        self.device.is_some()
    }
    fn has_height(&self) -> bool {
        self.height.is_some()
    }
    fn has_width(&self) -> bool {
        self.width.is_some()
    }
    fn has_frames(&self) -> bool {
        self.frames.is_some()
    }
    fn has_duration(&self) -> bool {
        self.duration.is_some()
    }
    fn has_content(&self) -> bool {
        true
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Media {
    fn resource_type(&self) -> &'static str {
        "Media"
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
        Some("http://hl7.org/fhir/StructureDefinition/Media")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::media::{MediaAccessors, MediaExistence, MediaMutators};
