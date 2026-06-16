use crate::bindings::medication_status::MedicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Medication
///
/// This resource is primarily used for the identification and definition of a medication, including ingredients, for the purposes of prescribing, dispensing, and administering a medication as well as for making statements about medication use.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Medication
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Medication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Medication {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for this medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Codes that identify this medication
    ///
    /// Binding: example (A coded concept that defines the type of a medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-codes
    pub code: Option<CodeableConcept>,
    /// active | inactive | entered-in-error
    pub status: Option<MedicationStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Organization that has authorization to market medication
    #[serde(rename = "marketingAuthorizationHolder")]
    pub marketing_authorization_holder: Option<Reference>,
    /// powder | tablets | capsule +
    ///
    /// Binding: example (A coded concept defining the form of a medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-form-codes
    #[serde(rename = "doseForm")]
    pub dose_form: Option<CodeableConcept>,
    /// When the specified product code does not infer a package size, this is the specific amount of drug in the product
    #[serde(rename = "totalVolume")]
    pub total_volume: Option<Quantity>,
    /// Active or inactive ingredient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<MedicationIngredient>,
    /// Details about packaged medications
    pub batch: Option<MedicationBatch>,
    /// Knowledge about this medication
    pub definition: Option<Reference>,
}
/// Medication nested structure for the 'batch' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationBatch {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identifier assigned to batch
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<StringType>,
    /// Extension element for the 'lotNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lotNumber")]
    pub _lot_number: Option<Element>,
    /// When batch will expire
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<DateTimeType>,
    /// Extension element for the 'expirationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_expirationDate")]
    pub _expiration_date: Option<Element>,
}
/// Medication nested structure for the 'ingredient' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationIngredient {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The ingredient (substance or medication) that the ingredient.strength relates to
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-codes
    pub item: CodeableReference,
    /// Active ingredient indicator
    #[serde(rename = "isActive")]
    pub is_active: Option<BooleanType>,
    /// Extension element for the 'isActive' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isActive")]
    pub _is_active: Option<Element>,
    /// Quantity of ingredient present (Ratio)
    #[serde(rename = "strengthRatio")]
    pub strength_ratio: Option<Ratio>,
    /// Quantity of ingredient present (CodeableConcept)
    #[serde(rename = "strengthCodeableConcept")]
    pub strength_codeable_concept: Option<CodeableConcept>,
    /// Quantity of ingredient present (Quantity)
    #[serde(rename = "strengthQuantity")]
    pub strength_quantity: Option<Quantity>,
}

impl Default for Medication {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            code: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            marketing_authorization_holder: Default::default(),
            dose_form: Default::default(),
            total_volume: Default::default(),
            ingredient: Default::default(),
            batch: Default::default(),
            definition: Default::default(),
        }
    }
}

impl Default for MedicationBatch {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            lot_number: Default::default(),
            _lot_number: Default::default(),
            expiration_date: Default::default(),
            _expiration_date: Default::default(),
        }
    }
}

impl Default for MedicationIngredient {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item: CodeableReference::default(),
            is_active: Default::default(),
            _is_active: Default::default(),
            strength_ratio: Default::default(),
            strength_codeable_concept: Default::default(),
            strength_quantity: Default::default(),
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
                "Medication.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Medication.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/medication-status|5.0.0",
            )
            .with_description("A coded concept defining if the medication is in active use."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Medication.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.contained", 0, None),
            rh_foundation::ElementCardinality::new("Medication.extension", 0, None),
            rh_foundation::ElementCardinality::new("Medication.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Medication.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Medication.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Medication.marketingAuthorizationHolder",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Medication.doseForm", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.totalVolume", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.ingredient", 0, None),
            rh_foundation::ElementCardinality::new("Medication.ingredient.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.ingredient.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Medication.ingredient.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Medication.ingredient.item", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.ingredient.isActive", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.ingredient.strength[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.batch", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.batch.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.batch.extension", 0, None),
            rh_foundation::ElementCardinality::new("Medication.batch.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Medication.batch.lotNumber", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.batch.expirationDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Medication.definition", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Medication {
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

impl crate::traits::resource::ResourceMutators for Medication {
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

impl crate::traits::resource::ResourceExistence for Medication {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Medication {
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

impl crate::traits::domain_resource::DomainResourceMutators for Medication {
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

impl crate::traits::domain_resource::DomainResourceExistence for Medication {
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

impl crate::traits::medication::MedicationAccessors for Medication {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn status(&self) -> Option<MedicationStatus> {
        self.status.clone()
    }
    fn marketing_authorization_holder(&self) -> Option<Reference> {
        self.marketing_authorization_holder.clone()
    }
    fn dose_form(&self) -> Option<CodeableConcept> {
        self.dose_form.clone()
    }
    fn total_volume(&self) -> Option<Quantity> {
        self.total_volume.clone()
    }
    fn ingredient(&self) -> &[MedicationIngredient] {
        self.ingredient.as_slice()
    }
    fn batch(&self) -> Option<MedicationBatch> {
        self.batch.clone()
    }
    fn definition(&self) -> Option<Reference> {
        self.definition.clone()
    }
}

impl crate::traits::medication::MedicationMutators for Medication {
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
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_status(self, value: MedicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_marketing_authorization_holder(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.marketing_authorization_holder = Some(value);
        resource
    }
    fn set_dose_form(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.dose_form = Some(value);
        resource
    }
    fn set_total_volume(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.total_volume = Some(value);
        resource
    }
    fn set_ingredient(self, value: Vec<MedicationIngredient>) -> Self {
        let mut resource = self.clone();
        resource.ingredient = value;
        resource
    }
    fn add_ingredient(self, item: MedicationIngredient) -> Self {
        let mut resource = self.clone();
        resource.ingredient.push(item);
        resource
    }
    fn set_batch(self, value: MedicationBatch) -> Self {
        let mut resource = self.clone();
        resource.batch = Some(value);
        resource
    }
    fn set_definition(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.definition = Some(value);
        resource
    }
}

impl crate::traits::medication::MedicationExistence for Medication {
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_marketing_authorization_holder(&self) -> bool {
        self.marketing_authorization_holder.is_some()
    }
    fn has_dose_form(&self) -> bool {
        self.dose_form.is_some()
    }
    fn has_total_volume(&self) -> bool {
        self.total_volume.is_some()
    }
    fn has_ingredient(&self) -> bool {
        !self.ingredient.is_empty()
    }
    fn has_batch(&self) -> bool {
        self.batch.is_some()
    }
    fn has_definition(&self) -> bool {
        self.definition.is_some()
    }
}

impl crate::validation::ValidatableResource for Medication {
    fn resource_type(&self) -> &'static str {
        "Medication"
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
        Some("http://hl7.org/fhir/StructureDefinition/Medication")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::medication::{MedicationAccessors, MedicationExistence, MedicationMutators};
