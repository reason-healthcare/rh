use crate::bindings::consent_provision_type::ConsentProvisionType;
use crate::bindings::permission_rule_combining::PermissionRuleCombining;
use crate::bindings::permission_status::PermissionStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Permission
///
/// Permission resource holds access rules for a given data and context.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Permission
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Permission
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// active | entered-in-error | draft | rejected
    pub status: PermissionStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The person or entity that asserts the permission
    pub asserter: Option<Reference>,
    /// The date that permission was asserted
    pub date: Option<Vec<DateTimeType>>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// The period in which the permission is active
    pub validity: Option<Period>,
    /// The asserted justification for using the data
    pub justification: Option<PermissionJustification>,
    /// deny-overrides | permit-overrides | ordered-deny-overrides | ordered-permit-overrides | deny-unless-permit | permit-unless-deny
    pub combining: PermissionRuleCombining,
    /// Extension element for the 'combining' primitive field. Contains metadata and extensions.
    pub _combining: Option<Element>,
    /// Constraints to the Permission
    pub rule: Option<Vec<PermissionRule>>,
}
/// Permission nested structure for the 'justification' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionJustification {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The regulatory grounds upon which this Permission builds
    ///
    /// Binding: example (Regulatory policy examples)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/consent-policy
    pub basis: Option<Vec<CodeableConcept>>,
    /// Justifing rational
    pub evidence: Option<Vec<Reference>>,
}
/// PermissionRule nested structure for the 'activity' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRuleActivity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Authorized actor(s)
    pub actor: Option<Vec<Reference>>,
    /// Actions controlled by this rule
    ///
    /// Binding: example (Detailed codes for the action.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/consent-action
    pub action: Option<Vec<CodeableConcept>>,
    /// The purpose for which the permission is given
    ///
    /// Binding: preferred (What purposes of use are controlled by this exception. If more than one label is specified, operations must have all the specified labels.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-PurposeOfUse
    pub purpose: Option<Vec<CodeableConcept>>,
}
/// PermissionRule nested structure for the 'data' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRuleData {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Security tag code on .meta.security
    pub security: Option<Vec<Coding>>,
    /// Timeframe encompasing data create/update
    pub period: Option<Vec<Period>>,
    /// Expression identifying the data
    pub expression: Option<Expression>,
}
/// Permission nested structure for the 'rule' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRule {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The selection criteria to identify data that is within scope of this provision
    pub data: Option<Vec<PermissionRuleData>>,
    /// A description or definition of which activities are allowed to be done on the data
    pub activity: Option<Vec<PermissionRuleActivity>>,
    /// deny | permit
    #[serde(rename = "type")]
    pub type_: Option<ConsentProvisionType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// What limits apply to the use of the data
    ///
    /// Binding: example (Obligations and Refrains)
    ///
    /// Available values:
    /// - `TREAT`
    /// - `HPAYMT`
    /// - `ETREAT`
    /// - `NOAUTH`
    /// - `DELAU`
    /// - `NORDSCLCD`
    pub limit: Option<Vec<CodeableConcept>>,
}

impl Default for Permission {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            status: PermissionStatus::default(),
            _status: Default::default(),
            asserter: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            validity: Default::default(),
            justification: Default::default(),
            combining: PermissionRuleCombining::default(),
            _combining: Default::default(),
            rule: Default::default(),
        }
    }
}

impl Default for PermissionJustification {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            basis: Default::default(),
            evidence: Default::default(),
        }
    }
}

impl Default for PermissionRuleActivity {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            actor: Default::default(),
            action: Default::default(),
            purpose: Default::default(),
        }
    }
}

impl Default for PermissionRuleData {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            security: Default::default(),
            period: Default::default(),
            expression: Default::default(),
        }
    }
}

impl Default for PermissionRule {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            data: Default::default(),
            activity: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            limit: Default::default(),
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
            rh_foundation::ElementBinding::new(
                "Permission.combining",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/permission-rule-combining|5.0.0",
            )
            .with_description("How the rules are to be combined."),
            rh_foundation::ElementBinding::new(
                "Permission.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Permission.rule.data.resource.meaning",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/consent-data-meaning|5.0.0",
            )
            .with_description(
                "How a resource reference is interpreted when testing consent restrictions.",
            ),
            rh_foundation::ElementBinding::new(
                "Permission.rule.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/consent-provision-type|5.0.0",
            )
            .with_description("How a rule statement is applied."),
            rh_foundation::ElementBinding::new(
                "Permission.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/permission-status|5.0.0",
            )
            .with_description("Codes identifying the lifecycle stage of a product."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Permission.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.contained", 0, None),
            rh_foundation::ElementCardinality::new("Permission.extension", 0, None),
            rh_foundation::ElementCardinality::new("Permission.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Permission.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.asserter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.date", 0, None),
            rh_foundation::ElementCardinality::new("Permission.validity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.justification", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.justification.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.justification.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Permission.justification.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Permission.justification.basis", 0, None),
            rh_foundation::ElementCardinality::new("Permission.justification.evidence", 0, None),
            rh_foundation::ElementCardinality::new("Permission.combining", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.rule", 0, None),
            rh_foundation::ElementCardinality::new("Permission.rule.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.rule.extension", 0, None),
            rh_foundation::ElementCardinality::new("Permission.rule.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Permission.rule.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.rule.data", 0, None),
            rh_foundation::ElementCardinality::new("Permission.rule.data.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.rule.data.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Permission.rule.data.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Permission.rule.data.resource", 0, None),
            rh_foundation::ElementCardinality::new("Permission.rule.data.resource.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Permission.rule.data.resource.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Permission.rule.data.resource.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Permission.rule.data.resource.meaning",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Permission.rule.data.resource.reference",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Permission.rule.data.security", 0, None),
            rh_foundation::ElementCardinality::new("Permission.rule.data.period", 0, None),
            rh_foundation::ElementCardinality::new("Permission.rule.data.expression", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.rule.activity", 0, None),
            rh_foundation::ElementCardinality::new("Permission.rule.activity.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Permission.rule.activity.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Permission.rule.activity.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Permission.rule.activity.actor", 0, None),
            rh_foundation::ElementCardinality::new("Permission.rule.activity.action", 0, None),
            rh_foundation::ElementCardinality::new("Permission.rule.activity.purpose", 0, None),
            rh_foundation::ElementCardinality::new("Permission.rule.limit", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Permission {
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

impl crate::traits::resource::ResourceMutators for Permission {
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

impl crate::traits::resource::ResourceExistence for Permission {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Permission {
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

impl crate::traits::domain_resource::DomainResourceMutators for Permission {
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

impl crate::traits::domain_resource::DomainResourceExistence for Permission {
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

impl crate::traits::permission::PermissionAccessors for Permission {
    fn status(&self) -> PermissionStatus {
        self.status.clone()
    }
    fn asserter(&self) -> Option<Reference> {
        self.asserter.clone()
    }
    fn date(&self) -> &[DateTimeType] {
        self.date.as_deref().unwrap_or(&[])
    }
    fn validity(&self) -> Option<Period> {
        self.validity.clone()
    }
    fn justification(&self) -> Option<PermissionJustification> {
        self.justification.clone()
    }
    fn combining(&self) -> PermissionRuleCombining {
        self.combining.clone()
    }
    fn rule(&self) -> &[PermissionRule] {
        self.rule.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::permission::PermissionMutators for Permission {
    fn new() -> Self {
        Self::default()
    }
    fn set_status(self, value: PermissionStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_asserter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.asserter = Some(value);
        resource
    }
    fn set_date(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn add_date(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.date.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_validity(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.validity = Some(value);
        resource
    }
    fn set_justification(self, value: PermissionJustification) -> Self {
        let mut resource = self.clone();
        resource.justification = Some(value);
        resource
    }
    fn set_combining(self, value: PermissionRuleCombining) -> Self {
        let mut resource = self.clone();
        resource.combining = value;
        resource
    }
    fn set_rule(self, value: Vec<PermissionRule>) -> Self {
        let mut resource = self.clone();
        resource.rule = Some(value);
        resource
    }
    fn add_rule(self, item: PermissionRule) -> Self {
        let mut resource = self.clone();
        resource.rule.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::permission::PermissionExistence for Permission {
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
    fn has_status(&self) -> bool {
        true
    }
    fn has_asserter(&self) -> bool {
        self.asserter.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_validity(&self) -> bool {
        self.validity.is_some()
    }
    fn has_justification(&self) -> bool {
        self.justification.is_some()
    }
    fn has_combining(&self) -> bool {
        true
    }
    fn has_rule(&self) -> bool {
        self.rule.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Permission {
    fn resource_type(&self) -> &'static str {
        "Permission"
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
        Some("http://hl7.org/fhir/StructureDefinition/Permission")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::permission::{PermissionAccessors, PermissionExistence, PermissionMutators};
