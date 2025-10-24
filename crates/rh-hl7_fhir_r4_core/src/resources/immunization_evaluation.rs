use crate::bindings::immunization_evaluation_status::ImmunizationEvaluationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ImmunizationEvaluation
///
/// Describes a comparison of an immunization event against published recommendations to determine if the administration is "valid" in relation to those  recommendations.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImmunizationEvaluation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImmunizationEvaluation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunizationEvaluation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier
    pub identifier: Option<Vec<Identifier>>,
    /// completed | entered-in-error
    pub status: ImmunizationEvaluationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Who this evaluation is for
    pub patient: Reference,
    /// Date evaluation was performed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Who is responsible for publishing the recommendations
    pub authority: Option<Reference>,
    /// Evaluation target disease
    ///
    /// Binding: example (The vaccine preventable disease the dose is being evaluated against.)
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
    pub target_disease: CodeableConcept,
    /// Immunization being evaluated
    #[serde(rename = "immunizationEvent")]
    pub immunization_event: Reference,
    /// Status of the dose relative to published recommendations
    ///
    /// Binding: example (The status of the administered dose relative to the published recommendations for the target disease.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/immunization-evaluation-dose-status
    #[serde(rename = "doseStatus")]
    pub dose_status: CodeableConcept,
    /// Reason for the dose status
    ///
    /// Binding: example (The reason the dose status was assigned.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/immunization-evaluation-dose-status-reason
    #[serde(rename = "doseStatusReason")]
    pub dose_status_reason: Option<Vec<CodeableConcept>>,
    /// Evaluation notes
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Name of vaccine series
    pub series: Option<StringType>,
    /// Extension element for the 'series' primitive field. Contains metadata and extensions.
    pub _series: Option<Element>,
    /// Dose number within series (positiveInt)
    #[serde(rename = "doseNumberPositiveInt")]
    pub dose_number_positive_int: Option<PositiveIntType>,
    /// Dose number within series (string)
    #[serde(rename = "doseNumberString")]
    pub dose_number_string: Option<StringType>,
    /// Recommended number of doses for immunity (positiveInt)
    #[serde(rename = "seriesDosesPositiveInt")]
    pub series_doses_positive_int: Option<PositiveIntType>,
    /// Recommended number of doses for immunity (string)
    #[serde(rename = "seriesDosesString")]
    pub series_doses_string: Option<StringType>,
}

impl Default for ImmunizationEvaluation {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: ImmunizationEvaluationStatus::default(),
            _status: Default::default(),
            patient: Reference::default(),
            date: Default::default(),
            _date: Default::default(),
            authority: Default::default(),
            target_disease: CodeableConcept::default(),
            immunization_event: Reference::default(),
            dose_status: CodeableConcept::default(),
            dose_status_reason: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            series: Default::default(),
            _series: Default::default(),
            dose_number_positive_int: Default::default(),
            dose_number_string: Default::default(),
            series_doses_positive_int: Default::default(),
            series_doses_string: Default::default(),
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
impl crate::traits::resource::ResourceAccessors for ImmunizationEvaluation {
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

impl crate::traits::resource::ResourceMutators for ImmunizationEvaluation {
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

impl crate::traits::resource::ResourceExistence for ImmunizationEvaluation {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ImmunizationEvaluation {
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

impl crate::traits::domain_resource::DomainResourceMutators for ImmunizationEvaluation {
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

impl crate::traits::domain_resource::DomainResourceExistence for ImmunizationEvaluation {
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

impl crate::traits::immunization_evaluation::ImmunizationEvaluationAccessors
    for ImmunizationEvaluation
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ImmunizationEvaluationStatus {
        self.status.clone()
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn authority(&self) -> Option<Reference> {
        self.authority.clone()
    }
    fn target_disease(&self) -> CodeableConcept {
        self.target_disease.clone()
    }
    fn immunization_event(&self) -> Reference {
        self.immunization_event.clone()
    }
    fn dose_status(&self) -> CodeableConcept {
        self.dose_status.clone()
    }
    fn dose_status_reason(&self) -> &[CodeableConcept] {
        self.dose_status_reason.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn series(&self) -> Option<StringType> {
        self.series.clone()
    }
}

impl crate::traits::immunization_evaluation::ImmunizationEvaluationMutators
    for ImmunizationEvaluation
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
    fn set_status(self, value: ImmunizationEvaluationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_authority(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.authority = Some(value);
        resource
    }
    fn set_target_disease(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.target_disease = value;
        resource
    }
    fn set_immunization_event(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.immunization_event = value;
        resource
    }
    fn set_dose_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.dose_status = value;
        resource
    }
    fn set_dose_status_reason(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.dose_status_reason = Some(value);
        resource
    }
    fn add_dose_status_reason(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .dose_status_reason
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_series(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.series = Some(value);
        resource
    }
}

impl crate::traits::immunization_evaluation::ImmunizationEvaluationExistence
    for ImmunizationEvaluation
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
    fn has_dose_number(&self) -> bool {
        self.dose_number_positive_int.is_some() || self.dose_number_string.is_some()
    }
    fn has_series_doses(&self) -> bool {
        self.series_doses_positive_int.is_some() || self.series_doses_string.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_patient(&self) -> bool {
        true
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_authority(&self) -> bool {
        self.authority.is_some()
    }
    fn has_target_disease(&self) -> bool {
        true
    }
    fn has_immunization_event(&self) -> bool {
        true
    }
    fn has_dose_status(&self) -> bool {
        true
    }
    fn has_dose_status_reason(&self) -> bool {
        self.dose_status_reason
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_series(&self) -> bool {
        self.series.is_some()
    }
}

impl crate::validation::ValidatableResource for ImmunizationEvaluation {
    fn resource_type(&self) -> &'static str {
        "ImmunizationEvaluation"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ImmunizationEvaluation")
    }
}
