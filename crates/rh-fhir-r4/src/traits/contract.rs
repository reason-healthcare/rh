use crate::bindings::contract_status::ContractStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::contract::ContractContentdefinition;
use crate::resources::contract::ContractFriendly;
use crate::resources::contract::ContractLegal;
use crate::resources::contract::ContractRule;
use crate::resources::contract::ContractSigner;
use crate::resources::contract::ContractTerm;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Contract Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a policy or agreement.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Contract
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Contract
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ContractAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the url field.
    fn url(&self) -> Option<StringType>;
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the status field.
    fn status(&self) -> Option<ContractStatus>;
    /// Returns a reference to the legalState field.
    fn legal_state(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the instantiatesCanonical field.
    fn instantiates_canonical(&self) -> Option<Reference>;
    /// Returns a reference to the instantiatesUri field.
    fn instantiates_uri(&self) -> Option<StringType>;
    /// Returns a reference to the contentDerivative field.
    fn content_derivative(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the issued field.
    fn issued(&self) -> Option<DateTimeType>;
    /// Returns a reference to the applies field.
    fn applies(&self) -> Option<Period>;
    /// Returns a reference to the expirationType field.
    fn expiration_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> &[Reference];
    /// Returns a reference to the authority field.
    fn authority(&self) -> &[Reference];
    /// Returns a reference to the domain field.
    fn domain(&self) -> &[Reference];
    /// Returns a reference to the site field.
    fn site(&self) -> &[Reference];
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the subtitle field.
    fn subtitle(&self) -> Option<StringType>;
    /// Returns a reference to the alias field.
    fn alias(&self) -> &[StringType];
    /// Returns a reference to the author field.
    fn author(&self) -> Option<Reference>;
    /// Returns a reference to the scope field.
    fn scope(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subType field.
    fn sub_type(&self) -> &[CodeableConcept];
    /// Returns a reference to the contentDefinition field.
    fn content_definition(&self) -> Option<ContractContentdefinition>;
    /// Returns a reference to the term field.
    fn term(&self) -> &[ContractTerm];
    /// Returns a reference to the supportingInfo field.
    fn supporting_info(&self) -> &[Reference];
    /// Returns a reference to the relevantHistory field.
    fn relevant_history(&self) -> &[Reference];
    /// Returns a reference to the signer field.
    fn signer(&self) -> &[ContractSigner];
    /// Returns a reference to the friendly field.
    fn friendly(&self) -> &[ContractFriendly];
    /// Returns a reference to the legal field.
    fn legal(&self) -> &[ContractLegal];
    /// Returns a reference to the rule field.
    fn rule(&self) -> &[ContractRule];
}
/// Contract Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a policy or agreement.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Contract
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Contract
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ContractMutators: DomainResourceMutators {
    /// Create a new Contract with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::contract::Contract;
    /// use hl7_fhir_r4_core::traits::contract::ContractMutators;
    ///
    /// let resource = Contract::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the url field and returns self for chaining.
    fn set_url(self, value: String) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ContractStatus) -> Self;
    /// Sets the legalState field and returns self for chaining.
    fn set_legal_state(self, value: CodeableConcept) -> Self;
    /// Sets the instantiatesCanonical field and returns self for chaining.
    fn set_instantiates_canonical(self, value: Reference) -> Self;
    /// Sets the instantiatesUri field and returns self for chaining.
    fn set_instantiates_uri(self, value: String) -> Self;
    /// Sets the contentDerivative field and returns self for chaining.
    fn set_content_derivative(self, value: CodeableConcept) -> Self;
    /// Sets the issued field and returns self for chaining.
    fn set_issued(self, value: String) -> Self;
    /// Sets the applies field and returns self for chaining.
    fn set_applies(self, value: Period) -> Self;
    /// Sets the expirationType field and returns self for chaining.
    fn set_expiration_type(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the subject field and returns self for chaining.
    fn add_subject(self, item: Reference) -> Self;
    /// Sets the authority field and returns self for chaining.
    fn set_authority(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the authority field and returns self for chaining.
    fn add_authority(self, item: Reference) -> Self;
    /// Sets the domain field and returns self for chaining.
    fn set_domain(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the domain field and returns self for chaining.
    fn add_domain(self, item: Reference) -> Self;
    /// Sets the site field and returns self for chaining.
    fn set_site(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the site field and returns self for chaining.
    fn add_site(self, item: Reference) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the subtitle field and returns self for chaining.
    fn set_subtitle(self, value: String) -> Self;
    /// Sets the alias field and returns self for chaining.
    fn set_alias(self, value: Vec<String>) -> Self;
    /// Adds an item to the alias field and returns self for chaining.
    fn add_alias(self, item: String) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Reference) -> Self;
    /// Sets the scope field and returns self for chaining.
    fn set_scope(self, value: CodeableConcept) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the subType field and returns self for chaining.
    fn set_sub_type(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the subType field and returns self for chaining.
    fn add_sub_type(self, item: CodeableConcept) -> Self;
    /// Sets the contentDefinition field and returns self for chaining.
    fn set_content_definition(self, value: ContractContentdefinition) -> Self;
    /// Sets the term field and returns self for chaining.
    fn set_term(self, value: Vec<ContractTerm>) -> Self;
    /// Adds an item to the term field and returns self for chaining.
    fn add_term(self, item: ContractTerm) -> Self;
    /// Sets the supportingInfo field and returns self for chaining.
    fn set_supporting_info(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supportingInfo field and returns self for chaining.
    fn add_supporting_info(self, item: Reference) -> Self;
    /// Sets the relevantHistory field and returns self for chaining.
    fn set_relevant_history(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the relevantHistory field and returns self for chaining.
    fn add_relevant_history(self, item: Reference) -> Self;
    /// Sets the signer field and returns self for chaining.
    fn set_signer(self, value: Vec<ContractSigner>) -> Self;
    /// Adds an item to the signer field and returns self for chaining.
    fn add_signer(self, item: ContractSigner) -> Self;
    /// Sets the friendly field and returns self for chaining.
    fn set_friendly(self, value: Vec<ContractFriendly>) -> Self;
    /// Adds an item to the friendly field and returns self for chaining.
    fn add_friendly(self, item: ContractFriendly) -> Self;
    /// Sets the legal field and returns self for chaining.
    fn set_legal(self, value: Vec<ContractLegal>) -> Self;
    /// Adds an item to the legal field and returns self for chaining.
    fn add_legal(self, item: ContractLegal) -> Self;
    /// Sets the rule field and returns self for chaining.
    fn set_rule(self, value: Vec<ContractRule>) -> Self;
    /// Adds an item to the rule field and returns self for chaining.
    fn add_rule(self, item: ContractRule) -> Self;
}
/// Contract Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a policy or agreement.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Contract
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Contract
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ContractExistence: DomainResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the text field is present (Some).
    fn has_text(&self) -> bool;
    /// Returns true if the contained field is not empty.
    fn has_contained(&self) -> bool;
    /// Returns true if the extension field is not empty.
    fn has_extension(&self) -> bool;
    /// Returns true if the modifier_extension field is not empty.
    fn has_modifier_extension(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the url field is present (Some).
    fn has_url(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the legal_state field is present (Some).
    fn has_legal_state(&self) -> bool;
    /// Returns true if the instantiates_canonical field is present (Some).
    fn has_instantiates_canonical(&self) -> bool;
    /// Returns true if the instantiates_uri field is present (Some).
    fn has_instantiates_uri(&self) -> bool;
    /// Returns true if the content_derivative field is present (Some).
    fn has_content_derivative(&self) -> bool;
    /// Returns true if the issued field is present (Some).
    fn has_issued(&self) -> bool;
    /// Returns true if the applies field is present (Some).
    fn has_applies(&self) -> bool;
    /// Returns true if the expiration_type field is present (Some).
    fn has_expiration_type(&self) -> bool;
    /// Returns true if the subject field is not empty.
    fn has_subject(&self) -> bool;
    /// Returns true if the authority field is not empty.
    fn has_authority(&self) -> bool;
    /// Returns true if the domain field is not empty.
    fn has_domain(&self) -> bool;
    /// Returns true if the site field is not empty.
    fn has_site(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the subtitle field is present (Some).
    fn has_subtitle(&self) -> bool;
    /// Returns true if the alias field is not empty.
    fn has_alias(&self) -> bool;
    /// Returns true if the author field is present (Some).
    fn has_author(&self) -> bool;
    /// Returns true if the scope field is present (Some).
    fn has_scope(&self) -> bool;
    /// Returns true if the topic field is present (Some).
    fn has_topic(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the sub_type field is not empty.
    fn has_sub_type(&self) -> bool;
    /// Returns true if the content_definition field is present (Some).
    fn has_content_definition(&self) -> bool;
    /// Returns true if the term field is not empty.
    fn has_term(&self) -> bool;
    /// Returns true if the supporting_info field is not empty.
    fn has_supporting_info(&self) -> bool;
    /// Returns true if the relevant_history field is not empty.
    fn has_relevant_history(&self) -> bool;
    /// Returns true if the signer field is not empty.
    fn has_signer(&self) -> bool;
    /// Returns true if the friendly field is not empty.
    fn has_friendly(&self) -> bool;
    /// Returns true if the legal field is not empty.
    fn has_legal(&self) -> bool;
    /// Returns true if the rule field is not empty.
    fn has_rule(&self) -> bool;
    /// Returns true if the legally_binding field is present (Some).
    fn has_legally_binding(&self) -> bool;
}
