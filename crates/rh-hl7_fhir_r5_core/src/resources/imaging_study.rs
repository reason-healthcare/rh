use crate::bindings::imagingstudy_status::ImagingstudyStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ImagingStudy
///
/// Representation of the content produced in a DICOM imaging study. A study comprises a set of series, each of which includes a set of Service-Object Pair Instances (SOP Instances - images or other data) acquired or produced in a common context.  A series is of only one modality (e.g. X-ray, CT, MR, ultrasound), but a study may have multiple series of different modalities.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImagingStudy
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ImagingStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingStudy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifiers for the whole study
    pub identifier: Option<Vec<Identifier>>,
    /// registered | available | cancelled | entered-in-error | unknown
    pub status: ImagingstudyStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// All of the distinct values for series' modalities
    ///
    /// Binding: extensible (Type of acquired data in the instance.)
    ///
    /// ValueSet: http://dicom.nema.org/medical/dicom/current/output/chtml/part16/sect_CID_33.html
    pub modality: Option<Vec<CodeableConcept>>,
    /// Who or what is the subject of the study
    pub subject: Reference,
    /// Encounter with which this imaging study is associated
    pub encounter: Option<Reference>,
    /// When the study was started
    pub started: Option<DateTimeType>,
    /// Extension element for the 'started' primitive field. Contains metadata and extensions.
    pub _started: Option<Element>,
    /// Request fulfilled
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// Referring physician
    pub referrer: Option<Reference>,
    /// Study access endpoint
    pub endpoint: Option<Vec<Reference>>,
    /// Number of Study Related Series
    #[serde(rename = "numberOfSeries")]
    pub number_of_series: Option<UnsignedIntType>,
    /// Extension element for the 'numberOfSeries' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfSeries")]
    pub _number_of_series: Option<Element>,
    /// Number of Study Related Instances
    #[serde(rename = "numberOfInstances")]
    pub number_of_instances: Option<UnsignedIntType>,
    /// Extension element for the 'numberOfInstances' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfInstances")]
    pub _number_of_instances: Option<Element>,
    /// The performed procedure or code
    ///
    /// Binding: preferred (Use of RadLex is preferred)
    ///
    /// ValueSet: http://loinc.org/vs/loinc-rsna-radiology-playbook
    pub procedure: Option<Vec<CodeableReference>>,
    /// Where ImagingStudy occurred
    pub location: Option<Reference>,
    /// Why the study was requested / performed
    ///
    /// Binding: example (The reason for the study.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-reason
    pub reason: Option<Vec<CodeableReference>>,
    /// User-defined comments
    pub note: Option<Vec<Annotation>>,
    /// Institution-generated description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Each study has one or more series of instances
    pub series: Option<Vec<ImagingStudySeries>>,
}
/// ImagingStudy nested structure for the 'series' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingStudySeries {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Who performed the series
    pub performer: Option<Vec<ImagingStudySeriesPerformer>>,
    /// A single SOP instance from the series
    pub instance: Option<Vec<ImagingStudySeriesInstance>>,
    /// DICOM Series Instance UID for the series
    pub uid: StringType,
    /// Extension element for the 'uid' primitive field. Contains metadata and extensions.
    pub _uid: Option<Element>,
    /// Numeric identifier of this series
    pub number: Option<UnsignedIntType>,
    /// Extension element for the 'number' primitive field. Contains metadata and extensions.
    pub _number: Option<Element>,
    /// The modality used for this series
    ///
    /// Binding: extensible (Type of acquired data in the instance.)
    ///
    /// ValueSet: http://dicom.nema.org/medical/dicom/current/output/chtml/part16/sect_CID_33.html
    pub modality: CodeableConcept,
    /// A short human readable summary of the series
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Number of Series Related Instances
    #[serde(rename = "numberOfInstances")]
    pub number_of_instances: Option<UnsignedIntType>,
    /// Extension element for the 'numberOfInstances' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfInstances")]
    pub _number_of_instances: Option<Element>,
    /// Series access endpoint
    pub endpoint: Option<Vec<Reference>>,
    /// Body part examined
    ///
    /// Binding: example (SNOMED CT Body site concepts)
    ///
    /// Available values:
    /// - `53075003`: Distal phalanx of hallux
    /// - `76986006`: Distal phalanx of second toe
    /// - `65258003`: Distal phalanx of third toe
    /// - `54333003`: Distal phalanx of fourth toe
    /// - `10770001`: Distal phalanx of fifth toe
    /// - `363670009`: Interphalangeal joint structure of great toe
    /// - `371216008`: Distal interphalangeal joint of second toe
    /// - `371219001`: Distal interphalangeal joint of third toe
    /// - `371205001`: Distal interphalangeal joint of fourth toe
    /// - `371203008`: Distal interphalangeal joint of fifth toe
    /// - ... and 30 more values
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableReference>,
    /// Body part laterality
    ///
    /// Binding: example (Codes describing body site laterality (left, right, etc.).)
    ///
    /// ValueSet: http://dicom.nema.org/medical/dicom/current/output/chtml/part16/sect_CID_244.html
    pub laterality: Option<CodeableConcept>,
    /// Specimen imaged
    pub specimen: Option<Vec<Reference>>,
    /// When the series started
    pub started: Option<DateTimeType>,
    /// Extension element for the 'started' primitive field. Contains metadata and extensions.
    pub _started: Option<Element>,
}
/// ImagingStudySeries nested structure for the 'performer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingStudySeriesPerformer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of performance
    ///
    /// Binding: extensible (The type of involvement of the performer.)
    ///
    /// Available values:
    /// - `CON`: consultant
    /// - `VRF`: verifier
    /// - `PRF`: performer
    /// - `SPRF`: secondary performer
    /// - `REF`: referrer
    pub function: Option<CodeableConcept>,
    /// Who performed the series
    pub actor: Reference,
}
/// ImagingStudySeries nested structure for the 'instance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingStudySeriesInstance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// DICOM SOP Instance UID
    pub uid: StringType,
    /// Extension element for the 'uid' primitive field. Contains metadata and extensions.
    pub _uid: Option<Element>,
    /// DICOM class type
    ///
    /// Binding: extensible (The sopClass for the instance.)
    ///
    /// ValueSet: http://dicom.nema.org/medical/dicom/current/output/chtml/part04/sect_B.5.html#table_B.5-1
    #[serde(rename = "sopClass")]
    pub sop_class: Coding,
    /// The number of this instance in the series
    pub number: Option<UnsignedIntType>,
    /// Extension element for the 'number' primitive field. Contains metadata and extensions.
    pub _number: Option<Element>,
    /// Description of instance
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
}

impl Default for ImagingStudy {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: ImagingstudyStatus::default(),
            _status: Default::default(),
            modality: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            started: Default::default(),
            _started: Default::default(),
            based_on: Default::default(),
            part_of: Default::default(),
            referrer: Default::default(),
            endpoint: Default::default(),
            number_of_series: Default::default(),
            _number_of_series: Default::default(),
            number_of_instances: Default::default(),
            _number_of_instances: Default::default(),
            procedure: Default::default(),
            location: Default::default(),
            reason: Default::default(),
            note: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            series: Default::default(),
        }
    }
}

impl Default for ImagingStudySeries {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            performer: Default::default(),
            instance: Default::default(),
            uid: StringType::default(),
            _uid: Default::default(),
            number: Default::default(),
            _number: Default::default(),
            modality: CodeableConcept::default(),
            description: Default::default(),
            _description: Default::default(),
            number_of_instances: Default::default(),
            _number_of_instances: Default::default(),
            endpoint: Default::default(),
            body_site: Default::default(),
            laterality: Default::default(),
            specimen: Default::default(),
            started: Default::default(),
            _started: Default::default(),
        }
    }
}

impl Default for ImagingStudySeriesPerformer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: Default::default(),
        }
    }
}

impl Default for ImagingStudySeriesInstance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            uid: Default::default(),
            _uid: Default::default(),
            sop_class: Default::default(),
            number: Default::default(),
            _number: Default::default(),
            title: Default::default(),
            _title: Default::default(),
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
                "ImagingStudy.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ImagingStudy.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/imagingstudy-status|5.0.0",
            )
            .with_description("The status of the ImagingStudy."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ImagingStudy.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.contained", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.extension", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.modality", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.started", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.partOf", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.referrer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.endpoint", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.numberOfSeries", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.numberOfInstances", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.procedure", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.reason", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.note", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.series", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ImagingStudy.series.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.uid", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.number", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.modality", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ImagingStudy.series.numberOfInstances",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.endpoint", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.bodySite", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.laterality", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.specimen", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.started", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.performer", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.performer.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ImagingStudy.series.performer.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingStudy.series.performer.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingStudy.series.performer.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingStudy.series.performer.actor",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.instance", 0, None),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.instance.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ImagingStudy.series.instance.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingStudy.series.instance.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ImagingStudy.series.instance.uid", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ImagingStudy.series.instance.sopClass",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingStudy.series.instance.number",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingStudy.series.instance.title",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ImagingStudy {
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

impl crate::traits::resource::ResourceMutators for ImagingStudy {
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

impl crate::traits::resource::ResourceExistence for ImagingStudy {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ImagingStudy {
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

impl crate::traits::domain_resource::DomainResourceMutators for ImagingStudy {
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

impl crate::traits::domain_resource::DomainResourceExistence for ImagingStudy {
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

impl crate::traits::imaging_study::ImagingStudyAccessors for ImagingStudy {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ImagingstudyStatus {
        self.status.clone()
    }
    fn modality(&self) -> &[CodeableConcept] {
        self.modality.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn started(&self) -> Option<DateTimeType> {
        self.started.clone()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn referrer(&self) -> Option<Reference> {
        self.referrer.clone()
    }
    fn endpoint(&self) -> &[Reference] {
        self.endpoint.as_deref().unwrap_or(&[])
    }
    fn number_of_series(&self) -> Option<UnsignedIntType> {
        self.number_of_series
    }
    fn number_of_instances(&self) -> Option<UnsignedIntType> {
        self.number_of_instances
    }
    fn procedure(&self) -> &[CodeableReference] {
        self.procedure.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn series(&self) -> &[ImagingStudySeries] {
        self.series.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::imaging_study::ImagingStudyMutators for ImagingStudy {
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
    fn set_status(self, value: ImagingstudyStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_modality(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.modality = Some(value);
        resource
    }
    fn add_modality(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.modality.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_started(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.started = Some(value);
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
    fn set_referrer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.referrer = Some(value);
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
    fn set_number_of_series(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.number_of_series = Some(value);
        resource
    }
    fn set_number_of_instances(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.number_of_instances = Some(value);
        resource
    }
    fn set_procedure(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.procedure = Some(value);
        resource
    }
    fn add_procedure(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.procedure.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_reason(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.reason = Some(value);
        resource
    }
    fn add_reason(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.reason.get_or_insert_with(Vec::new).push(item);
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
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_series(self, value: Vec<ImagingStudySeries>) -> Self {
        let mut resource = self.clone();
        resource.series = Some(value);
        resource
    }
    fn add_series(self, item: ImagingStudySeries) -> Self {
        let mut resource = self.clone();
        resource.series.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::imaging_study::ImagingStudyExistence for ImagingStudy {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_modality(&self) -> bool {
        self.modality.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_started(&self) -> bool {
        self.started.is_some()
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_referrer(&self) -> bool {
        self.referrer.is_some()
    }
    fn has_endpoint(&self) -> bool {
        self.endpoint.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_number_of_series(&self) -> bool {
        self.number_of_series.is_some()
    }
    fn has_number_of_instances(&self) -> bool {
        self.number_of_instances.is_some()
    }
    fn has_procedure(&self) -> bool {
        self.procedure.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_series(&self) -> bool {
        self.series.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for ImagingStudy {
    fn resource_type(&self) -> &'static str {
        "ImagingStudy"
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
        Some("http://hl7.org/fhir/StructureDefinition/ImagingStudy")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::imaging_study::{
    ImagingStudyAccessors, ImagingStudyExistence, ImagingStudyMutators,
};
