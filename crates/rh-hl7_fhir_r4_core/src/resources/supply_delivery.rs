use crate::bindings::supplydelivery_status::SupplydeliveryStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SupplyDelivery
///
/// Record of delivery of what is supplied.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SupplyDelivery
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SupplyDelivery
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyDelivery {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Fulfills plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// in-progress | completed | abandoned | entered-in-error
    pub status: Option<SupplydeliveryStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Patient for whom the item is supplied
    pub patient: Option<Reference>,
    /// Category of dispense event
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The item that is delivered or supplied
    #[serde(rename = "suppliedItem")]
    pub supplied_item: Option<SupplyDeliverySupplieditem>,
    /// When event occurred (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// When event occurred (Period)
    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,
    /// When event occurred (Timing)
    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,
    /// Dispenser
    pub supplier: Option<Reference>,
    /// Where the Supply was sent
    pub destination: Option<Reference>,
    /// Who collected the Supply
    pub receiver: Option<Vec<Reference>>,
}
/// SupplyDelivery nested structure for the 'suppliedItem' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyDeliverySupplieditem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Amount dispensed
    pub quantity: Option<Quantity>,
    /// Medication, Substance, or Device supplied (CodeableConcept)
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: Option<CodeableConcept>,
    /// Medication, Substance, or Device supplied (Reference)
    #[serde(rename = "itemReference")]
    pub item_reference: Option<Reference>,
}

impl Default for SupplyDelivery {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            part_of: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            patient: Default::default(),
            type_: Default::default(),
            supplied_item: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            occurrence_timing: Default::default(),
            supplier: Default::default(),
            destination: Default::default(),
            receiver: Default::default(),
        }
    }
}

impl Default for SupplyDeliverySupplieditem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            quantity: Default::default(),
            item_codeable_concept: Default::default(),
            item_reference: Default::default(),
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
                "SupplyDelivery.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/supplydelivery-status|4.0.1",
            )
            .with_description("Status of the supply delivery."),
            rh_foundation::ElementBinding::new(
                "SupplyDelivery.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/supplydelivery-type|4.0.1",
            )
            .with_description("The type of supply dispense."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("SupplyDelivery.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyDelivery.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyDelivery.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyDelivery.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyDelivery.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyDelivery.contained", 0, None),
            rh_foundation::ElementCardinality::new("SupplyDelivery.extension", 0, None),
            rh_foundation::ElementCardinality::new("SupplyDelivery.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("SupplyDelivery.identifier", 0, None),
            rh_foundation::ElementCardinality::new("SupplyDelivery.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("SupplyDelivery.partOf", 0, None),
            rh_foundation::ElementCardinality::new("SupplyDelivery.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyDelivery.patient", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyDelivery.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyDelivery.suppliedItem", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyDelivery.suppliedItem.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SupplyDelivery.suppliedItem.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SupplyDelivery.suppliedItem.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SupplyDelivery.suppliedItem.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SupplyDelivery.suppliedItem.item[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SupplyDelivery.occurrence[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyDelivery.supplier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyDelivery.destination", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SupplyDelivery.receiver", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SupplyDelivery {
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

impl crate::traits::resource::ResourceMutators for SupplyDelivery {
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

impl crate::traits::resource::ResourceExistence for SupplyDelivery {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SupplyDelivery {
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

impl crate::traits::domain_resource::DomainResourceMutators for SupplyDelivery {
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

impl crate::traits::domain_resource::DomainResourceExistence for SupplyDelivery {
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

impl crate::traits::supply_delivery::SupplyDeliveryAccessors for SupplyDelivery {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Option<SupplydeliveryStatus> {
        self.status.clone()
    }
    fn patient(&self) -> Option<Reference> {
        self.patient.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn supplied_item(&self) -> Option<SupplyDeliverySupplieditem> {
        self.supplied_item.clone()
    }
    fn supplier(&self) -> Option<Reference> {
        self.supplier.clone()
    }
    fn destination(&self) -> Option<Reference> {
        self.destination.clone()
    }
    fn receiver(&self) -> &[Reference] {
        self.receiver.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::supply_delivery::SupplyDeliveryMutators for SupplyDelivery {
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
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_part_of(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
        resource
    }
    fn add_part_of(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: SupplydeliveryStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_supplied_item(self, value: SupplyDeliverySupplieditem) -> Self {
        let mut resource = self.clone();
        resource.supplied_item = Some(value);
        resource
    }
    fn set_supplier(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.supplier = Some(value);
        resource
    }
    fn set_destination(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.destination = Some(value);
        resource
    }
    fn set_receiver(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.receiver = Some(value);
        resource
    }
    fn add_receiver(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.receiver.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::supply_delivery::SupplyDeliveryExistence for SupplyDelivery {
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
    fn has_occurrence(&self) -> bool {
        self.occurrence_date_time.is_some()
            || self.occurrence_period.is_some()
            || self.occurrence_timing.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_patient(&self) -> bool {
        self.patient.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_supplied_item(&self) -> bool {
        self.supplied_item.is_some()
    }
    fn has_supplier(&self) -> bool {
        self.supplier.is_some()
    }
    fn has_destination(&self) -> bool {
        self.destination.is_some()
    }
    fn has_receiver(&self) -> bool {
        self.receiver.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for SupplyDelivery {
    fn resource_type(&self) -> &'static str {
        "SupplyDelivery"
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
        Some("http://hl7.org/fhir/StructureDefinition/SupplyDelivery")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::supply_delivery::{
    SupplyDeliveryAccessors, SupplyDeliveryExistence, SupplyDeliveryMutators,
};
