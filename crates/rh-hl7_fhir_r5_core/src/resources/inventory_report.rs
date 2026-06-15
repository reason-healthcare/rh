use crate::bindings::inventoryreport_counttype::InventoryreportCounttype;
use crate::bindings::inventoryreport_status::InventoryreportStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// InventoryReport
///
/// A report of inventory or stock items.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/InventoryReport
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: InventoryReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReport {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for the report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// draft | requested | active | entered-in-error
    pub status: InventoryreportStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// snapshot | difference
    #[serde(rename = "countType")]
    pub count_type: InventoryreportCounttype,
    /// Extension element for the 'countType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_countType")]
    pub _count_type: Option<Element>,
    /// addition | subtraction
    #[serde(rename = "operationType")]
    pub operation_type: Option<CodeableConcept>,
    /// The reason for this count - regular count, ad-hoc count, new arrivals, etc
    #[serde(rename = "operationTypeReason")]
    pub operation_type_reason: Option<CodeableConcept>,
    /// When the report has been submitted
    #[serde(rename = "reportedDateTime")]
    pub reported_date_time: DateTimeType,
    /// Extension element for the 'reportedDateTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_reportedDateTime")]
    pub _reported_date_time: Option<Element>,
    /// Who submits the report
    pub reporter: Option<Reference>,
    /// The period the report refers to
    #[serde(rename = "reportingPeriod")]
    pub reporting_period: Option<Period>,
    /// An inventory listing section (grouped by any of the attributes)
    #[serde(rename = "inventoryListing")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inventory_listing: Vec<InventoryReportInventorylisting>,
    /// A note associated with the InventoryReport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
}
/// InventoryReportInventorylisting nested structure for the 'item' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReportInventorylistingItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The inventory category or classification of the items being reported
    pub category: Option<CodeableConcept>,
    /// The quantity of the item or items being reported
    pub quantity: Quantity,
    /// The code or reference to the item type
    pub item: CodeableReference,
}
/// InventoryReport nested structure for the 'inventoryListing' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReportInventorylisting {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The item or items in this listing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<InventoryReportInventorylistingItem>,
    /// Location of the inventory items
    pub location: Option<Reference>,
    /// The status of the items that are being reported
    #[serde(rename = "itemStatus")]
    pub item_status: Option<CodeableConcept>,
    /// The date and time when the items were counted
    #[serde(rename = "countingDateTime")]
    pub counting_date_time: Option<DateTimeType>,
    /// Extension element for the 'countingDateTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_countingDateTime")]
    pub _counting_date_time: Option<Element>,
}

impl Default for InventoryReport {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: InventoryreportStatus::default(),
            _status: Default::default(),
            count_type: InventoryreportCounttype::default(),
            _count_type: Default::default(),
            operation_type: Default::default(),
            operation_type_reason: Default::default(),
            reported_date_time: DateTimeType::default(),
            _reported_date_time: Default::default(),
            reporter: Default::default(),
            reporting_period: Default::default(),
            inventory_listing: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for InventoryReportInventorylistingItem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            category: Default::default(),
            quantity: Default::default(),
            item: Default::default(),
        }
    }
}

impl Default for InventoryReportInventorylisting {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item: Default::default(),
            location: Default::default(),
            item_status: Default::default(),
            counting_date_time: Default::default(),
            _counting_date_time: Default::default(),
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
                "InventoryReport.countType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/inventoryreport-counttype|5.0.0",
            )
            .with_description("The type of count."),
            rh_foundation::ElementBinding::new(
                "InventoryReport.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "InventoryReport.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/inventoryreport-status|5.0.0",
            )
            .with_description("The status of the InventoryReport."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("InventoryReport.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryReport.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryReport.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryReport.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryReport.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryReport.contained", 0, None),
            rh_foundation::ElementCardinality::new("InventoryReport.extension", 0, None),
            rh_foundation::ElementCardinality::new("InventoryReport.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("InventoryReport.identifier", 0, None),
            rh_foundation::ElementCardinality::new("InventoryReport.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryReport.countType", 1, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryReport.operationType", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.operationTypeReason",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("InventoryReport.reportedDateTime", 1, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryReport.reporter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryReport.reportingPeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryReport.inventoryListing", 0, None),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.location",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.itemStatus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.countingDateTime",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.item",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.item.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.item.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.item.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.item.category",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.item.quantity",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryReport.inventoryListing.item.item",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("InventoryReport.note", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for InventoryReport {
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

impl crate::traits::resource::ResourceMutators for InventoryReport {
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

impl crate::traits::resource::ResourceExistence for InventoryReport {
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

impl crate::traits::domain_resource::DomainResourceAccessors for InventoryReport {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for InventoryReport {
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for InventoryReport {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::inventory_report::InventoryReportAccessors for InventoryReport {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn status(&self) -> InventoryreportStatus {
        self.status.clone()
    }
    fn count_type(&self) -> InventoryreportCounttype {
        self.count_type.clone()
    }
    fn operation_type(&self) -> Option<CodeableConcept> {
        self.operation_type.clone()
    }
    fn operation_type_reason(&self) -> Option<CodeableConcept> {
        self.operation_type_reason.clone()
    }
    fn reported_date_time(&self) -> DateTimeType {
        self.reported_date_time.clone()
    }
    fn reporter(&self) -> Option<Reference> {
        self.reporter.clone()
    }
    fn reporting_period(&self) -> Option<Period> {
        self.reporting_period.clone()
    }
    fn inventory_listing(&self) -> &[InventoryReportInventorylisting] {
        self.inventory_listing.as_slice()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
}

impl crate::traits::inventory_report::InventoryReportMutators for InventoryReport {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
        resource
    }
    fn set_status(self, value: InventoryreportStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_count_type(self, value: InventoryreportCounttype) -> Self {
        let mut resource = self.clone();
        resource.count_type = value;
        resource
    }
    fn set_operation_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.operation_type = Some(value);
        resource
    }
    fn set_operation_type_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.operation_type_reason = Some(value);
        resource
    }
    fn set_reported_date_time(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.reported_date_time = value;
        resource
    }
    fn set_reporter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.reporter = Some(value);
        resource
    }
    fn set_reporting_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.reporting_period = Some(value);
        resource
    }
    fn set_inventory_listing(self, value: Vec<InventoryReportInventorylisting>) -> Self {
        let mut resource = self.clone();
        resource.inventory_listing = value;
        resource
    }
    fn add_inventory_listing(self, item: InventoryReportInventorylisting) -> Self {
        let mut resource = self.clone();
        resource.inventory_listing.push(item);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = value;
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.push(item);
        resource
    }
}

impl crate::traits::inventory_report::InventoryReportExistence for InventoryReport {
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_count_type(&self) -> bool {
        true
    }
    fn has_operation_type(&self) -> bool {
        self.operation_type.is_some()
    }
    fn has_operation_type_reason(&self) -> bool {
        self.operation_type_reason.is_some()
    }
    fn has_reported_date_time(&self) -> bool {
        true
    }
    fn has_reporter(&self) -> bool {
        self.reporter.is_some()
    }
    fn has_reporting_period(&self) -> bool {
        self.reporting_period.is_some()
    }
    fn has_inventory_listing(&self) -> bool {
        !self.inventory_listing.is_empty()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
    }
}

impl crate::validation::ValidatableResource for InventoryReport {
    fn resource_type(&self) -> &'static str {
        "InventoryReport"
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
        Some("http://hl7.org/fhir/StructureDefinition/InventoryReport")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::inventory_report::{
    InventoryReportAccessors, InventoryReportExistence, InventoryReportMutators,
};
