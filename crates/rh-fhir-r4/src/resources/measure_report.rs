use crate::bindings::measure_report_status::MeasureReportStatus;
use crate::bindings::measure_report_type::MeasureReportType;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MeasureReport
///
/// The MeasureReport resource contains the results of the calculation of a measure; and optionally a reference to the resources involved in that calculation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MeasureReport
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MeasureReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureReport {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Additional identifier for the MeasureReport
    pub identifier: Option<Vec<Identifier>>,
    /// complete | pending | error
    pub status: MeasureReportStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// individual | subject-list | summary | data-collection
    #[serde(rename = "type")]
    pub type_: MeasureReportType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// What measure was calculated
    pub measure: StringType,
    /// Extension element for the 'measure' primitive field. Contains metadata and extensions.
    pub _measure: Option<Element>,
    /// What individual(s) the report is for
    pub subject: Option<Reference>,
    /// When the report was generated
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Who is reporting the data
    pub reporter: Option<Reference>,
    /// What period the report covers
    pub period: Period,
    /// increase | decrease
    #[serde(rename = "improvementNotation")]
    pub improvement_notation: Option<CodeableConcept>,
    /// Measure results for each group
    pub group: Option<Vec<MeasureReportGroup>>,
    /// What data was used to calculate the measure score
    #[serde(rename = "evaluatedResource")]
    pub evaluated_resource: Option<Vec<Reference>>,
}
/// MeasureReport nested structure for the 'group' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureReportGroup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The populations in the group
    pub population: Option<Vec<MeasureReportGroupPopulation>>,
    /// Stratification results
    pub stratifier: Option<Vec<MeasureReportGroupStratifier>>,
    /// Meaning of the group
    pub code: Option<CodeableConcept>,
    /// What score this group achieved
    #[serde(rename = "measureScore")]
    pub measure_score: Option<Quantity>,
}
/// MeasureReportGroupStratifierStratum nested structure for the 'component' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureReportGroupStratifierStratumComponent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What stratifier component of the group
    pub code: CodeableConcept,
    /// The stratum component value, e.g. male
    pub value: CodeableConcept,
}
/// MeasureReportGroup nested structure for the 'stratifier' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureReportGroupStratifier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What stratifier of the group
    pub code: Option<Vec<CodeableConcept>>,
}
/// MeasureReportGroupStratifierStratum nested structure for the 'population' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureReportGroupStratifierStratumPopulation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    ///
    /// Binding: extensible (The type of population (e.g. initial, numerator, denominator, etc.).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/measure-population
    pub code: Option<CodeableConcept>,
    /// Size of the population
    pub count: Option<IntegerType>,
    /// Extension element for the 'count' primitive field. Contains metadata and extensions.
    pub _count: Option<Element>,
    /// For subject-list reports, the subject results in this population
    #[serde(rename = "subjectResults")]
    pub subject_results: Option<Reference>,
}
/// MeasureReportGroup nested structure for the 'population' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureReportGroupPopulation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    ///
    /// Binding: extensible (The type of population (e.g. initial, numerator, denominator, etc.).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/measure-population
    pub code: Option<CodeableConcept>,
    /// Size of the population
    pub count: Option<IntegerType>,
    /// Extension element for the 'count' primitive field. Contains metadata and extensions.
    pub _count: Option<Element>,
    /// For subject-list reports, the subject results in this population
    #[serde(rename = "subjectResults")]
    pub subject_results: Option<Reference>,
}
/// MeasureReportGroupStratifier nested structure for the 'stratum' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureReportGroupStratifierStratum {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The stratum value, e.g. male
    pub value: Option<CodeableConcept>,
    /// What score this stratum achieved
    #[serde(rename = "measureScore")]
    pub measure_score: Option<Quantity>,
}

impl Default for MeasureReport {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: MeasureReportStatus::default(),
            _status: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            measure: StringType::default(),
            _measure: Default::default(),
            subject: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            reporter: Default::default(),
            period: Period::default(),
            improvement_notation: Default::default(),
            group: Default::default(),
            evaluated_resource: Default::default(),
        }
    }
}

impl Default for MeasureReportGroup {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            population: Default::default(),
            stratifier: Default::default(),
            code: Default::default(),
            measure_score: Default::default(),
        }
    }
}

impl Default for MeasureReportGroupStratifierStratumComponent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            value: Default::default(),
        }
    }
}

impl Default for MeasureReportGroupStratifier {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
        }
    }
}

impl Default for MeasureReportGroupStratifierStratumPopulation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            count: Default::default(),
            _count: Default::default(),
            subject_results: Default::default(),
        }
    }
}

impl Default for MeasureReportGroupPopulation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            count: Default::default(),
            _count: Default::default(),
            subject_results: Default::default(),
        }
    }
}

impl Default for MeasureReportGroupStratifierStratum {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            value: Default::default(),
            measure_score: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MeasureReport {
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

impl crate::traits::resource::ResourceMutators for MeasureReport {
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

impl crate::traits::resource::ResourceExistence for MeasureReport {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MeasureReport {
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

impl crate::traits::domain_resource::DomainResourceMutators for MeasureReport {
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

impl crate::traits::domain_resource::DomainResourceExistence for MeasureReport {
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

impl crate::traits::measure_report::MeasureReportAccessors for MeasureReport {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> MeasureReportStatus {
        self.status.clone()
    }
    fn type_(&self) -> MeasureReportType {
        self.type_.clone()
    }
    fn measure(&self) -> StringType {
        self.measure.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn reporter(&self) -> Option<Reference> {
        self.reporter.clone()
    }
    fn period(&self) -> Period {
        self.period.clone()
    }
    fn improvement_notation(&self) -> Option<CodeableConcept> {
        self.improvement_notation.clone()
    }
    fn group(&self) -> &[MeasureReportGroup] {
        self.group.as_deref().unwrap_or(&[])
    }
    fn evaluated_resource(&self) -> &[Reference] {
        self.evaluated_resource.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::measure_report::MeasureReportMutators for MeasureReport {
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
    fn set_status(self, value: MeasureReportStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_type_(self, value: MeasureReportType) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn set_measure(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.measure = value;
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
    fn set_reporter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.reporter = Some(value);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = value;
        resource
    }
    fn set_improvement_notation(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.improvement_notation = Some(value);
        resource
    }
    fn set_group(self, value: Vec<MeasureReportGroup>) -> Self {
        let mut resource = self.clone();
        resource.group = Some(value);
        resource
    }
    fn add_group(self, item: MeasureReportGroup) -> Self {
        let mut resource = self.clone();
        resource.group.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_evaluated_resource(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.evaluated_resource = Some(value);
        resource
    }
    fn add_evaluated_resource(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .evaluated_resource
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::measure_report::MeasureReportExistence for MeasureReport {
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
    fn has_status(&self) -> bool {
        true
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_measure(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_reporter(&self) -> bool {
        self.reporter.is_some()
    }
    fn has_period(&self) -> bool {
        true
    }
    fn has_improvement_notation(&self) -> bool {
        self.improvement_notation.is_some()
    }
    fn has_group(&self) -> bool {
        self.group.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_evaluated_resource(&self) -> bool {
        self.evaluated_resource
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}
