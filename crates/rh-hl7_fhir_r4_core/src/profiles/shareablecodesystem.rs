use crate::resources::code_system::CodeSystem;
use serde::{Deserialize, Serialize};
/// Shareable CodeSystem
///
/// Enforces the minimum information set for the value set metadata required by HL7 and other organizations that share and publish value sets
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareablecodesystem
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CodeSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/CodeSystem
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Shareablecodesystem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: CodeSystem,
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("csd-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
    rh_foundation::Invariant::new("csd-1", rh_foundation::Severity::Error, "Within a code system definition, all the codes SHALL be unique", "concept.code.combine($this.descendants().concept.code).isDistinct()").with_xpath("count(distinct-values(descendant::f:concept/f:code/@value))=count(descendant::f:concept)"),
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
        vec![
    rh_foundation::ElementBinding::new("CodeSystem.content", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/codesystem-content-mode|4.0.1").with_description("The extent of the content of the code system (the concepts and codes it defines) are represented in a code system resource."),
    rh_foundation::ElementBinding::new("CodeSystem.filter.operator", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/filter-operator|4.0.1").with_description("The kind of operation to perform as a part of a property based filter."),
    rh_foundation::ElementBinding::new("CodeSystem.hierarchyMeaning", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/codesystem-hierarchy-meaning|4.0.1").with_description("The meaning of the hierarchy of concepts in a code system."),
    rh_foundation::ElementBinding::new("CodeSystem.property.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/concept-property-type|4.0.1").with_description("The type of a property value."),
    rh_foundation::ElementBinding::new("CodeSystem.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|4.0.1").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("CodeSystem", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.contained", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.extension", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.identifier", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.version", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.experimental", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.publisher", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.contact", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.description", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.useContext", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.caseSensitive", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.valueSet", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.hierarchyMeaning", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.compositional", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.versionNeeded", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.content", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.supplements", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.count", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.filter", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.extension", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.operator", 1, None),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.value", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.property", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.property.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.property.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CodeSystem.property.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.property.uri", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.property.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.property.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.concept", 1, None),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.extension", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.definition", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.designation", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.designation.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.designation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.designation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.designation.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.designation.use",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.designation.value",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.property", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.property.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.property.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.property.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.property.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.concept", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Shareablecodesystem {
    fn id(&self) -> Option<String> {
        self.base.id()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.meta()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.implicit_rules()
    }
    fn language(&self) -> Option<String> {
        self.base.language()
    }
}

impl crate::traits::resource::ResourceMutators for Shareablecodesystem {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_id(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_meta(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_implicit_rules(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_language(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for Shareablecodesystem {
    fn has_id(&self) -> bool {
        self.base.has_id()
    }
    fn has_meta(&self) -> bool {
        self.base.has_meta()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.has_implicit_rules()
    }
    fn has_language(&self) -> bool {
        self.base.has_language()
    }
}

impl crate::traits::shareablecodesystem::ShareablecodesystemMutators for Shareablecodesystem {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Shareablecodesystem {
    fn resource_type(&self) -> &'static str {
        "CodeSystem"
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
        Some("http://hl7.org/fhir/StructureDefinition/shareablecodesystem")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::shareablecodesystem::{
    ShareablecodesystemAccessors, ShareablecodesystemExistence, ShareablecodesystemMutators,
};
