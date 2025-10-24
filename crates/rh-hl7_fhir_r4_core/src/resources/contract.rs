use crate::bindings::contract_publicationstatus::ContractPublicationstatus;
use crate::bindings::contract_status::ContractStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::datatypes::signature::Signature;
use crate::datatypes::timing::Timing;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::primitives::time::TimeType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Contract
///
/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a policy or agreement.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Contract
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Contract
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Contract number
    pub identifier: Option<Vec<Identifier>>,
    /// Basal definition
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Business edition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// amended | appended | cancelled | disputed | entered-in-error | executable | executed | negotiable | offered | policy | rejected | renewed | revoked | resolved | terminated
    pub status: Option<ContractStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Negotiation status
    ///
    /// Binding: extensible (Detailed codes for the legal state of a contract.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-legalstate
    #[serde(rename = "legalState")]
    pub legal_state: Option<CodeableConcept>,
    /// Source Contract Definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Reference>,
    /// External Contract Definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<StringType>,
    /// Extension element for the 'instantiatesUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesUri")]
    pub _instantiates_uri: Option<Element>,
    /// Content derived from the basal information
    ///
    /// Binding: example (This is an example set of Content Derivative type codes, which represent the minimal content derived from the basal information source.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-content-derivative
    #[serde(rename = "contentDerivative")]
    pub content_derivative: Option<CodeableConcept>,
    /// When this Contract was issued
    pub issued: Option<DateTimeType>,
    /// Extension element for the 'issued' primitive field. Contains metadata and extensions.
    pub _issued: Option<Element>,
    /// Effective time
    pub applies: Option<Period>,
    /// Contract cessation cause
    ///
    /// Binding: example (Codes for the Cessation of Contracts.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-expiration-type
    #[serde(rename = "expirationType")]
    pub expiration_type: Option<CodeableConcept>,
    /// Contract Target Entity
    pub subject: Option<Vec<Reference>>,
    /// Authority under which this Contract has standing
    pub authority: Option<Vec<Reference>>,
    /// A sphere of control governed by an authoritative jurisdiction, organization, or person
    pub domain: Option<Vec<Reference>>,
    /// Specific Location
    pub site: Option<Vec<Reference>>,
    /// Computer friendly designation
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Human Friendly name
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Subordinate Friendly name
    pub subtitle: Option<StringType>,
    /// Extension element for the 'subtitle' primitive field. Contains metadata and extensions.
    pub _subtitle: Option<Element>,
    /// Acronym or short name
    pub alias: Option<Vec<StringType>>,
    /// Extension element for the 'alias' primitive field. Contains metadata and extensions.
    pub _alias: Option<Element>,
    /// Source of Contract
    pub author: Option<Reference>,
    /// Range of Legal Concerns
    ///
    /// Binding: example (Codes for the range of legal concerns.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-scope
    pub scope: Option<CodeableConcept>,
    /// Focus of contract interest (CodeableConcept)
    #[serde(rename = "topicCodeableConcept")]
    pub topic_codeable_concept: Option<CodeableConcept>,
    /// Focus of contract interest (Reference)
    #[serde(rename = "topicReference")]
    pub topic_reference: Option<Reference>,
    /// Legal instrument category
    ///
    /// Binding: example (List of overall contract codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Subtype within the context of type
    ///
    /// Binding: example (Detailed codes within the above.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-subtype
    #[serde(rename = "subType")]
    pub sub_type: Option<Vec<CodeableConcept>>,
    /// Contract precursor content
    #[serde(rename = "contentDefinition")]
    pub content_definition: Option<ContractContentdefinition>,
    /// Contract Term List
    pub term: Option<Vec<ContractTerm>>,
    /// Extra Information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Reference>>,
    /// Key event in Contract History
    #[serde(rename = "relevantHistory")]
    pub relevant_history: Option<Vec<Reference>>,
    /// Contract Signatory
    pub signer: Option<Vec<ContractSigner>>,
    /// Contract Friendly Language
    pub friendly: Option<Vec<ContractFriendly>>,
    /// Contract Legal Language
    pub legal: Option<Vec<ContractLegal>>,
    /// Computable Contract Language
    pub rule: Option<Vec<ContractRule>>,
    /// Binding Contract (Attachment)
    #[serde(rename = "legallyBindingAttachment")]
    pub legally_binding_attachment: Option<Attachment>,
    /// Binding Contract (Reference)
    #[serde(rename = "legallyBindingReference")]
    pub legally_binding_reference: Option<Reference>,
}
/// Contract nested structure for the 'term' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTerm {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Entity being ascribed responsibility
    pub action: Option<Vec<ContractTermAction>>,
    /// Protection for the Term
    #[serde(rename = "securityLabel")]
    pub security_label: Option<Vec<ContractTermSecuritylabel>>,
    /// Context of the Contract term
    pub offer: ContractTermOffer,
    /// Contract Term Asset List
    pub asset: Option<Vec<ContractTermAsset>>,
    /// Contract Term Number
    pub identifier: Option<Identifier>,
    /// Contract Term Issue Date Time
    pub issued: Option<DateTimeType>,
    /// Extension element for the 'issued' primitive field. Contains metadata and extensions.
    pub _issued: Option<Element>,
    /// Contract Term Effective Time
    pub applies: Option<Period>,
    /// Term Concern (CodeableConcept)
    #[serde(rename = "topicCodeableConcept")]
    pub topic_codeable_concept: Option<CodeableConcept>,
    /// Term Concern (Reference)
    #[serde(rename = "topicReference")]
    pub topic_reference: Option<Reference>,
    /// Contract Term Type or Form
    ///
    /// Binding: example (Detailed codes for the types of contract provisions.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-term-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Contract Term Type specific classification
    ///
    /// Binding: example (Detailed codes for the subtypes of contract provisions.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-term-subtype
    #[serde(rename = "subType")]
    pub sub_type: Option<CodeableConcept>,
    /// Term Statement
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
    /// Nested Contract Term Group
    pub group: Option<Vec<StringType>>,
}
/// ContractTermAsset nested structure for the 'context' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTermAssetContext {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Creator,custodian or owner
    pub reference: Option<Reference>,
    /// Codeable asset context
    ///
    /// Binding: example (Codes for the context of the asset.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-assetcontext
    pub code: Option<Vec<CodeableConcept>>,
    /// Context description
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
}
/// ContractTermAction nested structure for the 'subject' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTermActionSubject {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Entity of the action
    pub reference: Vec<Reference>,
    /// Role type of the agent
    ///
    /// Binding: example (Detailed codes for the contract actor role.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-actorrole
    pub role: Option<CodeableConcept>,
}
/// Contract nested structure for the 'legal' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractLegal {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Contract Legal Text (Attachment)
    #[serde(rename = "contentAttachment")]
    pub content_attachment: Attachment,
    /// Contract Legal Text (Reference)
    #[serde(rename = "contentReference")]
    pub content_reference: Reference,
}
/// ContractTerm nested structure for the 'offer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTermOffer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Offer business ID
    pub identifier: Option<Vec<Identifier>>,
    /// Negotiable offer asset
    pub topic: Option<Reference>,
    /// Contract Offer Type or Form
    ///
    /// Binding: example (Detailed codes for the types of contract provisions.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-term-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Accepting party choice
    ///
    /// Binding: extensible (The type of decision made by a grantor with respect to an offer made by a grantee.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActConsentDirective
    pub decision: Option<CodeableConcept>,
    /// How decision is conveyed
    ///
    /// Binding: example (Codes for conveying a decision.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-decision-mode
    #[serde(rename = "decisionMode")]
    pub decision_mode: Option<Vec<CodeableConcept>>,
    /// Human readable offer text
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
    /// Pointer to text
    #[serde(rename = "linkId")]
    pub link_id: Option<Vec<StringType>>,
    /// Extension element for the 'linkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_linkId")]
    pub _link_id: Option<Element>,
    /// Offer restriction numbers
    #[serde(rename = "securityLabelNumber")]
    pub security_label_number: Option<Vec<UnsignedIntType>>,
    /// Extension element for the 'securityLabelNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_securityLabelNumber")]
    pub _security_label_number: Option<Element>,
}
/// ContractTermAsset nested structure for the 'valuedItem' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTermAssetValueditem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Contract Valued Item Type (CodeableConcept)
    #[serde(rename = "entityCodeableConcept")]
    pub entity_codeable_concept: Option<CodeableConcept>,
    /// Contract Valued Item Type (Reference)
    #[serde(rename = "entityReference")]
    pub entity_reference: Option<Reference>,
    /// Contract Valued Item Number
    pub identifier: Option<Identifier>,
    /// Contract Valued Item Effective Tiem
    #[serde(rename = "effectiveTime")]
    pub effective_time: Option<DateTimeType>,
    /// Extension element for the 'effectiveTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_effectiveTime")]
    pub _effective_time: Option<Element>,
    /// Count of Contract Valued Items
    pub quantity: Option<Quantity>,
    /// Contract Valued Item fee, charge, or cost
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,
    /// Contract Valued Item Price Scaling Factor
    pub factor: Option<DecimalType>,
    /// Extension element for the 'factor' primitive field. Contains metadata and extensions.
    pub _factor: Option<Element>,
    /// Contract Valued Item Difficulty Scaling Factor
    pub points: Option<DecimalType>,
    /// Extension element for the 'points' primitive field. Contains metadata and extensions.
    pub _points: Option<Element>,
    /// Total Contract Valued Item Value
    pub net: Option<Money>,
    /// Terms of valuation
    pub payment: Option<StringType>,
    /// Extension element for the 'payment' primitive field. Contains metadata and extensions.
    pub _payment: Option<Element>,
    /// When payment is due
    #[serde(rename = "paymentDate")]
    pub payment_date: Option<DateTimeType>,
    /// Extension element for the 'paymentDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_paymentDate")]
    pub _payment_date: Option<Element>,
    /// Who will make payment
    pub responsible: Option<Reference>,
    /// Who will receive payment
    pub recipient: Option<Reference>,
    /// Pointer to specific item
    #[serde(rename = "linkId")]
    pub link_id: Option<Vec<StringType>>,
    /// Extension element for the 'linkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_linkId")]
    pub _link_id: Option<Element>,
    /// Security Labels that define affected terms
    #[serde(rename = "securityLabelNumber")]
    pub security_label_number: Option<Vec<UnsignedIntType>>,
    /// Extension element for the 'securityLabelNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_securityLabelNumber")]
    pub _security_label_number: Option<Element>,
}
/// Contract nested structure for the 'signer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractSigner {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Contract Signatory Role
    ///
    /// Binding: preferred (List of parties who may be signing.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-signer-type
    #[serde(rename = "type")]
    pub type_: Coding,
    /// Contract Signatory Party
    pub party: Reference,
    /// Contract Documentation Signature
    pub signature: Vec<Signature>,
}
/// ContractTermOffer nested structure for the 'party' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTermOfferParty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Referenced entity
    pub reference: Vec<Reference>,
    /// Participant engagement type
    ///
    /// Binding: example (Codes for offer participant roles.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-party-role
    pub role: CodeableConcept,
}
/// ContractTerm nested structure for the 'securityLabel' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTermSecuritylabel {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Link to Security Labels
    pub number: Option<Vec<UnsignedIntType>>,
    /// Extension element for the 'number' primitive field. Contains metadata and extensions.
    pub _number: Option<Element>,
    /// Confidentiality Protection
    ///
    /// Binding: example (Codes for confidentiality protection.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-security-classification
    pub classification: Coding,
    /// Applicable Policy
    ///
    /// Binding: example (Codes for policy category.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-security-category
    pub category: Option<Vec<Coding>>,
    /// Handling Instructions
    ///
    /// Binding: example (Codes for handling instructions.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-security-control
    pub control: Option<Vec<Coding>>,
}
/// Contract nested structure for the 'rule' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractRule {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Computable Contract Rules (Attachment)
    #[serde(rename = "contentAttachment")]
    pub content_attachment: Attachment,
    /// Computable Contract Rules (Reference)
    #[serde(rename = "contentReference")]
    pub content_reference: Reference,
}
/// Contract nested structure for the 'friendly' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFriendly {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Easily comprehended representation of this Contract (Attachment)
    #[serde(rename = "contentAttachment")]
    pub content_attachment: Attachment,
    /// Easily comprehended representation of this Contract (Reference)
    #[serde(rename = "contentReference")]
    pub content_reference: Reference,
}
/// ContractTerm nested structure for the 'action' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTermAction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// True if the term prohibits the  action
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<BooleanType>,
    /// Extension element for the 'doNotPerform' primitive field. Contains metadata and extensions.
    #[serde(rename = "_doNotPerform")]
    pub _do_not_perform: Option<Element>,
    /// Type or form of the action
    ///
    /// Binding: example (Detailed codes for the contract action.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-action
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Purpose for the Contract Term Action
    ///
    /// Binding: example (Detailed codes for the contract action reason.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-PurposeOfUse
    pub intent: CodeableConcept,
    /// Pointer to specific item
    #[serde(rename = "linkId")]
    pub link_id: Option<Vec<StringType>>,
    /// Extension element for the 'linkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_linkId")]
    pub _link_id: Option<Element>,
    /// State of the action
    ///
    /// Binding: example (Codes for the status of an term action.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-actionstatus
    pub status: CodeableConcept,
    /// Episode associated with action
    pub context: Option<Reference>,
    /// Pointer to specific item
    #[serde(rename = "contextLinkId")]
    pub context_link_id: Option<Vec<StringType>>,
    /// Extension element for the 'contextLinkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_contextLinkId")]
    pub _context_link_id: Option<Element>,
    /// When action happens (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// When action happens (Period)
    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,
    /// When action happens (Timing)
    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,
    /// Who asked for action
    pub requester: Option<Vec<Reference>>,
    /// Pointer to specific item
    #[serde(rename = "requesterLinkId")]
    pub requester_link_id: Option<Vec<StringType>>,
    /// Extension element for the 'requesterLinkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_requesterLinkId")]
    pub _requester_link_id: Option<Element>,
    /// Kind of service performer
    ///
    /// Binding: example (Codes for the types of action perfomer.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/provenance-agent-type
    #[serde(rename = "performerType")]
    pub performer_type: Option<Vec<CodeableConcept>>,
    /// Competency of the performer
    ///
    /// Binding: example (Codes for the role of the action performer.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/provenance-agent-role
    #[serde(rename = "performerRole")]
    pub performer_role: Option<CodeableConcept>,
    /// Actor that wil execute (or not) the action
    pub performer: Option<Reference>,
    /// Pointer to specific item
    #[serde(rename = "performerLinkId")]
    pub performer_link_id: Option<Vec<StringType>>,
    /// Extension element for the 'performerLinkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_performerLinkId")]
    pub _performer_link_id: Option<Element>,
    /// Why is action (not) needed?
    ///
    /// Binding: example (Detailed codes for the contract action reason.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-PurposeOfUse
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Why is action (not) needed?
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Why action is to be performed
    pub reason: Option<Vec<StringType>>,
    /// Extension element for the 'reason' primitive field. Contains metadata and extensions.
    pub _reason: Option<Element>,
    /// Pointer to specific item
    #[serde(rename = "reasonLinkId")]
    pub reason_link_id: Option<Vec<StringType>>,
    /// Extension element for the 'reasonLinkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_reasonLinkId")]
    pub _reason_link_id: Option<Element>,
    /// Comments about the action
    pub note: Option<Vec<Annotation>>,
    /// Action restriction numbers
    #[serde(rename = "securityLabelNumber")]
    pub security_label_number: Option<Vec<UnsignedIntType>>,
    /// Extension element for the 'securityLabelNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_securityLabelNumber")]
    pub _security_label_number: Option<Element>,
}
/// Contract nested structure for the 'contentDefinition' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractContentdefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Content structure and use
    ///
    /// Binding: example (Detailed codes for the definition of contracts.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-definition-type
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Detailed Content Type Definition
    ///
    /// Binding: example (Detailed codes for the additional definition of contracts.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-definition-subtype
    #[serde(rename = "subType")]
    pub sub_type: Option<CodeableConcept>,
    /// Publisher Entity
    pub publisher: Option<Reference>,
    /// When published
    #[serde(rename = "publicationDate")]
    pub publication_date: Option<DateTimeType>,
    /// Extension element for the 'publicationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_publicationDate")]
    pub _publication_date: Option<Element>,
    /// amended | appended | cancelled | disputed | entered-in-error | executable | executed | negotiable | offered | policy | rejected | renewed | revoked | resolved | terminated
    #[serde(rename = "publicationStatus")]
    pub publication_status: ContractPublicationstatus,
    /// Extension element for the 'publicationStatus' primitive field. Contains metadata and extensions.
    #[serde(rename = "_publicationStatus")]
    pub _publication_status: Option<Element>,
    /// Publication Ownership
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
}
/// ContractTermOffer nested structure for the 'answer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTermOfferAnswer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The actual answer response (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// The actual answer response (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: DecimalType,
    /// The actual answer response (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// The actual answer response (date)
    #[serde(rename = "valueDate")]
    pub value_date: DateType,
    /// The actual answer response (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: DateTimeType,
    /// The actual answer response (time)
    #[serde(rename = "valueTime")]
    pub value_time: TimeType,
    /// The actual answer response (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// The actual answer response (uri)
    #[serde(rename = "valueUri")]
    pub value_uri: StringType,
    /// The actual answer response (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,
    /// The actual answer response (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,
    /// The actual answer response (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// The actual answer response (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Reference,
}
/// ContractTerm nested structure for the 'asset' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTermAsset {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Range of asset
    ///
    /// Binding: example (Codes for scoping an asset.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-assetscope
    pub scope: Option<CodeableConcept>,
    /// Asset category
    ///
    /// Binding: example (Condes for the type of an asset.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-assettype
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Associated entities
    #[serde(rename = "typeReference")]
    pub type_reference: Option<Vec<Reference>>,
    /// Asset sub-category
    ///
    /// Binding: example (Condes for the sub-type of an asset.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contract-assetsubtype
    pub subtype: Option<Vec<CodeableConcept>>,
    /// Kinship of the asset
    ///
    /// Binding: extensible (The class (type) of information a consent rule covers.)
    ///
    /// Available values:
    /// - `http://hl7.org/fhir/StructureDefinition/lipidprofile`: Lipid Lab Report
    /// - `application/hl7-cda+xml`: CDA Documents
    pub relationship: Option<Coding>,
    /// Quality desctiption of asset
    pub condition: Option<StringType>,
    /// Extension element for the 'condition' primitive field. Contains metadata and extensions.
    pub _condition: Option<Element>,
    /// Asset availability types
    ///
    /// Binding: example (Codes for asset availability.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/asset-availability
    #[serde(rename = "periodType")]
    pub period_type: Option<Vec<CodeableConcept>>,
    /// Time period of the asset
    pub period: Option<Vec<Period>>,
    /// Time period
    #[serde(rename = "usePeriod")]
    pub use_period: Option<Vec<Period>>,
    /// Asset clause or question text
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
    /// Pointer to asset text
    #[serde(rename = "linkId")]
    pub link_id: Option<Vec<StringType>>,
    /// Extension element for the 'linkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_linkId")]
    pub _link_id: Option<Element>,
    /// Response to assets
    pub answer: Option<Vec<StringType>>,
    /// Asset restriction numbers
    #[serde(rename = "securityLabelNumber")]
    pub security_label_number: Option<Vec<UnsignedIntType>>,
    /// Extension element for the 'securityLabelNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_securityLabelNumber")]
    pub _security_label_number: Option<Element>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            url: Default::default(),
            _url: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            legal_state: Default::default(),
            instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            content_derivative: Default::default(),
            issued: Default::default(),
            _issued: Default::default(),
            applies: Default::default(),
            expiration_type: Default::default(),
            subject: Default::default(),
            authority: Default::default(),
            domain: Default::default(),
            site: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            subtitle: Default::default(),
            _subtitle: Default::default(),
            alias: Default::default(),
            _alias: Default::default(),
            author: Default::default(),
            scope: Default::default(),
            topic_codeable_concept: Default::default(),
            topic_reference: Default::default(),
            type_: Default::default(),
            sub_type: Default::default(),
            content_definition: Default::default(),
            term: Default::default(),
            supporting_info: Default::default(),
            relevant_history: Default::default(),
            signer: Default::default(),
            friendly: Default::default(),
            legal: Default::default(),
            rule: Default::default(),
            legally_binding_attachment: Default::default(),
            legally_binding_reference: Default::default(),
        }
    }
}

impl Default for ContractTerm {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            action: Default::default(),
            security_label: Default::default(),
            offer: ContractTermOffer::default(),
            asset: Default::default(),
            identifier: Default::default(),
            issued: Default::default(),
            _issued: Default::default(),
            applies: Default::default(),
            topic_codeable_concept: Default::default(),
            topic_reference: Default::default(),
            type_: Default::default(),
            sub_type: Default::default(),
            text: Default::default(),
            _text: Default::default(),
            group: Default::default(),
        }
    }
}

impl Default for ContractTermAssetContext {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            reference: Default::default(),
            code: Default::default(),
            text: Default::default(),
            _text: Default::default(),
        }
    }
}

impl Default for ContractTermActionSubject {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            reference: Default::default(),
            role: Default::default(),
        }
    }
}

impl Default for ContractLegal {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            content_attachment: Default::default(),
            content_reference: Default::default(),
        }
    }
}

impl Default for ContractTermOffer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            topic: Default::default(),
            type_: Default::default(),
            decision: Default::default(),
            decision_mode: Default::default(),
            text: Default::default(),
            _text: Default::default(),
            link_id: Default::default(),
            _link_id: Default::default(),
            security_label_number: Default::default(),
            _security_label_number: Default::default(),
        }
    }
}

impl Default for ContractTermAssetValueditem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            entity_codeable_concept: Default::default(),
            entity_reference: Default::default(),
            identifier: Default::default(),
            effective_time: Default::default(),
            _effective_time: Default::default(),
            quantity: Default::default(),
            unit_price: Default::default(),
            factor: Default::default(),
            _factor: Default::default(),
            points: Default::default(),
            _points: Default::default(),
            net: Default::default(),
            payment: Default::default(),
            _payment: Default::default(),
            payment_date: Default::default(),
            _payment_date: Default::default(),
            responsible: Default::default(),
            recipient: Default::default(),
            link_id: Default::default(),
            _link_id: Default::default(),
            security_label_number: Default::default(),
            _security_label_number: Default::default(),
        }
    }
}

impl Default for ContractSigner {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            party: Reference::default(),
            signature: Vec::new(),
        }
    }
}

impl Default for ContractTermOfferParty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            reference: Default::default(),
            role: Default::default(),
        }
    }
}

impl Default for ContractTermSecuritylabel {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            number: Default::default(),
            _number: Default::default(),
            classification: Default::default(),
            category: Default::default(),
            control: Default::default(),
        }
    }
}

impl Default for ContractRule {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            content_attachment: Default::default(),
            content_reference: Default::default(),
        }
    }
}

impl Default for ContractFriendly {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            content_attachment: Default::default(),
            content_reference: Default::default(),
        }
    }
}

impl Default for ContractTermAction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            do_not_perform: Default::default(),
            _do_not_perform: Default::default(),
            type_: Default::default(),
            intent: Default::default(),
            link_id: Default::default(),
            _link_id: Default::default(),
            status: Default::default(),
            context: Default::default(),
            context_link_id: Default::default(),
            _context_link_id: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            occurrence_timing: Default::default(),
            requester: Default::default(),
            requester_link_id: Default::default(),
            _requester_link_id: Default::default(),
            performer_type: Default::default(),
            performer_role: Default::default(),
            performer: Default::default(),
            performer_link_id: Default::default(),
            _performer_link_id: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            reason: Default::default(),
            _reason: Default::default(),
            reason_link_id: Default::default(),
            _reason_link_id: Default::default(),
            note: Default::default(),
            security_label_number: Default::default(),
            _security_label_number: Default::default(),
        }
    }
}

impl Default for ContractContentdefinition {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            sub_type: Default::default(),
            publisher: Default::default(),
            publication_date: Default::default(),
            _publication_date: Default::default(),
            publication_status: Default::default(),
            _publication_status: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
        }
    }
}

impl Default for ContractTermOfferAnswer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            value_boolean: Default::default(),
            value_decimal: Default::default(),
            value_integer: Default::default(),
            value_date: Default::default(),
            value_date_time: Default::default(),
            value_time: Default::default(),
            value_string: Default::default(),
            value_uri: Default::default(),
            value_attachment: Default::default(),
            value_coding: Default::default(),
            value_quantity: Default::default(),
            value_reference: Default::default(),
        }
    }
}

impl Default for ContractTermAsset {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            scope: Default::default(),
            type_: Default::default(),
            type_reference: Default::default(),
            subtype: Default::default(),
            relationship: Default::default(),
            condition: Default::default(),
            _condition: Default::default(),
            period_type: Default::default(),
            period: Default::default(),
            use_period: Default::default(),
            text: Default::default(),
            _text: Default::default(),
            link_id: Default::default(),
            _link_id: Default::default(),
            answer: Default::default(),
            security_label_number: Default::default(),
            _security_label_number: Default::default(),
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
impl crate::traits::resource::ResourceAccessors for Contract {
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

impl crate::traits::resource::ResourceMutators for Contract {
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

impl crate::traits::resource::ResourceExistence for Contract {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Contract {
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

impl crate::traits::domain_resource::DomainResourceMutators for Contract {
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

impl crate::traits::domain_resource::DomainResourceExistence for Contract {
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

impl crate::traits::contract::ContractAccessors for Contract {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn status(&self) -> Option<ContractStatus> {
        self.status.clone()
    }
    fn legal_state(&self) -> Option<CodeableConcept> {
        self.legal_state.clone()
    }
    fn instantiates_canonical(&self) -> Option<Reference> {
        self.instantiates_canonical.clone()
    }
    fn instantiates_uri(&self) -> Option<StringType> {
        self.instantiates_uri.clone()
    }
    fn content_derivative(&self) -> Option<CodeableConcept> {
        self.content_derivative.clone()
    }
    fn issued(&self) -> Option<DateTimeType> {
        self.issued.clone()
    }
    fn applies(&self) -> Option<Period> {
        self.applies.clone()
    }
    fn expiration_type(&self) -> Option<CodeableConcept> {
        self.expiration_type.clone()
    }
    fn subject(&self) -> &[Reference] {
        self.subject.as_deref().unwrap_or(&[])
    }
    fn authority(&self) -> &[Reference] {
        self.authority.as_deref().unwrap_or(&[])
    }
    fn domain(&self) -> &[Reference] {
        self.domain.as_deref().unwrap_or(&[])
    }
    fn site(&self) -> &[Reference] {
        self.site.as_deref().unwrap_or(&[])
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn subtitle(&self) -> Option<StringType> {
        self.subtitle.clone()
    }
    fn alias(&self) -> &[StringType] {
        self.alias.as_deref().unwrap_or(&[])
    }
    fn author(&self) -> Option<Reference> {
        self.author.clone()
    }
    fn scope(&self) -> Option<CodeableConcept> {
        self.scope.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn sub_type(&self) -> &[CodeableConcept] {
        self.sub_type.as_deref().unwrap_or(&[])
    }
    fn content_definition(&self) -> Option<ContractContentdefinition> {
        self.content_definition.clone()
    }
    fn term(&self) -> &[ContractTerm] {
        self.term.as_deref().unwrap_or(&[])
    }
    fn supporting_info(&self) -> &[Reference] {
        self.supporting_info.as_deref().unwrap_or(&[])
    }
    fn relevant_history(&self) -> &[Reference] {
        self.relevant_history.as_deref().unwrap_or(&[])
    }
    fn signer(&self) -> &[ContractSigner] {
        self.signer.as_deref().unwrap_or(&[])
    }
    fn friendly(&self) -> &[ContractFriendly] {
        self.friendly.as_deref().unwrap_or(&[])
    }
    fn legal(&self) -> &[ContractLegal] {
        self.legal.as_deref().unwrap_or(&[])
    }
    fn rule(&self) -> &[ContractRule] {
        self.rule.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::contract::ContractMutators for Contract {
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
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
    }
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_status(self, value: ContractStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_legal_state(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.legal_state = Some(value);
        resource
    }
    fn set_instantiates_canonical(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.instantiates_canonical = Some(value);
        resource
    }
    fn set_instantiates_uri(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.instantiates_uri = Some(value);
        resource
    }
    fn set_content_derivative(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.content_derivative = Some(value);
        resource
    }
    fn set_issued(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.issued = Some(value);
        resource
    }
    fn set_applies(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.applies = Some(value);
        resource
    }
    fn set_expiration_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.expiration_type = Some(value);
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
    fn set_authority(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.authority = Some(value);
        resource
    }
    fn add_authority(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.authority.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_domain(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.domain = Some(value);
        resource
    }
    fn add_domain(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.domain.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_site(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.site = Some(value);
        resource
    }
    fn add_site(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.site.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_subtitle(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.subtitle = Some(value);
        resource
    }
    fn set_alias(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.alias = Some(value);
        resource
    }
    fn add_alias(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.alias.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_author(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.author = Some(value);
        resource
    }
    fn set_scope(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.scope = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_sub_type(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.sub_type = Some(value);
        resource
    }
    fn add_sub_type(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.sub_type.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_content_definition(self, value: ContractContentdefinition) -> Self {
        let mut resource = self.clone();
        resource.content_definition = Some(value);
        resource
    }
    fn set_term(self, value: Vec<ContractTerm>) -> Self {
        let mut resource = self.clone();
        resource.term = Some(value);
        resource
    }
    fn add_term(self, item: ContractTerm) -> Self {
        let mut resource = self.clone();
        resource.term.get_or_insert_with(Vec::new).push(item);
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
    fn set_signer(self, value: Vec<ContractSigner>) -> Self {
        let mut resource = self.clone();
        resource.signer = Some(value);
        resource
    }
    fn add_signer(self, item: ContractSigner) -> Self {
        let mut resource = self.clone();
        resource.signer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_friendly(self, value: Vec<ContractFriendly>) -> Self {
        let mut resource = self.clone();
        resource.friendly = Some(value);
        resource
    }
    fn add_friendly(self, item: ContractFriendly) -> Self {
        let mut resource = self.clone();
        resource.friendly.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_legal(self, value: Vec<ContractLegal>) -> Self {
        let mut resource = self.clone();
        resource.legal = Some(value);
        resource
    }
    fn add_legal(self, item: ContractLegal) -> Self {
        let mut resource = self.clone();
        resource.legal.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_rule(self, value: Vec<ContractRule>) -> Self {
        let mut resource = self.clone();
        resource.rule = Some(value);
        resource
    }
    fn add_rule(self, item: ContractRule) -> Self {
        let mut resource = self.clone();
        resource.rule.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::contract::ContractExistence for Contract {
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
    fn has_topic(&self) -> bool {
        self.topic_codeable_concept.is_some() || self.topic_reference.is_some()
    }
    fn has_legally_binding(&self) -> bool {
        self.legally_binding_attachment.is_some() || self.legally_binding_reference.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_legal_state(&self) -> bool {
        self.legal_state.is_some()
    }
    fn has_instantiates_canonical(&self) -> bool {
        self.instantiates_canonical.is_some()
    }
    fn has_instantiates_uri(&self) -> bool {
        self.instantiates_uri.is_some()
    }
    fn has_content_derivative(&self) -> bool {
        self.content_derivative.is_some()
    }
    fn has_issued(&self) -> bool {
        self.issued.is_some()
    }
    fn has_applies(&self) -> bool {
        self.applies.is_some()
    }
    fn has_expiration_type(&self) -> bool {
        self.expiration_type.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_authority(&self) -> bool {
        self.authority.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_domain(&self) -> bool {
        self.domain.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_site(&self) -> bool {
        self.site.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_subtitle(&self) -> bool {
        self.subtitle.is_some()
    }
    fn has_alias(&self) -> bool {
        self.alias.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_author(&self) -> bool {
        self.author.is_some()
    }
    fn has_scope(&self) -> bool {
        self.scope.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_sub_type(&self) -> bool {
        self.sub_type.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_content_definition(&self) -> bool {
        self.content_definition.is_some()
    }
    fn has_term(&self) -> bool {
        self.term.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_supporting_info(&self) -> bool {
        self.supporting_info.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_relevant_history(&self) -> bool {
        self.relevant_history
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_signer(&self) -> bool {
        self.signer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_friendly(&self) -> bool {
        self.friendly.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_legal(&self) -> bool {
        self.legal.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_rule(&self) -> bool {
        self.rule.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Contract {
    fn resource_type(&self) -> &'static str {
        "Contract"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Contract")
    }
}
