use crate::bindings::request_priority::RequestPriority;
use crate::bindings::supplyrequest_status::SupplyrequestStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SupplyRequest
///
/// A record of a request for a medication, substance or device used in the healthcare setting.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SupplyRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SupplyRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyRequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for SupplyRequest
    pub identifier: Option<Vec<Identifier>>,
    /// draft | active | suspended +
    pub status: Option<SupplyrequestStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The kind of supply (central, non-stock, etc.)
    ///
    /// Binding: example (Category of supply request.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/supplyrequest-kind
    pub category: Option<CodeableConcept>,
    /// routine | urgent | asap | stat
    pub priority: Option<RequestPriority>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// Medication, Substance, or Device requested to be supplied (CodeableConcept)
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: CodeableConcept,
    /// Medication, Substance, or Device requested to be supplied (Reference)
    #[serde(rename = "itemReference")]
    pub item_reference: Reference,
    /// The requested amount of the item indicated
    pub quantity: Quantity,
    /// Ordered item details
    pub parameter: Option<Vec<SupplyRequestParameter>>,
    /// When the request should be fulfilled (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// When the request should be fulfilled (Period)
    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,
    /// When the request should be fulfilled (Timing)
    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,
    /// When the request was made
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<DateTimeType>,
    /// Extension element for the 'authoredOn' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authoredOn")]
    pub _authored_on: Option<Element>,
    /// Individual making the request
    pub requester: Option<Reference>,
    /// Who is intended to fulfill the request
    pub supplier: Option<Vec<Reference>>,
    /// The reason why the supply item was requested
    ///
    /// Binding: example (The reason why the supply item was requested.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/supplyrequest-reason
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// The reason why the supply item was requested
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// The origin of the supply
    #[serde(rename = "deliverFrom")]
    pub deliver_from: Option<Reference>,
    /// The destination of the supply
    #[serde(rename = "deliverTo")]
    pub deliver_to: Option<Reference>,
}
/// SupplyRequest nested structure for the 'parameter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyRequestParameter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Item detail
    ///
    /// Binding: example (A code that identifies the device detail.)
    pub code: Option<CodeableConcept>,
    /// Value of detail (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// Value of detail (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// Value of detail (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Option<Range>,
    /// Value of detail (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
}

impl Default for SupplyRequest {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            category: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            item_codeable_concept: Default::default(),
            item_reference: Default::default(),
            quantity: Quantity::default(),
            parameter: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            occurrence_timing: Default::default(),
            authored_on: Default::default(),
            _authored_on: Default::default(),
            requester: Default::default(),
            supplier: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            deliver_from: Default::default(),
            deliver_to: Default::default(),
        }
    }
}

impl Default for SupplyRequestParameter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            value_codeable_concept: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
            value_boolean: Default::default(),
        }
    }
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
                "SupplyRequest.priority",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/request-priority|4.0.1",
            )
            .with_description(
                "Identifies the level of importance to be assigned to actioning the request.",
            ),
            rh_foundation::ElementBinding::new(
                "SupplyRequest.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/supplyrequest-status|4.0.1",
            )
            .with_description("Status of the supply request."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("SupplyRequest.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.contained", 0, None),
            rh_foundation::ElementCardinality::new("SupplyRequest.extension", 0, None),
            rh_foundation::ElementCardinality::new("SupplyRequest.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("SupplyRequest.identifier", 0, None),
            rh_foundation::ElementCardinality::new("SupplyRequest.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.category", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.item[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.quantity", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.parameter", 0, None),
            rh_foundation::ElementCardinality::new("SupplyRequest.parameter.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.parameter.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "SupplyRequest.parameter.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SupplyRequest.parameter.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.parameter.value[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.occurrence[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.authoredOn", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.requester", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.supplier", 0, None),
            rh_foundation::ElementCardinality::new("SupplyRequest.reasonCode", 0, None),
            rh_foundation::ElementCardinality::new("SupplyRequest.reasonReference", 0, None),
            rh_foundation::ElementCardinality::new("SupplyRequest.deliverFrom", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyRequest.deliverTo", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SupplyRequest {
    fn id(&self) -> Option<String> {
        self.base.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for SupplyRequest {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for SupplyRequest {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for SupplyRequest {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for SupplyRequest {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: crate::datatypes::narrative::Narrative) -> Self {
        let mut resource = self.clone();
        resource.base.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<crate::resources::resource::Resource>) -> Self {
        let mut resource = self.clone();
        resource.base.contained = Some(value);
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .contained
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = Some(value);
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for SupplyRequest {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
}

impl crate::traits::supply_request::SupplyRequestAccessors for SupplyRequest {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Option<SupplyrequestStatus> {
        self.status.clone()
    }
    fn category(&self) -> Option<CodeableConcept> {
        self.category.clone()
    }
    fn priority(&self) -> Option<RequestPriority> {
        self.priority.clone()
    }
    fn quantity(&self) -> Quantity {
        self.quantity.clone()
    }
    fn parameter(&self) -> &[SupplyRequestParameter] {
        self.parameter.as_deref().unwrap_or(&[])
    }
    fn authored_on(&self) -> Option<DateTimeType> {
        self.authored_on.clone()
    }
    fn requester(&self) -> Option<Reference> {
        self.requester.clone()
    }
    fn supplier(&self) -> &[Reference] {
        self.supplier.as_deref().unwrap_or(&[])
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
    }
    fn deliver_from(&self) -> Option<Reference> {
        self.deliver_from.clone()
    }
    fn deliver_to(&self) -> Option<Reference> {
        self.deliver_to.clone()
    }
}

impl crate::traits::supply_request::SupplyRequestMutators for SupplyRequest {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: SupplyrequestStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_category(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn set_priority(self, value: RequestPriority) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_quantity(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.quantity = value;
        resource
    }
    fn set_parameter(self, value: Vec<SupplyRequestParameter>) -> Self {
        let mut resource = self.clone();
        resource.parameter = Some(value);
        resource
    }
    fn add_parameter(self, item: SupplyRequestParameter) -> Self {
        let mut resource = self.clone();
        resource.parameter.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_authored_on(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.authored_on = Some(value);
        resource
    }
    fn set_requester(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.requester = Some(value);
        resource
    }
    fn set_supplier(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supplier = Some(value);
        resource
    }
    fn add_supplier(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.supplier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.reason_code = Some(value);
        resource
    }
    fn add_reason_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason_code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reason_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.reason_reference = Some(value);
        resource
    }
    fn add_reason_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .reason_reference
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_deliver_from(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.deliver_from = Some(value);
        resource
    }
    fn set_deliver_to(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.deliver_to = Some(value);
        resource
    }
}

impl crate::traits::supply_request::SupplyRequestExistence for SupplyRequest {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
    fn has_occurrence(&self) -> bool {
        self.occurrence_date_time.is_some()
            || self.occurrence_period.is_some()
            || self.occurrence_timing.is_some()
    }
    fn has_item(&self) -> bool {
        true
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.is_some()
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_quantity(&self) -> bool {
        true
    }
    fn has_parameter(&self) -> bool {
        self.parameter.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_authored_on(&self) -> bool {
        self.authored_on.is_some()
    }
    fn has_requester(&self) -> bool {
        self.requester.is_some()
    }
    fn has_supplier(&self) -> bool {
        self.supplier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_deliver_from(&self) -> bool {
        self.deliver_from.is_some()
    }
    fn has_deliver_to(&self) -> bool {
        self.deliver_to.is_some()
    }
}

impl crate::validation::ValidatableResource for SupplyRequest {
    fn resource_type(&self) -> &'static str {
        "SupplyRequest"
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
        Some("http://hl7.org/fhir/StructureDefinition/SupplyRequest")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::supply_request::{
    SupplyRequestAccessors, SupplyRequestExistence, SupplyRequestMutators,
};
