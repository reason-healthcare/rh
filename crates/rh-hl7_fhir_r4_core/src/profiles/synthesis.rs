use crate::resources::evidence::Evidence;
use serde::{Deserialize, Serialize};
/// Evidence Synthesis Profile
///
/// Explanation of what this profile contains/is for.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/synthesis
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Evidence
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Evidence
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Synthesis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Evidence,
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
    rh_foundation::Invariant::new("evi-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
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
            "Evidence.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/publication-status|4.0.1",
        )
        .with_description("The lifecycle status of an artifact.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Evidence", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.contained", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.extension", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.shortTitle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.subtitle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.contact", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.note", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.useContext", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.topic", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.author", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.editor", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.endorser", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.exposureBackground", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.exposureVariant", 1, Some(2)),
            rh_foundation::ElementCardinality::new("Evidence.outcome", 1, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Synthesis {
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

impl crate::traits::resource::ResourceMutators for Synthesis {
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

impl crate::traits::resource::ResourceExistence for Synthesis {
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

impl crate::traits::synthesis::SynthesisMutators for Synthesis {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Synthesis {
    fn resource_type(&self) -> &'static str {
        "Evidence"
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
        Some("http://hl7.org/fhir/StructureDefinition/synthesis")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::synthesis::{SynthesisAccessors, SynthesisExistence, SynthesisMutators};
