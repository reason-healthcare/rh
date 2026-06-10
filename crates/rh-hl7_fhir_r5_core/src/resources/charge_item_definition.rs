use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::monetary_component::MonetaryComponent;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ChargeItemDefinition
///
/// The ChargeItemDefinition resource provides the properties that apply to the (billing) codes necessary to calculate costs and prices. The properties may differ largely depending on type and realm, therefore this resource gives only a rough structure and requires profiling for each type of billing code system.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ChargeItemDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ChargeItemDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeItemDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this charge item definition, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the charge item definition
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the charge item definition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this charge item definition (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this charge item definition (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Underlying externally-defined charge item definition
    #[serde(rename = "derivedFromUri")]
    pub derived_from_uri: Option<Vec<StringType>>,
    /// Extension element for the 'derivedFromUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_derivedFromUri")]
    pub _derived_from_uri: Option<Element>,
    /// A larger definition of which this particular definition is a component or step
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<StringType>>,
    /// Extension element for the 'partOf' primitive field. Contains metadata and extensions.
    #[serde(rename = "_partOf")]
    pub _part_of: Option<Element>,
    /// Completed or terminated request(s) whose function is taken by this new request
    pub replaces: Option<Vec<StringType>>,
    /// Extension element for the 'replaces' primitive field. Contains metadata and extensions.
    pub _replaces: Option<Element>,
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
    /// Natural language description of the charge item definition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for charge item definition (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this charge item definition is defined
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
    /// When the charge item definition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the charge item definition was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// Billing code or product type this definition applies to
    ///
    /// Binding: example (Billing Code defined by this ChargeItemDefinition.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/chargeitem-billingcodes
    pub code: Option<CodeableConcept>,
    /// Instances this definition applies to
    pub instance: Option<Vec<Reference>>,
    /// Whether or not the billing code is applicable
    pub applicability: Option<Vec<ChargeItemDefinitionApplicability>>,
    /// Group of properties which are applicable under the same conditions
    #[serde(rename = "propertyGroup")]
    pub property_group: Option<Vec<ChargeItemDefinitionPropertygroup>>,
}
/// ChargeItemDefinition nested structure for the 'propertyGroup' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeItemDefinitionPropertygroup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Conditions under which the priceComponent is applicable
    pub applicability: Option<Vec<StringType>>,
    /// Components of total line item price
    #[serde(rename = "priceComponent")]
    pub price_component: Option<Vec<MonetaryComponent>>,
}
/// ChargeItemDefinition nested structure for the 'applicability' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeItemDefinitionApplicability {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Boolean-valued expression
    pub condition: Option<Expression>,
    /// When the charge item definition is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// Reference to / quotation of the external source of the group of properties
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<RelatedArtifact>,
}

impl Default for ChargeItemDefinition {
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
            derived_from_uri: Default::default(),
            _derived_from_uri: Default::default(),
            part_of: Default::default(),
            _part_of: Default::default(),
            replaces: Default::default(),
            _replaces: Default::default(),
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
            approval_date: Default::default(),
            _approval_date: Default::default(),
            last_review_date: Default::default(),
            _last_review_date: Default::default(),
            code: Default::default(),
            instance: Default::default(),
            applicability: Default::default(),
            property_group: Default::default(),
        }
    }
}

impl Default for ChargeItemDefinitionPropertygroup {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            applicability: Default::default(),
            price_component: Default::default(),
        }
    }
}

impl Default for ChargeItemDefinitionApplicability {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            condition: Default::default(),
            effective_period: Default::default(),
            related_artifact: Default::default(),
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
                "ChargeItemDefinition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ChargeItemDefinition.status",
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
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.derivedFromUri", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.partOf", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.replaces", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.copyrightLabel",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.lastReviewDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.instance", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.applicability", 0, None),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.applicability.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.applicability.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.applicability.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.applicability.condition",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.applicability.effectivePeriod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.applicability.relatedArtifact",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ChargeItemDefinition.propertyGroup", 0, None),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.propertyGroup.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.propertyGroup.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.propertyGroup.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.propertyGroup.applicability",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ChargeItemDefinition.propertyGroup.priceComponent",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ChargeItemDefinition {
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

impl crate::traits::resource::ResourceMutators for ChargeItemDefinition {
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

impl crate::traits::resource::ResourceExistence for ChargeItemDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ChargeItemDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for ChargeItemDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for ChargeItemDefinition {
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

impl crate::traits::charge_item_definition::ChargeItemDefinitionAccessors for ChargeItemDefinition {
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
    fn derived_from_uri(&self) -> &[StringType] {
        self.derived_from_uri.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[StringType] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn replaces(&self) -> &[StringType] {
        self.replaces.as_deref().unwrap_or(&[])
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
    fn approval_date(&self) -> Option<DateType> {
        self.approval_date.clone()
    }
    fn last_review_date(&self) -> Option<DateType> {
        self.last_review_date.clone()
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn instance(&self) -> &[Reference] {
        self.instance.as_deref().unwrap_or(&[])
    }
    fn applicability(&self) -> &[ChargeItemDefinitionApplicability] {
        self.applicability.as_deref().unwrap_or(&[])
    }
    fn property_group(&self) -> &[ChargeItemDefinitionPropertygroup] {
        self.property_group.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::charge_item_definition::ChargeItemDefinitionMutators for ChargeItemDefinition {
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
    fn set_derived_from_uri(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.derived_from_uri = Some(value);
        resource
    }
    fn add_derived_from_uri(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .derived_from_uri
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_part_of(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
        resource
    }
    fn add_part_of(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.part_of.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_replaces(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.replaces = Some(value);
        resource
    }
    fn add_replaces(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.replaces.get_or_insert_with(Vec::new).push(item);
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
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_instance(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.instance = Some(value);
        resource
    }
    fn add_instance(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.instance.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_applicability(self, value: Vec<ChargeItemDefinitionApplicability>) -> Self {
        let mut resource = self.clone();
        resource.applicability = Some(value);
        resource
    }
    fn add_applicability(self, item: ChargeItemDefinitionApplicability) -> Self {
        let mut resource = self.clone();
        resource
            .applicability
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_property_group(self, value: Vec<ChargeItemDefinitionPropertygroup>) -> Self {
        let mut resource = self.clone();
        resource.property_group = Some(value);
        resource
    }
    fn add_property_group(self, item: ChargeItemDefinitionPropertygroup) -> Self {
        let mut resource = self.clone();
        resource
            .property_group
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::charge_item_definition::ChargeItemDefinitionExistence for ChargeItemDefinition {
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
    fn has_derived_from_uri(&self) -> bool {
        self.derived_from_uri
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_replaces(&self) -> bool {
        self.replaces.as_ref().is_some_and(|v| !v.is_empty())
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
    fn has_approval_date(&self) -> bool {
        self.approval_date.is_some()
    }
    fn has_last_review_date(&self) -> bool {
        self.last_review_date.is_some()
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_instance(&self) -> bool {
        self.instance.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_applicability(&self) -> bool {
        self.applicability.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_property_group(&self) -> bool {
        self.property_group.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for ChargeItemDefinition {
    fn resource_type(&self) -> &'static str {
        "ChargeItemDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/ChargeItemDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::charge_item_definition::{
    ChargeItemDefinitionAccessors, ChargeItemDefinitionExistence, ChargeItemDefinitionMutators,
};
