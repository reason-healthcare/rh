use crate::bindings::chargeitem_status::ChargeitemStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::monetary_component::MonetaryComponent;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ChargeItem
///
/// The resource ChargeItem describes the provision of healthcare provider products for a certain patient, therefore referring not only to the product, but containing in addition details of the provision, like date, time, amounts and participating organizations and persons. Main Usage of the ChargeItem is to enable the billing process and internal cost allocation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ChargeItem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ChargeItem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for item
    pub identifier: Option<Vec<Identifier>>,
    /// Defining information about the code of this charge item
    #[serde(rename = "definitionUri")]
    pub definition_uri: Option<Vec<StringType>>,
    /// Extension element for the 'definitionUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_definitionUri")]
    pub _definition_uri: Option<Element>,
    /// Resource defining the code of this ChargeItem
    #[serde(rename = "definitionCanonical")]
    pub definition_canonical: Option<Vec<StringType>>,
    /// Extension element for the 'definitionCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_definitionCanonical")]
    pub _definition_canonical: Option<Element>,
    /// planned | billable | not-billable | aborted | billed | entered-in-error | unknown
    pub status: ChargeitemStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Part of referenced ChargeItem
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// A code that identifies the charge, like a billing code
    ///
    /// Binding: example (Example codes for billing purposes)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/chargeitem-billingcodes
    pub code: CodeableConcept,
    /// Individual service was done for/to
    pub subject: Reference,
    /// Encounter associated with this ChargeItem
    pub encounter: Option<Reference>,
    /// When the charged service was applied (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// When the charged service was applied (Period)
    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,
    /// When the charged service was applied (Timing)
    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,
    /// Who performed charged service
    pub performer: Option<Vec<ChargeItemPerformer>>,
    /// Organization providing the charged service
    #[serde(rename = "performingOrganization")]
    pub performing_organization: Option<Reference>,
    /// Organization requesting the charged service
    #[serde(rename = "requestingOrganization")]
    pub requesting_organization: Option<Reference>,
    /// Organization that has ownership of the (potential, future) revenue
    #[serde(rename = "costCenter")]
    pub cost_center: Option<Reference>,
    /// Quantity of which the charge item has been serviced
    pub quantity: Option<Quantity>,
    /// Anatomical location, if relevant
    ///
    /// Binding: example (SNOMED CT Body site concepts)
    ///
    /// Available values:
    /// - `53075003`: Distal phalanx of hallux
    /// - `76986006`: Distal phalanx of second toe
    /// - `65258003`: Distal phalanx of third toe
    /// - `54333003`: Distal phalanx of fourth toe
    /// - `10770001`: Distal phalanx of fifth toe
    /// - `363670009`: Interphalangeal joint structure of great toe
    /// - `371216008`: Distal interphalangeal joint of second toe
    /// - `371219001`: Distal interphalangeal joint of third toe
    /// - `371205001`: Distal interphalangeal joint of fourth toe
    /// - `371203008`: Distal interphalangeal joint of fifth toe
    /// - ... and 30 more values
    pub bodysite: Option<Vec<CodeableConcept>>,
    /// Unit price overriding the associated rules
    #[serde(rename = "unitPriceComponent")]
    pub unit_price_component: Option<MonetaryComponent>,
    /// Total price overriding the associated rules
    #[serde(rename = "totalPriceComponent")]
    pub total_price_component: Option<MonetaryComponent>,
    /// Reason for overriding the list price/factor
    ///
    /// Binding: example (Local or regional codes covering why a price was overridden)
    #[serde(rename = "overrideReason")]
    pub override_reason: Option<CodeableConcept>,
    /// Individual who was entering
    pub enterer: Option<Reference>,
    /// Date the charge item was entered
    #[serde(rename = "enteredDate")]
    pub entered_date: Option<DateTimeType>,
    /// Extension element for the 'enteredDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_enteredDate")]
    pub _entered_date: Option<Element>,
    /// Why was the charged  service rendered?
    ///
    /// Binding: example (ICD 10 diagnosis codes)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/icd-10
    pub reason: Option<Vec<CodeableConcept>>,
    /// Which rendered service is being charged?
    pub service: Option<Vec<CodeableReference>>,
    /// Product charged
    ///
    /// Binding: example (Example binding for product type.)
    ///
    /// Available values:
    /// - `528391`: Blood Pressure Cuff
    /// - `528404`: Body Composition Analyzer
    /// - `528425`: Cardiovascular Device
    /// - `528402`: Coagulation meter
    /// - `528409`: Continuous Glucose Monitor
    /// - `528390`: Electro cardiograph
    /// - `528457`: Generic 20601 Device
    /// - `528401`: Glucose Monitor
    /// - `528455`: Independent Activity/Living Hub
    /// - `528403`: Insulin Pump
    /// - ... and 18 more values
    pub product: Option<Vec<CodeableReference>>,
    /// Account to place this charge
    pub account: Option<Vec<Reference>>,
    /// Comments made about the ChargeItem
    pub note: Option<Vec<Annotation>>,
    /// Further information supporting this charge
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Reference>>,
}
/// ChargeItem nested structure for the 'performer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeItemPerformer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What type of performance was done
    ///
    /// Binding: example (Codes describing the types of functional roles performers can take on when performing events.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/performer-role
    pub function: Option<CodeableConcept>,
    /// Individual who was performing
    pub actor: Reference,
}

impl Default for ChargeItem {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            definition_uri: Default::default(),
            _definition_uri: Default::default(),
            definition_canonical: Default::default(),
            _definition_canonical: Default::default(),
            status: ChargeitemStatus::default(),
            _status: Default::default(),
            part_of: Default::default(),
            code: CodeableConcept::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            occurrence_timing: Default::default(),
            performer: Default::default(),
            performing_organization: Default::default(),
            requesting_organization: Default::default(),
            cost_center: Default::default(),
            quantity: Default::default(),
            bodysite: Default::default(),
            unit_price_component: Default::default(),
            total_price_component: Default::default(),
            override_reason: Default::default(),
            enterer: Default::default(),
            entered_date: Default::default(),
            _entered_date: Default::default(),
            reason: Default::default(),
            service: Default::default(),
            product: Default::default(),
            account: Default::default(),
            note: Default::default(),
            supporting_information: Default::default(),
        }
    }
}

impl Default for ChargeItemPerformer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: Reference::default(),
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
                "ChargeItem.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ChargeItem.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/chargeitem-status|5.0.0",
            )
            .with_description("Codes identifying the lifecycle stage of a ChargeItem."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ChargeItem.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.contained", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.extension", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.definitionUri", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.definitionCanonical", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.partOf", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.occurrence[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.performer", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.performer.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.performer.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ChargeItem.performer.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ChargeItem.performer.function", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.performer.actor", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.performingOrganization", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.requestingOrganization", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.costCenter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.quantity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.bodysite", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.unitPriceComponent", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.totalPriceComponent", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.overrideReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.enterer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.enteredDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ChargeItem.reason", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.service", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.product", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.account", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.note", 0, None),
            rh_foundation::ElementCardinality::new("ChargeItem.supportingInformation", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ChargeItem {
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

impl crate::traits::resource::ResourceMutators for ChargeItem {
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

impl crate::traits::resource::ResourceExistence for ChargeItem {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ChargeItem {
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

impl crate::traits::domain_resource::DomainResourceMutators for ChargeItem {
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

impl crate::traits::domain_resource::DomainResourceExistence for ChargeItem {
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

impl crate::traits::charge_item::ChargeItemAccessors for ChargeItem {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn definition_uri(&self) -> &[StringType] {
        self.definition_uri.as_deref().unwrap_or(&[])
    }
    fn definition_canonical(&self) -> &[StringType] {
        self.definition_canonical.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ChargeitemStatus {
        self.status.clone()
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> CodeableConcept {
        self.code.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn performer(&self) -> &[ChargeItemPerformer] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn performing_organization(&self) -> Option<Reference> {
        self.performing_organization.clone()
    }
    fn requesting_organization(&self) -> Option<Reference> {
        self.requesting_organization.clone()
    }
    fn cost_center(&self) -> Option<Reference> {
        self.cost_center.clone()
    }
    fn quantity(&self) -> Option<Quantity> {
        self.quantity.clone()
    }
    fn bodysite(&self) -> &[CodeableConcept] {
        self.bodysite.as_deref().unwrap_or(&[])
    }
    fn unit_price_component(&self) -> Option<MonetaryComponent> {
        self.unit_price_component.clone()
    }
    fn total_price_component(&self) -> Option<MonetaryComponent> {
        self.total_price_component.clone()
    }
    fn override_reason(&self) -> Option<CodeableConcept> {
        self.override_reason.clone()
    }
    fn enterer(&self) -> Option<Reference> {
        self.enterer.clone()
    }
    fn entered_date(&self) -> Option<DateTimeType> {
        self.entered_date.clone()
    }
    fn reason(&self) -> &[CodeableConcept] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn service(&self) -> &[CodeableReference] {
        self.service.as_deref().unwrap_or(&[])
    }
    fn product(&self) -> &[CodeableReference] {
        self.product.as_deref().unwrap_or(&[])
    }
    fn account(&self) -> &[Reference] {
        self.account.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn supporting_information(&self) -> &[Reference] {
        self.supporting_information.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::charge_item::ChargeItemMutators for ChargeItem {
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
    fn set_definition_uri(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.definition_uri = Some(value);
        resource
    }
    fn add_definition_uri(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .definition_uri
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_definition_canonical(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.definition_canonical = Some(value);
        resource
    }
    fn add_definition_canonical(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .definition_canonical
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_status(self, value: ChargeitemStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
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
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = value;
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_performer(self, value: Vec<ChargeItemPerformer>) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: ChargeItemPerformer) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_performing_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.performing_organization = Some(value);
        resource
    }
    fn set_requesting_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.requesting_organization = Some(value);
        resource
    }
    fn set_cost_center(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.cost_center = Some(value);
        resource
    }
    fn set_quantity(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.quantity = Some(value);
        resource
    }
    fn set_bodysite(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.bodysite = Some(value);
        resource
    }
    fn add_bodysite(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.bodysite.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_unit_price_component(self, value: MonetaryComponent) -> Self {
        let mut resource = self.clone();
        resource.unit_price_component = Some(value);
        resource
    }
    fn set_total_price_component(self, value: MonetaryComponent) -> Self {
        let mut resource = self.clone();
        resource.total_price_component = Some(value);
        resource
    }
    fn set_override_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.override_reason = Some(value);
        resource
    }
    fn set_enterer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.enterer = Some(value);
        resource
    }
    fn set_entered_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.entered_date = Some(value);
        resource
    }
    fn set_reason(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.reason = Some(value);
        resource
    }
    fn add_reason(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_service(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.service = Some(value);
        resource
    }
    fn add_service(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.service.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_product(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.product = Some(value);
        resource
    }
    fn add_product(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.product.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_account(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.account = Some(value);
        resource
    }
    fn add_account(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.account.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = Some(value);
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_supporting_information(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_information = Some(value);
        resource
    }
    fn add_supporting_information(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_information
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::charge_item::ChargeItemExistence for ChargeItem {
    fn has_occurrence(&self) -> bool {
        self.occurrence_date_time.is_some()
            || self.occurrence_period.is_some()
            || self.occurrence_timing.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_definition_uri(&self) -> bool {
        self.definition_uri.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_definition_canonical(&self) -> bool {
        self.definition_canonical
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_performing_organization(&self) -> bool {
        self.performing_organization.is_some()
    }
    fn has_requesting_organization(&self) -> bool {
        self.requesting_organization.is_some()
    }
    fn has_cost_center(&self) -> bool {
        self.cost_center.is_some()
    }
    fn has_quantity(&self) -> bool {
        self.quantity.is_some()
    }
    fn has_bodysite(&self) -> bool {
        self.bodysite.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_unit_price_component(&self) -> bool {
        self.unit_price_component.is_some()
    }
    fn has_total_price_component(&self) -> bool {
        self.total_price_component.is_some()
    }
    fn has_override_reason(&self) -> bool {
        self.override_reason.is_some()
    }
    fn has_enterer(&self) -> bool {
        self.enterer.is_some()
    }
    fn has_entered_date(&self) -> bool {
        self.entered_date.is_some()
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_service(&self) -> bool {
        self.service.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_product(&self) -> bool {
        self.product.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_account(&self) -> bool {
        self.account.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_supporting_information(&self) -> bool {
        self.supporting_information
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for ChargeItem {
    fn resource_type(&self) -> &'static str {
        "ChargeItem"
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
        Some("http://hl7.org/fhir/StructureDefinition/ChargeItem")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::charge_item::{
    ChargeItemAccessors, ChargeItemExistence, ChargeItemMutators,
};
