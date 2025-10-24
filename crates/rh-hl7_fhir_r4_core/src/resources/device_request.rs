use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_priority::RequestPriority;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
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
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// DeviceRequest
///
/// Represents a request for a patient to employ a medical device. The device may be an implantable device, or an external assistive device, such as a walker.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DeviceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External Request identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<StringType>>,
    /// Extension element for the 'instantiatesCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesCanonical")]
    pub _instantiates_canonical: Option<Element>,
    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<StringType>>,
    /// Extension element for the 'instantiatesUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesUri")]
    pub _instantiates_uri: Option<Element>,
    /// What request fulfills
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// What request replaces
    #[serde(rename = "priorRequest")]
    pub prior_request: Option<Vec<Reference>>,
    /// Identifier of composite request
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Identifier>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: Option<RequestStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: RequestIntent,
    /// Extension element for the 'intent' primitive field. Contains metadata and extensions.
    pub _intent: Option<Element>,
    /// routine | urgent | asap | stat
    pub priority: Option<RequestPriority>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// Device requested (Reference)
    #[serde(rename = "codeReference")]
    pub code_reference: Reference,
    /// Device requested (CodeableConcept)
    #[serde(rename = "codeCodeableConcept")]
    pub code_codeable_concept: CodeableConcept,
    /// Device details
    pub parameter: Option<Vec<DeviceRequestParameter>>,
    /// Focus of request
    pub subject: Reference,
    /// Encounter motivating request
    pub encounter: Option<Reference>,
    /// Desired time or schedule for use (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// Desired time or schedule for use (Period)
    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,
    /// Desired time or schedule for use (Timing)
    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,
    /// When recorded
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<DateTimeType>,
    /// Extension element for the 'authoredOn' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authoredOn")]
    pub _authored_on: Option<Element>,
    /// Who/what is requesting diagnostics
    pub requester: Option<Reference>,
    /// Filler role
    ///
    /// Binding: example (Indicates specific responsibility of an individual within the care team, such as "Primary physician", "Team coordinator", "Caregiver", etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/participant-role
    #[serde(rename = "performerType")]
    pub performer_type: Option<CodeableConcept>,
    /// Requested Filler
    pub performer: Option<Reference>,
    /// Coded Reason for request
    ///
    /// Binding: example (Diagnosis or problem codes justifying the reason for requesting the device.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Linked Reason for request
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Associated insurance coverage
    pub insurance: Option<Vec<Reference>>,
    /// Additional clinical information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Reference>>,
    /// Notes or comments
    pub note: Option<Vec<Annotation>>,
    /// Request provenance
    #[serde(rename = "relevantHistory")]
    pub relevant_history: Option<Vec<Reference>>,
}
/// DeviceRequest nested structure for the 'parameter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRequestParameter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Device detail
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

impl Default for DeviceRequest {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            based_on: Default::default(),
            prior_request: Default::default(),
            group_identifier: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            intent: RequestIntent::default(),
            _intent: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            code_reference: Default::default(),
            code_codeable_concept: Default::default(),
            parameter: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            occurrence_timing: Default::default(),
            authored_on: Default::default(),
            _authored_on: Default::default(),
            requester: Default::default(),
            performer_type: Default::default(),
            performer: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            insurance: Default::default(),
            supporting_info: Default::default(),
            note: Default::default(),
            relevant_history: Default::default(),
        }
    }
}

impl Default for DeviceRequestParameter {
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

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DeviceRequest {
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

impl crate::traits::resource::ResourceMutators for DeviceRequest {
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

impl crate::traits::resource::ResourceExistence for DeviceRequest {
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

impl crate::traits::domain_resource::DomainResourceAccessors for DeviceRequest {
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

impl crate::traits::domain_resource::DomainResourceMutators for DeviceRequest {
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

impl crate::traits::domain_resource::DomainResourceExistence for DeviceRequest {
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

impl crate::traits::device_request::DeviceRequestAccessors for DeviceRequest {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn instantiates_canonical(&self) -> &[StringType] {
        self.instantiates_canonical.as_deref().unwrap_or(&[])
    }
    fn instantiates_uri(&self) -> &[StringType] {
        self.instantiates_uri.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn prior_request(&self) -> &[Reference] {
        self.prior_request.as_deref().unwrap_or(&[])
    }
    fn group_identifier(&self) -> Option<Identifier> {
        self.group_identifier.clone()
    }
    fn status(&self) -> Option<RequestStatus> {
        self.status.clone()
    }
    fn intent(&self) -> RequestIntent {
        self.intent.clone()
    }
    fn priority(&self) -> Option<RequestPriority> {
        self.priority.clone()
    }
    fn parameter(&self) -> &[DeviceRequestParameter] {
        self.parameter.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn authored_on(&self) -> Option<DateTimeType> {
        self.authored_on.clone()
    }
    fn requester(&self) -> Option<Reference> {
        self.requester.clone()
    }
    fn performer_type(&self) -> Option<CodeableConcept> {
        self.performer_type.clone()
    }
    fn performer(&self) -> Option<Reference> {
        self.performer.clone()
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
    }
    fn insurance(&self) -> &[Reference] {
        self.insurance.as_deref().unwrap_or(&[])
    }
    fn supporting_info(&self) -> &[Reference] {
        self.supporting_info.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn relevant_history(&self) -> &[Reference] {
        self.relevant_history.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::device_request::DeviceRequestMutators for DeviceRequest {
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
    fn set_instantiates_canonical(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_canonical = Some(value);
        resource
    }
    fn add_instantiates_canonical(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates_canonical
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_instantiates_uri(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_uri = Some(value);
        resource
    }
    fn add_instantiates_uri(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates_uri
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_prior_request(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.prior_request = Some(value);
        resource
    }
    fn add_prior_request(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .prior_request
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_group_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.group_identifier = Some(value);
        resource
    }
    fn set_status(self, value: RequestStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_intent(self, value: RequestIntent) -> Self {
        let mut resource = self.clone();
        resource.intent = value;
        resource
    }
    fn set_priority(self, value: RequestPriority) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_parameter(self, value: Vec<DeviceRequestParameter>) -> Self {
        let mut resource = self.clone();
        resource.parameter = Some(value);
        resource
    }
    fn add_parameter(self, item: DeviceRequestParameter) -> Self {
        let mut resource = self.clone();
        resource.parameter.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
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
    fn set_performer_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.performer_type = Some(value);
        resource
    }
    fn set_performer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
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
    fn set_insurance(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.insurance = Some(value);
        resource
    }
    fn add_insurance(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.insurance.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_supporting_info(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_info = Some(value);
        resource
    }
    fn add_supporting_info(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_info
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = Some(value);
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_relevant_history(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.relevant_history = Some(value);
        resource
    }
    fn add_relevant_history(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .relevant_history
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::device_request::DeviceRequestExistence for DeviceRequest {
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
    fn has_code(&self) -> bool {
        true
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates_canonical(&self) -> bool {
        self.instantiates_canonical
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates_uri(&self) -> bool {
        self.instantiates_uri
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_prior_request(&self) -> bool {
        self.prior_request.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_group_identifier(&self) -> bool {
        self.group_identifier.is_some()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_intent(&self) -> bool {
        true
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_parameter(&self) -> bool {
        self.parameter.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_authored_on(&self) -> bool {
        self.authored_on.is_some()
    }
    fn has_requester(&self) -> bool {
        self.requester.is_some()
    }
    fn has_performer_type(&self) -> bool {
        self.performer_type.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.is_some()
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_insurance(&self) -> bool {
        self.insurance.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_supporting_info(&self) -> bool {
        self.supporting_info.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_relevant_history(&self) -> bool {
        self.relevant_history
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for DeviceRequest {
    fn resource_type(&self) -> &'static str {
        "DeviceRequest"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/DeviceRequest")
    }
}
