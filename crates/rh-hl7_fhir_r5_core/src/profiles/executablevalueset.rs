use crate::profiles::shareablevalueset::Shareablevalueset;
use serde::{Deserialize, Serialize};
/// Executable ValueSet
///
/// Defines an executable value set as one that SHALL have an expansion included, as well as a usage warning indicating the expansion is a point-in-time snapshot and must be maintained over time for production usage. The value set expansion specifies the timestamp when the expansion was produced, SHOULD contain the parameters used for the expansion, and SHALL contain the codes that are obtained by evaluating the value set definition. If this is ONLY an executable value set, a computable definition of the value set must be obtained to compute the updated expansion.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/executablevalueset
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablevalueset
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Executablevalueset {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Shareablevalueset,
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("cnl-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.exists() implies name.matches('^[A-Z]([A-Za-z0-9_]){1,254}$')"),
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("evs-1", rh_foundation::Severity::Error, "For contains, a version SHALL be provided unless the parameters element has a \"system-version\" parameter corresponding to the system of the entry and the version matches the system-version parameter.", "contains.version.exists() xor  parameter.where(name = 'system-version').exists()"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
    rh_foundation::Invariant::new("vsd-1", rh_foundation::Severity::Error, "A value set include/exclude SHALL have a value set or a system", "valueSet.exists() or system.exists()"),
    rh_foundation::Invariant::new("vsd-10", rh_foundation::Severity::Error, "SHALL have a system if a code is present", "code.empty() or system.exists()"),
    rh_foundation::Invariant::new("vsd-11", rh_foundation::Severity::Error, "Must have a value for concept.designation.use if concept.designation.additionalUse is present", "additionalUse.exists() implies use.exists()"),
    rh_foundation::Invariant::new("vsd-2", rh_foundation::Severity::Error, "A value set with concepts or filters SHALL include a system", "(concept.exists() or filter.exists()) implies system.exists()"),
    rh_foundation::Invariant::new("vsd-3", rh_foundation::Severity::Error, "Cannot have both concept and filter", "concept.empty() or filter.empty()"),
    rh_foundation::Invariant::new("vsd-6", rh_foundation::Severity::Error, "SHALL have a code or a display", "code.exists() or display.exists()"),
    rh_foundation::Invariant::new("vsd-9", rh_foundation::Severity::Error, "SHALL have a code if not abstract", "code.exists() or abstract = true"),
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
                "ValueSet.compose.include.concept.designation.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ValueSet.compose.include.filter.op",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/filter-operator|5.0.0",
            )
            .with_description(
                "The kind of operation to perform as a part of a property based filter.",
            ),
            rh_foundation::ElementBinding::new(
                "ValueSet.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ValueSet.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
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
            rh_foundation::ElementCardinality::new("ValueSet.extension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.extension", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.extension", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.extension.extension", 0, Some(0)),
            rh_foundation::ElementCardinality::new("ValueSet.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.extension.value[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.version", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.experimental", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.contact", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.description", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.useContext", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.immutable", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.topic", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.author", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.editor", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.endorser", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.relatedArtifact", 0, None),
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
                "ValueSet.compose.include.concept.designation.additionalUse",
                0,
                None,
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
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.copyright",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ValueSet.compose.exclude", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.compose.property", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.extension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.next", 0, Some(1)),
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
            rh_foundation::ElementCardinality::new("ValueSet.expansion.property", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.property.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.property.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.property.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.property.uri", 0, Some(1)),
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
            rh_foundation::ElementCardinality::new("ValueSet.expansion.contains.code", 1, Some(1)),
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
            rh_foundation::ElementCardinality::new("ValueSet.expansion.contains.property", 0, None),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.property.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.property.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.property.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.property.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.property.subProperty",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.property.subProperty.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.property.subProperty.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.property.subProperty.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.property.subProperty.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.property.subProperty.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.contains.contains", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.scope", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.scope.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.scope.extension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.scope.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.scope.inclusionCriteria", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.scope.exclusionCriteria", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Executablevalueset {
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

impl crate::traits::resource::ResourceMutators for Executablevalueset {
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

impl crate::traits::resource::ResourceExistence for Executablevalueset {
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

impl crate::traits::executablevalueset::ExecutablevaluesetMutators for Executablevalueset {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Executablevalueset {
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
        Some("http://hl7.org/fhir/StructureDefinition/executablevalueset")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::executablevalueset::{
    ExecutablevaluesetAccessors, ExecutablevaluesetExistence, ExecutablevaluesetMutators,
};
