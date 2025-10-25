use crate::resources::value_set::ValueSet;
use serde::{Deserialize, Serialize};
/// Shareable ValueSet
///
/// Enforces the minimum information set for the value set metadata required by HL7 and other organizations that share and publish value sets
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareablevalueset
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ValueSet
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Shareablevalueset {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: ValueSet,
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
    rh_foundation::Invariant::new("vsd-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
    rh_foundation::Invariant::new("vsd-1", rh_foundation::Severity::Error, "A value set include/exclude SHALL have a value set or a system", "valueSet.exists() or system.exists()").with_xpath("exists(f:valueSet) or exists(f:system)"),
    rh_foundation::Invariant::new("vsd-10", rh_foundation::Severity::Error, "Must have a system if a code is present", "code.empty() or system.exists()").with_xpath("exists(f:system) or not(exists(f:code))"),
    rh_foundation::Invariant::new("vsd-2", rh_foundation::Severity::Error, "A value set with concepts or filters SHALL include a system", "(concept.exists() or filter.exists()) implies system.exists()").with_xpath("not(exists(f:concept) or exists(f:filter)) or exists(f:system)"),
    rh_foundation::Invariant::new("vsd-3", rh_foundation::Severity::Error, "Cannot have both concept and filter", "concept.empty() or filter.empty()").with_xpath("not(exists(f:concept)) or not(exists(f:filter))"),
    rh_foundation::Invariant::new("vsd-6", rh_foundation::Severity::Error, "SHALL have a code or a display", "code.exists() or display.exists()").with_xpath("exists(f:code) or exists(f:display)"),
    rh_foundation::Invariant::new("vsd-9", rh_foundation::Severity::Error, "Must have a code if not abstract", "code.exists() or abstract = true").with_xpath("exists(f:code) or (f:abstract/@value = true())"),
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
                "ValueSet.compose.include.filter.op",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/filter-operator|4.0.1",
            )
            .with_description(
                "The kind of operation to perform as a part of a property based filter.",
            ),
            rh_foundation::ElementBinding::new(
                "ValueSet.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|4.0.1",
            )
            .with_description("The lifecycle status of an artifact."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ValueSet", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.contained", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.extension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.version", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.experimental", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.publisher", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.contact", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.description", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.useContext", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.immutable", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.extension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.compose.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.compose.lockedDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.inactive", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include", 1, None),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.system", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.concept", 0, None),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.display",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation.use",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation.value",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.filter", 0, None),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.filter.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.filter.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.filter.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.filter.property",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.filter.op",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.filter.value",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.valueSet", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.compose.exclude", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.extension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.timestamp", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.total", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.offset", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.parameter", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.parameter.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.parameter.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.parameter.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.parameter.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.parameter.value[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.contains", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.contains.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.system",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.abstract",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.inactive",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.version",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.contains.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.display",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.designation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.contains.contains", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Shareablevalueset {
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

impl crate::traits::resource::ResourceMutators for Shareablevalueset {
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

impl crate::traits::resource::ResourceExistence for Shareablevalueset {
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

impl crate::traits::shareablevalueset::ShareablevaluesetMutators for Shareablevalueset {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Shareablevalueset {
    fn resource_type(&self) -> &'static str {
        "ValueSet"
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
        Some("http://hl7.org/fhir/StructureDefinition/shareablevalueset")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::shareablevalueset::{
    ShareablevaluesetAccessors, ShareablevaluesetExistence, ShareablevaluesetMutators,
};
