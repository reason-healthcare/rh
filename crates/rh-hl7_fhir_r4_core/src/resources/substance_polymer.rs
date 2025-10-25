use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::substance_amount::SubstanceAmount;
use crate::primitives::boolean::BooleanType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SubstancePolymer
///
/// Todo.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstancePolymer
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstancePolymer
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Todo
    pub class: Option<CodeableConcept>,
    /// Todo
    pub geometry: Option<CodeableConcept>,
    /// Todo
    #[serde(rename = "copolymerConnectivity")]
    pub copolymer_connectivity: Option<Vec<CodeableConcept>>,
    /// Todo
    pub modification: Option<Vec<StringType>>,
    /// Extension element for the 'modification' primitive field. Contains metadata and extensions.
    pub _modification: Option<Element>,
    /// Todo
    #[serde(rename = "monomerSet")]
    pub monomer_set: Option<Vec<SubstancePolymerMonomerset>>,
    /// Todo
    pub repeat: Option<Vec<SubstancePolymerRepeat>>,
}
/// SubstancePolymer nested structure for the 'repeat' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymerRepeat {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    #[serde(rename = "repeatUnit")]
    pub repeat_unit: Option<Vec<SubstancePolymerRepeatRepeatunit>>,
    /// Todo
    #[serde(rename = "numberOfUnits")]
    pub number_of_units: Option<IntegerType>,
    /// Extension element for the 'numberOfUnits' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfUnits")]
    pub _number_of_units: Option<Element>,
    /// Todo
    #[serde(rename = "averageMolecularFormula")]
    pub average_molecular_formula: Option<StringType>,
    /// Extension element for the 'averageMolecularFormula' primitive field. Contains metadata and extensions.
    #[serde(rename = "_averageMolecularFormula")]
    pub _average_molecular_formula: Option<Element>,
    /// Todo
    #[serde(rename = "repeatUnitAmountType")]
    pub repeat_unit_amount_type: Option<CodeableConcept>,
}
/// SubstancePolymerMonomerset nested structure for the 'startingMaterial' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymerMonomersetStartingmaterial {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    pub material: Option<CodeableConcept>,
    /// Todo
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Todo
    #[serde(rename = "isDefining")]
    pub is_defining: Option<BooleanType>,
    /// Extension element for the 'isDefining' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isDefining")]
    pub _is_defining: Option<Element>,
    /// Todo
    pub amount: Option<SubstanceAmount>,
}
/// SubstancePolymerRepeat nested structure for the 'repeatUnit' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymerRepeatRepeatunit {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    #[serde(rename = "orientationOfPolymerisation")]
    pub orientation_of_polymerisation: Option<CodeableConcept>,
    /// Todo
    #[serde(rename = "repeatUnit")]
    pub repeat_unit: Option<StringType>,
    /// Extension element for the 'repeatUnit' primitive field. Contains metadata and extensions.
    #[serde(rename = "_repeatUnit")]
    pub _repeat_unit: Option<Element>,
    /// Todo
    pub amount: Option<SubstanceAmount>,
}
/// SubstancePolymer nested structure for the 'monomerSet' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymerMonomerset {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    #[serde(rename = "startingMaterial")]
    pub starting_material: Option<Vec<SubstancePolymerMonomersetStartingmaterial>>,
    /// Todo
    #[serde(rename = "ratioType")]
    pub ratio_type: Option<CodeableConcept>,
}
/// SubstancePolymerRepeatRepeatunit nested structure for the 'structuralRepresentation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymerRepeatRepeatunitStructuralrepresentation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Todo
    pub representation: Option<StringType>,
    /// Extension element for the 'representation' primitive field. Contains metadata and extensions.
    pub _representation: Option<Element>,
    /// Todo
    pub attachment: Option<Attachment>,
}
/// SubstancePolymerRepeatRepeatunit nested structure for the 'degreeOfPolymerisation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymerRepeatRepeatunitDegreeofpolymerisation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    pub degree: Option<CodeableConcept>,
    /// Todo
    pub amount: Option<SubstanceAmount>,
}

impl Default for SubstancePolymer {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            class: Default::default(),
            geometry: Default::default(),
            copolymer_connectivity: Default::default(),
            modification: Default::default(),
            _modification: Default::default(),
            monomer_set: Default::default(),
            repeat: Default::default(),
        }
    }
}

impl Default for SubstancePolymerRepeat {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            repeat_unit: Default::default(),
            number_of_units: Default::default(),
            _number_of_units: Default::default(),
            average_molecular_formula: Default::default(),
            _average_molecular_formula: Default::default(),
            repeat_unit_amount_type: Default::default(),
        }
    }
}

impl Default for SubstancePolymerMonomersetStartingmaterial {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            material: Default::default(),
            type_: Default::default(),
            is_defining: Default::default(),
            _is_defining: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for SubstancePolymerRepeatRepeatunit {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            orientation_of_polymerisation: Default::default(),
            repeat_unit: Default::default(),
            _repeat_unit: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for SubstancePolymerMonomerset {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            starting_material: Default::default(),
            ratio_type: Default::default(),
        }
    }
}

impl Default for SubstancePolymerRepeatRepeatunitStructuralrepresentation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            representation: Default::default(),
            _representation: Default::default(),
            attachment: Default::default(),
        }
    }
}

impl Default for SubstancePolymerRepeatRepeatunitDegreeofpolymerisation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            degree: Default::default(),
            amount: Default::default(),
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

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("SubstancePolymer.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstancePolymer.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstancePolymer.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstancePolymer.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstancePolymer.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstancePolymer.contained", 0, None),
            rh_foundation::ElementCardinality::new("SubstancePolymer.extension", 0, None),
            rh_foundation::ElementCardinality::new("SubstancePolymer.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("SubstancePolymer.class", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstancePolymer.geometry", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.copolymerConnectivity",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubstancePolymer.modification", 0, None),
            rh_foundation::ElementCardinality::new("SubstancePolymer.monomerSet", 0, None),
            rh_foundation::ElementCardinality::new("SubstancePolymer.monomerSet.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.monomerSet.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.monomerSet.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.monomerSet.ratioType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.monomerSet.startingMaterial",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.monomerSet.startingMaterial.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.monomerSet.startingMaterial.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.monomerSet.startingMaterial.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.monomerSet.startingMaterial.material",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.monomerSet.startingMaterial.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.monomerSet.startingMaterial.isDefining",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.monomerSet.startingMaterial.amount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubstancePolymer.repeat", 0, None),
            rh_foundation::ElementCardinality::new("SubstancePolymer.repeat.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstancePolymer.repeat.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.numberOfUnits",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.averageMolecularFormula",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnitAmountType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubstancePolymer.repeat.repeatUnit", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.orientationOfPolymerisation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.repeatUnit",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.amount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.degreeOfPolymerisation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.degreeOfPolymerisation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.degreeOfPolymerisation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.degreeOfPolymerisation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.degreeOfPolymerisation.degree",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.degreeOfPolymerisation.amount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.structuralRepresentation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.structuralRepresentation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.structuralRepresentation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.structuralRepresentation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.structuralRepresentation.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.structuralRepresentation.representation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstancePolymer.repeat.repeatUnit.structuralRepresentation.attachment",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SubstancePolymer {
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

impl crate::traits::resource::ResourceMutators for SubstancePolymer {
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

impl crate::traits::resource::ResourceExistence for SubstancePolymer {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SubstancePolymer {
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

impl crate::traits::domain_resource::DomainResourceMutators for SubstancePolymer {
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

impl crate::traits::domain_resource::DomainResourceExistence for SubstancePolymer {
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

impl crate::traits::substance_polymer::SubstancePolymerAccessors for SubstancePolymer {
    fn class(&self) -> Option<CodeableConcept> {
        self.class.clone()
    }
    fn geometry(&self) -> Option<CodeableConcept> {
        self.geometry.clone()
    }
    fn copolymer_connectivity(&self) -> &[CodeableConcept] {
        self.copolymer_connectivity.as_deref().unwrap_or(&[])
    }
    fn modification(&self) -> &[StringType] {
        self.modification.as_deref().unwrap_or(&[])
    }
    fn monomer_set(&self) -> &[SubstancePolymerMonomerset] {
        self.monomer_set.as_deref().unwrap_or(&[])
    }
    fn repeat(&self) -> &[SubstancePolymerRepeat] {
        self.repeat.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::substance_polymer::SubstancePolymerMutators for SubstancePolymer {
    fn new() -> Self {
        Self::default()
    }
    fn set_class(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.class = Some(value);
        resource
    }
    fn set_geometry(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.geometry = Some(value);
        resource
    }
    fn set_copolymer_connectivity(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.copolymer_connectivity = Some(value);
        resource
    }
    fn add_copolymer_connectivity(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .copolymer_connectivity
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modification(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.modification = Some(value);
        resource
    }
    fn add_modification(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .modification
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_monomer_set(self, value: Vec<SubstancePolymerMonomerset>) -> Self {
        let mut resource = self.clone();
        resource.monomer_set = Some(value);
        resource
    }
    fn add_monomer_set(self, item: SubstancePolymerMonomerset) -> Self {
        let mut resource = self.clone();
        resource.monomer_set.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_repeat(self, value: Vec<SubstancePolymerRepeat>) -> Self {
        let mut resource = self.clone();
        resource.repeat = Some(value);
        resource
    }
    fn add_repeat(self, item: SubstancePolymerRepeat) -> Self {
        let mut resource = self.clone();
        resource.repeat.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::substance_polymer::SubstancePolymerExistence for SubstancePolymer {
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
    fn has_class(&self) -> bool {
        self.class.is_some()
    }
    fn has_geometry(&self) -> bool {
        self.geometry.is_some()
    }
    fn has_copolymer_connectivity(&self) -> bool {
        self.copolymer_connectivity
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_modification(&self) -> bool {
        self.modification.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_monomer_set(&self) -> bool {
        self.monomer_set.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_repeat(&self) -> bool {
        self.repeat.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for SubstancePolymer {
    fn resource_type(&self) -> &'static str {
        "SubstancePolymer"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/SubstancePolymer")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::substance_polymer::{
    SubstancePolymerAccessors, SubstancePolymerExistence, SubstancePolymerMutators,
};
