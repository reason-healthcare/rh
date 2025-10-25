use crate::datatypes::extension::Extension;
use crate::resources::service_request::ServiceRequest;
use serde::{Deserialize, Serialize};
/// ServiceRequest-Genetics
///
/// Describes how the ServiceRequest resource is used to for genetics
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/servicerequest-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ServiceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ServiceRequest
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServicerequestGenetics {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: ServiceRequest,
}
/// Item
///
/// The specific diagnostic investigations that are requested as part of this request. Sometimes, there can only be one item per request, but in most contexts, more than one investigation can be requested.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/servicerequest-geneticsItem
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicerequestGeneticsItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
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
    rh_foundation::Invariant::new("prr-1", rh_foundation::Severity::Error, "orderDetail SHALL only be present if code is present", "orderDetail.empty() or code.exists()").with_xpath("exists(f:code) or not(exists(f:orderDetail))"),
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
                "ServiceRequest.intent",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/request-intent|4.0.1",
            )
            .with_description("The kind of service request."),
            rh_foundation::ElementBinding::new(
                "ServiceRequest.priority",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/request-priority|4.0.1",
            )
            .with_description(
                "Identifies the level of importance to be assigned to actioning the request.",
            ),
            rh_foundation::ElementBinding::new(
                "ServiceRequest.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/request-status|4.0.1",
            )
            .with_description("The status of a service order."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ServiceRequest", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.contained", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.extension", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.extension", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.instantiatesCanonical", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.instantiatesUri", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.replaces", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.requisition", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.intent", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.category", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.doNotPerform", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.orderDetail", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.quantity[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.occurrence[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.asNeeded[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.authoredOn", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.requester", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.performerType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.performer", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.locationCode", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.locationReference", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.reasonCode", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.reasonReference", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.insurance", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.supportingInfo", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.specimen", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.bodySite", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.note", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.patientInstruction", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.relevantHistory", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ServicerequestGenetics {
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

impl crate::traits::resource::ResourceMutators for ServicerequestGenetics {
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

impl crate::traits::resource::ResourceExistence for ServicerequestGenetics {
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

impl crate::traits::servicerequest_genetics::ServicerequestGeneticsMutators
    for ServicerequestGenetics
{
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for ServicerequestGenetics {
    fn resource_type(&self) -> &'static str {
        "ServiceRequest"
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
        Some("http://hl7.org/fhir/StructureDefinition/servicerequest-genetics")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::servicerequest_genetics::{
    ServicerequestGeneticsAccessors, ServicerequestGeneticsExistence,
    ServicerequestGeneticsMutators,
};
