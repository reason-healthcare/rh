use crate::bindings::verificationresult_status::VerificationresultStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::reference::Reference;
use crate::datatypes::signature::Signature;
use crate::datatypes::timing::Timing;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// VerificationResult
///
/// Describes validation requirements, source(s), status and dates for one or more elements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/VerificationResult
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: VerificationResult
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// A resource that was validated
    pub target: Option<Vec<Reference>>,
    /// The fhirpath location(s) within the resource that was validated
    #[serde(rename = "targetLocation")]
    pub target_location: Option<Vec<StringType>>,
    /// Extension element for the 'targetLocation' primitive field. Contains metadata and extensions.
    #[serde(rename = "_targetLocation")]
    pub _target_location: Option<Element>,
    /// none | initial | periodic
    ///
    /// Binding: preferred (The frequency with which the target must be validated.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/verificationresult-need
    pub need: Option<CodeableConcept>,
    /// attested | validated | in-process | req-revalid | val-fail | reval-fail
    pub status: VerificationresultStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// When the validation status was updated
    #[serde(rename = "statusDate")]
    pub status_date: Option<DateTimeType>,
    /// Extension element for the 'statusDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_statusDate")]
    pub _status_date: Option<Element>,
    /// nothing | primary | multiple
    ///
    /// Binding: preferred (What the target is validated against.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/verificationresult-validation-type
    #[serde(rename = "validationType")]
    pub validation_type: Option<CodeableConcept>,
    /// The primary process by which the target is validated (edit check; value set; primary source; multiple sources; standalone; in context)
    ///
    /// Binding: example (The primary process by which the target is validated.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/verificationresult-validation-process
    #[serde(rename = "validationProcess")]
    pub validation_process: Option<Vec<CodeableConcept>>,
    /// Frequency of revalidation
    pub frequency: Option<Timing>,
    /// The date/time validation was last completed (including failed validations)
    #[serde(rename = "lastPerformed")]
    pub last_performed: Option<DateTimeType>,
    /// Extension element for the 'lastPerformed' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastPerformed")]
    pub _last_performed: Option<Element>,
    /// The date when target is next validated, if appropriate
    #[serde(rename = "nextScheduled")]
    pub next_scheduled: Option<DateType>,
    /// Extension element for the 'nextScheduled' primitive field. Contains metadata and extensions.
    #[serde(rename = "_nextScheduled")]
    pub _next_scheduled: Option<Element>,
    /// fatal | warn | rec-only | none
    ///
    /// Binding: preferred (The result if validation fails.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/verificationresult-failure-action
    #[serde(rename = "failureAction")]
    pub failure_action: Option<CodeableConcept>,
    /// Information about the primary source(s) involved in validation
    #[serde(rename = "primarySource")]
    pub primary_source: Option<Vec<VerificationResultPrimarysource>>,
    /// Information about the entity attesting to information
    pub attestation: Option<VerificationResultAttestation>,
    /// Information about the entity validating information
    pub validator: Option<Vec<VerificationResultValidator>>,
}
/// VerificationResult nested structure for the 'primarySource' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResultPrimarysource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reference to the primary source
    pub who: Option<Reference>,
    /// Type of primary source (License Board; Primary Education; Continuing Education; Postal Service; Relationship owner; Registration Authority; legal source; issuing source; authoritative source)
    ///
    /// Binding: example (Type of the validation primary source.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/verificationresult-primary-source-type
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Method for exchanging information with the primary source
    ///
    /// Binding: example (Method for communicating with the data source (manual; API; Push).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/verificationresult-communication-method
    #[serde(rename = "communicationMethod")]
    pub communication_method: Option<Vec<CodeableConcept>>,
    /// successful | failed | unknown
    ///
    /// Binding: preferred (Status of the validation of the target against the primary source.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/verificationresult-validation-status
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<CodeableConcept>,
    /// When the target was validated against the primary source
    #[serde(rename = "validationDate")]
    pub validation_date: Option<DateTimeType>,
    /// Extension element for the 'validationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_validationDate")]
    pub _validation_date: Option<Element>,
    /// yes | no | undetermined
    ///
    /// Binding: preferred (Ability of the primary source to push updates/alerts.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/verificationresult-can-push-updates
    #[serde(rename = "canPushUpdates")]
    pub can_push_updates: Option<CodeableConcept>,
    /// specific | any | source
    ///
    /// Binding: preferred (Type of alerts/updates the primary source can send.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/verificationresult-push-type-available
    #[serde(rename = "pushTypeAvailable")]
    pub push_type_available: Option<Vec<CodeableConcept>>,
}
/// VerificationResult nested structure for the 'attestation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResultAttestation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The individual or organization attesting to information
    pub who: Option<Reference>,
    /// When the who is asserting on behalf of another (organization or individual)
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Reference>,
    /// The method by which attested information was submitted/retrieved
    ///
    /// Binding: example (Method for communicating with the data source (manual; API; Push).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/verificationresult-communication-method
    #[serde(rename = "communicationMethod")]
    pub communication_method: Option<CodeableConcept>,
    /// The date the information was attested to
    pub date: Option<DateType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// A digital identity certificate associated with the attestation source
    #[serde(rename = "sourceIdentityCertificate")]
    pub source_identity_certificate: Option<StringType>,
    /// Extension element for the 'sourceIdentityCertificate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_sourceIdentityCertificate")]
    pub _source_identity_certificate: Option<Element>,
    /// A digital identity certificate associated with the proxy entity submitting attested information on behalf of the attestation source
    #[serde(rename = "proxyIdentityCertificate")]
    pub proxy_identity_certificate: Option<StringType>,
    /// Extension element for the 'proxyIdentityCertificate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_proxyIdentityCertificate")]
    pub _proxy_identity_certificate: Option<Element>,
    /// Proxy signature
    #[serde(rename = "proxySignature")]
    pub proxy_signature: Option<Signature>,
    /// Attester signature
    #[serde(rename = "sourceSignature")]
    pub source_signature: Option<Signature>,
}
/// VerificationResult nested structure for the 'validator' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResultValidator {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reference to the organization validating information
    pub organization: Reference,
    /// A digital identity certificate associated with the validator
    #[serde(rename = "identityCertificate")]
    pub identity_certificate: Option<StringType>,
    /// Extension element for the 'identityCertificate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_identityCertificate")]
    pub _identity_certificate: Option<Element>,
    /// Validator signature
    #[serde(rename = "attestationSignature")]
    pub attestation_signature: Option<Signature>,
}

impl Default for VerificationResult {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            target: Default::default(),
            target_location: Default::default(),
            _target_location: Default::default(),
            need: Default::default(),
            status: VerificationresultStatus::default(),
            _status: Default::default(),
            status_date: Default::default(),
            _status_date: Default::default(),
            validation_type: Default::default(),
            validation_process: Default::default(),
            frequency: Default::default(),
            last_performed: Default::default(),
            _last_performed: Default::default(),
            next_scheduled: Default::default(),
            _next_scheduled: Default::default(),
            failure_action: Default::default(),
            primary_source: Default::default(),
            attestation: Default::default(),
            validator: Default::default(),
        }
    }
}

impl Default for VerificationResultPrimarysource {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            who: Default::default(),
            type_: Default::default(),
            communication_method: Default::default(),
            validation_status: Default::default(),
            validation_date: Default::default(),
            _validation_date: Default::default(),
            can_push_updates: Default::default(),
            push_type_available: Default::default(),
        }
    }
}

impl Default for VerificationResultAttestation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            who: Default::default(),
            on_behalf_of: Default::default(),
            communication_method: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            source_identity_certificate: Default::default(),
            _source_identity_certificate: Default::default(),
            proxy_identity_certificate: Default::default(),
            _proxy_identity_certificate: Default::default(),
            proxy_signature: Default::default(),
            source_signature: Default::default(),
        }
    }
}

impl Default for VerificationResultValidator {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            organization: Reference::default(),
            identity_certificate: Default::default(),
            _identity_certificate: Default::default(),
            attestation_signature: Default::default(),
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
impl crate::traits::resource::ResourceAccessors for VerificationResult {
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

impl crate::traits::resource::ResourceMutators for VerificationResult {
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

impl crate::traits::resource::ResourceExistence for VerificationResult {
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

impl crate::traits::domain_resource::DomainResourceAccessors for VerificationResult {
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

impl crate::traits::domain_resource::DomainResourceMutators for VerificationResult {
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

impl crate::traits::domain_resource::DomainResourceExistence for VerificationResult {
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

impl crate::traits::verification_result::VerificationResultAccessors for VerificationResult {
    fn target(&self) -> &[Reference] {
        self.target.as_deref().unwrap_or(&[])
    }
    fn target_location(&self) -> &[StringType] {
        self.target_location.as_deref().unwrap_or(&[])
    }
    fn need(&self) -> Option<CodeableConcept> {
        self.need.clone()
    }
    fn status(&self) -> VerificationresultStatus {
        self.status.clone()
    }
    fn status_date(&self) -> Option<DateTimeType> {
        self.status_date.clone()
    }
    fn validation_type(&self) -> Option<CodeableConcept> {
        self.validation_type.clone()
    }
    fn validation_process(&self) -> &[CodeableConcept] {
        self.validation_process.as_deref().unwrap_or(&[])
    }
    fn frequency(&self) -> Option<Timing> {
        self.frequency.clone()
    }
    fn last_performed(&self) -> Option<DateTimeType> {
        self.last_performed.clone()
    }
    fn next_scheduled(&self) -> Option<DateType> {
        self.next_scheduled.clone()
    }
    fn failure_action(&self) -> Option<CodeableConcept> {
        self.failure_action.clone()
    }
    fn primary_source(&self) -> &[VerificationResultPrimarysource] {
        self.primary_source.as_deref().unwrap_or(&[])
    }
    fn attestation(&self) -> Option<VerificationResultAttestation> {
        self.attestation.clone()
    }
    fn validator(&self) -> &[VerificationResultValidator] {
        self.validator.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::verification_result::VerificationResultMutators for VerificationResult {
    fn new() -> Self {
        Self::default()
    }
    fn set_target(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.target = Some(value);
        resource
    }
    fn add_target(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.target.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_target_location(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.target_location = Some(value);
        resource
    }
    fn add_target_location(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .target_location
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_need(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.need = Some(value);
        resource
    }
    fn set_status(self, value: VerificationresultStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.status_date = Some(value);
        resource
    }
    fn set_validation_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.validation_type = Some(value);
        resource
    }
    fn set_validation_process(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.validation_process = Some(value);
        resource
    }
    fn add_validation_process(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .validation_process
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_frequency(self, value: Timing) -> Self {
        let mut resource = self.clone();
        resource.frequency = Some(value);
        resource
    }
    fn set_last_performed(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.last_performed = Some(value);
        resource
    }
    fn set_next_scheduled(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.next_scheduled = Some(value);
        resource
    }
    fn set_failure_action(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.failure_action = Some(value);
        resource
    }
    fn set_primary_source(self, value: Vec<VerificationResultPrimarysource>) -> Self {
        let mut resource = self.clone();
        resource.primary_source = Some(value);
        resource
    }
    fn add_primary_source(self, item: VerificationResultPrimarysource) -> Self {
        let mut resource = self.clone();
        resource
            .primary_source
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_attestation(self, value: VerificationResultAttestation) -> Self {
        let mut resource = self.clone();
        resource.attestation = Some(value);
        resource
    }
    fn set_validator(self, value: Vec<VerificationResultValidator>) -> Self {
        let mut resource = self.clone();
        resource.validator = Some(value);
        resource
    }
    fn add_validator(self, item: VerificationResultValidator) -> Self {
        let mut resource = self.clone();
        resource.validator.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::verification_result::VerificationResultExistence for VerificationResult {
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
    fn has_target(&self) -> bool {
        self.target.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_target_location(&self) -> bool {
        self.target_location.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_need(&self) -> bool {
        self.need.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_status_date(&self) -> bool {
        self.status_date.is_some()
    }
    fn has_validation_type(&self) -> bool {
        self.validation_type.is_some()
    }
    fn has_validation_process(&self) -> bool {
        self.validation_process
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_frequency(&self) -> bool {
        self.frequency.is_some()
    }
    fn has_last_performed(&self) -> bool {
        self.last_performed.is_some()
    }
    fn has_next_scheduled(&self) -> bool {
        self.next_scheduled.is_some()
    }
    fn has_failure_action(&self) -> bool {
        self.failure_action.is_some()
    }
    fn has_primary_source(&self) -> bool {
        self.primary_source.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_attestation(&self) -> bool {
        self.attestation.is_some()
    }
    fn has_validator(&self) -> bool {
        self.validator.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for VerificationResult {
    fn resource_type(&self) -> &'static str {
        "VerificationResult"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/VerificationResult")
    }
}
