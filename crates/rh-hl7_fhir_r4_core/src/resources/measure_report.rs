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
/// MeasureReportGroup nested structure for the 'stratifier' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureReportGroupStratifier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What stratifier of the group
    pub code: Option<Vec<CodeableConcept>>,
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

impl Default for MeasureReportGroupStratifierStratumComponent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            value: Default::default(),
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

impl Default for MeasureReportGroupStratifier {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
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
    rh_foundation::Invariant::new("mrp-1", rh_foundation::Severity::Error, "Measure Reports used for data collection SHALL NOT communicate group and score information", "(type != 'data-collection') or group.exists().not()").with_xpath("not(f:kind/@value='data-collection') or not(exists(f:group))"),
    rh_foundation::Invariant::new("mrp-2", rh_foundation::Severity::Error, "Stratifiers SHALL be either a single criteria or a set of criteria components", "group.stratifier.stratum.all(value.exists() xor component.exists())").with_xpath("not(f:kind/@value='data-collection') or not(exists(f:group))"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("MeasureReport.improvementNotation", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/measure-improvement-notation|4.0.1").with_description("Observation values that indicate what change in a measurement value or score is indicative of an improvement in the measured item or scored issue."),
    rh_foundation::ElementBinding::new("MeasureReport.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/measure-report-status|4.0.1").with_description("The status of the measure report."),
    rh_foundation::ElementBinding::new("MeasureReport.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/measure-report-type|4.0.1").with_description("The type of the measure report."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("MeasureReport.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.contained", 0, None),
            rh_foundation::ElementCardinality::new("MeasureReport.extension", 0, None),
            rh_foundation::ElementCardinality::new("MeasureReport.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("MeasureReport.identifier", 0, None),
            rh_foundation::ElementCardinality::new("MeasureReport.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.measure", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.reporter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.period", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.improvementNotation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.group", 0, None),
            rh_foundation::ElementCardinality::new("MeasureReport.group.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.group.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MeasureReport.group.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.group.population", 0, None),
            rh_foundation::ElementCardinality::new("MeasureReport.group.population.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.population.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.population.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.population.code",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.population.count",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.population.subjectResults",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MeasureReport.group.measureScore", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MeasureReport.group.stratifier", 0, None),
            rh_foundation::ElementCardinality::new("MeasureReport.group.stratifier.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MeasureReport.group.stratifier.code", 0, None),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.value",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.component",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.component.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.component.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.component.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.component.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.component.value",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.population",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.population.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.population.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.population.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.population.code",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.population.count",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.population.subjectResults",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MeasureReport.group.stratifier.stratum.measureScore",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MeasureReport.evaluatedResource", 0, None),
        ]
    });

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

impl crate::validation::ValidatableResource for MeasureReport {
    fn resource_type(&self) -> &'static str {
        "MeasureReport"
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
        Some("http://hl7.org/fhir/StructureDefinition/MeasureReport")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::measure_report::{
    MeasureReportAccessors, MeasureReportExistence, MeasureReportMutators,
};
