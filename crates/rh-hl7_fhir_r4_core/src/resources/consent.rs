use crate::bindings::consent_data_meaning::ConsentDataMeaning;
use crate::bindings::consent_provision_type::ConsentProvisionType;
use crate::bindings::consent_state_codes::ConsentStateCodes;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Consent
///
/// A record of a healthcare consumer’s  choices, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Consent
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Consent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifier for this record (external references)
    pub identifier: Option<Vec<Identifier>>,
    /// draft | proposed | active | rejected | inactive | entered-in-error
    pub status: ConsentStateCodes,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Which of the four areas this resource covers (extensible)
    ///
    /// Binding: extensible (The four anticipated uses for the Consent Resource.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/consent-scope
    pub scope: CodeableConcept,
    /// Classification of the consent statement - for indexing/retrieval
    ///
    /// Binding: extensible (A classification of the type of consents found in a consent statement.)
    ///
    /// Available values:
    /// - `59284-0`: Patient Consent
    /// - `57016-8`: Privacy policy acknowledgement Document
    /// - `57017-6`: Privacy policy Organization Document
    /// - `64292-6`: Release of information consent
    pub category: Vec<CodeableConcept>,
    /// Who the consent applies to
    pub patient: Option<Reference>,
    /// When this Consent was created or indexed
    #[serde(rename = "dateTime")]
    pub date_time: Option<DateTimeType>,
    /// Extension element for the 'dateTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dateTime")]
    pub _date_time: Option<Element>,
    /// Who is agreeing to the policy and rules
    pub performer: Option<Vec<Reference>>,
    /// Custodian of the consent
    pub organization: Option<Vec<Reference>>,
    /// Source from which this consent is taken (Attachment)
    #[serde(rename = "sourceAttachment")]
    pub source_attachment: Option<Attachment>,
    /// Source from which this consent is taken (Reference)
    #[serde(rename = "sourceReference")]
    pub source_reference: Option<Reference>,
    /// Policies covered by this consent
    pub policy: Option<Vec<ConsentPolicy>>,
    /// Regulation that this consents to
    ///
    /// Binding: extensible (Regulatory policy examples.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/consent-policy
    #[serde(rename = "policyRule")]
    pub policy_rule: Option<CodeableConcept>,
    /// Consent Verified by patient or family
    pub verification: Option<Vec<ConsentVerification>>,
    /// Constraints to the base Consent.policyRule
    pub provision: Option<ConsentProvision>,
}
/// Consent nested structure for the 'provision' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentProvision {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Data controlled by this rule
    pub data: Option<Vec<ConsentProvisionData>>,
    /// Who|what controlled by this rule (or group, by role)
    pub actor: Option<Vec<ConsentProvisionActor>>,
    /// deny | permit
    #[serde(rename = "type")]
    pub type_: Option<ConsentProvisionType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Timeframe for this rule
    pub period: Option<Period>,
    /// Actions controlled by this rule
    ///
    /// Binding: example (Detailed codes for the consent action.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/consent-action
    pub action: Option<Vec<CodeableConcept>>,
    /// Security Labels that define affected resources
    ///
    /// Binding: extensible (Security Labels from the Healthcare Privacy and Security Classification System.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/security-labels
    #[serde(rename = "securityLabel")]
    pub security_label: Option<Vec<Coding>>,
    /// Context of activities covered by this rule
    ///
    /// Binding: extensible (What purposes of use are controlled by this exception. If more than one label is specified, operations must have all the specified labels.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-PurposeOfUse
    pub purpose: Option<Vec<Coding>>,
    /// e.g. Resource Type, Profile, CDA, etc.
    ///
    /// Binding: extensible (The class (type) of information a consent rule covers.)
    ///
    /// Available values:
    /// - `http://hl7.org/fhir/StructureDefinition/lipidprofile`: Lipid Lab Report
    /// - `application/hl7-cda+xml`: CDA Documents
    pub class: Option<Vec<Coding>>,
    /// e.g. LOINC or SNOMED CT code, etc. in the content
    ///
    /// Binding: example (If this code is found in an instance, then the exception applies.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/consent-content-code
    pub code: Option<Vec<CodeableConcept>>,
    /// Timeframe for data controlled by this rule
    #[serde(rename = "dataPeriod")]
    pub data_period: Option<Period>,
    /// Nested Exception Rules
    pub provision: Option<Vec<StringType>>,
}
/// Location of Access restriction
///
/// Restricts this exception to only apply a specific location as defined.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/consent-location
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentLocation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// ConsentProvision nested structure for the 'actor' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentProvisionActor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// How the actor is involved
    ///
    /// Binding: extensible (How an actor is involved in the consent considerations.)
    ///
    /// Available values:
    /// - `AMENDER`
    /// - `COAUTH`
    /// - `CONT`
    /// - `EVTWIT`
    /// - `PRIMAUTH`
    /// - `REVIEWER`
    /// - `SOURCE`
    /// - `TRANS`
    /// - `VALID`
    /// - `VERF`
    /// - ... and 53 more values
    pub role: CodeableConcept,
    /// Resource for the actor (or group, by role)
    pub reference: Reference,
}
/// Consent nested structure for the 'verification' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentVerification {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Has been verified
    pub verified: BooleanType,
    /// Extension element for the 'verified' primitive field. Contains metadata and extensions.
    pub _verified: Option<Element>,
    /// Person who verified
    #[serde(rename = "verifiedWith")]
    pub verified_with: Option<Reference>,
    /// When consent verified
    #[serde(rename = "verificationDate")]
    pub verification_date: Option<DateTimeType>,
    /// Extension element for the 'verificationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_verificationDate")]
    pub _verification_date: Option<Element>,
}
/// Disclosure Notification Endpoint
///
/// Endpoint for sending Disclosure notifications in the form of FHIR AuditEvent records.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/consent-NotificationEndpoint
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentNotificationEndpoint {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Consent nested structure for the 'policy' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentPolicy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Enforcement source for policy
    pub authority: Option<StringType>,
    /// Extension element for the 'authority' primitive field. Contains metadata and extensions.
    pub _authority: Option<Element>,
    /// Specific policy covered by this consent
    pub uri: Option<StringType>,
    /// Extension element for the 'uri' primitive field. Contains metadata and extensions.
    pub _uri: Option<Element>,
}
/// Transcriber
///
/// Any person/thing who transcribed the consent into the system.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/consent-Transcriber
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentTranscriber {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// ConsentProvision nested structure for the 'data' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentProvisionData {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// instance | related | dependents | authoredby
    pub meaning: ConsentDataMeaning,
    /// Extension element for the 'meaning' primitive field. Contains metadata and extensions.
    pub _meaning: Option<Element>,
    /// The actual data reference
    pub reference: Reference,
}

impl Default for Consent {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: ConsentStateCodes::default(),
            _status: Default::default(),
            scope: CodeableConcept::default(),
            category: Vec::new(),
            patient: Default::default(),
            date_time: Default::default(),
            _date_time: Default::default(),
            performer: Default::default(),
            organization: Default::default(),
            source_attachment: Default::default(),
            source_reference: Default::default(),
            policy: Default::default(),
            policy_rule: Default::default(),
            verification: Default::default(),
            provision: Default::default(),
        }
    }
}

impl Default for ConsentProvision {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            data: Default::default(),
            actor: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            period: Default::default(),
            action: Default::default(),
            security_label: Default::default(),
            purpose: Default::default(),
            class: Default::default(),
            code: Default::default(),
            data_period: Default::default(),
            provision: Default::default(),
        }
    }
}

impl Default for ConsentLocation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ConsentProvisionActor {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            role: Default::default(),
            reference: Default::default(),
        }
    }
}

impl Default for ConsentVerification {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            verified: BooleanType::default(),
            _verified: Default::default(),
            verified_with: Default::default(),
            verification_date: Default::default(),
            _verification_date: Default::default(),
        }
    }
}

impl Default for ConsentNotificationEndpoint {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ConsentPolicy {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            authority: Default::default(),
            _authority: Default::default(),
            uri: Default::default(),
            _uri: Default::default(),
        }
    }
}

impl Default for ConsentTranscriber {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ConsentProvisionData {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            meaning: Default::default(),
            _meaning: Default::default(),
            reference: Default::default(),
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
    rh_foundation::Invariant::new("ppc-1", rh_foundation::Severity::Error, "Either a Policy or PolicyRule", "policy.exists() or policyRule.exists()").with_xpath("exists(f:policy) or exists(f:policyRule)"),
    rh_foundation::Invariant::new("ppc-2", rh_foundation::Severity::Error, "IF Scope=privacy, there must be a patient", "patient.exists() or scope.coding.where(system='something' and code='patient-privacy').exists().not()").with_xpath("exists(f:patient) or not(exists(f:scope/f:coding[f:system/@value='something' and f:code/@value='patient-privacy'])))"),
    rh_foundation::Invariant::new("ppc-3", rh_foundation::Severity::Error, "IF Scope=research, there must be a patient", "patient.exists() or scope.coding.where(system='something' and code='research').exists().not()").with_xpath("exists(f:patient) or not(exists(f:scope/f:coding[f:system/@value='something' and f:code/@value='research'])))"),
    rh_foundation::Invariant::new("ppc-4", rh_foundation::Severity::Error, "IF Scope=adr, there must be a patient", "patient.exists() or scope.coding.where(system='something' and code='adr').exists().not()").with_xpath("exists(f:patient) or not(exists(f:scope/f:coding[f:system/@value='something' and f:code/@value='adr'])))"),
    rh_foundation::Invariant::new("ppc-5", rh_foundation::Severity::Error, "IF Scope=treatment, there must be a patient", "patient.exists() or scope.coding.where(system='something' and code='treatment').exists().not()").with_xpath("exists(f:patient) or not(exists(f:scope/f:coding[f:system/@value='something' and f:code/@value='treatment'])))"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Consent {
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

impl crate::traits::resource::ResourceMutators for Consent {
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

impl crate::traits::resource::ResourceExistence for Consent {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Consent {
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

impl crate::traits::domain_resource::DomainResourceMutators for Consent {
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

impl crate::traits::domain_resource::DomainResourceExistence for Consent {
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

impl crate::traits::consent::ConsentAccessors for Consent {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ConsentStateCodes {
        self.status.clone()
    }
    fn scope(&self) -> CodeableConcept {
        self.scope.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        &self.category
    }
    fn patient(&self) -> Option<Reference> {
        self.patient.clone()
    }
    fn date_time(&self) -> Option<DateTimeType> {
        self.date_time.clone()
    }
    fn performer(&self) -> &[Reference] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn organization(&self) -> &[Reference] {
        self.organization.as_deref().unwrap_or(&[])
    }
    fn policy(&self) -> &[ConsentPolicy] {
        self.policy.as_deref().unwrap_or(&[])
    }
    fn policy_rule(&self) -> Option<CodeableConcept> {
        self.policy_rule.clone()
    }
    fn verification(&self) -> &[ConsentVerification] {
        self.verification.as_deref().unwrap_or(&[])
    }
    fn provision(&self) -> Option<ConsentProvision> {
        self.provision.clone()
    }
}

impl crate::traits::consent::ConsentMutators for Consent {
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
    fn set_status(self, value: ConsentStateCodes) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_scope(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.scope = value;
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
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = Some(value);
        resource
    }
    fn set_date_time(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date_time = Some(value);
        resource
    }
    fn set_performer(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_organization(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.organization = Some(value);
        resource
    }
    fn add_organization(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .organization
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_policy(self, value: Vec<ConsentPolicy>) -> Self {
        let mut resource = self.clone();
        resource.policy = Some(value);
        resource
    }
    fn add_policy(self, item: ConsentPolicy) -> Self {
        let mut resource = self.clone();
        resource.policy.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_policy_rule(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.policy_rule = Some(value);
        resource
    }
    fn set_verification(self, value: Vec<ConsentVerification>) -> Self {
        let mut resource = self.clone();
        resource.verification = Some(value);
        resource
    }
    fn add_verification(self, item: ConsentVerification) -> Self {
        let mut resource = self.clone();
        resource
            .verification
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_provision(self, value: ConsentProvision) -> Self {
        let mut resource = self.clone();
        resource.provision = Some(value);
        resource
    }
}

impl crate::traits::consent::ConsentExistence for Consent {
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
    fn has_source(&self) -> bool {
        self.source_attachment.is_some() || self.source_reference.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_scope(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        !self.category.is_empty()
    }
    fn has_patient(&self) -> bool {
        self.patient.is_some()
    }
    fn has_date_time(&self) -> bool {
        self.date_time.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_organization(&self) -> bool {
        self.organization.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_policy(&self) -> bool {
        self.policy.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_policy_rule(&self) -> bool {
        self.policy_rule.is_some()
    }
    fn has_verification(&self) -> bool {
        self.verification.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_provision(&self) -> bool {
        self.provision.is_some()
    }
}

impl crate::validation::ValidatableResource for Consent {
    fn resource_type(&self) -> &'static str {
        "Consent"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Consent")
    }
}
