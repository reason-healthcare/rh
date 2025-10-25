use crate::resources::plan_definition::PlanDefinition;
use serde::{Deserialize, Serialize};
/// CDS Hooks Service PlanDefinition
///
/// Defines a PlanDefinition that implements the behavior for a CDS Hooks service
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksserviceplandefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/PlanDefinition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cdshooksserviceplandefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: PlanDefinition,
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
    rh_foundation::Invariant::new("pdf-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("PlanDefinition.action.cardinalityBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-cardinality-behavior|4.0.1").with_description("Defines behavior for an action or a group for how many times that item may be repeated."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.condition.kind", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-condition-kind|4.0.1").with_description("Defines the kinds of conditions that can appear on actions."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.groupingBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-grouping-behavior|4.0.1").with_description("Defines organization behavior of a group."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.participant.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-participant-type|4.0.1").with_description("The type of participant for the action."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.precheckBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-precheck-behavior|4.0.1").with_description("Defines selection frequency behavior for an action or group."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|4.0.1").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.relatedAction.relationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-relationship-type|4.0.1").with_description("Defines the types of relationships between actions."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.requiredBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-required-behavior|4.0.1").with_description("Defines expectations around whether an action or action group is required."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.selectionBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-selection-behavior|4.0.1").with_description("Defines selection behavior of a group."),
    rh_foundation::ElementBinding::new("PlanDefinition.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|4.0.1").with_description("The lifecycle status of an artifact."),
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
            rh_foundation::ElementCardinality::new("PlanDefinition.extension", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.subtitle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.usage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.topic", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.author", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.editor", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.endorser", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.relatedArtifact", 0, None),
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
            rh_foundation::ElementCardinality::new("PlanDefinition.action", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.prefix", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.textEquivalent",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.code", 0, None),
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
            rh_foundation::ElementCardinality::new("PlanDefinition.action.output", 0, None),
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
                "PlanDefinition.action.relatedAction.actionId",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.relationship",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.offset[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.timing[x]", 0, Some(1)),
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
                "PlanDefinition.action.participant.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.role",
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
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Cdshooksserviceplandefinition {
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

impl crate::traits::resource::ResourceMutators for Cdshooksserviceplandefinition {
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

impl crate::traits::resource::ResourceExistence for Cdshooksserviceplandefinition {
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

impl crate::traits::cdshooksserviceplandefinition::CdshooksserviceplandefinitionMutators
    for Cdshooksserviceplandefinition
{
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Cdshooksserviceplandefinition {
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
        Some("http://hl7.org/fhir/StructureDefinition/cdshooksserviceplandefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::cdshooksserviceplandefinition::{
    CdshooksserviceplandefinitionAccessors, CdshooksserviceplandefinitionExistence,
    CdshooksserviceplandefinitionMutators,
};
