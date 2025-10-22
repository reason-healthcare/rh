use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::relation_type::RelationType;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// CatalogEntry
///
/// Catalog entries are wrappers that contextualize items included in a catalog.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CatalogEntry
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CatalogEntry
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique identifier of the catalog item
    pub identifier: Option<Vec<Identifier>>,
    /// The type of item - medication, device, service, protocol or other
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Whether the entry represents an orderable item
    pub orderable: BooleanType,
    /// Extension element for the 'orderable' primitive field. Contains metadata and extensions.
    pub _orderable: Option<Element>,
    /// The item that is being defined
    #[serde(rename = "referencedItem")]
    pub referenced_item: Reference,
    /// Any additional identifier(s) for the catalog item, in the same granularity or concept
    #[serde(rename = "additionalIdentifier")]
    pub additional_identifier: Option<Vec<Identifier>>,
    /// Classification (category or class) of the item entry
    pub classification: Option<Vec<CodeableConcept>>,
    /// draft | active | retired | unknown
    pub status: Option<PublicationStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The time period in which this catalog entry is expected to be active
    #[serde(rename = "validityPeriod")]
    pub validity_period: Option<Period>,
    /// The date until which this catalog entry is expected to be active
    #[serde(rename = "validTo")]
    pub valid_to: Option<DateTimeType>,
    /// Extension element for the 'validTo' primitive field. Contains metadata and extensions.
    #[serde(rename = "_validTo")]
    pub _valid_to: Option<Element>,
    /// When was this catalog last updated
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<DateTimeType>,
    /// Extension element for the 'lastUpdated' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastUpdated")]
    pub _last_updated: Option<Element>,
    /// Additional characteristics of the catalog entry
    #[serde(rename = "additionalCharacteristic")]
    pub additional_characteristic: Option<Vec<CodeableConcept>>,
    /// Additional classification of the catalog entry
    #[serde(rename = "additionalClassification")]
    pub additional_classification: Option<Vec<CodeableConcept>>,
    /// An item that this catalog entry is related to
    #[serde(rename = "relatedEntry")]
    pub related_entry: Option<Vec<CatalogEntryRelatedentry>>,
}
/// CatalogEntry nested structure for the 'relatedEntry' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntryRelatedentry {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// triggers | is-replaced-by
    pub relationtype: RelationType,
    /// Extension element for the 'relationtype' primitive field. Contains metadata and extensions.
    pub _relationtype: Option<Element>,
    /// The reference to the related item
    pub item: Reference,
}

impl Default for CatalogEntry {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            type_: Default::default(),
            orderable: BooleanType::default(),
            _orderable: Default::default(),
            referenced_item: Reference::default(),
            additional_identifier: Default::default(),
            classification: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            validity_period: Default::default(),
            valid_to: Default::default(),
            _valid_to: Default::default(),
            last_updated: Default::default(),
            _last_updated: Default::default(),
            additional_characteristic: Default::default(),
            additional_classification: Default::default(),
            related_entry: Default::default(),
        }
    }
}

impl Default for CatalogEntryRelatedentry {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            relationtype: Default::default(),
            _relationtype: Default::default(),
            item: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for CatalogEntry {
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

impl crate::traits::resource::ResourceMutators for CatalogEntry {
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

impl crate::traits::resource::ResourceExistence for CatalogEntry {
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

impl crate::traits::domain_resource::DomainResourceAccessors for CatalogEntry {
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

impl crate::traits::domain_resource::DomainResourceMutators for CatalogEntry {
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

impl crate::traits::domain_resource::DomainResourceExistence for CatalogEntry {
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

impl crate::traits::catalog_entry::CatalogEntryAccessors for CatalogEntry {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn orderable(&self) -> BooleanType {
        self.orderable
    }
    fn referenced_item(&self) -> Reference {
        self.referenced_item.clone()
    }
    fn additional_identifier(&self) -> &[Identifier] {
        self.additional_identifier.as_deref().unwrap_or(&[])
    }
    fn classification(&self) -> &[CodeableConcept] {
        self.classification.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Option<PublicationStatus> {
        self.status.clone()
    }
    fn validity_period(&self) -> Option<Period> {
        self.validity_period.clone()
    }
    fn valid_to(&self) -> Option<DateTimeType> {
        self.valid_to.clone()
    }
    fn last_updated(&self) -> Option<DateTimeType> {
        self.last_updated.clone()
    }
    fn additional_characteristic(&self) -> &[CodeableConcept] {
        self.additional_characteristic.as_deref().unwrap_or(&[])
    }
    fn additional_classification(&self) -> &[CodeableConcept] {
        self.additional_classification.as_deref().unwrap_or(&[])
    }
    fn related_entry(&self) -> &[CatalogEntryRelatedentry] {
        self.related_entry.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::catalog_entry::CatalogEntryMutators for CatalogEntry {
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
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_orderable(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.orderable = value;
        resource
    }
    fn set_referenced_item(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.referenced_item = value;
        resource
    }
    fn set_additional_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.additional_identifier = Some(value);
        resource
    }
    fn add_additional_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource
            .additional_identifier
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_classification(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.classification = Some(value);
        resource
    }
    fn add_classification(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .classification
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_validity_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.validity_period = Some(value);
        resource
    }
    fn set_valid_to(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.valid_to = Some(value);
        resource
    }
    fn set_last_updated(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.last_updated = Some(value);
        resource
    }
    fn set_additional_characteristic(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.additional_characteristic = Some(value);
        resource
    }
    fn add_additional_characteristic(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .additional_characteristic
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_additional_classification(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.additional_classification = Some(value);
        resource
    }
    fn add_additional_classification(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .additional_classification
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_related_entry(self, value: Vec<CatalogEntryRelatedentry>) -> Self {
        let mut resource = self.clone();
        resource.related_entry = Some(value);
        resource
    }
    fn add_related_entry(self, item: CatalogEntryRelatedentry) -> Self {
        let mut resource = self.clone();
        resource
            .related_entry
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::catalog_entry::CatalogEntryExistence for CatalogEntry {
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
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_orderable(&self) -> bool {
        true
    }
    fn has_referenced_item(&self) -> bool {
        true
    }
    fn has_additional_identifier(&self) -> bool {
        self.additional_identifier
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_classification(&self) -> bool {
        self.classification.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_validity_period(&self) -> bool {
        self.validity_period.is_some()
    }
    fn has_valid_to(&self) -> bool {
        self.valid_to.is_some()
    }
    fn has_last_updated(&self) -> bool {
        self.last_updated.is_some()
    }
    fn has_additional_characteristic(&self) -> bool {
        self.additional_characteristic
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_additional_classification(&self) -> bool {
        self.additional_classification
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_related_entry(&self) -> bool {
        self.related_entry.as_ref().is_some_and(|v| !v.is_empty())
    }
}
