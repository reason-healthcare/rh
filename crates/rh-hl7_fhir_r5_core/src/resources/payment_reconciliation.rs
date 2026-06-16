use crate::bindings::fm_status::FmStatus;
use crate::bindings::note_type::NoteType;
use crate::bindings::payment_outcome::PaymentOutcome;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// PaymentReconciliation
///
/// This resource provides the details including amount of a payment and allocates the payment items being paid.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PaymentReconciliation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PaymentReconciliation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentReconciliation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for a payment reconciliation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Category of payment
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/payment-type
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// active | cancelled | draft | entered-in-error
    pub status: FmStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Workflow originating payment
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/payment-kind
    pub kind: Option<CodeableConcept>,
    /// Period covered
    pub period: Option<Period>,
    /// Creation date
    pub created: DateTimeType,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Who entered the payment
    pub enterer: Option<Reference>,
    /// Nature of the source
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/payment-issuertype
    #[serde(rename = "issuerType")]
    pub issuer_type: Option<CodeableConcept>,
    /// Party generating payment
    #[serde(rename = "paymentIssuer")]
    pub payment_issuer: Option<Reference>,
    /// Reference to requesting resource
    pub request: Option<Reference>,
    /// Responsible practitioner
    pub requestor: Option<Reference>,
    /// queued | complete | error | partial
    pub outcome: Option<PaymentOutcome>,
    /// Extension element for the 'outcome' primitive field. Contains metadata and extensions.
    pub _outcome: Option<Element>,
    /// Disposition message
    pub disposition: Option<StringType>,
    /// Extension element for the 'disposition' primitive field. Contains metadata and extensions.
    pub _disposition: Option<Element>,
    /// When payment issued
    pub date: DateType,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Where payment collected
    pub location: Option<Reference>,
    /// Payment instrument
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0570
    pub method: Option<CodeableConcept>,
    /// Type of card
    #[serde(rename = "cardBrand")]
    pub card_brand: Option<StringType>,
    /// Extension element for the 'cardBrand' primitive field. Contains metadata and extensions.
    #[serde(rename = "_cardBrand")]
    pub _card_brand: Option<Element>,
    /// Digits for verification
    #[serde(rename = "accountNumber")]
    pub account_number: Option<StringType>,
    /// Extension element for the 'accountNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_accountNumber")]
    pub _account_number: Option<Element>,
    /// Expiration year-month
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<DateType>,
    /// Extension element for the 'expirationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_expirationDate")]
    pub _expiration_date: Option<Element>,
    /// Processor name
    pub processor: Option<StringType>,
    /// Extension element for the 'processor' primitive field. Contains metadata and extensions.
    pub _processor: Option<Element>,
    /// Check number or payment reference
    #[serde(rename = "referenceNumber")]
    pub reference_number: Option<StringType>,
    /// Extension element for the 'referenceNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_referenceNumber")]
    pub _reference_number: Option<Element>,
    /// Authorization number
    pub authorization: Option<StringType>,
    /// Extension element for the 'authorization' primitive field. Contains metadata and extensions.
    pub _authorization: Option<Element>,
    /// Amount offered by the issuer
    #[serde(rename = "tenderedAmount")]
    pub tendered_amount: Option<Money>,
    /// Amount returned by the receiver
    #[serde(rename = "returnedAmount")]
    pub returned_amount: Option<Money>,
    /// Total amount of Payment
    pub amount: Money,
    /// Business identifier for the payment
    #[serde(rename = "paymentIdentifier")]
    pub payment_identifier: Option<Identifier>,
    /// Settlement particulars
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allocation: Vec<PaymentReconciliationAllocation>,
    /// Printed form identifier
    ///
    /// Binding: example (The forms codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/forms
    #[serde(rename = "formCode")]
    pub form_code: Option<CodeableConcept>,
    /// Note concerning processing
    #[serde(rename = "processNote")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process_note: Vec<PaymentReconciliationProcessnote>,
}
/// PaymentReconciliation nested structure for the 'allocation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentReconciliationAllocation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Business identifier of the payment detail
    pub identifier: Option<Identifier>,
    /// Business identifier of the prior payment detail
    pub predecessor: Option<Identifier>,
    /// Subject of the payment
    pub target: Option<Reference>,
    /// Sub-element of the subject (string)
    #[serde(rename = "targetItemString")]
    pub target_item_string: Option<StringType>,
    /// Sub-element of the subject (Identifier)
    #[serde(rename = "targetItemIdentifier")]
    pub target_item_identifier: Option<Identifier>,
    /// Sub-element of the subject (positiveInt)
    #[serde(rename = "targetItemPositiveInt")]
    pub target_item_positive_int: Option<PositiveIntType>,
    /// Applied-to encounter
    pub encounter: Option<Reference>,
    /// Applied-to account
    pub account: Option<Reference>,
    /// Category of payment
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/payment-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Submitter of the request
    pub submitter: Option<Reference>,
    /// Response committing to a payment
    pub response: Option<Reference>,
    /// Date of commitment to pay
    pub date: Option<DateType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Contact for the response
    pub responsible: Option<Reference>,
    /// Recipient of the payment
    pub payee: Option<Reference>,
    /// Amount allocated to this payable
    pub amount: Option<Money>,
}
/// PaymentReconciliation nested structure for the 'processNote' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentReconciliationProcessnote {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// display | print | printoper
    #[serde(rename = "type")]
    pub type_: Option<NoteType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Note explanatory text
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
}

impl Default for PaymentReconciliation {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            type_: Default::default(),
            status: FmStatus::default(),
            _status: Default::default(),
            kind: Default::default(),
            period: Default::default(),
            created: DateTimeType::default(),
            _created: Default::default(),
            enterer: Default::default(),
            issuer_type: Default::default(),
            payment_issuer: Default::default(),
            request: Default::default(),
            requestor: Default::default(),
            outcome: Default::default(),
            _outcome: Default::default(),
            disposition: Default::default(),
            _disposition: Default::default(),
            date: DateType::default(),
            _date: Default::default(),
            location: Default::default(),
            method: Default::default(),
            card_brand: Default::default(),
            _card_brand: Default::default(),
            account_number: Default::default(),
            _account_number: Default::default(),
            expiration_date: Default::default(),
            _expiration_date: Default::default(),
            processor: Default::default(),
            _processor: Default::default(),
            reference_number: Default::default(),
            _reference_number: Default::default(),
            authorization: Default::default(),
            _authorization: Default::default(),
            tendered_amount: Default::default(),
            returned_amount: Default::default(),
            amount: Money::default(),
            payment_identifier: Default::default(),
            allocation: Default::default(),
            form_code: Default::default(),
            process_note: Default::default(),
        }
    }
}

impl Default for PaymentReconciliationAllocation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            predecessor: Default::default(),
            target: Default::default(),
            target_item_string: Default::default(),
            target_item_identifier: Default::default(),
            target_item_positive_int: Default::default(),
            encounter: Default::default(),
            account: Default::default(),
            type_: Default::default(),
            submitter: Default::default(),
            response: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            responsible: Default::default(),
            payee: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for PaymentReconciliationProcessnote {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            text: Default::default(),
            _text: Default::default(),
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
                "PaymentReconciliation.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "PaymentReconciliation.outcome",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/payment-outcome|5.0.0",
            )
            .with_description("The outcome of the processing."),
            rh_foundation::ElementBinding::new(
                "PaymentReconciliation.processNote.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/note-type|5.0.0",
            )
            .with_description("The presentation types of notes."),
            rh_foundation::ElementBinding::new(
                "PaymentReconciliation.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/fm-status|5.0.0",
            )
            .with_description("A code specifying the state of the resource instance."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("PaymentReconciliation.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.contained", 0, None),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.identifier", 0, None),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.kind", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.created", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.enterer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.issuerType", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.paymentIssuer",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.request", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.requestor", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.disposition", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.method", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.cardBrand", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.accountNumber",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.expirationDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.processor", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.referenceNumber",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.authorization",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.tenderedAmount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.returnedAmount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.amount", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.paymentIdentifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.allocation", 0, None),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.identifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.predecessor",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.target",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.targetItem[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.encounter",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.account",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.submitter",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.response",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.date",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.responsible",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.payee",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.allocation.amount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.formCode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PaymentReconciliation.processNote", 0, None),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.processNote.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.processNote.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.processNote.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.processNote.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PaymentReconciliation.processNote.text",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for PaymentReconciliation {
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

impl crate::traits::resource::ResourceMutators for PaymentReconciliation {
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

impl crate::traits::resource::ResourceExistence for PaymentReconciliation {
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

impl crate::traits::domain_resource::DomainResourceAccessors for PaymentReconciliation {
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

impl crate::traits::domain_resource::DomainResourceMutators for PaymentReconciliation {
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

impl crate::traits::domain_resource::DomainResourceExistence for PaymentReconciliation {
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

impl crate::traits::payment_reconciliation::PaymentReconciliationAccessors
    for PaymentReconciliation
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn type_(&self) -> CodeableConcept {
        self.type_.clone()
    }
    fn status(&self) -> FmStatus {
        self.status.clone()
    }
    fn kind(&self) -> Option<CodeableConcept> {
        self.kind.clone()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn created(&self) -> DateTimeType {
        self.created.clone()
    }
    fn enterer(&self) -> Option<Reference> {
        self.enterer.clone()
    }
    fn issuer_type(&self) -> Option<CodeableConcept> {
        self.issuer_type.clone()
    }
    fn payment_issuer(&self) -> Option<Reference> {
        self.payment_issuer.clone()
    }
    fn request(&self) -> Option<Reference> {
        self.request.clone()
    }
    fn requestor(&self) -> Option<Reference> {
        self.requestor.clone()
    }
    fn outcome(&self) -> Option<PaymentOutcome> {
        self.outcome.clone()
    }
    fn disposition(&self) -> Option<StringType> {
        self.disposition.clone()
    }
    fn date(&self) -> DateType {
        self.date.clone()
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn method(&self) -> Option<CodeableConcept> {
        self.method.clone()
    }
    fn card_brand(&self) -> Option<StringType> {
        self.card_brand.clone()
    }
    fn account_number(&self) -> Option<StringType> {
        self.account_number.clone()
    }
    fn expiration_date(&self) -> Option<DateType> {
        self.expiration_date.clone()
    }
    fn processor(&self) -> Option<StringType> {
        self.processor.clone()
    }
    fn reference_number(&self) -> Option<StringType> {
        self.reference_number.clone()
    }
    fn authorization(&self) -> Option<StringType> {
        self.authorization.clone()
    }
    fn tendered_amount(&self) -> Option<Money> {
        self.tendered_amount.clone()
    }
    fn returned_amount(&self) -> Option<Money> {
        self.returned_amount.clone()
    }
    fn amount(&self) -> Money {
        self.amount.clone()
    }
    fn payment_identifier(&self) -> Option<Identifier> {
        self.payment_identifier.clone()
    }
    fn allocation(&self) -> &[PaymentReconciliationAllocation] {
        self.allocation.as_slice()
    }
    fn form_code(&self) -> Option<CodeableConcept> {
        self.form_code.clone()
    }
    fn process_note(&self) -> &[PaymentReconciliationProcessnote] {
        self.process_note.as_slice()
    }
}

impl crate::traits::payment_reconciliation::PaymentReconciliationMutators
    for PaymentReconciliation
{
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
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn set_status(self, value: FmStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_kind(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.kind = Some(value);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_created(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.created = value;
        resource
    }
    fn set_enterer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.enterer = Some(value);
        resource
    }
    fn set_issuer_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.issuer_type = Some(value);
        resource
    }
    fn set_payment_issuer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.payment_issuer = Some(value);
        resource
    }
    fn set_request(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.request = Some(value);
        resource
    }
    fn set_requestor(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.requestor = Some(value);
        resource
    }
    fn set_outcome(self, value: PaymentOutcome) -> Self {
        let mut resource = self.clone();
        resource.outcome = Some(value);
        resource
    }
    fn set_disposition(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.disposition = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = value;
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_method(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.method = Some(value);
        resource
    }
    fn set_card_brand(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.card_brand = Some(value);
        resource
    }
    fn set_account_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.account_number = Some(value);
        resource
    }
    fn set_expiration_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.expiration_date = Some(value);
        resource
    }
    fn set_processor(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.processor = Some(value);
        resource
    }
    fn set_reference_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.reference_number = Some(value);
        resource
    }
    fn set_authorization(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.authorization = Some(value);
        resource
    }
    fn set_tendered_amount(self, value: Money) -> Self {
        let mut resource = self.clone();
        resource.tendered_amount = Some(value);
        resource
    }
    fn set_returned_amount(self, value: Money) -> Self {
        let mut resource = self.clone();
        resource.returned_amount = Some(value);
        resource
    }
    fn set_amount(self, value: Money) -> Self {
        let mut resource = self.clone();
        resource.amount = value;
        resource
    }
    fn set_payment_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.payment_identifier = Some(value);
        resource
    }
    fn set_allocation(self, value: Vec<PaymentReconciliationAllocation>) -> Self {
        let mut resource = self.clone();
        resource.allocation = value;
        resource
    }
    fn add_allocation(self, item: PaymentReconciliationAllocation) -> Self {
        let mut resource = self.clone();
        resource.allocation.push(item);
        resource
    }
    fn set_form_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.form_code = Some(value);
        resource
    }
    fn set_process_note(self, value: Vec<PaymentReconciliationProcessnote>) -> Self {
        let mut resource = self.clone();
        resource.process_note = value;
        resource
    }
    fn add_process_note(self, item: PaymentReconciliationProcessnote) -> Self {
        let mut resource = self.clone();
        resource.process_note.push(item);
        resource
    }
}

impl crate::traits::payment_reconciliation::PaymentReconciliationExistence
    for PaymentReconciliation
{
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_kind(&self) -> bool {
        self.kind.is_some()
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_created(&self) -> bool {
        true
    }
    fn has_enterer(&self) -> bool {
        self.enterer.is_some()
    }
    fn has_issuer_type(&self) -> bool {
        self.issuer_type.is_some()
    }
    fn has_payment_issuer(&self) -> bool {
        self.payment_issuer.is_some()
    }
    fn has_request(&self) -> bool {
        self.request.is_some()
    }
    fn has_requestor(&self) -> bool {
        self.requestor.is_some()
    }
    fn has_outcome(&self) -> bool {
        self.outcome.is_some()
    }
    fn has_disposition(&self) -> bool {
        self.disposition.is_some()
    }
    fn has_date(&self) -> bool {
        true
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_method(&self) -> bool {
        self.method.is_some()
    }
    fn has_card_brand(&self) -> bool {
        self.card_brand.is_some()
    }
    fn has_account_number(&self) -> bool {
        self.account_number.is_some()
    }
    fn has_expiration_date(&self) -> bool {
        self.expiration_date.is_some()
    }
    fn has_processor(&self) -> bool {
        self.processor.is_some()
    }
    fn has_reference_number(&self) -> bool {
        self.reference_number.is_some()
    }
    fn has_authorization(&self) -> bool {
        self.authorization.is_some()
    }
    fn has_tendered_amount(&self) -> bool {
        self.tendered_amount.is_some()
    }
    fn has_returned_amount(&self) -> bool {
        self.returned_amount.is_some()
    }
    fn has_amount(&self) -> bool {
        true
    }
    fn has_payment_identifier(&self) -> bool {
        self.payment_identifier.is_some()
    }
    fn has_allocation(&self) -> bool {
        !self.allocation.is_empty()
    }
    fn has_form_code(&self) -> bool {
        self.form_code.is_some()
    }
    fn has_process_note(&self) -> bool {
        !self.process_note.is_empty()
    }
}

impl crate::validation::ValidatableResource for PaymentReconciliation {
    fn resource_type(&self) -> &'static str {
        "PaymentReconciliation"
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
        Some("http://hl7.org/fhir/StructureDefinition/PaymentReconciliation")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::payment_reconciliation::{
    PaymentReconciliationAccessors, PaymentReconciliationExistence, PaymentReconciliationMutators,
};
