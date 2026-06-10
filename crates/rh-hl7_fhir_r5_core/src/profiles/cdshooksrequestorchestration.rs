use crate::resources::request_orchestration::RequestOrchestration;
use serde::{Deserialize, Serialize};
/// C D S  Hooks  Request Orchestration
///
/// Defines a RequestOrchestration that can represent a CDS Hooks response
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksrequestorchestration
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: RequestOrchestration
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/RequestOrchestration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cdshooksrequestorchestration {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: RequestOrchestration,
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
    rh_foundation::Invariant::new("pld-0", rh_foundation::Severity::Error, "Input data elements must have a requirement or a relatedData, but not both", "requirement.exists() xor relatedData.exists()"),
    rh_foundation::Invariant::new("pld-1", rh_foundation::Severity::Error, "Output data element must have a requirement or a relatedData, but not both", "requirement.exists() xor relatedData.exists()"),
    rh_foundation::Invariant::new("rqg-1", rh_foundation::Severity::Error, "Must have resource or action but not both", "resource.exists() != action.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("RequestOrchestration.action.cardinalityBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-cardinality-behavior|5.0.0").with_description("Defines behavior for an action or a group for how many times that item may be repeated."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.condition.kind", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-condition-kind|5.0.0").with_description("The kind of condition for the action."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.groupingBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-grouping-behavior|5.0.0").with_description("Defines organization behavior of a group."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.participant.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-participant-type|5.0.0").with_description("The type of participant in the activity."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.precheckBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-precheck-behavior|5.0.0").with_description("Defines selection frequency behavior for an action or group."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|5.0.0").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.relatedAction.endRelationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-relationship-type|5.0.0").with_description("Defines the types of relationships between actions."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.relatedAction.relationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-relationship-type|5.0.0").with_description("Defines the types of relationships between actions."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.requiredBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-required-behavior|5.0.0").with_description("Defines expectations around whether an action or action group is required."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.selectionBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-selection-behavior|5.0.0").with_description("Defines selection behavior of a group."),
    rh_foundation::ElementBinding::new("RequestOrchestration.intent", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-intent|5.0.0").with_description("Codes indicating the degree of authority/intentionality associated with a request."),
    rh_foundation::ElementBinding::new("RequestOrchestration.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("RequestOrchestration.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|5.0.0").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("RequestOrchestration.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-status|5.0.0").with_description("Codes identifying the lifecycle stage of a request."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("RequestOrchestration", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.contained", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.identifier", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.instantiatesCanonical",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.instantiatesUri",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.replaces", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.groupIdentifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.intent", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.authoredOn", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.author", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.reason", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.goal", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.note", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.linkId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.prefix",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.textEquivalent",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.priority",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.code", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.documentation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.goal", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.condition",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.condition.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.condition.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.condition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.condition.kind",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.condition.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.input", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.input.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.input.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.input.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.input.title",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.input.requirement",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.input.relatedData",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.output", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.output.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.output.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.output.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.output.title",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.output.requirement",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.output.relatedData",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.targetId",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.relationship",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.endRelationship",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.offset[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.timing[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.location",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.typeCanonical",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.typeReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.role",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.actor[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.groupingBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.selectionBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.requiredBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.precheckBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.cardinalityBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.resource",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.definition[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.transform",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.dynamicValue",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.dynamicValue.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.dynamicValue.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.dynamicValue.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.dynamicValue.path",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.dynamicValue.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.action", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Cdshooksrequestorchestration {
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

impl crate::traits::resource::ResourceMutators for Cdshooksrequestorchestration {
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

impl crate::traits::resource::ResourceExistence for Cdshooksrequestorchestration {
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

impl crate::traits::cdshooksrequestorchestration::CdshooksrequestorchestrationMutators
    for Cdshooksrequestorchestration
{
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Cdshooksrequestorchestration {
    fn resource_type(&self) -> &'static str {
        "RequestOrchestration"
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
        Some("http://hl7.org/fhir/StructureDefinition/cdshooksrequestorchestration")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::cdshooksrequestorchestration::{
    CdshooksrequestorchestrationAccessors, CdshooksrequestorchestrationExistence,
    CdshooksrequestorchestrationMutators,
};
