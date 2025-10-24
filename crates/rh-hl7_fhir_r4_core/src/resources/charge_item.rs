use crate::bindings::chargeitem_status::ChargeitemStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ChargeItem
///
/// The resource ChargeItem describes the provision of healthcare provider products for a certain patient, therefore referring not only to the product, but containing in addition details of the provision, like date, time, amounts and participating organizations and persons. Main Usage of the ChargeItem is to enable the billing process and internal cost allocation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ChargeItem
/// - Version: 4.0.1
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
    /// Binding: example (Example set of codes that can be used for billing purposes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/chargeitem-billingcodes
    pub code: CodeableConcept,
    /// Individual service was done for/to
    pub subject: Reference,
    /// Encounter / Episode associated with event
    pub context: Option<Reference>,
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
    /// Binding: example (Codes describing anatomical locations. May include laterality.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/body-site
    pub bodysite: Option<Vec<CodeableConcept>>,
    /// Factor overriding the associated rules
    #[serde(rename = "factorOverride")]
    pub factor_override: Option<DecimalType>,
    /// Extension element for the 'factorOverride' primitive field. Contains metadata and extensions.
    #[serde(rename = "_factorOverride")]
    pub _factor_override: Option<Element>,
    /// Price overriding the associated rules
    #[serde(rename = "priceOverride")]
    pub price_override: Option<Money>,
    /// Reason for overriding the list price/factor
    #[serde(rename = "overrideReason")]
    pub override_reason: Option<StringType>,
    /// Extension element for the 'overrideReason' primitive field. Contains metadata and extensions.
    #[serde(rename = "_overrideReason")]
    pub _override_reason: Option<Element>,
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
    /// Binding: example (Example binding for reason.)
    ///
    /// Available values:
    /// - `123456`: DIAG-1
    /// - `123457`: DIAG-1a
    /// - `987654`: DIAG-2
    /// - `123987`: DIAG-3
    /// - `112233`: DIAG-4
    /// - `997755`: DIAG-5
    /// - `321789`: DIAG-6
    pub reason: Option<Vec<CodeableConcept>>,
    /// Which rendered service is being charged?
    pub service: Option<Vec<Reference>>,
    /// Product charged (Reference)
    #[serde(rename = "productReference")]
    pub product_reference: Option<Reference>,
    /// Product charged (CodeableConcept)
    #[serde(rename = "productCodeableConcept")]
    pub product_codeable_concept: Option<CodeableConcept>,
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
            context: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            occurrence_timing: Default::default(),
            performer: Default::default(),
            performing_organization: Default::default(),
            requesting_organization: Default::default(),
            cost_center: Default::default(),
            quantity: Default::default(),
            bodysite: Default::default(),
            factor_override: Default::default(),
            _factor_override: Default::default(),
            price_override: Default::default(),
            override_reason: Default::default(),
            _override_reason: Default::default(),
            enterer: Default::default(),
            entered_date: Default::default(),
            _entered_date: Default::default(),
            reason: Default::default(),
            service: Default::default(),
            product_reference: Default::default(),
            product_codeable_concept: Default::default(),
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
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
    fn context(&self) -> Option<Reference> {
        self.context.clone()
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
    fn factor_override(&self) -> Option<DecimalType> {
        self.factor_override
    }
    fn price_override(&self) -> Option<Money> {
        self.price_override.clone()
    }
    fn override_reason(&self) -> Option<StringType> {
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
    fn service(&self) -> &[Reference] {
        self.service.as_deref().unwrap_or(&[])
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
    fn set_context(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.context = Some(value);
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
    fn set_factor_override(self, value: f64) -> Self {
        let mut resource = self.clone();
        resource.factor_override = Some(value);
        resource
    }
    fn set_price_override(self, value: Money) -> Self {
        let mut resource = self.clone();
        resource.price_override = Some(value);
        resource
    }
    fn set_override_reason(self, value: String) -> Self {
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
    fn set_service(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.service = Some(value);
        resource
    }
    fn add_service(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.service.get_or_insert_with(Vec::new).push(item);
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
    fn has_product(&self) -> bool {
        self.product_reference.is_some() || self.product_codeable_concept.is_some()
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
    fn has_context(&self) -> bool {
        self.context.is_some()
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
    fn has_factor_override(&self) -> bool {
        self.factor_override.is_some()
    }
    fn has_price_override(&self) -> bool {
        self.price_override.is_some()
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

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ChargeItem")
    }
}
