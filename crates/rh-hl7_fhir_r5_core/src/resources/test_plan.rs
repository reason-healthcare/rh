use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// TestPlan
///
/// A plan for executing testing on an artifact or specifications
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TestPlan
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: TestPlan
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPlan {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this test plan, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Business identifier identifier for the test plan
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the test plan
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this test plan (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this test plan (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the test plan
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction where the test plan applies (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this test plan is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<StringType>,
    /// Extension element for the 'copyrightLabel' primitive field. Contains metadata and extensions.
    #[serde(rename = "_copyrightLabel")]
    pub _copyright_label: Option<Element>,
    /// The category of the Test Plan - can be acceptance, unit, performance
    ///
    /// Binding: example (The high-level category for this plan.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/testscript-scope-phase-codes
    pub category: Option<Vec<CodeableConcept>>,
    /// What is being tested with this Test Plan - a conformance resource, or narrative criteria, or an external reference
    pub scope: Option<Vec<Reference>>,
    /// A description of test tools to be used in the test plan - narrative for now
    #[serde(rename = "testTools")]
    pub test_tools: Option<StringType>,
    /// Extension element for the 'testTools' primitive field. Contains metadata and extensions.
    #[serde(rename = "_testTools")]
    pub _test_tools: Option<Element>,
    /// The required criteria to execute the test plan - e.g. preconditions, previous tests
    pub dependency: Option<Vec<TestPlanDependency>>,
    /// The threshold or criteria for the test plan to be considered successfully executed - narrative
    #[serde(rename = "exitCriteria")]
    pub exit_criteria: Option<StringType>,
    /// Extension element for the 'exitCriteria' primitive field. Contains metadata and extensions.
    #[serde(rename = "_exitCriteria")]
    pub _exit_criteria: Option<Element>,
    /// The test cases that constitute this plan
    #[serde(rename = "testCase")]
    pub test_case: Option<Vec<TestPlanTestcase>>,
}
/// TestPlanTestcase nested structure for the 'dependency' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPlanTestcaseDependency {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Description of the criteria
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Link to predecessor test plans
    pub predecessor: Option<Reference>,
}
/// TestPlanTestcase nested structure for the 'testRun' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPlanTestcaseTestrun {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The narrative description of the tests
    pub narrative: Option<StringType>,
    /// Extension element for the 'narrative' primitive field. Contains metadata and extensions.
    pub _narrative: Option<Element>,
}
/// TestPlanTestcase nested structure for the 'testData' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPlanTestcaseTestdata {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of test data description, e.g. 'synthea'
    #[serde(rename = "type")]
    pub type_: Coding,
    /// The actual test resources when they exist
    pub content: Option<Reference>,
    /// Pointer to a definition of test resources - narrative or structured e.g. synthetic data generation, etc (string)
    #[serde(rename = "sourceString")]
    pub source_string: Option<StringType>,
    /// Pointer to a definition of test resources - narrative or structured e.g. synthetic data generation, etc (Reference)
    #[serde(rename = "sourceReference")]
    pub source_reference: Option<Reference>,
}
/// TestPlan nested structure for the 'testCase' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPlanTestcase {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The actual test to be executed
    #[serde(rename = "testRun")]
    pub test_run: Option<Vec<TestPlanTestcaseTestrun>>,
    /// Required criteria to execute the test case
    pub dependency: Option<Vec<TestPlanTestcaseDependency>>,
    /// The test data used in the test case
    #[serde(rename = "testData")]
    pub test_data: Option<Vec<TestPlanTestcaseTestdata>>,
    /// Test assertions or expectations
    pub assertion: Option<Vec<TestPlanTestcaseAssertion>>,
    /// Sequence of test case in the test plan
    pub sequence: Option<IntegerType>,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// The scope or artifact covered by the case
    pub scope: Option<Vec<Reference>>,
}
/// TestPlanTestcase nested structure for the 'assertion' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPlanTestcaseAssertion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Assertion type - for example 'informative' or 'required'
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// The focus or object of the assertion
    pub object: Option<Vec<CodeableReference>>,
    /// The actual result assertion
    pub result: Option<Vec<CodeableReference>>,
}
/// TestPlan nested structure for the 'dependency' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPlanDependency {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Description of the dependency criterium
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Link to predecessor test plans
    pub predecessor: Option<Reference>,
}

impl Default for TestPlan {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            version_algorithm_string: Default::default(),
            version_algorithm_coding: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            use_context: Default::default(),
            jurisdiction: Default::default(),
            purpose: Default::default(),
            _purpose: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            copyright_label: Default::default(),
            _copyright_label: Default::default(),
            category: Default::default(),
            scope: Default::default(),
            test_tools: Default::default(),
            _test_tools: Default::default(),
            dependency: Default::default(),
            exit_criteria: Default::default(),
            _exit_criteria: Default::default(),
            test_case: Default::default(),
        }
    }
}

impl Default for TestPlanTestcaseDependency {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            predecessor: Default::default(),
        }
    }
}

impl Default for TestPlanTestcaseTestrun {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            narrative: Default::default(),
            _narrative: Default::default(),
        }
    }
}

impl Default for TestPlanTestcaseTestdata {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            content: Default::default(),
            source_string: Default::default(),
            source_reference: Default::default(),
        }
    }
}

impl Default for TestPlanTestcase {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            test_run: Default::default(),
            dependency: Default::default(),
            test_data: Default::default(),
            assertion: Default::default(),
            sequence: Default::default(),
            _sequence: Default::default(),
            scope: Default::default(),
        }
    }
}

impl Default for TestPlanTestcaseAssertion {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            object: Default::default(),
            result: Default::default(),
        }
    }
}

impl Default for TestPlanDependency {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            predecessor: Default::default(),
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
    rh_foundation::Invariant::new("cnl-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.exists() implies name.matches('^[A-Z]([A-Za-z0-9_]){1,254}$')"),
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
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
                "TestPlan.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "TestPlan.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("The lifecycle status of an artifact."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("TestPlan.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.contained", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.extension", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.identifier", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.contact", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.useContext", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.category", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.scope", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.testTools", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.dependency", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.dependency.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.dependency.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestPlan.dependency.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestPlan.dependency.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.dependency.predecessor", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.exitCriteria", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.testCase", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.extension", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.sequence", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.scope", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.dependency", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.dependency.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.dependency.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.dependency.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.dependency.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.dependency.predecessor",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.testRun", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.testRun.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.testRun.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.testRun.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.testRun.narrative",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.testRun.script", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.testRun.script.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.testRun.script.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.testRun.script.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.testRun.script.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.testRun.script.source[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.testData", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.testData.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.testData.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.testData.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.testData.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.testData.content",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.testData.source[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.assertion", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.assertion.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.assertion.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestPlan.testCase.assertion.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.assertion.type", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.assertion.object", 0, None),
            rh_foundation::ElementCardinality::new("TestPlan.testCase.assertion.result", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for TestPlan {
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

impl crate::traits::resource::ResourceMutators for TestPlan {
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

impl crate::traits::resource::ResourceExistence for TestPlan {
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

impl crate::traits::domain_resource::DomainResourceAccessors for TestPlan {
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

impl crate::traits::domain_resource::DomainResourceMutators for TestPlan {
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

impl crate::traits::domain_resource::DomainResourceExistence for TestPlan {
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

impl crate::traits::test_plan::TestPlanAccessors for TestPlan {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn experimental(&self) -> Option<BooleanType> {
        self.experimental
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn publisher(&self) -> Option<StringType> {
        self.publisher.clone()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_deref().unwrap_or(&[])
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_deref().unwrap_or(&[])
    }
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn copyright_label(&self) -> Option<StringType> {
        self.copyright_label.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn scope(&self) -> &[Reference] {
        self.scope.as_deref().unwrap_or(&[])
    }
    fn test_tools(&self) -> Option<StringType> {
        self.test_tools.clone()
    }
    fn dependency(&self) -> &[TestPlanDependency] {
        self.dependency.as_deref().unwrap_or(&[])
    }
    fn exit_criteria(&self) -> Option<StringType> {
        self.exit_criteria.clone()
    }
    fn test_case(&self) -> &[TestPlanTestcase] {
        self.test_case.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::test_plan::TestPlanMutators for TestPlan {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
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
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
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
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_experimental(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.experimental = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_publisher(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.publisher = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = Some(value);
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.get_or_insert_with(Vec::new).push(item);
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
    fn set_purpose(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.purpose = Some(value);
        resource
    }
    fn set_copyright(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright = Some(value);
        resource
    }
    fn set_copyright_label(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright_label = Some(value);
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_scope(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.scope = Some(value);
        resource
    }
    fn add_scope(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.scope.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_test_tools(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.test_tools = Some(value);
        resource
    }
    fn set_dependency(self, value: Vec<TestPlanDependency>) -> Self {
        let mut resource = self.clone();
        resource.dependency = Some(value);
        resource
    }
    fn add_dependency(self, item: TestPlanDependency) -> Self {
        let mut resource = self.clone();
        resource.dependency.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_exit_criteria(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.exit_criteria = Some(value);
        resource
    }
    fn set_test_case(self, value: Vec<TestPlanTestcase>) -> Self {
        let mut resource = self.clone();
        resource.test_case = Some(value);
        resource
    }
    fn add_test_case(self, item: TestPlanTestcase) -> Self {
        let mut resource = self.clone();
        resource.test_case.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::test_plan::TestPlanExistence for TestPlan {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_experimental(&self) -> bool {
        self.experimental.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_use_context(&self) -> bool {
        self.use_context.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_jurisdiction(&self) -> bool {
        self.jurisdiction.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_copyright_label(&self) -> bool {
        self.copyright_label.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_scope(&self) -> bool {
        self.scope.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_test_tools(&self) -> bool {
        self.test_tools.is_some()
    }
    fn has_dependency(&self) -> bool {
        self.dependency.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_exit_criteria(&self) -> bool {
        self.exit_criteria.is_some()
    }
    fn has_test_case(&self) -> bool {
        self.test_case.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for TestPlan {
    fn resource_type(&self) -> &'static str {
        "TestPlan"
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
        Some("http://hl7.org/fhir/StructureDefinition/TestPlan")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::test_plan::{TestPlanAccessors, TestPlanExistence, TestPlanMutators};
