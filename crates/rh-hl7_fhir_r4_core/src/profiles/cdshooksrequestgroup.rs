use crate::resources::request_group::RequestGroup;
use serde::{Deserialize, Serialize};
/// CDS Hooks RequestGroup
///
/// Defines a RequestGroup that can represent a CDS Hooks response
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksrequestgroup
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RequestGroup
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/RequestGroup
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cdshooksrequestgroup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: RequestGroup,
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
    rh_foundation::Invariant::new("rqg-1", rh_foundation::Severity::Error, "Must have resource or action but not both", "resource.exists() != action.exists()").with_xpath("exists(f:resource) != exists(f:action)"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("RequestGroup.action.cardinalityBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-cardinality-behavior|4.0.1").with_description("Defines behavior for an action or a group for how many times that item may be repeated."),
    rh_foundation::ElementBinding::new("RequestGroup.action.condition.kind", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-condition-kind|4.0.1").with_description("The kind of condition for the action."),
    rh_foundation::ElementBinding::new("RequestGroup.action.groupingBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-grouping-behavior|4.0.1").with_description("Defines organization behavior of a group."),
    rh_foundation::ElementBinding::new("RequestGroup.action.precheckBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-precheck-behavior|4.0.1").with_description("Defines selection frequency behavior for an action or group."),
    rh_foundation::ElementBinding::new("RequestGroup.action.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|4.0.1").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("RequestGroup.action.relatedAction.relationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-relationship-type|4.0.1").with_description("Defines the types of relationships between actions."),
    rh_foundation::ElementBinding::new("RequestGroup.action.requiredBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-required-behavior|4.0.1").with_description("Defines expectations around whether an action or action group is required."),
    rh_foundation::ElementBinding::new("RequestGroup.action.selectionBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-selection-behavior|4.0.1").with_description("Defines selection behavior of a group."),
    rh_foundation::ElementBinding::new("RequestGroup.intent", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-intent|4.0.1").with_description("Codes indicating the degree of authority/intentionality associated with a request."),
    rh_foundation::ElementBinding::new("RequestGroup.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|4.0.1").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("RequestGroup.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-status|4.0.1").with_description("Codes identifying the lifecycle stage of a request."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("RequestGroup", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.contained", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.extension", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.identifier", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.instantiatesCanonical", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.instantiatesUri", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.replaces", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.groupIdentifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.intent", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.authoredOn", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.author", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.reasonCode", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.reasonReference", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.note", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.action", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.action.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.action.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("RequestGroup.action.prefix", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.action.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.action.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.textEquivalent",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestGroup.action.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.action.code", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.action.documentation", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.action.condition", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.action.condition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.condition.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.condition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.condition.kind",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.condition.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestGroup.action.relatedAction", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.relatedAction.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.relatedAction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.relatedAction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.relatedAction.actionId",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.relatedAction.relationship",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.relatedAction.offset[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestGroup.action.timing[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.action.participant", 0, None),
            rh_foundation::ElementCardinality::new("RequestGroup.action.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.groupingBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.selectionBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.requiredBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.precheckBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestGroup.action.cardinalityBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestGroup.action.resource", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestGroup.action.action", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Cdshooksrequestgroup {
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

impl crate::traits::resource::ResourceMutators for Cdshooksrequestgroup {
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

impl crate::traits::resource::ResourceExistence for Cdshooksrequestgroup {
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

impl crate::traits::cdshooksrequestgroup::CdshooksrequestgroupMutators for Cdshooksrequestgroup {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Cdshooksrequestgroup {
    fn resource_type(&self) -> &'static str {
        "RequestGroup"
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
        Some("http://hl7.org/fhir/StructureDefinition/cdshooksrequestgroup")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::cdshooksrequestgroup::{
    CdshooksrequestgroupAccessors, CdshooksrequestgroupExistence, CdshooksrequestgroupMutators,
};
