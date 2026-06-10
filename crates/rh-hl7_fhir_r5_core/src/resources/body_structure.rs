use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// BodyStructure
///
/// Record details about an anatomical structure.  This resource may be used when a coded concept does not provide the necessary detail needed for the use case.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BodyStructure
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: BodyStructure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyStructure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Bodystructure identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this record is in active use
    pub active: Option<BooleanType>,
    /// Extension element for the 'active' primitive field. Contains metadata and extensions.
    pub _active: Option<Element>,
    /// Kind of Structure
    ///
    /// Binding: example (Codes describing anatomic morphology.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/bodystructure-code
    pub morphology: Option<CodeableConcept>,
    /// Included anatomic location(s)
    #[serde(rename = "includedStructure")]
    pub included_structure: Vec<BodyStructureIncludedstructure>,
    /// Excluded anatomic locations(s)
    #[serde(rename = "excludedStructure")]
    pub excluded_structure: Option<Vec<StringType>>,
    /// Text description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Attached images
    pub image: Option<Vec<Attachment>>,
    /// Who this is about
    pub patient: Reference,
}
/// BodyStructureIncludedstructureBodylandmarkorientation nested structure for the 'distanceFromLandmark' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyStructureIncludedstructureBodylandmarkorientationDistancefromlandmark {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Measurement device
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
    pub device: Option<Vec<CodeableReference>>,
    /// Measured distance from body landmark
    pub value: Option<Vec<Quantity>>,
}
/// BodyStructureIncludedstructure nested structure for the 'bodyLandmarkOrientation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyStructureIncludedstructureBodylandmarkorientation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Body ]andmark description
    ///
    /// Binding: example (Select SNOMED code system values. Values used in a podiatry setting to decsribe landmarks on the body.)
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
    #[serde(rename = "landmarkDescription")]
    pub landmark_description: Option<Vec<CodeableConcept>>,
    /// Clockface orientation
    ///
    /// Binding: example (Select SNOMED CT codes. A set of codes that describe a things orientation based on a hourly positions of a clock face.)
    ///
    /// Available values:
    /// - `260318004`: 1 o'clock position
    /// - `260328008`: 2 o'clock position
    /// - `260330005`: 3 o'clock position
    /// - `260333007`: 4 o'clock position
    /// - `260335000`: 5 o'clock position
    /// - `260337008`: 6 o'clock position
    /// - `260339006`: 7 o'clock position
    /// - `260341007`: 8 o'clock position
    /// - `260343005`: 9 o'clock position
    /// - `260322009`: 10 o'clock position
    /// - ... and 2 more values
    #[serde(rename = "clockFacePosition")]
    pub clock_face_position: Option<Vec<CodeableConcept>>,
    /// Relative landmark surface orientation
    ///
    /// Binding: preferred (Select SNOMED code system values. The surface area a body location is in relation to a landmark.)
    ///
    /// Available values:
    /// - `7771000`: Left (qualifier value)
    /// - `24028007`: Right (qualifier value)
    /// - `51440002`: Bilateral
    /// - `46053002`: Distal
    /// - `255554000`: Dorsal
    /// - `264147007`: Plantar
    /// - `261183002`: Upper
    /// - `261122009`: Lower
    /// - `255561001`: Medial
    /// - `49370004`: Lateral
    /// - ... and 5 more values
    #[serde(rename = "surfaceOrientation")]
    pub surface_orientation: Option<Vec<CodeableConcept>>,
}
/// BodyStructure nested structure for the 'includedStructure' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyStructureIncludedstructure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Landmark relative location
    #[serde(rename = "bodyLandmarkOrientation")]
    pub body_landmark_orientation:
        Option<Vec<BodyStructureIncludedstructureBodylandmarkorientation>>,
    /// Code that represents the included structure
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
    pub structure: CodeableConcept,
    /// Code that represents the included structure laterality
    ///
    /// Binding: example (Concepts modifying the anatomic location.)
    ///
    /// Available values:
    /// - `7771000`: Left (qualifier value)
    /// - `24028007`: Right (qualifier value)
    /// - `51440002`: Bilateral
    /// - `46053002`: Distal
    /// - `255554000`: Dorsal
    /// - `264147007`: Plantar
    /// - `261183002`: Upper
    /// - `261122009`: Lower
    /// - `255561001`: Medial
    /// - `49370004`: Lateral
    /// - ... and 5 more values
    pub laterality: Option<CodeableConcept>,
    /// Cartesian reference for structure
    #[serde(rename = "spatialReference")]
    pub spatial_reference: Option<Vec<Reference>>,
    /// Code that represents the included structure qualifier
    ///
    /// Binding: example (Concepts modifying the anatomic location.)
    ///
    /// Available values:
    /// - `7771000`: Left (qualifier value)
    /// - `24028007`: Right (qualifier value)
    /// - `51440002`: Bilateral
    /// - `46053002`: Distal
    /// - `255554000`: Dorsal
    /// - `264147007`: Plantar
    /// - `261183002`: Upper
    /// - `261122009`: Lower
    /// - `255561001`: Medial
    /// - `49370004`: Lateral
    /// - ... and 5 more values
    pub qualifier: Option<Vec<CodeableConcept>>,
}

impl Default for BodyStructure {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            active: Default::default(),
            _active: Default::default(),
            morphology: Default::default(),
            included_structure: Vec::new(),
            excluded_structure: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            image: Default::default(),
            patient: Reference::default(),
        }
    }
}

impl Default for BodyStructureIncludedstructureBodylandmarkorientationDistancefromlandmark {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            device: Default::default(),
            value: Default::default(),
        }
    }
}

impl Default for BodyStructureIncludedstructureBodylandmarkorientation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            landmark_description: Default::default(),
            clock_face_position: Default::default(),
            surface_orientation: Default::default(),
        }
    }
}

impl Default for BodyStructureIncludedstructure {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            body_landmark_orientation: Default::default(),
            structure: Default::default(),
            laterality: Default::default(),
            spatial_reference: Default::default(),
            qualifier: Default::default(),
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
        vec![rh_foundation::ElementBinding::new(
            "BodyStructure.language",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
        )
        .with_description("IETF language tag for a human language")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementCardinality::new("BodyStructure.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.meta", 0, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.implicitRules", 0, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.language", 0, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.text", 0, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.contained", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.extension", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.identifier", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.active", 0, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.morphology", 0, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure", 1, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.extension", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.structure", 1, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.laterality", 0, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation.extension", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation.landmarkDescription", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation.clockFacePosition", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation.distanceFromLandmark", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation.distanceFromLandmark.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation.distanceFromLandmark.extension", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation.distanceFromLandmark.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation.distanceFromLandmark.device", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation.distanceFromLandmark.value", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.bodyLandmarkOrientation.surfaceOrientation", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.spatialReference", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.includedStructure.qualifier", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.excludedStructure", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.description", 0, Some(1)),
    rh_foundation::ElementCardinality::new("BodyStructure.image", 0, None),
    rh_foundation::ElementCardinality::new("BodyStructure.patient", 1, Some(1)),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for BodyStructure {
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

impl crate::traits::resource::ResourceMutators for BodyStructure {
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

impl crate::traits::resource::ResourceExistence for BodyStructure {
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

impl crate::traits::domain_resource::DomainResourceAccessors for BodyStructure {
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

impl crate::traits::domain_resource::DomainResourceMutators for BodyStructure {
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

impl crate::traits::domain_resource::DomainResourceExistence for BodyStructure {
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

impl crate::traits::body_structure::BodyStructureAccessors for BodyStructure {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn active(&self) -> Option<BooleanType> {
        self.active
    }
    fn morphology(&self) -> Option<CodeableConcept> {
        self.morphology.clone()
    }
    fn included_structure(&self) -> &[BodyStructureIncludedstructure] {
        &self.included_structure
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn image(&self) -> &[Attachment] {
        self.image.as_deref().unwrap_or(&[])
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
}

impl crate::traits::body_structure::BodyStructureMutators for BodyStructure {
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
    fn set_active(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.active = Some(value);
        resource
    }
    fn set_morphology(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.morphology = Some(value);
        resource
    }
    fn set_included_structure(self, value: Vec<BodyStructureIncludedstructure>) -> Self {
        let mut resource = self.clone();
        resource.included_structure = value;
        resource
    }
    fn add_included_structure(self, item: BodyStructureIncludedstructure) -> Self {
        let mut resource = self.clone();
        resource.included_structure.push(item);
        resource
    }
    fn set_excluded_structure(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.excluded_structure = Some(value);
        resource
    }
    fn add_excluded_structure(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .excluded_structure
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_image(self, value: Vec<Attachment>) -> Self {
        let mut resource = self.clone();
        resource.image = Some(value);
        resource
    }
    fn add_image(self, item: Attachment) -> Self {
        let mut resource = self.clone();
        resource.image.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
}

impl crate::traits::body_structure::BodyStructureExistence for BodyStructure {
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
    fn has_active(&self) -> bool {
        self.active.is_some()
    }
    fn has_morphology(&self) -> bool {
        self.morphology.is_some()
    }
    fn has_included_structure(&self) -> bool {
        !self.included_structure.is_empty()
    }
    fn has_excluded_structure(&self) -> bool {
        self.excluded_structure
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_image(&self) -> bool {
        self.image.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_patient(&self) -> bool {
        true
    }
}

impl crate::validation::ValidatableResource for BodyStructure {
    fn resource_type(&self) -> &'static str {
        "BodyStructure"
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
        Some("http://hl7.org/fhir/StructureDefinition/BodyStructure")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::body_structure::{
    BodyStructureAccessors, BodyStructureExistence, BodyStructureMutators,
};
