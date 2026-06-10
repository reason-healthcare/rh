use crate::profiles::shareablemeasure::Shareablemeasure;
use serde::{Deserialize, Serialize};
/// Publishable Measure
///
/// Supports declaration of the Measure metadata required by HL7 and other organizations that share and publish measures with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablemeasure
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Measure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablemeasure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Publishablemeasure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Shareablemeasure,
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
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
    rh_foundation::Invariant::new("mea-1", rh_foundation::Severity::Error, "Stratifier SHALL be either a single criteria or a set of criteria components", "group.stratifier.all((code | description | criteria).exists() xor component.exists())"),
    rh_foundation::Invariant::new("mea-2", rh_foundation::Severity::Warning, "Link ids should be 255 characters or less", "$this.length() <= 255"),
    rh_foundation::Invariant::new("mea-3", rh_foundation::Severity::Warning, "Link ids should be 255 characters or less", "$this.length() <= 255"),
    rh_foundation::Invariant::new("mea-4", rh_foundation::Severity::Warning, "Link ids should be 255 characters or less", "$this.length() <= 255"),
    rh_foundation::Invariant::new("mea-5", rh_foundation::Severity::Warning, "Link ids should be 255 characters or less", "$this.length() <= 255"),
    rh_foundation::Invariant::new("mea-6", rh_foundation::Severity::Warning, "Link ids should be 255 characters or less", "$this.length() <= 255"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("Measure.basis", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/fhir-types|5.0.0"),
    rh_foundation::ElementBinding::new("Measure.group.basis", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/fhir-types|5.0.0"),
    rh_foundation::ElementBinding::new("Measure.group.improvementNotation", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/measure-improvement-notation|5.0.0"),
    rh_foundation::ElementBinding::new("Measure.improvementNotation", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/measure-improvement-notation|5.0.0").with_description("Observation values that indicate what change in a measurement value or score is indicative of an improvement in the measured item or scored issue."),
    rh_foundation::ElementBinding::new("Measure.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("Measure.relatedArtifact.publicationStatus", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("Publication status of an artifact being referred to."),
    rh_foundation::ElementBinding::new("Measure.relatedArtifact.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/related-artifact-type|5.0.0").with_description("The type of relationship to the related artifact."),
    rh_foundation::ElementBinding::new("Measure.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Measure", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.contained", 0, None),
            rh_foundation::ElementCardinality::new("Measure.extension", 0, None),
            rh_foundation::ElementCardinality::new("Measure.extension", 0, None),
            rh_foundation::ElementCardinality::new("Measure.extension", 0, None),
            rh_foundation::ElementCardinality::new("Measure.extension", 0, None),
            rh_foundation::ElementCardinality::new("Measure.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Measure.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Measure.version", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.subtitle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.basis", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.contact", 0, None),
            rh_foundation::ElementCardinality::new("Measure.description", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.useContext", 0, None),
            rh_foundation::ElementCardinality::new("Measure.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("Measure.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.usage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.topic", 0, None),
            rh_foundation::ElementCardinality::new("Measure.author", 0, None),
            rh_foundation::ElementCardinality::new("Measure.editor", 0, None),
            rh_foundation::ElementCardinality::new("Measure.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("Measure.endorser", 0, None),
            rh_foundation::ElementCardinality::new("Measure.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("Measure.relatedArtifact.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.relatedArtifact.extension", 0, None),
            rh_foundation::ElementCardinality::new("Measure.relatedArtifact.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.relatedArtifact.classifier", 0, None),
            rh_foundation::ElementCardinality::new("Measure.relatedArtifact.label", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.relatedArtifact.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.relatedArtifact.citation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.relatedArtifact.document", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.relatedArtifact.resource", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Measure.relatedArtifact.resourceReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Measure.relatedArtifact.publicationStatus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Measure.relatedArtifact.publicationDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Measure.library", 0, None),
            rh_foundation::ElementCardinality::new("Measure.disclaimer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.scoring", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.scoringUnit", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.compositeScoring", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.type", 0, None),
            rh_foundation::ElementCardinality::new("Measure.riskAdjustment", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.rateAggregation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.rationale", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Measure.clinicalRecommendationStatement",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Measure.improvementNotation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.term", 0, None),
            rh_foundation::ElementCardinality::new("Measure.term.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.term.extension", 0, None),
            rh_foundation::ElementCardinality::new("Measure.term.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Measure.term.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.term.definition", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.guidance", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group", 0, None),
            rh_foundation::ElementCardinality::new("Measure.group.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.extension", 0, None),
            rh_foundation::ElementCardinality::new("Measure.group.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Measure.group.linkId", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.type", 0, None),
            rh_foundation::ElementCardinality::new("Measure.group.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.basis", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.scoring", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.scoringUnit", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.rateAggregation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.improvementNotation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.library", 0, None),
            rh_foundation::ElementCardinality::new("Measure.group.population", 0, None),
            rh_foundation::ElementCardinality::new("Measure.group.population.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.population.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Measure.group.population.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Measure.group.population.linkId", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.population.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Measure.group.population.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Measure.group.population.criteria", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Measure.group.population.groupDefinition",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Measure.group.population.inputPopulationId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Measure.group.population.aggregateMethod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Measure.group.stratifier", 0, None),
            rh_foundation::ElementCardinality::new("Measure.group.stratifier.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.stratifier.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Measure.group.stratifier.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Measure.group.stratifier.linkId", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.group.stratifier.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Measure.group.stratifier.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Measure.group.stratifier.criteria", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Measure.group.stratifier.groupDefinition",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Measure.group.stratifier.component", 0, None),
            rh_foundation::ElementCardinality::new(
                "Measure.group.stratifier.component.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Measure.group.stratifier.component.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Measure.group.stratifier.component.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Measure.group.stratifier.component.linkId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Measure.group.stratifier.component.code",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Measure.group.stratifier.component.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Measure.group.stratifier.component.criteria",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Measure.group.stratifier.component.groupDefinition",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Measure.supplementalData", 0, None),
            rh_foundation::ElementCardinality::new("Measure.supplementalData.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.supplementalData.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Measure.supplementalData.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Measure.supplementalData.linkId", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.supplementalData.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Measure.supplementalData.usage", 0, None),
            rh_foundation::ElementCardinality::new(
                "Measure.supplementalData.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Measure.supplementalData.criteria", 1, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Publishablemeasure {
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

impl crate::traits::resource::ResourceMutators for Publishablemeasure {
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

impl crate::traits::resource::ResourceExistence for Publishablemeasure {
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

impl crate::traits::publishablemeasure::PublishablemeasureMutators for Publishablemeasure {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Publishablemeasure {
    fn resource_type(&self) -> &'static str {
        "Measure"
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
        Some("http://hl7.org/fhir/StructureDefinition/publishablemeasure")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::publishablemeasure::{
    PublishablemeasureAccessors, PublishablemeasureExistence, PublishablemeasureMutators,
};
