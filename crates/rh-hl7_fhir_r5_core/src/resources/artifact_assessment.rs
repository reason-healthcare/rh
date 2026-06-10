use crate::bindings::artifactassessment_disposition::ArtifactassessmentDisposition;
use crate::bindings::artifactassessment_information_type::ArtifactassessmentInformationType;
use crate::bindings::artifactassessment_workflow_status::ArtifactassessmentWorkflowStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ArtifactAssessment
///
/// This Resource provides one or more comments, classifiers or ratings about a Resource and supports attribution and rights management metadata for the added content.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ArtifactAssessment
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ArtifactAssessment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactAssessment {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Additional identifier for the artifact assessment
    pub identifier: Option<Vec<Identifier>>,
    /// A short title for the assessment for use in displaying and selecting
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// How to cite the comment or rating (Reference)
    #[serde(rename = "citeAsReference")]
    pub cite_as_reference: Option<Reference>,
    /// How to cite the comment or rating (markdown)
    #[serde(rename = "citeAsMarkdown")]
    pub cite_as_markdown: Option<StringType>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// When the artifact assessment was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the artifact assessment was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// The artifact assessed, commented upon or rated (Reference)
    #[serde(rename = "artifactReference")]
    pub artifact_reference: Reference,
    /// The artifact assessed, commented upon or rated (canonical)
    #[serde(rename = "artifactCanonical")]
    pub artifact_canonical: StringType,
    /// The artifact assessed, commented upon or rated (uri)
    #[serde(rename = "artifactUri")]
    pub artifact_uri: StringType,
    /// Comment, classifier, or rating content
    pub content: Option<Vec<ArtifactAssessmentContent>>,
    /// submitted | triaged | waiting-for-input | resolved-no-change | resolved-change-required | deferred | duplicate | applied | published | entered-in-error
    #[serde(rename = "workflowStatus")]
    pub workflow_status: Option<ArtifactassessmentWorkflowStatus>,
    /// Extension element for the 'workflowStatus' primitive field. Contains metadata and extensions.
    #[serde(rename = "_workflowStatus")]
    pub _workflow_status: Option<Element>,
    /// unresolved | not-persuasive | persuasive | persuasive-with-modification | not-persuasive-with-modification
    pub disposition: Option<ArtifactassessmentDisposition>,
    /// Extension element for the 'disposition' primitive field. Contains metadata and extensions.
    pub _disposition: Option<Element>,
}
/// ArtifactAssessment nested structure for the 'content' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactAssessmentContent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// comment | classifier | rating | container | response | change-request
    #[serde(rename = "informationType")]
    pub information_type: Option<ArtifactassessmentInformationType>,
    /// Extension element for the 'informationType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_informationType")]
    pub _information_type: Option<Element>,
    /// Brief summary of the content
    pub summary: Option<StringType>,
    /// Extension element for the 'summary' primitive field. Contains metadata and extensions.
    pub _summary: Option<Element>,
    /// What type of content
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/certainty-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Rating, classifier, or assessment
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/certainty-rating
    pub classifier: Option<Vec<CodeableConcept>>,
    /// Quantitative rating
    pub quantity: Option<Quantity>,
    /// Who authored the content
    pub author: Option<Reference>,
    /// What the comment is directed to
    pub path: Option<Vec<StringType>>,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
    /// Additional information
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Acceptable to publicly share the resource content
    #[serde(rename = "freeToShare")]
    pub free_to_share: Option<BooleanType>,
    /// Extension element for the 'freeToShare' primitive field. Contains metadata and extensions.
    #[serde(rename = "_freeToShare")]
    pub _free_to_share: Option<Element>,
    /// Contained content
    pub component: Option<Vec<StringType>>,
}

impl Default for ArtifactAssessment {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            cite_as_reference: Default::default(),
            cite_as_markdown: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            approval_date: Default::default(),
            _approval_date: Default::default(),
            last_review_date: Default::default(),
            _last_review_date: Default::default(),
            artifact_reference: Default::default(),
            artifact_canonical: Default::default(),
            artifact_uri: Default::default(),
            content: Default::default(),
            workflow_status: Default::default(),
            _workflow_status: Default::default(),
            disposition: Default::default(),
            _disposition: Default::default(),
        }
    }
}

impl Default for ArtifactAssessmentContent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            information_type: Default::default(),
            _information_type: Default::default(),
            summary: Default::default(),
            _summary: Default::default(),
            type_: Default::default(),
            classifier: Default::default(),
            quantity: Default::default(),
            author: Default::default(),
            path: Default::default(),
            _path: Default::default(),
            related_artifact: Default::default(),
            free_to_share: Default::default(),
            _free_to_share: Default::default(),
            component: Default::default(),
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
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
                "ArtifactAssessment.content.informationType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/artifactassessment-information-type|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "ArtifactAssessment.disposition",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/artifactassessment-disposition|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "ArtifactAssessment.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ArtifactAssessment.workflowStatus",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/artifactassessment-workflow-status|5.0.0",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ArtifactAssessment.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.contained", 0, None),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.extension", 0, None),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.citeAs[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.artifact[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content", 0, None),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.informationType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.summary",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.classifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content.author", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content.path", 0, None),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.relatedArtifact",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ArtifactAssessment.content.freeToShare",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.content.component", 0, None),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.workflowStatus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ArtifactAssessment.disposition", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ArtifactAssessment {
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

impl crate::traits::resource::ResourceMutators for ArtifactAssessment {
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

impl crate::traits::resource::ResourceExistence for ArtifactAssessment {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ArtifactAssessment {
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

impl crate::traits::domain_resource::DomainResourceMutators for ArtifactAssessment {
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

impl crate::traits::domain_resource::DomainResourceExistence for ArtifactAssessment {
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

impl crate::traits::artifact_assessment::ArtifactAssessmentAccessors for ArtifactAssessment {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn approval_date(&self) -> Option<DateType> {
        self.approval_date.clone()
    }
    fn last_review_date(&self) -> Option<DateType> {
        self.last_review_date.clone()
    }
    fn content(&self) -> &[ArtifactAssessmentContent] {
        self.content.as_deref().unwrap_or(&[])
    }
    fn workflow_status(&self) -> Option<ArtifactassessmentWorkflowStatus> {
        self.workflow_status.clone()
    }
    fn disposition(&self) -> Option<ArtifactassessmentDisposition> {
        self.disposition.clone()
    }
}

impl crate::traits::artifact_assessment::ArtifactAssessmentMutators for ArtifactAssessment {
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
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_copyright(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright = Some(value);
        resource
    }
    fn set_approval_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.approval_date = Some(value);
        resource
    }
    fn set_last_review_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.last_review_date = Some(value);
        resource
    }
    fn set_content(self, value: Vec<ArtifactAssessmentContent>) -> Self {
        let mut resource = self.clone();
        resource.content = Some(value);
        resource
    }
    fn add_content(self, item: ArtifactAssessmentContent) -> Self {
        let mut resource = self.clone();
        resource.content.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_workflow_status(self, value: ArtifactassessmentWorkflowStatus) -> Self {
        let mut resource = self.clone();
        resource.workflow_status = Some(value);
        resource
    }
    fn set_disposition(self, value: ArtifactassessmentDisposition) -> Self {
        let mut resource = self.clone();
        resource.disposition = Some(value);
        resource
    }
}

impl crate::traits::artifact_assessment::ArtifactAssessmentExistence for ArtifactAssessment {
    fn has_artifact(&self) -> bool {
        true
    }
    fn has_cite_as(&self) -> bool {
        self.cite_as_reference.is_some() || self.cite_as_markdown.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_approval_date(&self) -> bool {
        self.approval_date.is_some()
    }
    fn has_last_review_date(&self) -> bool {
        self.last_review_date.is_some()
    }
    fn has_content(&self) -> bool {
        self.content.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_workflow_status(&self) -> bool {
        self.workflow_status.is_some()
    }
    fn has_disposition(&self) -> bool {
        self.disposition.is_some()
    }
}

impl crate::validation::ValidatableResource for ArtifactAssessment {
    fn resource_type(&self) -> &'static str {
        "ArtifactAssessment"
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
        Some("http://hl7.org/fhir/StructureDefinition/ArtifactAssessment")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::artifact_assessment::{
    ArtifactAssessmentAccessors, ArtifactAssessmentExistence, ArtifactAssessmentMutators,
};
