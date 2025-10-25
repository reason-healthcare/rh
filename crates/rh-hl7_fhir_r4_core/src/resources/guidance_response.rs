use crate::bindings::guidance_response_status::GuidanceResponseStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::data_requirement::DataRequirement;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// GuidanceResponse
///
/// A guidance response is the formal response to a guidance request, including any output parameters returned by the evaluation, as well as the description of any proposed actions to be taken.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/GuidanceResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: GuidanceResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuidanceResponse {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// The identifier of the request associated with this response, if any
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: Option<Identifier>,
    /// Business identifier
    pub identifier: Option<Vec<Identifier>>,
    /// What guidance was requested (uri)
    #[serde(rename = "moduleUri")]
    pub module_uri: StringType,
    /// What guidance was requested (canonical)
    #[serde(rename = "moduleCanonical")]
    pub module_canonical: StringType,
    /// What guidance was requested (CodeableConcept)
    #[serde(rename = "moduleCodeableConcept")]
    pub module_codeable_concept: CodeableConcept,
    /// success | data-requested | data-required | in-progress | failure | entered-in-error
    pub status: GuidanceResponseStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Patient the request was performed for
    pub subject: Option<Reference>,
    /// Encounter during which the response was returned
    pub encounter: Option<Reference>,
    /// When the guidance response was processed
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// Extension element for the 'occurrenceDateTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_occurrenceDateTime")]
    pub _occurrence_date_time: Option<Element>,
    /// Device returning the guidance
    pub performer: Option<Reference>,
    /// Why guidance is needed
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Why guidance is needed
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Additional notes about the response
    pub note: Option<Vec<Annotation>>,
    /// Messages resulting from the evaluation of the artifact or artifacts
    #[serde(rename = "evaluationMessage")]
    pub evaluation_message: Option<Vec<Reference>>,
    /// The output parameters of the evaluation, if any
    #[serde(rename = "outputParameters")]
    pub output_parameters: Option<Reference>,
    /// Proposed actions, if any
    pub result: Option<Reference>,
    /// Additional required data
    #[serde(rename = "dataRequirement")]
    pub data_requirement: Option<Vec<DataRequirement>>,
}

impl Default for GuidanceResponse {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            request_identifier: Default::default(),
            identifier: Default::default(),
            module_uri: Default::default(),
            module_canonical: Default::default(),
            module_codeable_concept: Default::default(),
            status: GuidanceResponseStatus::default(),
            _status: Default::default(),
            subject: Default::default(),
            encounter: Default::default(),
            occurrence_date_time: Default::default(),
            _occurrence_date_time: Default::default(),
            performer: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            note: Default::default(),
            evaluation_message: Default::default(),
            output_parameters: Default::default(),
            result: Default::default(),
            data_requirement: Default::default(),
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
        vec![rh_foundation::ElementBinding::new(
            "GuidanceResponse.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/guidance-response-status|4.0.1",
        )
        .with_description("The status of a guidance response.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("GuidanceResponse.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GuidanceResponse.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GuidanceResponse.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GuidanceResponse.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GuidanceResponse.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GuidanceResponse.contained", 0, None),
            rh_foundation::ElementCardinality::new("GuidanceResponse.extension", 0, None),
            rh_foundation::ElementCardinality::new("GuidanceResponse.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new(
                "GuidanceResponse.requestIdentifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("GuidanceResponse.identifier", 0, None),
            rh_foundation::ElementCardinality::new("GuidanceResponse.module[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("GuidanceResponse.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("GuidanceResponse.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GuidanceResponse.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "GuidanceResponse.occurrenceDateTime",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("GuidanceResponse.performer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GuidanceResponse.reasonCode", 0, None),
            rh_foundation::ElementCardinality::new("GuidanceResponse.reasonReference", 0, None),
            rh_foundation::ElementCardinality::new("GuidanceResponse.note", 0, None),
            rh_foundation::ElementCardinality::new("GuidanceResponse.evaluationMessage", 0, None),
            rh_foundation::ElementCardinality::new("GuidanceResponse.outputParameters", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GuidanceResponse.result", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GuidanceResponse.dataRequirement", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for GuidanceResponse {
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

impl crate::traits::resource::ResourceMutators for GuidanceResponse {
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

impl crate::traits::resource::ResourceExistence for GuidanceResponse {
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

impl crate::traits::domain_resource::DomainResourceAccessors for GuidanceResponse {
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

impl crate::traits::domain_resource::DomainResourceMutators for GuidanceResponse {
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

impl crate::traits::domain_resource::DomainResourceExistence for GuidanceResponse {
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

impl crate::traits::guidance_response::GuidanceResponseAccessors for GuidanceResponse {
    fn request_identifier(&self) -> Option<Identifier> {
        self.request_identifier.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> GuidanceResponseStatus {
        self.status.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn occurrence_date_time(&self) -> Option<DateTimeType> {
        self.occurrence_date_time.clone()
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
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn evaluation_message(&self) -> &[Reference] {
        self.evaluation_message.as_deref().unwrap_or(&[])
    }
    fn output_parameters(&self) -> Option<Reference> {
        self.output_parameters.clone()
    }
    fn result(&self) -> Option<Reference> {
        self.result.clone()
    }
    fn data_requirement(&self) -> &[DataRequirement] {
        self.data_requirement.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::guidance_response::GuidanceResponseMutators for GuidanceResponse {
    fn new() -> Self {
        Self::default()
    }
    fn set_request_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.request_identifier = Some(value);
        resource
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
    fn set_status(self, value: GuidanceResponseStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_occurrence_date_time(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.occurrence_date_time = Some(value);
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
    fn set_evaluation_message(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.evaluation_message = Some(value);
        resource
    }
    fn add_evaluation_message(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .evaluation_message
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_output_parameters(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.output_parameters = Some(value);
        resource
    }
    fn set_result(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.result = Some(value);
        resource
    }
    fn set_data_requirement(self, value: Vec<DataRequirement>) -> Self {
        let mut resource = self.clone();
        resource.data_requirement = Some(value);
        resource
    }
    fn add_data_requirement(self, item: DataRequirement) -> Self {
        let mut resource = self.clone();
        resource
            .data_requirement
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::guidance_response::GuidanceResponseExistence for GuidanceResponse {
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
    fn has_module(&self) -> bool {
        true
    }
    fn has_request_identifier(&self) -> bool {
        self.request_identifier.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_occurrence_date_time(&self) -> bool {
        self.occurrence_date_time.is_some()
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
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_evaluation_message(&self) -> bool {
        self.evaluation_message
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_output_parameters(&self) -> bool {
        self.output_parameters.is_some()
    }
    fn has_result(&self) -> bool {
        self.result.is_some()
    }
    fn has_data_requirement(&self) -> bool {
        self.data_requirement
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for GuidanceResponse {
    fn resource_type(&self) -> &'static str {
        "GuidanceResponse"
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
        Some("http://hl7.org/fhir/StructureDefinition/GuidanceResponse")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::guidance_response::{
    GuidanceResponseAccessors, GuidanceResponseExistence, GuidanceResponseMutators,
};
