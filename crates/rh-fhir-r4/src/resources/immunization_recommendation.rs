use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ImmunizationRecommendation
///
/// A patient's point-in-time set of recommendations (i.e. forecasting) according to a published schedule with optional supporting justification.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImmunizationRecommendation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImmunizationRecommendation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunizationRecommendation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Who this profile is for
    pub patient: Reference,
    /// Date recommendation(s) created
    pub date: DateTimeType,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Who is responsible for protocol
    pub authority: Option<Reference>,
    /// Vaccine administration recommendations
    pub recommendation: Vec<ImmunizationRecommendationRecommendation>,
}
/// ImmunizationRecommendation nested structure for the 'recommendation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunizationRecommendationRecommendation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Dates governing proposed immunization
    #[serde(rename = "dateCriterion")]
    pub date_criterion: Option<Vec<ImmunizationRecommendationRecommendationDatecriterion>>,
    /// Vaccine  or vaccine group recommendation applies to
    ///
    /// Binding: example (The type of vaccine administered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/vaccine-code
    #[serde(rename = "vaccineCode")]
    pub vaccine_code: Option<Vec<CodeableConcept>>,
    /// Disease to be immunized against
    ///
    /// Binding: example (The disease that the recommended vaccination targets.)
    ///
    /// Available values:
    /// - `1857005`
    /// - `397430003`
    /// - `14189004`
    /// - `36989005`
    /// - `36653000`
    /// - `76902006`
    /// - `709410003`
    /// - `27836007`
    /// - `398102009`
    #[serde(rename = "targetDisease")]
    pub target_disease: Option<CodeableConcept>,
    /// Vaccine which is contraindicated to fulfill the recommendation
    ///
    /// Binding: example (The type of vaccine administered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/vaccine-code
    #[serde(rename = "contraindicatedVaccineCode")]
    pub contraindicated_vaccine_code: Option<Vec<CodeableConcept>>,
    /// Vaccine recommendation status
    ///
    /// Binding: example (The patient's status with respect to a vaccination protocol.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/immunization-recommendation-status
    #[serde(rename = "forecastStatus")]
    pub forecast_status: CodeableConcept,
    /// Vaccine administration status reason
    ///
    /// Binding: example (The reason for the patient's status with respect to a vaccination protocol.)
    ///
    /// Available values:
    /// - `77176002`
    /// - `77386006`
    #[serde(rename = "forecastReason")]
    pub forecast_reason: Option<Vec<CodeableConcept>>,
    /// Protocol details
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Name of vaccination series
    pub series: Option<StringType>,
    /// Extension element for the 'series' primitive field. Contains metadata and extensions.
    pub _series: Option<Element>,
    /// Recommended dose number within series (positiveInt)
    #[serde(rename = "doseNumberPositiveInt")]
    pub dose_number_positive_int: Option<PositiveIntType>,
    /// Recommended dose number within series (string)
    #[serde(rename = "doseNumberString")]
    pub dose_number_string: Option<StringType>,
    /// Recommended number of doses for immunity (positiveInt)
    #[serde(rename = "seriesDosesPositiveInt")]
    pub series_doses_positive_int: Option<PositiveIntType>,
    /// Recommended number of doses for immunity (string)
    #[serde(rename = "seriesDosesString")]
    pub series_doses_string: Option<StringType>,
    /// Past immunizations supporting recommendation
    #[serde(rename = "supportingImmunization")]
    pub supporting_immunization: Option<Vec<Reference>>,
    /// Patient observations supporting recommendation
    #[serde(rename = "supportingPatientInformation")]
    pub supporting_patient_information: Option<Vec<Reference>>,
}
/// ImmunizationRecommendationRecommendation nested structure for the 'dateCriterion' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunizationRecommendationRecommendationDatecriterion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of date
    ///
    /// Binding: example (Classifies date criterion with respect to conveying information about a patient's vaccination status (e.g. due date, latest to give date, etc.).)
    ///
    /// Available values:
    /// - `30981-5`
    /// - `30980-7`
    /// - `59777-3`
    /// - `59778-1`
    pub code: CodeableConcept,
    /// Recommended date
    pub value: DateTimeType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}

impl Default for ImmunizationRecommendation {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            patient: Reference::default(),
            date: DateTimeType::default(),
            _date: Default::default(),
            authority: Default::default(),
            recommendation: Vec::new(),
        }
    }
}

impl Default for ImmunizationRecommendationRecommendation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            date_criterion: Default::default(),
            vaccine_code: Default::default(),
            target_disease: Default::default(),
            contraindicated_vaccine_code: Default::default(),
            forecast_status: CodeableConcept::default(),
            forecast_reason: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            series: Default::default(),
            _series: Default::default(),
            dose_number_positive_int: Default::default(),
            dose_number_string: Default::default(),
            series_doses_positive_int: Default::default(),
            series_doses_string: Default::default(),
            supporting_immunization: Default::default(),
            supporting_patient_information: Default::default(),
        }
    }
}

impl Default for ImmunizationRecommendationRecommendationDatecriterion {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            value: Default::default(),
            _value: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ImmunizationRecommendation {
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

impl crate::traits::resource::ResourceMutators for ImmunizationRecommendation {
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

impl crate::traits::resource::ResourceExistence for ImmunizationRecommendation {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ImmunizationRecommendation {
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

impl crate::traits::domain_resource::DomainResourceMutators for ImmunizationRecommendation {
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

impl crate::traits::domain_resource::DomainResourceExistence for ImmunizationRecommendation {
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

impl crate::traits::immunization_recommendation::ImmunizationRecommendationAccessors
    for ImmunizationRecommendation
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn date(&self) -> DateTimeType {
        self.date.clone()
    }
    fn authority(&self) -> Option<Reference> {
        self.authority.clone()
    }
    fn recommendation(&self) -> &[ImmunizationRecommendationRecommendation] {
        &self.recommendation
    }
}

impl crate::traits::immunization_recommendation::ImmunizationRecommendationMutators
    for ImmunizationRecommendation
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
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = value;
        resource
    }
    fn set_authority(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.authority = Some(value);
        resource
    }
    fn set_recommendation(self, value: Vec<ImmunizationRecommendationRecommendation>) -> Self {
        let mut resource = self.clone();
        resource.recommendation = value;
        resource
    }
    fn add_recommendation(self, item: ImmunizationRecommendationRecommendation) -> Self {
        let mut resource = self.clone();
        resource.recommendation.push(item);
        resource
    }
}

impl crate::traits::immunization_recommendation::ImmunizationRecommendationExistence
    for ImmunizationRecommendation
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
    fn has_patient(&self) -> bool {
        true
    }
    fn has_date(&self) -> bool {
        true
    }
    fn has_authority(&self) -> bool {
        self.authority.is_some()
    }
    fn has_recommendation(&self) -> bool {
        !self.recommendation.is_empty()
    }
}
