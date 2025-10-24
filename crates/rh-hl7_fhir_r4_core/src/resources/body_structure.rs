use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
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
/// - Version: 4.0.1
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
    /// Body site
    ///
    /// Binding: example (Codes describing anatomical locations. May include laterality.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/body-site
    pub location: Option<CodeableConcept>,
    /// Body site modifier
    ///
    /// Binding: example (Concepts modifying the anatomic location.)
    ///
    /// Available values:
    /// - `419161000`: Unilateral left
    /// - `419465000`: Unilateral right
    /// - `51440002`: Bilateral
    /// - `261183002`: Upper
    /// - `261122009`: Lower
    /// - `255561001`: Medial
    /// - `49370004`: Lateral
    /// - `264217000`: Superior
    /// - `261089000`: Inferior
    /// - `255551008`: Posterior
    /// - ... and 2 more values
    #[serde(rename = "locationQualifier")]
    pub location_qualifier: Option<Vec<CodeableConcept>>,
    /// Text description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Attached images
    pub image: Option<Vec<Attachment>>,
    /// Who this is about
    pub patient: Reference,
}

impl Default for BodyStructure {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            active: Default::default(),
            _active: Default::default(),
            morphology: Default::default(),
            location: Default::default(),
            location_qualifier: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            image: Default::default(),
            patient: Reference::default(),
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
    fn location(&self) -> Option<CodeableConcept> {
        self.location.clone()
    }
    fn location_qualifier(&self) -> &[CodeableConcept] {
        self.location_qualifier.as_deref().unwrap_or(&[])
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
    fn set_location(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_location_qualifier(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.location_qualifier = Some(value);
        resource
    }
    fn add_location_qualifier(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .location_qualifier
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
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_location_qualifier(&self) -> bool {
        self.location_qualifier
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

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/BodyStructure")
    }
}
