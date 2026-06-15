use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_priority::RequestPriority;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ServiceRequest
///
/// A record of a request for service such as diagnostic investigations, treatments, or operations to be performed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ServiceRequest
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ServiceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifiers assigned to this order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical: Vec<StringType>,
    /// Extension element for the 'instantiatesCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _instantiates_canonical: Vec<Element>,
    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri: Vec<StringType>,
    /// Extension element for the 'instantiatesUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _instantiates_uri: Vec<Element>,
    /// What request fulfills
    #[serde(rename = "basedOn")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<Reference>,
    /// What request replaces
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<Reference>,
    /// Composite Request ID
    pub requisition: Option<Identifier>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: RequestStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// proposal | plan | directive | order +
    pub intent: RequestIntent,
    /// Extension element for the 'intent' primitive field. Contains metadata and extensions.
    pub _intent: Option<Element>,
    /// Classification of service
    ///
    /// Binding: example (Classification of the requested service.)
    ///
    /// Available values:
    /// - `108252007`: Laboratory procedure
    /// - `363679005`: Imaging
    /// - `409063005`: Counselling
    /// - `409073007`: Education
    /// - `387713003`: Surgical procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<CodeableConcept>,
    /// routine | urgent | asap | stat
    pub priority: Option<RequestPriority>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// True if service/procedure should not be performed
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<BooleanType>,
    /// Extension element for the 'doNotPerform' primitive field. Contains metadata and extensions.
    #[serde(rename = "_doNotPerform")]
    pub _do_not_perform: Option<Element>,
    /// What is being requested/ordered
    ///
    /// Binding: example (Codes for tests or services that can be carried out by a designated individual, organization or healthcare service.  For laboratory, LOINC is  [preferred](terminologies.html#preferred) and a valueset using LOINC Order codes is available [here](valueset-diagnostic-requests.html).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-code
    pub code: Option<CodeableReference>,
    /// Additional order information
    #[serde(rename = "orderDetail")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_detail: Vec<ServiceRequestOrderdetail>,
    /// Service amount (Quantity)
    #[serde(rename = "quantityQuantity")]
    pub quantity_quantity: Option<Quantity>,
    /// Service amount (Ratio)
    #[serde(rename = "quantityRatio")]
    pub quantity_ratio: Option<Ratio>,
    /// Service amount (Range)
    #[serde(rename = "quantityRange")]
    pub quantity_range: Option<Range>,
    /// Individual or Entity the service is ordered for
    pub subject: Reference,
    /// What the service request is about, when it is not about the subject of record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<Reference>,
    /// Encounter in which the request was created
    pub encounter: Option<Reference>,
    /// When service should occur (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// When service should occur (Period)
    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,
    /// When service should occur (Timing)
    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,
    /// Preconditions for service (boolean)
    #[serde(rename = "asNeededBoolean")]
    pub as_needed_boolean: Option<BooleanType>,
    /// Preconditions for service (CodeableConcept)
    #[serde(rename = "asNeededCodeableConcept")]
    pub as_needed_codeable_concept: Option<CodeableConcept>,
    /// Date request signed
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<DateTimeType>,
    /// Extension element for the 'authoredOn' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authoredOn")]
    pub _authored_on: Option<Element>,
    /// Who/what is requesting service
    pub requester: Option<Reference>,
    /// Performer role
    ///
    /// Binding: example (Indicates specific responsibility of an individual within the care team, such as "Primary physician", "Team coordinator", "Caregiver", etc.)
    ///
    /// Available values:
    /// - `429577009`
    /// - `116154003`
    /// - `133932002`
    #[serde(rename = "performerType")]
    pub performer_type: Option<CodeableConcept>,
    /// Requested performer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<Reference>,
    /// Requested location
    ///
    /// Binding: example (A location type where services are delivered.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ServiceDeliveryLocationRoleType
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<CodeableReference>,
    /// Explanation/Justification for procedure or service
    ///
    /// Binding: example (SNOMED CT Condition/Problem/Diagnosis Codes)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-reason
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<CodeableReference>,
    /// Associated insurance coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<Reference>,
    /// Additional clinical information
    #[serde(rename = "supportingInfo")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<CodeableReference>,
    /// Procedure Samples
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<Reference>,
    /// Coded location on Body
    ///
    /// Binding: example (SNOMED CT Body site concepts)
    ///
    /// Available values:
    /// - `53075003`: Distal phalanx of hallux
    /// - `76986006`: Distal phalanx of second toe
    /// - `65258003`: Distal phalanx of third toe
    /// - `54333003`: Distal phalanx of fourth toe
    /// - `10770001`: Distal phalanx of fifth toe
    /// - `363670009`: Interphalangeal joint structure of great toe
    /// - `371216008`: Distal interphalangeal joint of second toe
    /// - `371219001`: Distal interphalangeal joint of third toe
    /// - `371205001`: Distal interphalangeal joint of fourth toe
    /// - `371203008`: Distal interphalangeal joint of fifth toe
    /// - ... and 30 more values
    #[serde(rename = "bodySite")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<CodeableConcept>,
    /// BodyStructure-based location on the body
    #[serde(rename = "bodyStructure")]
    pub body_structure: Option<Reference>,
    /// Comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// Patient or consumer-oriented instructions
    #[serde(rename = "patientInstruction")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patient_instruction: Vec<ServiceRequestPatientinstruction>,
    /// Request provenance
    #[serde(rename = "relevantHistory")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relevant_history: Vec<Reference>,
}
/// ServiceRequest nested structure for the 'patientInstruction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequestPatientinstruction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Patient or consumer-oriented instructions (markdown)
    #[serde(rename = "instructionMarkdown")]
    pub instruction_markdown: Option<StringType>,
    /// Patient or consumer-oriented instructions (Reference)
    #[serde(rename = "instructionReference")]
    pub instruction_reference: Option<Reference>,
}
/// ServiceRequest nested structure for the 'orderDetail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequestOrderdetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The parameter details for the service being requested
    pub parameter: Vec<ServiceRequestOrderdetailParameter>,
    /// The context of the order details by reference
    #[serde(rename = "parameterFocus")]
    pub parameter_focus: Option<CodeableReference>,
}
/// ServiceRequestOrderdetail nested structure for the 'parameter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequestOrderdetailParameter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The detail of the order being requested
    ///
    /// Binding: example (Codes for order detail parameters.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/servicerequest-orderdetail-parameter-code
    pub code: CodeableConcept,
    /// The value for the order detail (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// The value for the order detail (Ratio)
    #[serde(rename = "valueRatio")]
    pub value_ratio: Ratio,
    /// The value for the order detail (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// The value for the order detail (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// The value for the order detail (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// The value for the order detail (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// The value for the order detail (Period)
    #[serde(rename = "valuePeriod")]
    pub value_period: Period,
}

impl Default for ServiceRequest {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            based_on: Default::default(),
            replaces: Default::default(),
            requisition: Default::default(),
            status: RequestStatus::default(),
            _status: Default::default(),
            intent: RequestIntent::default(),
            _intent: Default::default(),
            category: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            do_not_perform: Default::default(),
            _do_not_perform: Default::default(),
            code: Default::default(),
            order_detail: Default::default(),
            quantity_quantity: Default::default(),
            quantity_ratio: Default::default(),
            quantity_range: Default::default(),
            subject: Reference::default(),
            focus: Default::default(),
            encounter: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            occurrence_timing: Default::default(),
            as_needed_boolean: Default::default(),
            as_needed_codeable_concept: Default::default(),
            authored_on: Default::default(),
            _authored_on: Default::default(),
            requester: Default::default(),
            performer_type: Default::default(),
            performer: Default::default(),
            location: Default::default(),
            reason: Default::default(),
            insurance: Default::default(),
            supporting_info: Default::default(),
            specimen: Default::default(),
            body_site: Default::default(),
            body_structure: Default::default(),
            note: Default::default(),
            patient_instruction: Default::default(),
            relevant_history: Default::default(),
        }
    }
}

impl Default for ServiceRequestPatientinstruction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            instruction_markdown: Default::default(),
            instruction_reference: Default::default(),
        }
    }
}

impl Default for ServiceRequestOrderdetail {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            parameter: Default::default(),
            parameter_focus: Default::default(),
        }
    }
}

impl Default for ServiceRequestOrderdetailParameter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            value_quantity: Default::default(),
            value_ratio: Default::default(),
            value_range: Default::default(),
            value_boolean: Default::default(),
            value_codeable_concept: Default::default(),
            value_string: Default::default(),
            value_period: Default::default(),
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
    rh_foundation::Invariant::new("bdystr-1", rh_foundation::Severity::Error, "bodyStructure SHALL only be present if bodySite is not present", "bodySite.exists() implies bodyStructure.empty()"),
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
    rh_foundation::Invariant::new("prr-1", rh_foundation::Severity::Error, "orderDetail SHALL only be present if code is present", "orderDetail.empty() or code.exists()"),
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
                "http://hl7.org/fhir/ValueSet/request-intent|5.0.0",
            )
            .with_description("The kind of service request."),
            rh_foundation::ElementBinding::new(
                "ServiceRequest.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ServiceRequest.priority",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/request-priority|5.0.0",
            )
            .with_description(
                "Identifies the level of importance to be assigned to actioning the request.",
            ),
            rh_foundation::ElementBinding::new(
                "ServiceRequest.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/request-status|5.0.0",
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
            rh_foundation::ElementCardinality::new("ServiceRequest.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.contained", 0, None),
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
            rh_foundation::ElementCardinality::new("ServiceRequest.orderDetail.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.orderDetail.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ServiceRequest.orderDetail.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ServiceRequest.orderDetail.parameterFocus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ServiceRequest.orderDetail.parameter", 1, None),
            rh_foundation::ElementCardinality::new(
                "ServiceRequest.orderDetail.parameter.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ServiceRequest.orderDetail.parameter.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ServiceRequest.orderDetail.parameter.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ServiceRequest.orderDetail.parameter.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ServiceRequest.orderDetail.parameter.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ServiceRequest.quantity[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.focus", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.occurrence[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.asNeeded[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.authoredOn", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.requester", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.performerType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.performer", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.location", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.reason", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.insurance", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.supportingInfo", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.specimen", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.bodySite", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.bodyStructure", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ServiceRequest.note", 0, None),
            rh_foundation::ElementCardinality::new("ServiceRequest.patientInstruction", 0, None),
            rh_foundation::ElementCardinality::new(
                "ServiceRequest.patientInstruction.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ServiceRequest.patientInstruction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ServiceRequest.patientInstruction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ServiceRequest.patientInstruction.instruction[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ServiceRequest.relevantHistory", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ServiceRequest {
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

impl crate::traits::resource::ResourceMutators for ServiceRequest {
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

impl crate::traits::resource::ResourceExistence for ServiceRequest {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ServiceRequest {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for ServiceRequest {
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for ServiceRequest {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::service_request::ServiceRequestAccessors for ServiceRequest {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn instantiates_canonical(&self) -> &[StringType] {
        self.instantiates_canonical.as_slice()
    }
    fn instantiates_uri(&self) -> &[StringType] {
        self.instantiates_uri.as_slice()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_slice()
    }
    fn replaces(&self) -> &[Reference] {
        self.replaces.as_slice()
    }
    fn requisition(&self) -> Option<Identifier> {
        self.requisition.clone()
    }
    fn status(&self) -> RequestStatus {
        self.status.clone()
    }
    fn intent(&self) -> RequestIntent {
        self.intent.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_slice()
    }
    fn priority(&self) -> Option<RequestPriority> {
        self.priority.clone()
    }
    fn do_not_perform(&self) -> Option<BooleanType> {
        self.do_not_perform
    }
    fn code(&self) -> Option<CodeableReference> {
        self.code.clone()
    }
    fn order_detail(&self) -> &[ServiceRequestOrderdetail] {
        self.order_detail.as_slice()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn focus(&self) -> &[Reference] {
        self.focus.as_slice()
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
    fn performer(&self) -> &[Reference] {
        self.performer.as_slice()
    }
    fn location(&self) -> &[CodeableReference] {
        self.location.as_slice()
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_slice()
    }
    fn insurance(&self) -> &[Reference] {
        self.insurance.as_slice()
    }
    fn supporting_info(&self) -> &[CodeableReference] {
        self.supporting_info.as_slice()
    }
    fn specimen(&self) -> &[Reference] {
        self.specimen.as_slice()
    }
    fn body_site(&self) -> &[CodeableConcept] {
        self.body_site.as_slice()
    }
    fn body_structure(&self) -> Option<Reference> {
        self.body_structure.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
    fn patient_instruction(&self) -> &[ServiceRequestPatientinstruction] {
        self.patient_instruction.as_slice()
    }
    fn relevant_history(&self) -> &[Reference] {
        self.relevant_history.as_slice()
    }
}

impl crate::traits::service_request::ServiceRequestMutators for ServiceRequest {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
        resource
    }
    fn set_instantiates_canonical(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_canonical = value;
        resource
    }
    fn add_instantiates_canonical(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.instantiates_canonical.push(item);
        resource
    }
    fn set_instantiates_uri(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_uri = value;
        resource
    }
    fn add_instantiates_uri(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.instantiates_uri.push(item);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = value;
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.push(item);
        resource
    }
    fn set_replaces(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.replaces = value;
        resource
    }
    fn add_replaces(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.replaces.push(item);
        resource
    }
    fn set_requisition(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.requisition = Some(value);
        resource
    }
    fn set_status(self, value: RequestStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_intent(self, value: RequestIntent) -> Self {
        let mut resource = self.clone();
        resource.intent = value;
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = value;
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.push(item);
        resource
    }
    fn set_priority(self, value: RequestPriority) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_do_not_perform(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.do_not_perform = Some(value);
        resource
    }
    fn set_code(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_order_detail(self, value: Vec<ServiceRequestOrderdetail>) -> Self {
        let mut resource = self.clone();
        resource.order_detail = value;
        resource
    }
    fn add_order_detail(self, item: ServiceRequestOrderdetail) -> Self {
        let mut resource = self.clone();
        resource.order_detail.push(item);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_focus(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.focus = value;
        resource
    }
    fn add_focus(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.focus.push(item);
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
    fn set_performer(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.performer = value;
        resource
    }
    fn add_performer(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.performer.push(item);
        resource
    }
    fn set_location(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.location = value;
        resource
    }
    fn add_location(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.location.push(item);
        resource
    }
    fn set_reason(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.reason = value;
        resource
    }
    fn add_reason(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.reason.push(item);
        resource
    }
    fn set_insurance(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.insurance = value;
        resource
    }
    fn add_insurance(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.insurance.push(item);
        resource
    }
    fn set_supporting_info(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_info = value;
        resource
    }
    fn add_supporting_info(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.supporting_info.push(item);
        resource
    }
    fn set_specimen(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.specimen = value;
        resource
    }
    fn add_specimen(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.specimen.push(item);
        resource
    }
    fn set_body_site(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.body_site = value;
        resource
    }
    fn add_body_site(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.body_site.push(item);
        resource
    }
    fn set_body_structure(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.body_structure = Some(value);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = value;
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.push(item);
        resource
    }
    fn set_patient_instruction(self, value: Vec<ServiceRequestPatientinstruction>) -> Self {
        let mut resource = self.clone();
        resource.patient_instruction = value;
        resource
    }
    fn add_patient_instruction(self, item: ServiceRequestPatientinstruction) -> Self {
        let mut resource = self.clone();
        resource.patient_instruction.push(item);
        resource
    }
    fn set_relevant_history(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.relevant_history = value;
        resource
    }
    fn add_relevant_history(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.relevant_history.push(item);
        resource
    }
}

impl crate::traits::service_request::ServiceRequestExistence for ServiceRequest {
    fn has_as_needed(&self) -> bool {
        self.as_needed_boolean.is_some() || self.as_needed_codeable_concept.is_some()
    }
    fn has_quantity(&self) -> bool {
        self.quantity_quantity.is_some()
            || self.quantity_ratio.is_some()
            || self.quantity_range.is_some()
    }
    fn has_occurrence(&self) -> bool {
        self.occurrence_date_time.is_some()
            || self.occurrence_period.is_some()
            || self.occurrence_timing.is_some()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_instantiates_canonical(&self) -> bool {
        !self.instantiates_canonical.is_empty()
    }
    fn has_instantiates_uri(&self) -> bool {
        !self.instantiates_uri.is_empty()
    }
    fn has_based_on(&self) -> bool {
        !self.based_on.is_empty()
    }
    fn has_replaces(&self) -> bool {
        !self.replaces.is_empty()
    }
    fn has_requisition(&self) -> bool {
        self.requisition.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_intent(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        !self.category.is_empty()
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_do_not_perform(&self) -> bool {
        self.do_not_perform.is_some()
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_order_detail(&self) -> bool {
        !self.order_detail.is_empty()
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_focus(&self) -> bool {
        !self.focus.is_empty()
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
        !self.performer.is_empty()
    }
    fn has_location(&self) -> bool {
        !self.location.is_empty()
    }
    fn has_reason(&self) -> bool {
        !self.reason.is_empty()
    }
    fn has_insurance(&self) -> bool {
        !self.insurance.is_empty()
    }
    fn has_supporting_info(&self) -> bool {
        !self.supporting_info.is_empty()
    }
    fn has_specimen(&self) -> bool {
        !self.specimen.is_empty()
    }
    fn has_body_site(&self) -> bool {
        !self.body_site.is_empty()
    }
    fn has_body_structure(&self) -> bool {
        self.body_structure.is_some()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
    }
    fn has_patient_instruction(&self) -> bool {
        !self.patient_instruction.is_empty()
    }
    fn has_relevant_history(&self) -> bool {
        !self.relevant_history.is_empty()
    }
}

impl crate::validation::ValidatableResource for ServiceRequest {
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
        Some("http://hl7.org/fhir/StructureDefinition/ServiceRequest")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::service_request::{
    ServiceRequestAccessors, ServiceRequestExistence, ServiceRequestMutators,
};
