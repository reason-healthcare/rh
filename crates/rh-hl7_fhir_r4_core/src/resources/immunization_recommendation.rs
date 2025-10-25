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
    rh_foundation::Invariant::new("imr-1", rh_foundation::Severity::Error, "One of vaccineCode or targetDisease SHALL be present", "vaccineCode.exists() or targetDisease.exists()").with_xpath("exists(f:vaccineCode) or exists(f:targetDisease)"),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ImmunizationRecommendation.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImmunizationRecommendation.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ImmunizationRecommendation.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ImmunizationRecommendation.contained", 0, None),
            rh_foundation::ElementCardinality::new("ImmunizationRecommendation.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.identifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.patient",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ImmunizationRecommendation.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.authority",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation",
                1,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.vaccineCode",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.targetDisease",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.contraindicatedVaccineCode",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.forecastStatus",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.forecastReason",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.dateCriterion",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.dateCriterion.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.dateCriterion.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.dateCriterion.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.dateCriterion.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.dateCriterion.value",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.series",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.doseNumber[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.seriesDoses[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.supportingImmunization",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ImmunizationRecommendation.recommendation.supportingPatientInformation",
                0,
                None,
            ),
        ]
    });

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

impl crate::validation::ValidatableResource for ImmunizationRecommendation {
    fn resource_type(&self) -> &'static str {
        "ImmunizationRecommendation"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ImmunizationRecommendation")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::immunization_recommendation::{
    ImmunizationRecommendationAccessors, ImmunizationRecommendationExistence,
    ImmunizationRecommendationMutators,
};
