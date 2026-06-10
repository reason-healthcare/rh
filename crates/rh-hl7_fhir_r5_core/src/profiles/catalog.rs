use crate::resources::composition::Composition;
use serde::{Deserialize, Serialize};
/// Profile for  Catalog
///
/// A set of resources composed into a single coherent clinical statement with clinical attestation
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/catalog
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Composition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Catalog {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Composition,
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("cmp-1", rh_foundation::Severity::Error, "A section must contain at least one of text, entries, or sub-sections", "text.exists() or entry.exists() or section.exists()"),
    rh_foundation::Invariant::new("cmp-2", rh_foundation::Severity::Error, "A section can only have an emptyReason if it is empty", "emptyReason.empty() or entry.empty()"),
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
                "Composition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Composition.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/composition-status|5.0.0",
            )
            .with_description("The workflow/clinical status of the composition."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Composition", 0, None),
            rh_foundation::ElementCardinality::new("Composition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.contained", 0, None),
            rh_foundation::ElementCardinality::new("Composition.extension", 0, None),
            rh_foundation::ElementCardinality::new("Composition.extension", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Composition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Composition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.category", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.subject", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Composition.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("Composition.author", 1, None),
            rh_foundation::ElementCardinality::new("Composition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.note", 0, None),
            rh_foundation::ElementCardinality::new("Composition.attester", 0, None),
            rh_foundation::ElementCardinality::new("Composition.attester.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.attester.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Composition.attester.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Composition.attester.mode", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.attester.time", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.attester.party", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.custodian", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.relatesTo", 0, None),
            rh_foundation::ElementCardinality::new("Composition.event", 0, None),
            rh_foundation::ElementCardinality::new("Composition.event.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.event.extension", 0, None),
            rh_foundation::ElementCardinality::new("Composition.event.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Composition.event.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.event.detail", 0, None),
            rh_foundation::ElementCardinality::new("Composition.section", 0, None),
            rh_foundation::ElementCardinality::new("Composition.section.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Composition.section.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Composition.section.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.author", 0, None),
            rh_foundation::ElementCardinality::new("Composition.section.focus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.orderedBy", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.entry", 0, None),
            rh_foundation::ElementCardinality::new("Composition.section.emptyReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.section", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Catalog {
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

impl crate::traits::resource::ResourceMutators for Catalog {
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

impl crate::traits::resource::ResourceExistence for Catalog {
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

impl crate::traits::catalog::CatalogMutators for Catalog {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Catalog {
    fn resource_type(&self) -> &'static str {
        "Composition"
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
        Some("http://hl7.org/fhir/StructureDefinition/catalog")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::catalog::{CatalogAccessors, CatalogExistence, CatalogMutators};
