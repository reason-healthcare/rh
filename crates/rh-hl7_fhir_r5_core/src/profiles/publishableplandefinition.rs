use crate::profiles::shareableplandefinition::Shareableplandefinition;
use serde::{Deserialize, Serialize};
/// Publishable PlanDefinition
///
/// Supports declaration of the PlanDefinition metadata required by HL7 and other organizations that share and publish plandefinitions with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishableplandefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareableplandefinition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Publishableplandefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Shareableplandefinition,
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
    rh_foundation::Invariant::new("pld-0", rh_foundation::Severity::Error, "Input data elements must have a requirement or a relatedData, but not both", "requirement.exists() xor relatedData.exists()"),
    rh_foundation::Invariant::new("pld-1", rh_foundation::Severity::Error, "Output data element must have a requirement or a relatedData, but not both", "requirement.exists() xor relatedData.exists()"),
    rh_foundation::Invariant::new("pld-3", rh_foundation::Severity::Warning, "goalid should reference the id of a goal definition", "%context.repeat(action).where((goalId in %context.goal.id).not()).exists().not()"),
    rh_foundation::Invariant::new("pld-4", rh_foundation::Severity::Warning, "targetId should reference the id of an action", "%context.repeat(action).relatedAction.where((targetId in %context.repeat(action).id).not()).exists().not()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("PlanDefinition.action.cardinalityBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-cardinality-behavior|5.0.0").with_description("Defines behavior for an action or a group for how many times that item may be repeated."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.condition.kind", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-condition-kind|5.0.0").with_description("Defines the kinds of conditions that can appear on actions."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.groupingBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-grouping-behavior|5.0.0").with_description("Defines organization behavior of a group."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.participant.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-participant-type|5.0.0").with_description("The type of participant in the activity."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.precheckBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-precheck-behavior|5.0.0").with_description("Defines selection frequency behavior for an action or group."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|5.0.0").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.relatedAction.endRelationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-relationship-type|5.0.0").with_description("Defines the types of relationships between actions."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.relatedAction.relationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-relationship-type|5.0.0").with_description("Defines the types of relationships between actions."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.requiredBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-required-behavior|5.0.0").with_description("Defines expectations around whether an action or action group is required."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.selectionBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-selection-behavior|5.0.0").with_description("Defines selection behavior of a group."),
    rh_foundation::ElementBinding::new("PlanDefinition.actor.option.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-participant-type|5.0.0").with_description("The type of participant in the activity."),
    rh_foundation::ElementBinding::new("PlanDefinition.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("PlanDefinition.relatedArtifact.publicationStatus", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("Publication status of an artifact being referred to."),
    rh_foundation::ElementBinding::new("PlanDefinition.relatedArtifact.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/related-artifact-type|5.0.0").with_description("The type of relationship to the related artifact."),
    rh_foundation::ElementBinding::new("PlanDefinition.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("PlanDefinition", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.version", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.subtitle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.description", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.usage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.topic", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.author", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.editor", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.endorser", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.relatedArtifact.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.relatedArtifact.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.relatedArtifact.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.relatedArtifact.classifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.relatedArtifact.label",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.relatedArtifact.display",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.relatedArtifact.citation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.relatedArtifact.document",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.relatedArtifact.resource",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.relatedArtifact.resourceReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.relatedArtifact.publicationStatus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.relatedArtifact.publicationDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.library", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.goal.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.category", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.description", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.start", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.addresses", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.documentation", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.target", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.target.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.target.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.goal.target.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.goal.target.measure",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.goal.target.detail[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.target.due", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.actor.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.option", 1, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.option.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.actor.option.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.actor.option.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.option.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.actor.option.typeCanonical",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.actor.option.typeReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.option.role", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.linkId", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.prefix", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.textEquivalent",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.reason", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.documentation", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.goalId", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.trigger", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.condition", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.condition.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.condition.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.condition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.condition.kind",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.condition.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.input", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.input.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.input.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.input.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.input.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.input.requirement",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.input.relatedData",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.output", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.output.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.output.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.output.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.output.title",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.output.requirement",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.output.relatedData",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.relatedAction", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.targetId",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.relationship",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.endRelationship",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.offset[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.timing[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.participant", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.actorId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.typeCanonical",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.typeReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.role",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.groupingBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.selectionBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.requiredBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.precheckBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.cardinalityBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.definition[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.transform", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.dynamicValue", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.dynamicValue.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.dynamicValue.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.dynamicValue.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.dynamicValue.path",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.dynamicValue.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.action", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.asNeeded[x]", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Publishableplandefinition {
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

impl crate::traits::resource::ResourceMutators for Publishableplandefinition {
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

impl crate::traits::resource::ResourceExistence for Publishableplandefinition {
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

impl crate::traits::publishableplandefinition::PublishableplandefinitionMutators
    for Publishableplandefinition
{
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Publishableplandefinition {
    fn resource_type(&self) -> &'static str {
        "PlanDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/publishableplandefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::publishableplandefinition::{
    PublishableplandefinitionAccessors, PublishableplandefinitionExistence,
    PublishableplandefinitionMutators,
};
