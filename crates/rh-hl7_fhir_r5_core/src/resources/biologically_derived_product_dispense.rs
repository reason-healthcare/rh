use crate::bindings::biologicallyderivedproductdispense_status::BiologicallyderivedproductdispenseStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// BiologicallyDerivedProductDispense
///
/// A record of dispensation of a biologically derived product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProductDispense
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: BiologicallyDerivedProductDispense
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicallyDerivedProductDispense {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for this dispense
    pub identifier: Option<Vec<Identifier>>,
    /// The order or request that this dispense is fulfilling
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Short description
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// preparation | in-progress | allocated | issued | unfulfilled | returned | entered-in-error | unknown
    pub status: BiologicallyderivedproductdispenseStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Relationship between the donor and intended recipient
    ///
    /// Binding: example (Describes the relationship between the recipient and origin of the dispensed product.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/biologicallyderivedproductdispense-origin-relationship
    #[serde(rename = "originRelationshipType")]
    pub origin_relationship_type: Option<CodeableConcept>,
    /// The BiologicallyDerivedProduct that is dispensed
    pub product: Reference,
    /// The intended recipient of the dispensed product
    pub patient: Reference,
    /// Indicates the type of matching associated with the dispense
    ///
    /// Binding: example (Describes the type of matching between the recipient and origin of the dispensed product.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/biologicallyderivedproductdispense-match-status
    #[serde(rename = "matchStatus")]
    pub match_status: Option<CodeableConcept>,
    /// Indicates who or what performed an action
    pub performer: Option<Vec<BiologicallyDerivedProductDispensePerformer>>,
    /// Where the dispense occurred
    pub location: Option<Reference>,
    /// Amount dispensed
    pub quantity: Option<Quantity>,
    /// When product was selected/matched
    #[serde(rename = "preparedDate")]
    pub prepared_date: Option<DateTimeType>,
    /// Extension element for the 'preparedDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_preparedDate")]
    pub _prepared_date: Option<Element>,
    /// When the product was dispatched
    #[serde(rename = "whenHandedOver")]
    pub when_handed_over: Option<DateTimeType>,
    /// Extension element for the 'whenHandedOver' primitive field. Contains metadata and extensions.
    #[serde(rename = "_whenHandedOver")]
    pub _when_handed_over: Option<Element>,
    /// Where the product was dispatched to
    pub destination: Option<Reference>,
    /// Additional notes
    pub note: Option<Vec<Annotation>>,
    /// Specific instructions for use
    #[serde(rename = "usageInstruction")]
    pub usage_instruction: Option<StringType>,
    /// Extension element for the 'usageInstruction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_usageInstruction")]
    pub _usage_instruction: Option<Element>,
}
/// BiologicallyDerivedProductDispense nested structure for the 'performer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicallyDerivedProductDispensePerformer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identifies the function of the performer during the dispense
    ///
    /// Binding: example (Describes the the role or function of the performer in the dispense.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/biologicallyderivedproductdispense-performer-function
    pub function: Option<CodeableConcept>,
    /// Who performed the action
    pub actor: Reference,
}

impl Default for BiologicallyDerivedProductDispense {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            part_of: Default::default(),
            status: BiologicallyderivedproductdispenseStatus::default(),
            _status: Default::default(),
            origin_relationship_type: Default::default(),
            product: Reference::default(),
            patient: Reference::default(),
            match_status: Default::default(),
            performer: Default::default(),
            location: Default::default(),
            quantity: Default::default(),
            prepared_date: Default::default(),
            _prepared_date: Default::default(),
            when_handed_over: Default::default(),
            _when_handed_over: Default::default(),
            destination: Default::default(),
            note: Default::default(),
            usage_instruction: Default::default(),
            _usage_instruction: Default::default(),
        }
    }
}

impl Default for BiologicallyDerivedProductDispensePerformer {
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
                "BiologicallyDerivedProductDispense.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "BiologicallyDerivedProductDispense.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/biologicallyderivedproductdispense-status|5.0.0",
            )
            .with_description("Describes the lifecycle of the dispense."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.meta",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.text",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.contained",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.identifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.basedOn",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.partOf",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.status",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.originRelationshipType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.product",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.patient",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.matchStatus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.performer",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.performer.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.performer.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.performer.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.performer.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.performer.actor",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.location",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.preparedDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.whenHandedOver",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.destination",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.note",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProductDispense.usageInstruction",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for BiologicallyDerivedProductDispense {
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

impl crate::traits::resource::ResourceMutators for BiologicallyDerivedProductDispense {
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

impl crate::traits::resource::ResourceExistence for BiologicallyDerivedProductDispense {
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

impl crate::traits::domain_resource::DomainResourceAccessors
    for BiologicallyDerivedProductDispense
{
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

impl crate::traits::domain_resource::DomainResourceMutators for BiologicallyDerivedProductDispense {
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

impl crate::traits::domain_resource::DomainResourceExistence
    for BiologicallyDerivedProductDispense
{
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

impl crate::traits::biologically_derived_product_dispense::BiologicallyDerivedProductDispenseAccessors
for BiologicallyDerivedProductDispense {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> BiologicallyderivedproductdispenseStatus {
        self.status.clone()
    }
    fn origin_relationship_type(&self) -> Option<CodeableConcept> {
        self.origin_relationship_type.clone()
    }
    fn product(&self) -> Reference {
        self.product.clone()
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn match_status(&self) -> Option<CodeableConcept> {
        self.match_status.clone()
    }
    fn performer(&self) -> &[BiologicallyDerivedProductDispensePerformer] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn quantity(&self) -> Option<Quantity> {
        self.quantity.clone()
    }
    fn prepared_date(&self) -> Option<DateTimeType> {
        self.prepared_date.clone()
    }
    fn when_handed_over(&self) -> Option<DateTimeType> {
        self.when_handed_over.clone()
    }
    fn destination(&self) -> Option<Reference> {
        self.destination.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn usage_instruction(&self) -> Option<StringType> {
        self.usage_instruction.clone()
    }
}

impl
    crate::traits::biologically_derived_product_dispense::BiologicallyDerivedProductDispenseMutators
    for BiologicallyDerivedProductDispense
{
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
    fn set_status(self, value: BiologicallyderivedproductdispenseStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_origin_relationship_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.origin_relationship_type = Some(value);
        resource
    }
    fn set_product(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.product = value;
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_match_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.match_status = Some(value);
        resource
    }
    fn set_performer(self, value: Vec<BiologicallyDerivedProductDispensePerformer>) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: BiologicallyDerivedProductDispensePerformer) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_quantity(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.quantity = Some(value);
        resource
    }
    fn set_prepared_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.prepared_date = Some(value);
        resource
    }
    fn set_when_handed_over(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.when_handed_over = Some(value);
        resource
    }
    fn set_destination(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.destination = Some(value);
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
    fn set_usage_instruction(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.usage_instruction = Some(value);
        resource
    }
}

impl crate::traits::biologically_derived_product_dispense::BiologicallyDerivedProductDispenseExistence
for BiologicallyDerivedProductDispense {
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
        true
    }
    fn has_origin_relationship_type(&self) -> bool {
        self.origin_relationship_type.is_some()
    }
    fn has_product(&self) -> bool {
        true
    }
    fn has_patient(&self) -> bool {
        true
    }
    fn has_match_status(&self) -> bool {
        self.match_status.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_quantity(&self) -> bool {
        self.quantity.is_some()
    }
    fn has_prepared_date(&self) -> bool {
        self.prepared_date.is_some()
    }
    fn has_when_handed_over(&self) -> bool {
        self.when_handed_over.is_some()
    }
    fn has_destination(&self) -> bool {
        self.destination.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_usage_instruction(&self) -> bool {
        self.usage_instruction.is_some()
    }
}

impl crate::validation::ValidatableResource for BiologicallyDerivedProductDispense {
    fn resource_type(&self) -> &'static str {
        "BiologicallyDerivedProductDispense"
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
        Some("http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProductDispense")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::biologically_derived_product_dispense::{
    BiologicallyDerivedProductDispenseAccessors, BiologicallyDerivedProductDispenseExistence,
    BiologicallyDerivedProductDispenseMutators,
};
