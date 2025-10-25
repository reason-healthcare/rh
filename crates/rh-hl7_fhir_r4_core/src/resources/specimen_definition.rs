use crate::bindings::specimen_contained_preference::SpecimenContainedPreference;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SpecimenDefinition
///
/// A kind of specimen with associated set of requirements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SpecimenDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SpecimenDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier of a kind of specimen
    pub identifier: Option<Identifier>,
    /// Kind of material to collect
    ///
    /// Binding: example (The type of the specimen to be collected.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0487
    #[serde(rename = "typeCollected")]
    pub type_collected: Option<CodeableConcept>,
    /// Patient preparation for collection
    ///
    /// Binding: example (Checks on the patient prior specimen collection.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/prepare-patient-prior-specimen-collection
    #[serde(rename = "patientPreparation")]
    pub patient_preparation: Option<Vec<CodeableConcept>>,
    /// Time aspect for collection
    #[serde(rename = "timeAspect")]
    pub time_aspect: Option<StringType>,
    /// Extension element for the 'timeAspect' primitive field. Contains metadata and extensions.
    #[serde(rename = "_timeAspect")]
    pub _time_aspect: Option<Element>,
    /// Specimen collection procedure
    ///
    /// Binding: example (The action to collect a type of specimen.)
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
    pub collection: Option<Vec<CodeableConcept>>,
    /// Specimen in container intended for testing by lab
    #[serde(rename = "typeTested")]
    pub type_tested: Option<Vec<SpecimenDefinitionTypetested>>,
}
/// SpecimenDefinitionTypetestedContainer nested structure for the 'additive' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenDefinitionTypetestedContainerAdditive {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Additive associated with container (CodeableConcept)
    #[serde(rename = "additiveCodeableConcept")]
    pub additive_codeable_concept: CodeableConcept,
    /// Additive associated with container (Reference)
    #[serde(rename = "additiveReference")]
    pub additive_reference: Reference,
}
/// SpecimenDefinitionTypetested nested structure for the 'handling' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenDefinitionTypetestedHandling {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Temperature qualifier
    ///
    /// Binding: example (Set of handling instructions prior testing of the specimen.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/handling-condition
    #[serde(rename = "temperatureQualifier")]
    pub temperature_qualifier: Option<CodeableConcept>,
    /// Temperature range
    #[serde(rename = "temperatureRange")]
    pub temperature_range: Option<Range>,
    /// Maximum preservation time
    #[serde(rename = "maxDuration")]
    pub max_duration: Option<Duration>,
    /// Preservation instruction
    pub instruction: Option<StringType>,
    /// Extension element for the 'instruction' primitive field. Contains metadata and extensions.
    pub _instruction: Option<Element>,
}
/// SpecimenDefinition nested structure for the 'typeTested' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenDefinitionTypetested {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Specimen handling before testing
    pub handling: Option<Vec<SpecimenDefinitionTypetestedHandling>>,
    /// The specimen's container
    pub container: Option<SpecimenDefinitionTypetestedContainer>,
    /// Primary or secondary specimen
    #[serde(rename = "isDerived")]
    pub is_derived: Option<BooleanType>,
    /// Extension element for the 'isDerived' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isDerived")]
    pub _is_derived: Option<Element>,
    /// Type of intended specimen
    ///
    /// Binding: example (The type of specimen conditioned in a container for lab testing.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0487
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// preferred | alternate
    pub preference: SpecimenContainedPreference,
    /// Extension element for the 'preference' primitive field. Contains metadata and extensions.
    pub _preference: Option<Element>,
    /// Specimen requirements
    pub requirement: Option<StringType>,
    /// Extension element for the 'requirement' primitive field. Contains metadata and extensions.
    pub _requirement: Option<Element>,
    /// Specimen retention time
    #[serde(rename = "retentionTime")]
    pub retention_time: Option<Duration>,
    /// Rejection criterion
    ///
    /// Binding: example (Criterion for rejection of the specimen by laboratory.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/rejection-criteria
    #[serde(rename = "rejectionCriterion")]
    pub rejection_criterion: Option<Vec<CodeableConcept>>,
}
/// SpecimenDefinitionTypetested nested structure for the 'container' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenDefinitionTypetestedContainer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Container material
    ///
    /// Binding: example (Types of material for specimen containers.)
    ///
    /// Available values:
    /// - `32039001`: glass
    /// - `61088005`: plastic
    /// - `425620007`: metal
    pub material: Option<CodeableConcept>,
    /// Kind of container associated with the kind of specimen
    ///
    /// Binding: example (Type of specimen container.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/specimen-container-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Color of container cap
    ///
    /// Binding: example (Color of the container cap.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/container-cap
    pub cap: Option<CodeableConcept>,
    /// Container description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Container capacity
    pub capacity: Option<Quantity>,
    /// Minimum volume (Quantity)
    #[serde(rename = "minimumVolumeQuantity")]
    pub minimum_volume_quantity: Option<Quantity>,
    /// Minimum volume (string)
    #[serde(rename = "minimumVolumeString")]
    pub minimum_volume_string: Option<StringType>,
    /// Specimen container preparation
    pub preparation: Option<StringType>,
    /// Extension element for the 'preparation' primitive field. Contains metadata and extensions.
    pub _preparation: Option<Element>,
}

impl Default for SpecimenDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            type_collected: Default::default(),
            patient_preparation: Default::default(),
            time_aspect: Default::default(),
            _time_aspect: Default::default(),
            collection: Default::default(),
            type_tested: Default::default(),
        }
    }
}

impl Default for SpecimenDefinitionTypetestedContainerAdditive {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            additive_codeable_concept: Default::default(),
            additive_reference: Default::default(),
        }
    }
}

impl Default for SpecimenDefinitionTypetestedHandling {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            temperature_qualifier: Default::default(),
            temperature_range: Default::default(),
            max_duration: Default::default(),
            instruction: Default::default(),
            _instruction: Default::default(),
        }
    }
}

impl Default for SpecimenDefinitionTypetested {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            handling: Default::default(),
            container: Default::default(),
            is_derived: Default::default(),
            _is_derived: Default::default(),
            type_: Default::default(),
            preference: Default::default(),
            _preference: Default::default(),
            requirement: Default::default(),
            _requirement: Default::default(),
            retention_time: Default::default(),
            rejection_criterion: Default::default(),
        }
    }
}

impl Default for SpecimenDefinitionTypetestedContainer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            material: Default::default(),
            type_: Default::default(),
            cap: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            capacity: Default::default(),
            minimum_volume_quantity: Default::default(),
            minimum_volume_string: Default::default(),
            preparation: Default::default(),
            _preparation: Default::default(),
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
            "SpecimenDefinition.typeTested.preference",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/specimen-contained-preference|4.0.1",
        )
        .with_description("Degree of preference of a type of conditioned specimen.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("SpecimenDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.typeCollected", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.patientPreparation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.timeAspect", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.collection", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.typeTested", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.typeTested.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.isDerived",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.preference",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.material",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.cap",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.capacity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.minimumVolume[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.additive",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.additive.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.additive.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.additive.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.additive.additive[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.preparation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.requirement",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.retentionTime",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.rejectionCriterion",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.temperatureQualifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.temperatureRange",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.maxDuration",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.instruction",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SpecimenDefinition {
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

impl crate::traits::resource::ResourceMutators for SpecimenDefinition {
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

impl crate::traits::resource::ResourceExistence for SpecimenDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SpecimenDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for SpecimenDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for SpecimenDefinition {
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

impl crate::traits::specimen_definition::SpecimenDefinitionAccessors for SpecimenDefinition {
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn type_collected(&self) -> Option<CodeableConcept> {
        self.type_collected.clone()
    }
    fn patient_preparation(&self) -> &[CodeableConcept] {
        self.patient_preparation.as_deref().unwrap_or(&[])
    }
    fn time_aspect(&self) -> Option<StringType> {
        self.time_aspect.clone()
    }
    fn collection(&self) -> &[CodeableConcept] {
        self.collection.as_deref().unwrap_or(&[])
    }
    fn type_tested(&self) -> &[SpecimenDefinitionTypetested] {
        self.type_tested.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::specimen_definition::SpecimenDefinitionMutators for SpecimenDefinition {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_type_collected(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_collected = Some(value);
        resource
    }
    fn set_patient_preparation(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.patient_preparation = Some(value);
        resource
    }
    fn add_patient_preparation(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .patient_preparation
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_time_aspect(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.time_aspect = Some(value);
        resource
    }
    fn set_collection(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.collection = Some(value);
        resource
    }
    fn add_collection(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.collection.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_type_tested(self, value: Vec<SpecimenDefinitionTypetested>) -> Self {
        let mut resource = self.clone();
        resource.type_tested = Some(value);
        resource
    }
    fn add_type_tested(self, item: SpecimenDefinitionTypetested) -> Self {
        let mut resource = self.clone();
        resource.type_tested.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::specimen_definition::SpecimenDefinitionExistence for SpecimenDefinition {
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
        self.identifier.is_some()
    }
    fn has_type_collected(&self) -> bool {
        self.type_collected.is_some()
    }
    fn has_patient_preparation(&self) -> bool {
        self.patient_preparation
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_time_aspect(&self) -> bool {
        self.time_aspect.is_some()
    }
    fn has_collection(&self) -> bool {
        self.collection.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_type_tested(&self) -> bool {
        self.type_tested.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for SpecimenDefinition {
    fn resource_type(&self) -> &'static str {
        "SpecimenDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/SpecimenDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::specimen_definition::{
    SpecimenDefinitionAccessors, SpecimenDefinitionExistence, SpecimenDefinitionMutators,
};
