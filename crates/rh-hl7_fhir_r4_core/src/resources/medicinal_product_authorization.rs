use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicinalProductAuthorization
///
/// The regulatory authorization of a medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductAuthorization
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductAuthorization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductAuthorization {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for the marketing authorization, as assigned by a regulator
    pub identifier: Option<Vec<Identifier>>,
    /// The medicinal product that is being authorized
    pub subject: Option<Reference>,
    /// The country in which the marketing authorization has been granted
    pub country: Option<Vec<CodeableConcept>>,
    /// Jurisdiction within a country
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// The status of the marketing authorization
    pub status: Option<CodeableConcept>,
    /// The date at which the given status has become applicable
    #[serde(rename = "statusDate")]
    pub status_date: Option<DateTimeType>,
    /// Extension element for the 'statusDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_statusDate")]
    pub _status_date: Option<Element>,
    /// The date when a suspended the marketing or the marketing authorization of the product is anticipated to be restored
    #[serde(rename = "restoreDate")]
    pub restore_date: Option<DateTimeType>,
    /// Extension element for the 'restoreDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_restoreDate")]
    pub _restore_date: Option<Element>,
    /// The beginning of the time period in which the marketing authorization is in the specific status shall be specified A complete date consisting of day, month and year shall be specified using the ISO 8601 date format
    #[serde(rename = "validityPeriod")]
    pub validity_period: Option<Period>,
    /// A period of time after authorization before generic product applicatiosn can be submitted
    #[serde(rename = "dataExclusivityPeriod")]
    pub data_exclusivity_period: Option<Period>,
    /// The date when the first authorization was granted by a Medicines Regulatory Agency
    #[serde(rename = "dateOfFirstAuthorization")]
    pub date_of_first_authorization: Option<DateTimeType>,
    /// Extension element for the 'dateOfFirstAuthorization' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dateOfFirstAuthorization")]
    pub _date_of_first_authorization: Option<Element>,
    /// Date of first marketing authorization for a company's new medicinal product in any country in the World
    #[serde(rename = "internationalBirthDate")]
    pub international_birth_date: Option<DateTimeType>,
    /// Extension element for the 'internationalBirthDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_internationalBirthDate")]
    pub _international_birth_date: Option<Element>,
    /// The legal framework against which this authorization is granted
    #[serde(rename = "legalBasis")]
    pub legal_basis: Option<CodeableConcept>,
    /// Authorization in areas within a country
    #[serde(rename = "jurisdictionalAuthorization")]
    pub jurisdictional_authorization:
        Option<Vec<MedicinalProductAuthorizationJurisdictionalauthorization>>,
    /// Marketing Authorization Holder
    pub holder: Option<Reference>,
    /// Medicines Regulatory Agency
    pub regulator: Option<Reference>,
    /// The regulatory procedure for granting or amending a marketing authorization
    pub procedure: Option<MedicinalProductAuthorizationProcedure>,
}
/// MedicinalProductAuthorization nested structure for the 'jurisdictionalAuthorization' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductAuthorizationJurisdictionalauthorization {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The assigned number for the marketing authorization
    pub identifier: Option<Vec<Identifier>>,
    /// Country of authorization
    pub country: Option<CodeableConcept>,
    /// Jurisdiction within a country
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// The legal status of supply in a jurisdiction or region
    #[serde(rename = "legalStatusOfSupply")]
    pub legal_status_of_supply: Option<CodeableConcept>,
    /// The start and expected end date of the authorization
    #[serde(rename = "validityPeriod")]
    pub validity_period: Option<Period>,
}
/// MedicinalProductAuthorization nested structure for the 'procedure' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductAuthorizationProcedure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identifier for this procedure
    pub identifier: Option<Identifier>,
    /// Type of procedure
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Date of procedure (Period)
    #[serde(rename = "datePeriod")]
    pub date_period: Option<Period>,
    /// Date of procedure (dateTime)
    #[serde(rename = "dateDateTime")]
    pub date_date_time: Option<DateTimeType>,
    /// Applcations submitted to obtain a marketing authorization
    pub application: Option<Vec<StringType>>,
}

impl Default for MedicinalProductAuthorization {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            subject: Default::default(),
            country: Default::default(),
            jurisdiction: Default::default(),
            status: Default::default(),
            status_date: Default::default(),
            _status_date: Default::default(),
            restore_date: Default::default(),
            _restore_date: Default::default(),
            validity_period: Default::default(),
            data_exclusivity_period: Default::default(),
            date_of_first_authorization: Default::default(),
            _date_of_first_authorization: Default::default(),
            international_birth_date: Default::default(),
            _international_birth_date: Default::default(),
            legal_basis: Default::default(),
            jurisdictional_authorization: Default::default(),
            holder: Default::default(),
            regulator: Default::default(),
            procedure: Default::default(),
        }
    }
}

impl Default for MedicinalProductAuthorizationJurisdictionalauthorization {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            country: Default::default(),
            jurisdiction: Default::default(),
            legal_status_of_supply: Default::default(),
            validity_period: Default::default(),
        }
    }
}

impl Default for MedicinalProductAuthorizationProcedure {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            type_: Default::default(),
            date_period: Default::default(),
            date_date_time: Default::default(),
            application: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicinalProductAuthorization {
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

impl crate::traits::resource::ResourceMutators for MedicinalProductAuthorization {
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

impl crate::traits::resource::ResourceExistence for MedicinalProductAuthorization {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicinalProductAuthorization {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicinalProductAuthorization {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicinalProductAuthorization {
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

impl crate::traits::medicinal_product_authorization::MedicinalProductAuthorizationAccessors
    for MedicinalProductAuthorization
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn country(&self) -> &[CodeableConcept] {
        self.country.as_deref().unwrap_or(&[])
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Option<CodeableConcept> {
        self.status.clone()
    }
    fn status_date(&self) -> Option<DateTimeType> {
        self.status_date.clone()
    }
    fn restore_date(&self) -> Option<DateTimeType> {
        self.restore_date.clone()
    }
    fn validity_period(&self) -> Option<Period> {
        self.validity_period.clone()
    }
    fn data_exclusivity_period(&self) -> Option<Period> {
        self.data_exclusivity_period.clone()
    }
    fn date_of_first_authorization(&self) -> Option<DateTimeType> {
        self.date_of_first_authorization.clone()
    }
    fn international_birth_date(&self) -> Option<DateTimeType> {
        self.international_birth_date.clone()
    }
    fn legal_basis(&self) -> Option<CodeableConcept> {
        self.legal_basis.clone()
    }
    fn jurisdictional_authorization(
        &self,
    ) -> &[MedicinalProductAuthorizationJurisdictionalauthorization] {
        self.jurisdictional_authorization.as_deref().unwrap_or(&[])
    }
    fn holder(&self) -> Option<Reference> {
        self.holder.clone()
    }
    fn regulator(&self) -> Option<Reference> {
        self.regulator.clone()
    }
    fn procedure(&self) -> Option<MedicinalProductAuthorizationProcedure> {
        self.procedure.clone()
    }
}

impl crate::traits::medicinal_product_authorization::MedicinalProductAuthorizationMutators
    for MedicinalProductAuthorization
{
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
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_country(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.country = Some(value);
        resource
    }
    fn add_country(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.country.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = Some(value);
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .jurisdiction
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_status_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.status_date = Some(value);
        resource
    }
    fn set_restore_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.restore_date = Some(value);
        resource
    }
    fn set_validity_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.validity_period = Some(value);
        resource
    }
    fn set_data_exclusivity_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.data_exclusivity_period = Some(value);
        resource
    }
    fn set_date_of_first_authorization(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date_of_first_authorization = Some(value);
        resource
    }
    fn set_international_birth_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.international_birth_date = Some(value);
        resource
    }
    fn set_legal_basis(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.legal_basis = Some(value);
        resource
    }
    fn set_jurisdictional_authorization(
        self,
        value: Vec<MedicinalProductAuthorizationJurisdictionalauthorization>,
    ) -> Self {
        let mut resource = self.clone();
        resource.jurisdictional_authorization = Some(value);
        resource
    }
    fn add_jurisdictional_authorization(
        self,
        item: MedicinalProductAuthorizationJurisdictionalauthorization,
    ) -> Self {
        let mut resource = self.clone();
        resource
            .jurisdictional_authorization
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_holder(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.holder = Some(value);
        resource
    }
    fn set_regulator(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.regulator = Some(value);
        resource
    }
    fn set_procedure(self, value: MedicinalProductAuthorizationProcedure) -> Self {
        let mut resource = self.clone();
        resource.procedure = Some(value);
        resource
    }
}

impl crate::traits::medicinal_product_authorization::MedicinalProductAuthorizationExistence
    for MedicinalProductAuthorization
{
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
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_country(&self) -> bool {
        self.country.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_jurisdiction(&self) -> bool {
        self.jurisdiction.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_status_date(&self) -> bool {
        self.status_date.is_some()
    }
    fn has_restore_date(&self) -> bool {
        self.restore_date.is_some()
    }
    fn has_validity_period(&self) -> bool {
        self.validity_period.is_some()
    }
    fn has_data_exclusivity_period(&self) -> bool {
        self.data_exclusivity_period.is_some()
    }
    fn has_date_of_first_authorization(&self) -> bool {
        self.date_of_first_authorization.is_some()
    }
    fn has_international_birth_date(&self) -> bool {
        self.international_birth_date.is_some()
    }
    fn has_legal_basis(&self) -> bool {
        self.legal_basis.is_some()
    }
    fn has_jurisdictional_authorization(&self) -> bool {
        self.jurisdictional_authorization
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_holder(&self) -> bool {
        self.holder.is_some()
    }
    fn has_regulator(&self) -> bool {
        self.regulator.is_some()
    }
    fn has_procedure(&self) -> bool {
        self.procedure.is_some()
    }
}
