use crate::bindings::account_status::AccountStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::instant::InstantType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Account
///
/// A financial tool for tracking value accrued for a particular purpose.  In the healthcare field, used to track charges for a patient, cost centers, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Account
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Account
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Account number
    pub identifier: Option<Vec<Identifier>>,
    /// active | inactive | entered-in-error | on-hold | unknown
    pub status: AccountStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Tracks the lifecycle of the account through the billing process
    ///
    /// Binding: example (Indicates whether the account is available to be used for billing purposes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/account-billing-status
    #[serde(rename = "billingStatus")]
    pub billing_status: Option<CodeableConcept>,
    /// E.g. patient, expense, depreciation
    ///
    /// Binding: example (The usage type of this account, permits categorization of accounts.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/account-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Human-readable label
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// The entity that caused the expenses
    pub subject: Option<Vec<Reference>>,
    /// Transaction window
    #[serde(rename = "servicePeriod")]
    pub service_period: Option<Period>,
    /// The party(s) that are responsible for covering the payment of this account, and what order should they be applied to the account
    pub coverage: Option<Vec<AccountCoverage>>,
    /// Entity managing the Account
    pub owner: Option<Reference>,
    /// Explanation of purpose/use
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The parties ultimately responsible for balancing the Account
    pub guarantor: Option<Vec<AccountGuarantor>>,
    /// The list of diagnoses relevant to this account
    pub diagnosis: Option<Vec<AccountDiagnosis>>,
    /// The list of procedures relevant to this account
    pub procedure: Option<Vec<AccountProcedure>>,
    /// Other associated accounts related to this account
    #[serde(rename = "relatedAccount")]
    pub related_account: Option<Vec<AccountRelatedaccount>>,
    /// The base or default currency
    pub currency: Option<CodeableConcept>,
    /// Calculated account balance(s)
    pub balance: Option<Vec<AccountBalance>>,
    /// Time the balance amount was calculated
    #[serde(rename = "calculatedAt")]
    pub calculated_at: Option<InstantType>,
    /// Extension element for the 'calculatedAt' primitive field. Contains metadata and extensions.
    #[serde(rename = "_calculatedAt")]
    pub _calculated_at: Option<Element>,
}
/// Account nested structure for the 'procedure' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountProcedure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Ranking of the procedure (for each type)
    pub sequence: Option<PositiveIntType>,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// The procedure relevant to the account
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-code
    pub code: CodeableReference,
    /// Date of the procedure (when coded procedure)
    #[serde(rename = "dateOfService")]
    pub date_of_service: Option<DateTimeType>,
    /// Extension element for the 'dateOfService' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dateOfService")]
    pub _date_of_service: Option<Element>,
    /// How this procedure value should be used in charging the account
    ///
    /// Binding: example (Usage for the specific procedure - e.g. billing)
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Package Code specific for billing
    ///
    /// Binding: example (Local or Regional package codes, e.g. DRGs)
    #[serde(rename = "packageCode")]
    pub package_code: Option<Vec<CodeableConcept>>,
    /// Any devices that were associated with the procedure
    pub device: Option<Vec<Reference>>,
}
/// Account nested structure for the 'relatedAccount' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRelatedaccount {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Relationship of the associated Account
    ///
    /// Binding: example (Indicates the relationship between the accounts)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/account-relationship
    pub relationship: Option<CodeableConcept>,
    /// Reference to an associated Account
    pub account: Reference,
}
/// Account nested structure for the 'diagnosis' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDiagnosis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Ranking of the diagnosis (for each type)
    pub sequence: Option<PositiveIntType>,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// The diagnosis relevant to the account
    ///
    /// Binding: example (No description)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    pub condition: CodeableReference,
    /// Date of the diagnosis (when coded diagnosis)
    #[serde(rename = "dateOfDiagnosis")]
    pub date_of_diagnosis: Option<DateTimeType>,
    /// Extension element for the 'dateOfDiagnosis' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dateOfDiagnosis")]
    pub _date_of_diagnosis: Option<Element>,
    /// Type that this diagnosis has relevant to the account (e.g. admission, billing, discharge …)
    ///
    /// Binding: preferred (The type of diagnosis this condition represents.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-diagnosis-use
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Diagnosis present on Admission
    #[serde(rename = "onAdmission")]
    pub on_admission: Option<BooleanType>,
    /// Extension element for the 'onAdmission' primitive field. Contains metadata and extensions.
    #[serde(rename = "_onAdmission")]
    pub _on_admission: Option<Element>,
    /// Package Code specific for billing
    ///
    /// Binding: example (Local or Regional package codes, e.g. DRGs)
    #[serde(rename = "packageCode")]
    pub package_code: Option<Vec<CodeableConcept>>,
}
/// Account nested structure for the 'balance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Who is expected to pay this part of the balance
    ///
    /// Binding: extensible (Indicates the balance was outstanding at the given age.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/account-aggregate
    pub aggregate: Option<CodeableConcept>,
    /// current | 30 | 60 | 90 | 120
    ///
    /// Binding: extensible (Indicates the balance was outstanding at the given age.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/account-balance-term
    pub term: Option<CodeableConcept>,
    /// Estimated balance
    pub estimate: Option<BooleanType>,
    /// Extension element for the 'estimate' primitive field. Contains metadata and extensions.
    pub _estimate: Option<Element>,
    /// Calculated amount
    pub amount: Money,
}
/// Account nested structure for the 'coverage' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCoverage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The party(s), such as insurances, that may contribute to the payment of this account
    pub coverage: Reference,
    /// The priority of the coverage in the context of this account
    pub priority: Option<PositiveIntType>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
}
/// Account nested structure for the 'guarantor' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountGuarantor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Responsible entity
    pub party: Reference,
    /// Credit or other hold applied
    #[serde(rename = "onHold")]
    pub on_hold: Option<BooleanType>,
    /// Extension element for the 'onHold' primitive field. Contains metadata and extensions.
    #[serde(rename = "_onHold")]
    pub _on_hold: Option<Element>,
    /// Guarantee account during
    pub period: Option<Period>,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: AccountStatus::default(),
            _status: Default::default(),
            billing_status: Default::default(),
            type_: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            subject: Default::default(),
            service_period: Default::default(),
            coverage: Default::default(),
            owner: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            guarantor: Default::default(),
            diagnosis: Default::default(),
            procedure: Default::default(),
            related_account: Default::default(),
            currency: Default::default(),
            balance: Default::default(),
            calculated_at: Default::default(),
            _calculated_at: Default::default(),
        }
    }
}

impl Default for AccountProcedure {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sequence: Default::default(),
            _sequence: Default::default(),
            code: CodeableReference::default(),
            date_of_service: Default::default(),
            _date_of_service: Default::default(),
            type_: Default::default(),
            package_code: Default::default(),
            device: Default::default(),
        }
    }
}

impl Default for AccountRelatedaccount {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            relationship: Default::default(),
            account: Default::default(),
        }
    }
}

impl Default for AccountDiagnosis {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sequence: Default::default(),
            _sequence: Default::default(),
            condition: CodeableReference::default(),
            date_of_diagnosis: Default::default(),
            _date_of_diagnosis: Default::default(),
            type_: Default::default(),
            on_admission: Default::default(),
            _on_admission: Default::default(),
            package_code: Default::default(),
        }
    }
}

impl Default for AccountBalance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            aggregate: Default::default(),
            term: Default::default(),
            estimate: Default::default(),
            _estimate: Default::default(),
            amount: Money::default(),
        }
    }
}

impl Default for AccountCoverage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            coverage: Reference::default(),
            priority: Default::default(),
            _priority: Default::default(),
        }
    }
}

impl Default for AccountGuarantor {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            party: Reference::default(),
            on_hold: Default::default(),
            _on_hold: Default::default(),
            period: Default::default(),
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
    rh_foundation::Invariant::new("act-1", rh_foundation::Severity::Error, "The dateOfDiagnosis is not valid when using a reference to a diagnosis", "condition.reference.empty().not() implies dateOfDiagnosis.empty()"),
    rh_foundation::Invariant::new("act-2", rh_foundation::Severity::Error, "The dateOfService is not valid when using a reference to a procedure", "code.reference.empty().not() implies dateOfService.empty()"),
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
                "Account.currency",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/currencies|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "Account.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Account.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/account-status|5.0.0",
            )
            .with_description("Indicates whether the account is available to be used."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Account.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.contained", 0, None),
            rh_foundation::ElementCardinality::new("Account.extension", 0, None),
            rh_foundation::ElementCardinality::new("Account.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Account.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Account.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Account.billingStatus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.subject", 0, None),
            rh_foundation::ElementCardinality::new("Account.servicePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.coverage", 0, None),
            rh_foundation::ElementCardinality::new("Account.coverage.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.coverage.extension", 0, None),
            rh_foundation::ElementCardinality::new("Account.coverage.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Account.coverage.coverage", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Account.coverage.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.owner", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.guarantor", 0, None),
            rh_foundation::ElementCardinality::new("Account.guarantor.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.guarantor.extension", 0, None),
            rh_foundation::ElementCardinality::new("Account.guarantor.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Account.guarantor.party", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Account.guarantor.onHold", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.guarantor.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.diagnosis", 0, None),
            rh_foundation::ElementCardinality::new("Account.diagnosis.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.diagnosis.extension", 0, None),
            rh_foundation::ElementCardinality::new("Account.diagnosis.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Account.diagnosis.sequence", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.diagnosis.condition", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Account.diagnosis.dateOfDiagnosis", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.diagnosis.type", 0, None),
            rh_foundation::ElementCardinality::new("Account.diagnosis.onAdmission", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.diagnosis.packageCode", 0, None),
            rh_foundation::ElementCardinality::new("Account.procedure", 0, None),
            rh_foundation::ElementCardinality::new("Account.procedure.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.procedure.extension", 0, None),
            rh_foundation::ElementCardinality::new("Account.procedure.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Account.procedure.sequence", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.procedure.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Account.procedure.dateOfService", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.procedure.type", 0, None),
            rh_foundation::ElementCardinality::new("Account.procedure.packageCode", 0, None),
            rh_foundation::ElementCardinality::new("Account.procedure.device", 0, None),
            rh_foundation::ElementCardinality::new("Account.relatedAccount", 0, None),
            rh_foundation::ElementCardinality::new("Account.relatedAccount.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.relatedAccount.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Account.relatedAccount.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Account.relatedAccount.relationship",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Account.relatedAccount.account", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Account.currency", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.balance", 0, None),
            rh_foundation::ElementCardinality::new("Account.balance.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.balance.extension", 0, None),
            rh_foundation::ElementCardinality::new("Account.balance.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Account.balance.aggregate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.balance.term", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.balance.estimate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Account.balance.amount", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Account.calculatedAt", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Account {
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

impl crate::traits::resource::ResourceMutators for Account {
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

impl crate::traits::resource::ResourceExistence for Account {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Account {
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

impl crate::traits::domain_resource::DomainResourceMutators for Account {
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

impl crate::traits::domain_resource::DomainResourceExistence for Account {
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

impl crate::traits::account::AccountAccessors for Account {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> AccountStatus {
        self.status.clone()
    }
    fn billing_status(&self) -> Option<CodeableConcept> {
        self.billing_status.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn subject(&self) -> &[Reference] {
        self.subject.as_deref().unwrap_or(&[])
    }
    fn service_period(&self) -> Option<Period> {
        self.service_period.clone()
    }
    fn coverage(&self) -> &[AccountCoverage] {
        self.coverage.as_deref().unwrap_or(&[])
    }
    fn owner(&self) -> Option<Reference> {
        self.owner.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn guarantor(&self) -> &[AccountGuarantor] {
        self.guarantor.as_deref().unwrap_or(&[])
    }
    fn diagnosis(&self) -> &[AccountDiagnosis] {
        self.diagnosis.as_deref().unwrap_or(&[])
    }
    fn procedure(&self) -> &[AccountProcedure] {
        self.procedure.as_deref().unwrap_or(&[])
    }
    fn related_account(&self) -> &[AccountRelatedaccount] {
        self.related_account.as_deref().unwrap_or(&[])
    }
    fn currency(&self) -> Option<CodeableConcept> {
        self.currency.clone()
    }
    fn balance(&self) -> &[AccountBalance] {
        self.balance.as_deref().unwrap_or(&[])
    }
    fn calculated_at(&self) -> Option<InstantType> {
        self.calculated_at.clone()
    }
}

impl crate::traits::account::AccountMutators for Account {
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
    fn set_status(self, value: AccountStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_billing_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.billing_status = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_subject(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn add_subject(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_service_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.service_period = Some(value);
        resource
    }
    fn set_coverage(self, value: Vec<AccountCoverage>) -> Self {
        let mut resource = self.clone();
        resource.coverage = Some(value);
        resource
    }
    fn add_coverage(self, item: AccountCoverage) -> Self {
        let mut resource = self.clone();
        resource.coverage.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_owner(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.owner = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_guarantor(self, value: Vec<AccountGuarantor>) -> Self {
        let mut resource = self.clone();
        resource.guarantor = Some(value);
        resource
    }
    fn add_guarantor(self, item: AccountGuarantor) -> Self {
        let mut resource = self.clone();
        resource.guarantor.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_diagnosis(self, value: Vec<AccountDiagnosis>) -> Self {
        let mut resource = self.clone();
        resource.diagnosis = Some(value);
        resource
    }
    fn add_diagnosis(self, item: AccountDiagnosis) -> Self {
        let mut resource = self.clone();
        resource.diagnosis.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_procedure(self, value: Vec<AccountProcedure>) -> Self {
        let mut resource = self.clone();
        resource.procedure = Some(value);
        resource
    }
    fn add_procedure(self, item: AccountProcedure) -> Self {
        let mut resource = self.clone();
        resource.procedure.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_related_account(self, value: Vec<AccountRelatedaccount>) -> Self {
        let mut resource = self.clone();
        resource.related_account = Some(value);
        resource
    }
    fn add_related_account(self, item: AccountRelatedaccount) -> Self {
        let mut resource = self.clone();
        resource
            .related_account
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_currency(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.currency = Some(value);
        resource
    }
    fn set_balance(self, value: Vec<AccountBalance>) -> Self {
        let mut resource = self.clone();
        resource.balance = Some(value);
        resource
    }
    fn add_balance(self, item: AccountBalance) -> Self {
        let mut resource = self.clone();
        resource.balance.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_calculated_at(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.calculated_at = Some(value);
        resource
    }
}

impl crate::traits::account::AccountExistence for Account {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_billing_status(&self) -> bool {
        self.billing_status.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_service_period(&self) -> bool {
        self.service_period.is_some()
    }
    fn has_coverage(&self) -> bool {
        self.coverage.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_owner(&self) -> bool {
        self.owner.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_guarantor(&self) -> bool {
        self.guarantor.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_diagnosis(&self) -> bool {
        self.diagnosis.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_procedure(&self) -> bool {
        self.procedure.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_related_account(&self) -> bool {
        self.related_account.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_currency(&self) -> bool {
        self.currency.is_some()
    }
    fn has_balance(&self) -> bool {
        self.balance.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_calculated_at(&self) -> bool {
        self.calculated_at.is_some()
    }
}

impl crate::validation::ValidatableResource for Account {
    fn resource_type(&self) -> &'static str {
        "Account"
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
        Some("http://hl7.org/fhir/StructureDefinition/Account")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::account::{AccountAccessors, AccountExistence, AccountMutators};
