use crate::bindings::imagingselection2dgraphictype::Imagingselection2dgraphictype;
use crate::bindings::imagingselection3dgraphictype::Imagingselection3dgraphictype;
use crate::bindings::imagingselection_status::ImagingselectionStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::decimal::DecimalType;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ImagingSelection
///
/// A selection of DICOM SOP instances and/or frames within a single Study and Series. This might include additional specifics such as an image region, an Observation UID or a Segmentation Number, allowing linkage to an Observation Resource or transferring this information along with the ImagingStudy Resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImagingSelection
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ImagingSelection
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingSelection {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for Imaging Selection
    pub identifier: Option<Vec<Identifier>>,
    /// available | entered-in-error | unknown
    pub status: ImagingselectionStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Subject of the selected instances
    pub subject: Option<Reference>,
    /// Date / Time when this imaging selection was created
    pub issued: Option<InstantType>,
    /// Extension element for the 'issued' primitive field. Contains metadata and extensions.
    pub _issued: Option<Element>,
    /// Selector of the instances (human or machine)
    pub performer: Option<Vec<ImagingSelectionPerformer>>,
    /// Associated request
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Classifies the imaging selection
    ///
    /// Binding: example (Key Object Selection Document Title.)
    ///
    /// ValueSet: http://dicom.nema.org/medical/dicom/current/output/chtml/part16/sect_CID_7010.html
    pub category: Option<Vec<CodeableConcept>>,
    /// Imaging Selection purpose text or code
    ///
    /// Binding: example (Key Object Selection Document Title.)
    ///
    /// ValueSet: http://dicom.nema.org/medical/dicom/current/output/chtml/part16/sect_CID_7010.html
    pub code: CodeableConcept,
    /// DICOM Study Instance UID
    #[serde(rename = "studyUid")]
    pub study_uid: Option<StringType>,
    /// Extension element for the 'studyUid' primitive field. Contains metadata and extensions.
    #[serde(rename = "_studyUid")]
    pub _study_uid: Option<Element>,
    /// The imaging study from which the imaging selection is derived
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Reference>>,
    /// The network service providing retrieval for the images referenced in the imaging selection
    pub endpoint: Option<Vec<Reference>>,
    /// DICOM Series Instance UID
    #[serde(rename = "seriesUid")]
    pub series_uid: Option<StringType>,
    /// Extension element for the 'seriesUid' primitive field. Contains metadata and extensions.
    #[serde(rename = "_seriesUid")]
    pub _series_uid: Option<Element>,
    /// DICOM Series Number
    #[serde(rename = "seriesNumber")]
    pub series_number: Option<UnsignedIntType>,
    /// Extension element for the 'seriesNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_seriesNumber")]
    pub _series_number: Option<Element>,
    /// The Frame of Reference UID for the selected images
    #[serde(rename = "frameOfReferenceUid")]
    pub frame_of_reference_uid: Option<StringType>,
    /// Extension element for the 'frameOfReferenceUid' primitive field. Contains metadata and extensions.
    #[serde(rename = "_frameOfReferenceUid")]
    pub _frame_of_reference_uid: Option<Element>,
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
    /// Related resource that is the focus for the imaging selection
    pub focus: Option<Vec<Reference>>,
    /// The selected instances
    pub instance: Option<Vec<ImagingSelectionInstance>>,
}
/// ImagingSelection nested structure for the 'instance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingSelectionInstance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A specific 2D region in a DICOM image / frame
    #[serde(rename = "imageRegion2D")]
    pub image_region2_d: Option<Vec<ImagingSelectionInstanceImageregion2d>>,
    /// A specific 3D region in a DICOM frame of reference
    #[serde(rename = "imageRegion3D")]
    pub image_region3_d: Option<Vec<ImagingSelectionInstanceImageregion3d>>,
    /// DICOM SOP Instance UID
    pub uid: StringType,
    /// Extension element for the 'uid' primitive field. Contains metadata and extensions.
    pub _uid: Option<Element>,
    /// DICOM Instance Number
    pub number: Option<UnsignedIntType>,
    /// Extension element for the 'number' primitive field. Contains metadata and extensions.
    pub _number: Option<Element>,
    /// DICOM SOP Class UID
    ///
    /// Binding: extensible (DICOM SOP Classes.)
    ///
    /// ValueSet: http://dicom.nema.org/medical/dicom/current/output/chtml/part04/sect_B.5.html#table_B.5-1
    #[serde(rename = "sopClass")]
    pub sop_class: Option<Coding>,
    /// The selected subset of the SOP Instance
    pub subset: Option<Vec<StringType>>,
    /// Extension element for the 'subset' primitive field. Contains metadata and extensions.
    pub _subset: Option<Element>,
}
/// ImagingSelection nested structure for the 'performer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingSelectionPerformer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of performer
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
    /// Author (human or machine)
    pub actor: Option<Reference>,
}
/// ImagingSelectionInstance nested structure for the 'imageRegion2D' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingSelectionInstanceImageregion2d {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// point | polyline | interpolated | circle | ellipse
    #[serde(rename = "regionType")]
    pub region_type: Imagingselection2dgraphictype,
    /// Extension element for the 'regionType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_regionType")]
    pub _region_type: Option<Element>,
    /// Specifies the coordinates that define the image region
    pub coordinate: Vec<DecimalType>,
    /// Extension element for the 'coordinate' primitive field. Contains metadata and extensions.
    pub _coordinate: Option<Element>,
}
/// ImagingSelectionInstance nested structure for the 'imageRegion3D' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingSelectionInstanceImageregion3d {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// point | multipoint | polyline | polygon | ellipse | ellipsoid
    #[serde(rename = "regionType")]
    pub region_type: Imagingselection3dgraphictype,
    /// Extension element for the 'regionType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_regionType")]
    pub _region_type: Option<Element>,
    /// Specifies the coordinates that define the image region
    pub coordinate: Vec<DecimalType>,
    /// Extension element for the 'coordinate' primitive field. Contains metadata and extensions.
    pub _coordinate: Option<Element>,
}

impl Default for ImagingSelection {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: ImagingselectionStatus::default(),
            _status: Default::default(),
            subject: Default::default(),
            issued: Default::default(),
            _issued: Default::default(),
            performer: Default::default(),
            based_on: Default::default(),
            category: Default::default(),
            code: CodeableConcept::default(),
            study_uid: Default::default(),
            _study_uid: Default::default(),
            derived_from: Default::default(),
            endpoint: Default::default(),
            series_uid: Default::default(),
            _series_uid: Default::default(),
            series_number: Default::default(),
            _series_number: Default::default(),
            frame_of_reference_uid: Default::default(),
            _frame_of_reference_uid: Default::default(),
            body_site: Default::default(),
            focus: Default::default(),
            instance: Default::default(),
        }
    }
}

impl Default for ImagingSelectionInstance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            image_region2_d: Default::default(),
            image_region3_d: Default::default(),
            uid: StringType::default(),
            _uid: Default::default(),
            number: Default::default(),
            _number: Default::default(),
            sop_class: Default::default(),
            subset: Default::default(),
            _subset: Default::default(),
        }
    }
}

impl Default for ImagingSelectionPerformer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: Default::default(),
        }
    }
}

impl Default for ImagingSelectionInstanceImageregion2d {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            region_type: Default::default(),
            _region_type: Default::default(),
            coordinate: Default::default(),
            _coordinate: Default::default(),
        }
    }
}

impl Default for ImagingSelectionInstanceImageregion3d {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            region_type: Default::default(),
            _region_type: Default::default(),
            coordinate: Default::default(),
            _coordinate: Default::default(),
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
                "ImagingSelection.instance.imageRegion2D.regionType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/imagingselection-2dgraphictype|5.0.0",
            )
            .with_description("The type of image region."),
            rh_foundation::ElementBinding::new(
                "ImagingSelection.instance.imageRegion3D.regionType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/imagingselection-3dgraphictype|5.0.0",
            )
            .with_description("The type of image region."),
            rh_foundation::ElementBinding::new(
                "ImagingSelection.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ImagingSelection.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/imagingselection-status|5.0.0",
            )
            .with_description("The status of the ImagingSelection."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ImagingSelection.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.contained", 0, None),
            rh_foundation::ElementCardinality::new("ImagingSelection.extension", 0, None),
            rh_foundation::ElementCardinality::new("ImagingSelection.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ImagingSelection.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ImagingSelection.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.issued", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.performer", 0, None),
            rh_foundation::ElementCardinality::new("ImagingSelection.performer.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.performer.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.performer.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.performer.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ImagingSelection.performer.actor", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("ImagingSelection.category", 0, None),
            rh_foundation::ElementCardinality::new("ImagingSelection.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.studyUid", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.derivedFrom", 0, None),
            rh_foundation::ElementCardinality::new("ImagingSelection.endpoint", 0, None),
            rh_foundation::ElementCardinality::new("ImagingSelection.seriesUid", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.seriesNumber", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.frameOfReferenceUid",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ImagingSelection.bodySite", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.focus", 0, None),
            rh_foundation::ElementCardinality::new("ImagingSelection.instance", 0, None),
            rh_foundation::ElementCardinality::new("ImagingSelection.instance.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.instance.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ImagingSelection.instance.uid", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ImagingSelection.instance.number", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.sopClass",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ImagingSelection.instance.subset", 0, None),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.imageRegion2D",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.imageRegion2D.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.imageRegion2D.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.imageRegion2D.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.imageRegion2D.regionType",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.imageRegion2D.coordinate",
                1,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.imageRegion3D",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.imageRegion3D.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.imageRegion3D.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.imageRegion3D.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.imageRegion3D.regionType",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImagingSelection.instance.imageRegion3D.coordinate",
                1,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ImagingSelection {
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

impl crate::traits::resource::ResourceMutators for ImagingSelection {
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

impl crate::traits::resource::ResourceExistence for ImagingSelection {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ImagingSelection {
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

impl crate::traits::domain_resource::DomainResourceMutators for ImagingSelection {
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

impl crate::traits::domain_resource::DomainResourceExistence for ImagingSelection {
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

impl crate::traits::imaging_selection::ImagingSelectionAccessors for ImagingSelection {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ImagingselectionStatus {
        self.status.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn issued(&self) -> Option<InstantType> {
        self.issued.clone()
    }
    fn performer(&self) -> &[ImagingSelectionPerformer] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> CodeableConcept {
        self.code.clone()
    }
    fn study_uid(&self) -> Option<StringType> {
        self.study_uid.clone()
    }
    fn derived_from(&self) -> &[Reference] {
        self.derived_from.as_deref().unwrap_or(&[])
    }
    fn endpoint(&self) -> &[Reference] {
        self.endpoint.as_deref().unwrap_or(&[])
    }
    fn series_uid(&self) -> Option<StringType> {
        self.series_uid.clone()
    }
    fn series_number(&self) -> Option<UnsignedIntType> {
        self.series_number
    }
    fn frame_of_reference_uid(&self) -> Option<StringType> {
        self.frame_of_reference_uid.clone()
    }
    fn body_site(&self) -> Option<CodeableReference> {
        self.body_site.clone()
    }
    fn focus(&self) -> &[Reference] {
        self.focus.as_deref().unwrap_or(&[])
    }
    fn instance(&self) -> &[ImagingSelectionInstance] {
        self.instance.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::imaging_selection::ImagingSelectionMutators for ImagingSelection {
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
    fn set_status(self, value: ImagingselectionStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_issued(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.issued = Some(value);
        resource
    }
    fn set_performer(self, value: Vec<ImagingSelectionPerformer>) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: ImagingSelectionPerformer) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
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
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = value;
        resource
    }
    fn set_study_uid(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.study_uid = Some(value);
        resource
    }
    fn set_derived_from(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.derived_from = Some(value);
        resource
    }
    fn add_derived_from(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .derived_from
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_series_uid(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.series_uid = Some(value);
        resource
    }
    fn set_series_number(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.series_number = Some(value);
        resource
    }
    fn set_frame_of_reference_uid(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.frame_of_reference_uid = Some(value);
        resource
    }
    fn set_body_site(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.body_site = Some(value);
        resource
    }
    fn set_focus(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.focus = Some(value);
        resource
    }
    fn add_focus(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.focus.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_instance(self, value: Vec<ImagingSelectionInstance>) -> Self {
        let mut resource = self.clone();
        resource.instance = Some(value);
        resource
    }
    fn add_instance(self, item: ImagingSelectionInstance) -> Self {
        let mut resource = self.clone();
        resource.instance.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::imaging_selection::ImagingSelectionExistence for ImagingSelection {
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
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_issued(&self) -> bool {
        self.issued.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        true
    }
    fn has_study_uid(&self) -> bool {
        self.study_uid.is_some()
    }
    fn has_derived_from(&self) -> bool {
        self.derived_from.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_endpoint(&self) -> bool {
        self.endpoint.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_series_uid(&self) -> bool {
        self.series_uid.is_some()
    }
    fn has_series_number(&self) -> bool {
        self.series_number.is_some()
    }
    fn has_frame_of_reference_uid(&self) -> bool {
        self.frame_of_reference_uid.is_some()
    }
    fn has_body_site(&self) -> bool {
        self.body_site.is_some()
    }
    fn has_focus(&self) -> bool {
        self.focus.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_instance(&self) -> bool {
        self.instance.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for ImagingSelection {
    fn resource_type(&self) -> &'static str {
        "ImagingSelection"
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
        Some("http://hl7.org/fhir/StructureDefinition/ImagingSelection")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::imaging_selection::{
    ImagingSelectionAccessors, ImagingSelectionExistence, ImagingSelectionMutators,
};
