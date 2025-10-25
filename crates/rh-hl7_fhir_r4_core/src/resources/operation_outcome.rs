use crate::bindings::issue_severity::IssueSeverity;
use crate::bindings::issue_type::IssueType;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// OperationOutcome
///
/// A collection of error, warning, or information messages that result from a system action.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/OperationOutcome
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: OperationOutcome
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationOutcome {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// A single issue associated with the action
    pub issue: Vec<OperationOutcomeIssue>,
}
/// OperationOutcome nested structure for the 'issue' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationOutcomeIssue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// fatal | error | warning | information
    pub severity: IssueSeverity,
    /// Extension element for the 'severity' primitive field. Contains metadata and extensions.
    pub _severity: Option<Element>,
    /// Error or warning code
    pub code: IssueType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Additional details about the error
    ///
    /// Binding: example (A code that provides details as the exact issue.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/operation-outcome
    pub details: Option<CodeableConcept>,
    /// Additional diagnostic information about the issue
    pub diagnostics: Option<StringType>,
    /// Extension element for the 'diagnostics' primitive field. Contains metadata and extensions.
    pub _diagnostics: Option<Element>,
    /// Deprecated: Path of element(s) related to issue
    pub location: Option<Vec<StringType>>,
    /// Extension element for the 'location' primitive field. Contains metadata and extensions.
    pub _location: Option<Element>,
    /// FHIRPath of element(s) related to issue
    pub expression: Option<Vec<StringType>>,
    /// Extension element for the 'expression' primitive field. Contains metadata and extensions.
    pub _expression: Option<Element>,
}

impl Default for OperationOutcome {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            issue: Vec::new(),
        }
    }
}

impl Default for OperationOutcomeIssue {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            severity: IssueSeverity::default(),
            _severity: Default::default(),
            code: IssueType::default(),
            _code: Default::default(),
            details: Default::default(),
            diagnostics: Default::default(),
            _diagnostics: Default::default(),
            location: Default::default(),
            _location: Default::default(),
            expression: Default::default(),
            _expression: Default::default(),
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

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "OperationOutcome.issue.code",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/issue-type|4.0.1",
            )
            .with_description("A code that describes the type of issue."),
            rh_foundation::ElementBinding::new(
                "OperationOutcome.issue.severity",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/issue-severity|4.0.1",
            )
            .with_description("How the issue affects the success of the action."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("OperationOutcome.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationOutcome.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationOutcome.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationOutcome.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationOutcome.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationOutcome.contained", 0, None),
            rh_foundation::ElementCardinality::new("OperationOutcome.extension", 0, None),
            rh_foundation::ElementCardinality::new("OperationOutcome.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("OperationOutcome.issue", 1, None),
            rh_foundation::ElementCardinality::new("OperationOutcome.issue.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationOutcome.issue.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "OperationOutcome.issue.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("OperationOutcome.issue.severity", 1, Some(1)),
            rh_foundation::ElementCardinality::new("OperationOutcome.issue.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("OperationOutcome.issue.details", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "OperationOutcome.issue.diagnostics",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("OperationOutcome.issue.location", 0, None),
            rh_foundation::ElementCardinality::new("OperationOutcome.issue.expression", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for OperationOutcome {
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

impl crate::traits::resource::ResourceMutators for OperationOutcome {
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

impl crate::traits::resource::ResourceExistence for OperationOutcome {
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

impl crate::traits::domain_resource::DomainResourceAccessors for OperationOutcome {
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

impl crate::traits::domain_resource::DomainResourceMutators for OperationOutcome {
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

impl crate::traits::domain_resource::DomainResourceExistence for OperationOutcome {
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

impl crate::traits::operation_outcome::OperationOutcomeAccessors for OperationOutcome {
    fn issue(&self) -> &[OperationOutcomeIssue] {
        &self.issue
    }
}

impl crate::traits::operation_outcome::OperationOutcomeMutators for OperationOutcome {
    fn new() -> Self {
        Self::default()
    }
    fn set_issue(self, value: Vec<OperationOutcomeIssue>) -> Self {
        let mut resource = self.clone();
        resource.issue = value;
        resource
    }
    fn add_issue(self, item: OperationOutcomeIssue) -> Self {
        let mut resource = self.clone();
        resource.issue.push(item);
        resource
    }
}

impl crate::traits::operation_outcome::OperationOutcomeExistence for OperationOutcome {
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
    fn has_issue(&self) -> bool {
        !self.issue.is_empty()
    }
}

impl crate::validation::ValidatableResource for OperationOutcome {
    fn resource_type(&self) -> &'static str {
        "OperationOutcome"
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
        Some("http://hl7.org/fhir/StructureDefinition/OperationOutcome")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::operation_outcome::{
    OperationOutcomeAccessors, OperationOutcomeExistence, OperationOutcomeMutators,
};
