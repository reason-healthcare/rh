use crate::bindings::specimen_status::SpecimenStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Specimen
///
/// A sample to be used for analysis.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Specimen
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Specimen
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specimen {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External Identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Identifier assigned by the lab
    #[serde(rename = "accessionIdentifier")]
    pub accession_identifier: Option<Identifier>,
    /// available | unavailable | unsatisfactory | entered-in-error
    pub status: Option<SpecimenStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Kind of material that forms the specimen
    ///
    /// Binding: example (The type of the specimen.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0487
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Where the specimen came from. This may be from patient(s), from a location (e.g., the source of an environmental sample), or a sampling of a substance or a device
    pub subject: Option<Reference>,
    /// The time when specimen was received for processing
    #[serde(rename = "receivedTime")]
    pub received_time: Option<DateTimeType>,
    /// Extension element for the 'receivedTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_receivedTime")]
    pub _received_time: Option<Element>,
    /// Specimen from which this specimen originated
    pub parent: Option<Vec<Reference>>,
    /// Why the specimen was collected
    pub request: Option<Vec<Reference>>,
    /// Collection details
    pub collection: Option<SpecimenCollection>,
    /// Processing and processing step details
    pub processing: Option<Vec<SpecimenProcessing>>,
    /// Direct container of specimen (tube/slide, etc.)
    pub container: Option<Vec<SpecimenContainer>>,
    /// State of the specimen
    ///
    /// Binding: extensible (Codes describing the state of the specimen.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0493
    pub condition: Option<Vec<CodeableConcept>>,
    /// Comments
    pub note: Option<Vec<Annotation>>,
}
/// Specimen nested structure for the 'collection' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenCollection {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Who collected the specimen
    pub collector: Option<Reference>,
    /// Collection time (dateTime)
    #[serde(rename = "collectedDateTime")]
    pub collected_date_time: Option<DateTimeType>,
    /// Collection time (Period)
    #[serde(rename = "collectedPeriod")]
    pub collected_period: Option<Period>,
    /// How long it took to collect specimen
    pub duration: Option<Duration>,
    /// The quantity of specimen collected
    pub quantity: Option<Quantity>,
    /// Technique used to perform collection
    ///
    /// Binding: example (The  technique that is used to perform the procedure.)
    ///
    /// Available values:
    /// - `129316008`: Aspiration - action
    /// - `129314006`: Biopsy - action
    /// - `129300006`: Puncture - action
    /// - `129304002`: Excision - action
    /// - `129323009`: Scraping - action
    /// - `73416001`: Urine specimen collection, clean catch
    /// - `225113003`: Timed urine collection
    /// - `70777001`: Urine specimen collection, catheterized
    /// - `386089008`: Collection of coughed sputum
    /// - `278450005`: Finger-prick sampling
    pub method: Option<CodeableConcept>,
    /// Anatomical collection site
    ///
    /// Binding: example (Codes describing anatomical locations. May include laterality.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/body-site
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,
    /// Whether or how long patient abstained from food and/or drink (CodeableConcept)
    #[serde(rename = "fastingStatusCodeableConcept")]
    pub fasting_status_codeable_concept: Option<CodeableConcept>,
    /// Whether or how long patient abstained from food and/or drink (Duration)
    #[serde(rename = "fastingStatusDuration")]
    pub fasting_status_duration: Option<Duration>,
}
/// sequenceNumber
///
/// An assigned number on the specimen denoting the order of collection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/specimen-sequenceNumber
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenSequenceNumber {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Specimen nested structure for the 'processing' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenProcessing {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Textual description of procedure
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Indicates the treatment step  applied to the specimen
    ///
    /// Binding: example (Type indicating the technique used to process the specimen.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/specimen-processing-procedure
    pub procedure: Option<CodeableConcept>,
    /// Material used in the processing step
    pub additive: Option<Vec<Reference>>,
    /// Date and time of specimen processing (dateTime)
    #[serde(rename = "timeDateTime")]
    pub time_date_time: Option<DateTimeType>,
    /// Date and time of specimen processing (Period)
    #[serde(rename = "timePeriod")]
    pub time_period: Option<Period>,
}
/// Specimen nested structure for the 'container' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenContainer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Id for the container
    pub identifier: Option<Vec<Identifier>>,
    /// Textual description of the container
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Kind of container directly associated with specimen
    ///
    /// Binding: example (Type of specimen container.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/specimen-container-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Container volume or size
    pub capacity: Option<Quantity>,
    /// Quantity of specimen within container
    #[serde(rename = "specimenQuantity")]
    pub specimen_quantity: Option<Quantity>,
    /// Additive associated with container (CodeableConcept)
    #[serde(rename = "additiveCodeableConcept")]
    pub additive_codeable_concept: Option<CodeableConcept>,
    /// Additive associated with container (Reference)
    #[serde(rename = "additiveReference")]
    pub additive_reference: Option<Reference>,
}

impl Default for Specimen {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            accession_identifier: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            type_: Default::default(),
            subject: Default::default(),
            received_time: Default::default(),
            _received_time: Default::default(),
            parent: Default::default(),
            request: Default::default(),
            collection: Default::default(),
            processing: Default::default(),
            container: Default::default(),
            condition: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for SpecimenCollection {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            collector: Default::default(),
            collected_date_time: Default::default(),
            collected_period: Default::default(),
            duration: Default::default(),
            quantity: Default::default(),
            method: Default::default(),
            body_site: Default::default(),
            fasting_status_codeable_concept: Default::default(),
            fasting_status_duration: Default::default(),
        }
    }
}

impl Default for SpecimenSequenceNumber {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for SpecimenProcessing {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            procedure: Default::default(),
            additive: Default::default(),
            time_date_time: Default::default(),
            time_period: Default::default(),
        }
    }
}

impl Default for SpecimenContainer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            type_: Default::default(),
            capacity: Default::default(),
            specimen_quantity: Default::default(),
            additive_codeable_concept: Default::default(),
            additive_reference: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Specimen {
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

impl crate::traits::resource::ResourceMutators for Specimen {
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

impl crate::traits::resource::ResourceExistence for Specimen {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Specimen {
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

impl crate::traits::domain_resource::DomainResourceMutators for Specimen {
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

impl crate::traits::domain_resource::DomainResourceExistence for Specimen {
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

impl crate::traits::specimen::SpecimenAccessors for Specimen {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn accession_identifier(&self) -> Option<Identifier> {
        self.accession_identifier.clone()
    }
    fn status(&self) -> Option<SpecimenStatus> {
        self.status.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn received_time(&self) -> Option<DateTimeType> {
        self.received_time.clone()
    }
    fn parent(&self) -> &[Reference] {
        self.parent.as_deref().unwrap_or(&[])
    }
    fn request(&self) -> &[Reference] {
        self.request.as_deref().unwrap_or(&[])
    }
    fn collection(&self) -> Option<SpecimenCollection> {
        self.collection.clone()
    }
    fn processing(&self) -> &[SpecimenProcessing] {
        self.processing.as_deref().unwrap_or(&[])
    }
    fn container(&self) -> &[SpecimenContainer] {
        self.container.as_deref().unwrap_or(&[])
    }
    fn condition(&self) -> &[CodeableConcept] {
        self.condition.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::specimen::SpecimenMutators for Specimen {
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
    fn set_accession_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.accession_identifier = Some(value);
        resource
    }
    fn set_status(self, value: SpecimenStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_received_time(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.received_time = Some(value);
        resource
    }
    fn set_parent(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.parent = Some(value);
        resource
    }
    fn add_parent(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.parent.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_request(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.request = Some(value);
        resource
    }
    fn add_request(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.request.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_collection(self, value: SpecimenCollection) -> Self {
        let mut resource = self.clone();
        resource.collection = Some(value);
        resource
    }
    fn set_processing(self, value: Vec<SpecimenProcessing>) -> Self {
        let mut resource = self.clone();
        resource.processing = Some(value);
        resource
    }
    fn add_processing(self, item: SpecimenProcessing) -> Self {
        let mut resource = self.clone();
        resource.processing.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_container(self, value: Vec<SpecimenContainer>) -> Self {
        let mut resource = self.clone();
        resource.container = Some(value);
        resource
    }
    fn add_container(self, item: SpecimenContainer) -> Self {
        let mut resource = self.clone();
        resource.container.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_condition(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.condition = Some(value);
        resource
    }
    fn add_condition(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.condition.get_or_insert_with(Vec::new).push(item);
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

impl crate::traits::specimen::SpecimenExistence for Specimen {
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
    fn has_accession_identifier(&self) -> bool {
        self.accession_identifier.is_some()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_received_time(&self) -> bool {
        self.received_time.is_some()
    }
    fn has_parent(&self) -> bool {
        self.parent.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_request(&self) -> bool {
        self.request.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_collection(&self) -> bool {
        self.collection.is_some()
    }
    fn has_processing(&self) -> bool {
        self.processing.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_container(&self) -> bool {
        self.container.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_condition(&self) -> bool {
        self.condition.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}
