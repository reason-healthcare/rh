use crate::resources::artifact_assessment::ArtifactAssessment;
use serde::{Deserialize, Serialize};
/// EBM Recommendation
///
/// Represents justification for a recommendation
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ebmrecommendation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ArtifactAssessment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ArtifactAssessment
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Ebmrecommendation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: ArtifactAssessment,
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
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
                "ArtifactAssessment.content.informationType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/artifactassessment-information-type|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "ArtifactAssessment.disposition",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/artifactassessment-disposition|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "ArtifactAssessment.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ArtifactAssessment.workflowStatus",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/artifactassessment-workflow-status|5.0.0",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ArtifactAssessment", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.contained", 0, None),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.extension", 0, None),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.citeAs[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.artifact[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content", 0, None),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.informationType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.summary",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.classifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content.author", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content.path", 0, None),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.relatedArtifact",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.freeToShare",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content.component", 0, None),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.workflowStatus", 0, Some(0)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.disposition", 0, Some(0)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Ebmrecommendation {
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

impl crate::traits::resource::ResourceMutators for Ebmrecommendation {
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

impl crate::traits::resource::ResourceExistence for Ebmrecommendation {
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

impl crate::traits::ebmrecommendation::EbmrecommendationMutators for Ebmrecommendation {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Ebmrecommendation {
    fn resource_type(&self) -> &'static str {
        "ArtifactAssessment"
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
        Some("http://hl7.org/fhir/StructureDefinition/ebmrecommendation")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::ebmrecommendation::{
    EbmrecommendationAccessors, EbmrecommendationExistence, EbmrecommendationMutators,
};
