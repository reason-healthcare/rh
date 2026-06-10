use crate::profiles::shareableactivitydefinition::Shareableactivitydefinition;
use serde::{Deserialize, Serialize};
/// Publishable ActivityDefinition
///
/// Supports declaration of the ActivityDefinition metadata required by HL7 and other organizations that share and publish activity definitions with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishableactivitydefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ActivityDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareableactivitydefinition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Publishableactivitydefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Shareableactivitydefinition,
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
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("ActivityDefinition.intent", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-intent|5.0.0").with_description("Codes indicating the degree of authority/intentionality associated with a request."),
    rh_foundation::ElementBinding::new("ActivityDefinition.kind", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-resource-types|5.0.0").with_description("The kind of activity the definition is describing."),
    rh_foundation::ElementBinding::new("ActivityDefinition.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("ActivityDefinition.participant.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-participant-type|5.0.0").with_description("The type of participant in the activity."),
    rh_foundation::ElementBinding::new("ActivityDefinition.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|5.0.0").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("ActivityDefinition.relatedArtifact.publicationStatus", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("Publication status of an artifact being referred to."),
    rh_foundation::ElementBinding::new("ActivityDefinition.relatedArtifact.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/related-artifact-type|5.0.0").with_description("The type of relationship to the related artifact."),
    rh_foundation::ElementBinding::new("ActivityDefinition.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ActivityDefinition", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.version", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ActivityDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.subtitle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.experimental", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.description", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.usage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.effectivePeriod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ActivityDefinition.topic", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.author", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.editor", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.endorser", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.relatedArtifact.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.relatedArtifact.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.relatedArtifact.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.relatedArtifact.classifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.relatedArtifact.label",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.relatedArtifact.display",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.relatedArtifact.citation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.relatedArtifact.document",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.relatedArtifact.resource",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.relatedArtifact.resourceReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.relatedArtifact.publicationStatus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.relatedArtifact.publicationDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ActivityDefinition.library", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.kind", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.profile", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.intent", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.doNotPerform", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.timing[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.asNeeded[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.participant", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.participant.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.participant.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.participant.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.participant.typeCanonical",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.participant.typeReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.participant.role",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.participant.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ActivityDefinition.product[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.quantity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.dosage", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.bodySite", 0, None),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.specimenRequirement",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.observationRequirement",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.observationResultRequirement",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ActivityDefinition.transform", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.dynamicValue", 0, None),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.dynamicValue.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.dynamicValue.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.dynamicValue.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.dynamicValue.path",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.dynamicValue.expression",
                1,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Publishableactivitydefinition {
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

impl crate::traits::resource::ResourceMutators for Publishableactivitydefinition {
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

impl crate::traits::resource::ResourceExistence for Publishableactivitydefinition {
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

impl crate::traits::publishableactivitydefinition::PublishableactivitydefinitionMutators
    for Publishableactivitydefinition
{
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Publishableactivitydefinition {
    fn resource_type(&self) -> &'static str {
        "ActivityDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/publishableactivitydefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::publishableactivitydefinition::{
    PublishableactivitydefinitionAccessors, PublishableactivitydefinitionExistence,
    PublishableactivitydefinitionMutators,
};
