use crate::bindings::consent_data_meaning::ConsentDataMeaning;
use crate::bindings::consent_provision_type::ConsentProvisionType;
use crate::bindings::consent_state_codes::ConsentStateCodes;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Consent
///
/// A record of a healthcare consumer’s  choices  or choices made on their behalf by a third party, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Consent
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Consent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifier for this record (external references)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// draft | active | inactive | not-done | entered-in-error | unknown
    pub status: ConsentStateCodes,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Classification of the consent statement - for indexing/retrieval
    ///
    /// Binding: example (A classification of the type of consents found in a consent statement.)
    ///
    /// Available values:
    /// - `59284-0`: Patient Consent
    /// - `57016-8`: Privacy policy acknowledgement Document
    /// - `57017-6`: Privacy policy Organization Document
    /// - `64292-6`: Release of information consent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<CodeableConcept>,
    /// Who the consent applies to
    pub subject: Option<Reference>,
    /// Fully executed date of the consent
    pub date: Option<DateType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Effective period for this Consent
    pub period: Option<Period>,
    /// Who is granting rights according to the policy and rules
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grantor: Vec<Reference>,
    /// Who is agreeing to the policy and rules
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grantee: Vec<Reference>,
    /// Consent workflow management
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manager: Vec<Reference>,
    /// Consent Enforcer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub controller: Vec<Reference>,
    /// Source from which this consent is taken
    #[serde(rename = "sourceAttachment")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_attachment: Vec<Attachment>,
    /// Source from which this consent is taken
    #[serde(rename = "sourceReference")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_reference: Vec<Reference>,
    /// Regulations establishing base Consent
    ///
    /// Binding: example (Regulatory policy examples)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/consent-policy
    #[serde(rename = "regulatoryBasis")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regulatory_basis: Vec<CodeableConcept>,
    /// Computable version of the backing policy
    #[serde(rename = "policyBasis")]
    pub policy_basis: Option<ConsentPolicybasis>,
    /// Human Readable Policy
    #[serde(rename = "policyText")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_text: Vec<Reference>,
    /// Consent Verified by patient or family
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verification: Vec<ConsentVerification>,
    /// deny | permit
    pub decision: Option<ConsentProvisionType>,
    /// Extension element for the 'decision' primitive field. Contains metadata and extensions.
    pub _decision: Option<Element>,
    /// Constraints to the base Consent.policyRule/Consent.policy
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provision: Vec<ConsentProvision>,
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
    /// ValueSet: http://hl7.org/fhir/ValueSet/participation-role-type
    pub role: Option<CodeableConcept>,
    /// Resource for the actor (or group, by role)
    pub reference: Option<Reference>,
}
/// Consent nested structure for the 'provision' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentProvision {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Who|what controlled by this provision (or group, by role)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<ConsentProvisionActor>,
    /// Data controlled by this provision
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<ConsentProvisionData>,
    /// Timeframe for this provision
    pub period: Option<Period>,
    /// Actions controlled by this provision
    ///
    /// Binding: example (Detailed codes for the consent action.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/consent-action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<CodeableConcept>,
    /// Security Labels that define affected resources
    ///
    /// Binding: example (Example Security Labels from the Healthcare Privacy and Security Classification System.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/security-label-examples
    #[serde(rename = "securityLabel")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<Coding>,
    /// Context of activities covered by this provision
    ///
    /// Binding: extensible (What purposes of use are controlled by this exception. If more than one label is specified, operations must have all the specified labels.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-PurposeOfUse
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose: Vec<Coding>,
    /// e.g. Resource Type, Profile, CDA, etc
    ///
    /// Binding: preferred (The document type a consent provision covers.)
    ///
    /// Available values:
    /// - `http://hl7.org/fhir/StructureDefinition/lipidprofile`: Lipid Lab Report
    /// - `application/hl7-cda+xml`: CDA Documents
    #[serde(rename = "documentType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_type: Vec<Coding>,
    /// e.g. Resource Type, Profile, etc
    ///
    /// Binding: extensible (The resource types a consent provision covers.)
    ///
    /// Available values:
    /// - `Account`
    /// - `ActivityDefinition`
    /// - `ActorDefinition`
    /// - `AdministrableProductDefinition`
    /// - `AdverseEvent`
    /// - `AllergyIntolerance`
    /// - `Appointment`
    /// - `AppointmentResponse`
    /// - `ArtifactAssessment`
    /// - `AuditEvent`
    /// - ... and 148 more values
    #[serde(rename = "resourceType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_type: Vec<Coding>,
    /// e.g. LOINC or SNOMED CT code, etc. in the content
    ///
    /// Binding: example (If this code is found in an instance, then the exception applies.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/consent-content-code
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<CodeableConcept>,
    /// Timeframe for data controlled by this provision
    #[serde(rename = "dataPeriod")]
    pub data_period: Option<Period>,
    /// A computable expression of the consent
    pub expression: Option<Expression>,
    /// Nested Exception Provisions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provision: Vec<StringType>,
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
    /// Business case of verification
    ///
    /// Binding: example (Types of Verification/Validation.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/consent-verification
    #[serde(rename = "verificationType")]
    pub verification_type: Option<CodeableConcept>,
    /// Person conducting verification
    #[serde(rename = "verifiedBy")]
    pub verified_by: Option<Reference>,
    /// Person who verified
    #[serde(rename = "verifiedWith")]
    pub verified_with: Option<Reference>,
    /// When consent verified
    #[serde(rename = "verificationDate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verification_date: Vec<DateTimeType>,
    /// Extension element for the 'verificationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_verificationDate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _verification_date: Vec<Element>,
}
/// Consent nested structure for the 'policyBasis' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentPolicybasis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reference backing policy resource
    pub reference: Option<Reference>,
    /// URL to a computable backing policy
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
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
            category: Default::default(),
            subject: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            period: Default::default(),
            grantor: Default::default(),
            grantee: Default::default(),
            manager: Default::default(),
            controller: Default::default(),
            source_attachment: Default::default(),
            source_reference: Default::default(),
            regulatory_basis: Default::default(),
            policy_basis: Default::default(),
            policy_text: Default::default(),
            verification: Default::default(),
            decision: Default::default(),
            _decision: Default::default(),
            provision: Default::default(),
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

impl Default for ConsentProvision {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            actor: Default::default(),
            data: Default::default(),
            period: Default::default(),
            action: Default::default(),
            security_label: Default::default(),
            purpose: Default::default(),
            document_type: Default::default(),
            resource_type: Default::default(),
            code: Default::default(),
            data_period: Default::default(),
            expression: Default::default(),
            provision: Default::default(),
        }
    }
}

impl Default for ConsentVerification {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            verified: BooleanType::default(),
            _verified: Default::default(),
            verification_type: Default::default(),
            verified_by: Default::default(),
            verified_with: Default::default(),
            verification_date: Default::default(),
            _verification_date: Default::default(),
        }
    }
}

impl Default for ConsentPolicybasis {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            reference: Default::default(),
            url: Default::default(),
            _url: Default::default(),
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("Consent.decision", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/consent-provision-type|5.0.0").with_description("Sets the base decision for Consent to be either permit or deny, with provisions assumed to be a negation of the previous level."),
    rh_foundation::ElementBinding::new("Consent.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("Consent.provision.data.meaning", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/consent-data-meaning|5.0.0").with_description("How a resource reference is interpreted when testing consent restrictions."),
    rh_foundation::ElementBinding::new("Consent.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/consent-state-codes|5.0.0").with_description("Indicates the state of the consent."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Consent.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.contained", 0, None),
            rh_foundation::ElementCardinality::new("Consent.extension", 0, None),
            rh_foundation::ElementCardinality::new("Consent.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Consent.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Consent.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.category", 0, None),
            rh_foundation::ElementCardinality::new("Consent.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.grantor", 0, None),
            rh_foundation::ElementCardinality::new("Consent.grantee", 0, None),
            rh_foundation::ElementCardinality::new("Consent.manager", 0, None),
            rh_foundation::ElementCardinality::new("Consent.controller", 0, None),
            rh_foundation::ElementCardinality::new("Consent.sourceAttachment", 0, None),
            rh_foundation::ElementCardinality::new("Consent.sourceReference", 0, None),
            rh_foundation::ElementCardinality::new("Consent.regulatoryBasis", 0, None),
            rh_foundation::ElementCardinality::new("Consent.policyBasis", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.policyBasis.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.policyBasis.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Consent.policyBasis.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Consent.policyBasis.reference", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.policyBasis.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.policyText", 0, None),
            rh_foundation::ElementCardinality::new("Consent.verification", 0, None),
            rh_foundation::ElementCardinality::new("Consent.verification.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.verification.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Consent.verification.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Consent.verification.verified", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Consent.verification.verificationType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Consent.verification.verifiedBy", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.verification.verifiedWith", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Consent.verification.verificationDate",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Consent.decision", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.provision", 0, None),
            rh_foundation::ElementCardinality::new("Consent.provision.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.provision.extension", 0, None),
            rh_foundation::ElementCardinality::new("Consent.provision.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Consent.provision.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.provision.actor", 0, None),
            rh_foundation::ElementCardinality::new("Consent.provision.actor.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.provision.actor.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Consent.provision.actor.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Consent.provision.actor.role", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.provision.actor.reference", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.provision.action", 0, None),
            rh_foundation::ElementCardinality::new("Consent.provision.securityLabel", 0, None),
            rh_foundation::ElementCardinality::new("Consent.provision.purpose", 0, None),
            rh_foundation::ElementCardinality::new("Consent.provision.documentType", 0, None),
            rh_foundation::ElementCardinality::new("Consent.provision.resourceType", 0, None),
            rh_foundation::ElementCardinality::new("Consent.provision.code", 0, None),
            rh_foundation::ElementCardinality::new("Consent.provision.dataPeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.provision.data", 0, None),
            rh_foundation::ElementCardinality::new("Consent.provision.data.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.provision.data.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Consent.provision.data.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Consent.provision.data.meaning", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.provision.data.reference", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.provision.expression", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Consent.provision.provision", 0, None),
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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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

impl crate::traits::domain_resource::DomainResourceExistence for Consent {
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

impl crate::traits::consent::ConsentAccessors for Consent {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn status(&self) -> ConsentStateCodes {
        self.status.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_slice()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn date(&self) -> Option<DateType> {
        self.date.clone()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn grantor(&self) -> &[Reference] {
        self.grantor.as_slice()
    }
    fn grantee(&self) -> &[Reference] {
        self.grantee.as_slice()
    }
    fn manager(&self) -> &[Reference] {
        self.manager.as_slice()
    }
    fn controller(&self) -> &[Reference] {
        self.controller.as_slice()
    }
    fn source_attachment(&self) -> &[Attachment] {
        self.source_attachment.as_slice()
    }
    fn source_reference(&self) -> &[Reference] {
        self.source_reference.as_slice()
    }
    fn regulatory_basis(&self) -> &[CodeableConcept] {
        self.regulatory_basis.as_slice()
    }
    fn policy_basis(&self) -> Option<ConsentPolicybasis> {
        self.policy_basis.clone()
    }
    fn policy_text(&self) -> &[Reference] {
        self.policy_text.as_slice()
    }
    fn verification(&self) -> &[ConsentVerification] {
        self.verification.as_slice()
    }
    fn decision(&self) -> Option<ConsentProvisionType> {
        self.decision.clone()
    }
    fn provision(&self) -> &[ConsentProvision] {
        self.provision.as_slice()
    }
}

impl crate::traits::consent::ConsentMutators for Consent {
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
    fn set_status(self, value: ConsentStateCodes) -> Self {
        let mut resource = self.clone();
        resource.status = value;
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
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_grantor(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.grantor = value;
        resource
    }
    fn add_grantor(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.grantor.push(item);
        resource
    }
    fn set_grantee(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.grantee = value;
        resource
    }
    fn add_grantee(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.grantee.push(item);
        resource
    }
    fn set_manager(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.manager = value;
        resource
    }
    fn add_manager(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.manager.push(item);
        resource
    }
    fn set_controller(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.controller = value;
        resource
    }
    fn add_controller(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.controller.push(item);
        resource
    }
    fn set_source_attachment(self, value: Vec<Attachment>) -> Self {
        let mut resource = self.clone();
        resource.source_attachment = value;
        resource
    }
    fn add_source_attachment(self, item: Attachment) -> Self {
        let mut resource = self.clone();
        resource.source_attachment.push(item);
        resource
    }
    fn set_source_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.source_reference = value;
        resource
    }
    fn add_source_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.source_reference.push(item);
        resource
    }
    fn set_regulatory_basis(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.regulatory_basis = value;
        resource
    }
    fn add_regulatory_basis(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.regulatory_basis.push(item);
        resource
    }
    fn set_policy_basis(self, value: ConsentPolicybasis) -> Self {
        let mut resource = self.clone();
        resource.policy_basis = Some(value);
        resource
    }
    fn set_policy_text(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.policy_text = value;
        resource
    }
    fn add_policy_text(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.policy_text.push(item);
        resource
    }
    fn set_verification(self, value: Vec<ConsentVerification>) -> Self {
        let mut resource = self.clone();
        resource.verification = value;
        resource
    }
    fn add_verification(self, item: ConsentVerification) -> Self {
        let mut resource = self.clone();
        resource.verification.push(item);
        resource
    }
    fn set_decision(self, value: ConsentProvisionType) -> Self {
        let mut resource = self.clone();
        resource.decision = Some(value);
        resource
    }
    fn set_provision(self, value: Vec<ConsentProvision>) -> Self {
        let mut resource = self.clone();
        resource.provision = value;
        resource
    }
    fn add_provision(self, item: ConsentProvision) -> Self {
        let mut resource = self.clone();
        resource.provision.push(item);
        resource
    }
}

impl crate::traits::consent::ConsentExistence for Consent {
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        !self.category.is_empty()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_grantor(&self) -> bool {
        !self.grantor.is_empty()
    }
    fn has_grantee(&self) -> bool {
        !self.grantee.is_empty()
    }
    fn has_manager(&self) -> bool {
        !self.manager.is_empty()
    }
    fn has_controller(&self) -> bool {
        !self.controller.is_empty()
    }
    fn has_source_attachment(&self) -> bool {
        !self.source_attachment.is_empty()
    }
    fn has_source_reference(&self) -> bool {
        !self.source_reference.is_empty()
    }
    fn has_regulatory_basis(&self) -> bool {
        !self.regulatory_basis.is_empty()
    }
    fn has_policy_basis(&self) -> bool {
        self.policy_basis.is_some()
    }
    fn has_policy_text(&self) -> bool {
        !self.policy_text.is_empty()
    }
    fn has_verification(&self) -> bool {
        !self.verification.is_empty()
    }
    fn has_decision(&self) -> bool {
        self.decision.is_some()
    }
    fn has_provision(&self) -> bool {
        !self.provision.is_empty()
    }
}

impl crate::validation::ValidatableResource for Consent {
    fn resource_type(&self) -> &'static str {
        "Consent"
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
        Some("http://hl7.org/fhir/StructureDefinition/Consent")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::consent::{ConsentAccessors, ConsentExistence, ConsentMutators};
