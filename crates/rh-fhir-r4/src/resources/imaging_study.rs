use crate::bindings::imagingstudy_status::ImagingstudyStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
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
/// - Version: 4.0.1
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
    /// All series modality if actual acquisition modalities
    ///
    /// Binding: extensible (Type of acquired data in the instance.)
    ///
    /// ValueSet: http://dicom.nema.org/medical/dicom/current/output/chtml/part16/sect_CID_29.html
    pub modality: Option<Vec<Coding>>,
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
    /// Referring physician
    pub referrer: Option<Reference>,
    /// Who interpreted images
    pub interpreter: Option<Vec<Reference>>,
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
    /// The performed Procedure reference
    #[serde(rename = "procedureReference")]
    pub procedure_reference: Option<Reference>,
    /// The performed procedure code
    ///
    /// Binding: extensible (The performed procedure type.)
    ///
    /// ValueSet: http://www.rsna.org/RadLex_Playbook.aspx
    #[serde(rename = "procedureCode")]
    pub procedure_code: Option<Vec<CodeableConcept>>,
    /// Where ImagingStudy occurred
    pub location: Option<Reference>,
    /// Why the study was requested
    ///
    /// Binding: example (The reason for the study.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-reason
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Why was study performed
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// User-defined comments
    pub note: Option<Vec<Annotation>>,
    /// Institution-generated description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Each study has one or more series of instances
    pub series: Option<Vec<ImagingStudySeries>>,
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
    /// The modality of the instances in the series
    ///
    /// Binding: extensible (Type of acquired data in the instance.)
    ///
    /// ValueSet: http://dicom.nema.org/medical/dicom/current/output/chtml/part16/sect_CID_29.html
    pub modality: Coding,
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
    /// Binding: example (Codes describing anatomical locations. May include laterality.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/body-site
    #[serde(rename = "bodySite")]
    pub body_site: Option<Coding>,
    /// Body part laterality
    ///
    /// Binding: example (Codes describing body site laterality (left, right, etc.).)
    ///
    /// Available values:
    /// - `419161000`: Unilateral left
    /// - `419465000`: Unilateral right
    /// - `51440002`: Bilateral
    pub laterality: Option<Coding>,
    /// Specimen imaged
    pub specimen: Option<Vec<Reference>>,
    /// When the series started
    pub started: Option<DateTimeType>,
    /// Extension element for the 'started' primitive field. Contains metadata and extensions.
    pub _started: Option<Element>,
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
            referrer: Default::default(),
            interpreter: Default::default(),
            endpoint: Default::default(),
            number_of_series: Default::default(),
            _number_of_series: Default::default(),
            number_of_instances: Default::default(),
            _number_of_instances: Default::default(),
            procedure_reference: Default::default(),
            procedure_code: Default::default(),
            location: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            note: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            series: Default::default(),
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

impl Default for ImagingStudySeriesPerformer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: Default::default(),
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
            modality: Coding::default(),
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

impl crate::traits::imaging_study::ImagingStudyAccessors for ImagingStudy {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ImagingstudyStatus {
        self.status.clone()
    }
    fn modality(&self) -> &[Coding] {
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
    fn referrer(&self) -> Option<Reference> {
        self.referrer.clone()
    }
    fn interpreter(&self) -> &[Reference] {
        self.interpreter.as_deref().unwrap_or(&[])
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
    fn procedure_reference(&self) -> Option<Reference> {
        self.procedure_reference.clone()
    }
    fn procedure_code(&self) -> &[CodeableConcept] {
        self.procedure_code.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
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
    fn set_modality(self, value: Vec<Coding>) -> Self {
        let mut resource = self.clone();
        resource.modality = Some(value);
        resource
    }
    fn add_modality(self, item: Coding) -> Self {
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
    fn set_referrer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.referrer = Some(value);
        resource
    }
    fn set_interpreter(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.interpreter = Some(value);
        resource
    }
    fn add_interpreter(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.interpreter.get_or_insert_with(Vec::new).push(item);
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
    fn set_procedure_reference(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.procedure_reference = Some(value);
        resource
    }
    fn set_procedure_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.procedure_code = Some(value);
        resource
    }
    fn add_procedure_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .procedure_code
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
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
    fn set_reason_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.reason_reference = Some(value);
        resource
    }
    fn add_reason_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .reason_reference
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn has_referrer(&self) -> bool {
        self.referrer.is_some()
    }
    fn has_interpreter(&self) -> bool {
        self.interpreter.as_ref().is_some_and(|v| !v.is_empty())
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
    fn has_procedure_reference(&self) -> bool {
        self.procedure_reference.is_some()
    }
    fn has_procedure_code(&self) -> bool {
        self.procedure_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
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
