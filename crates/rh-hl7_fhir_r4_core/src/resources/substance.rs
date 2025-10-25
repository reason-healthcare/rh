use crate::bindings::substance_status::SubstanceStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Substance
///
/// A homogeneous material with a definite composition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Substance
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Substance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Substance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique identifier
    pub identifier: Option<Vec<Identifier>>,
    /// active | inactive | entered-in-error
    pub status: Option<SubstanceStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// What class/type of substance this is
    ///
    /// Binding: extensible (Category or classification of substance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-category
    pub category: Option<Vec<CodeableConcept>>,
    /// What substance this is
    ///
    /// Binding: example (Substance codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-code
    pub code: CodeableConcept,
    /// Textual description of the substance, comments
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// If this describes a specific package/container of the substance
    pub instance: Option<Vec<SubstanceInstance>>,
    /// Composition information about the substance
    pub ingredient: Option<Vec<SubstanceIngredient>>,
}
/// Substance nested structure for the 'ingredient' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceIngredient {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Optional amount (concentration)
    pub quantity: Option<Ratio>,
    /// A component of the substance (CodeableConcept)
    #[serde(rename = "substanceCodeableConcept")]
    pub substance_codeable_concept: CodeableConcept,
    /// A component of the substance (Reference)
    #[serde(rename = "substanceReference")]
    pub substance_reference: Reference,
}
/// Substance nested structure for the 'instance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceInstance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identifier of the package/container
    pub identifier: Option<Identifier>,
    /// When no longer valid to use
    pub expiry: Option<DateTimeType>,
    /// Extension element for the 'expiry' primitive field. Contains metadata and extensions.
    pub _expiry: Option<Element>,
    /// Amount of substance in the package
    pub quantity: Option<Quantity>,
}

impl Default for Substance {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            category: Default::default(),
            code: CodeableConcept::default(),
            description: Default::default(),
            _description: Default::default(),
            instance: Default::default(),
            ingredient: Default::default(),
        }
    }
}

impl Default for SubstanceIngredient {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            quantity: Default::default(),
            substance_codeable_concept: Default::default(),
            substance_reference: Default::default(),
        }
    }
}

impl Default for SubstanceInstance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            expiry: Default::default(),
            _expiry: Default::default(),
            quantity: Default::default(),
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
            "Substance.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/substance-status|4.0.1",
        )
        .with_description("A code to indicate if the substance is actively used.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Substance.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.contained", 0, None),
            rh_foundation::ElementCardinality::new("Substance.extension", 0, None),
            rh_foundation::ElementCardinality::new("Substance.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Substance.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Substance.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.category", 0, None),
            rh_foundation::ElementCardinality::new("Substance.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.instance", 0, None),
            rh_foundation::ElementCardinality::new("Substance.instance.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.instance.extension", 0, None),
            rh_foundation::ElementCardinality::new("Substance.instance.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Substance.instance.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.instance.expiry", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.instance.quantity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.ingredient", 0, None),
            rh_foundation::ElementCardinality::new("Substance.ingredient.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.ingredient.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Substance.ingredient.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Substance.ingredient.quantity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Substance.ingredient.substance[x]", 1, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Substance {
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

impl crate::traits::resource::ResourceMutators for Substance {
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

impl crate::traits::resource::ResourceExistence for Substance {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Substance {
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

impl crate::traits::domain_resource::DomainResourceMutators for Substance {
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

impl crate::traits::domain_resource::DomainResourceExistence for Substance {
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

impl crate::traits::substance::SubstanceAccessors for Substance {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Option<SubstanceStatus> {
        self.status.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> CodeableConcept {
        self.code.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn instance(&self) -> &[SubstanceInstance] {
        self.instance.as_deref().unwrap_or(&[])
    }
    fn ingredient(&self) -> &[SubstanceIngredient] {
        self.ingredient.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::substance::SubstanceMutators for Substance {
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
    fn set_status(self, value: SubstanceStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
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
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_instance(self, value: Vec<SubstanceInstance>) -> Self {
        let mut resource = self.clone();
        resource.instance = Some(value);
        resource
    }
    fn add_instance(self, item: SubstanceInstance) -> Self {
        let mut resource = self.clone();
        resource.instance.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_ingredient(self, value: Vec<SubstanceIngredient>) -> Self {
        let mut resource = self.clone();
        resource.ingredient = Some(value);
        resource
    }
    fn add_ingredient(self, item: SubstanceIngredient) -> Self {
        let mut resource = self.clone();
        resource.ingredient.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::substance::SubstanceExistence for Substance {
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
        self.status.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        true
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_instance(&self) -> bool {
        self.instance.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_ingredient(&self) -> bool {
        self.ingredient.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Substance {
    fn resource_type(&self) -> &'static str {
        "Substance"
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
        Some("http://hl7.org/fhir/StructureDefinition/Substance")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::substance::{SubstanceAccessors, SubstanceExistence, SubstanceMutators};
