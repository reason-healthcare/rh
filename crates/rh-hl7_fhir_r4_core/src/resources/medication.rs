use crate::bindings::medication_status::MedicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Medication
///
/// This resource is primarily used for the identification and definition of a medication for the purposes of prescribing, dispensing, and administering a medication as well as for making statements about medication use.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Medication
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Medication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Medication {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for this medication
    pub identifier: Option<Vec<Identifier>>,
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
    /// Manufacturer of the item
    pub manufacturer: Option<Reference>,
    /// powder | tablets | capsule +
    ///
    /// Binding: example (A coded concept defining the form of a medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-form-codes
    pub form: Option<CodeableConcept>,
    /// Amount of drug in package
    pub amount: Option<Ratio>,
    /// Active or inactive ingredient
    pub ingredient: Option<Vec<MedicationIngredient>>,
    /// Details about packaged medications
    pub batch: Option<MedicationBatch>,
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
    /// The actual ingredient or content (CodeableConcept)
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: CodeableConcept,
    /// The actual ingredient or content (Reference)
    #[serde(rename = "itemReference")]
    pub item_reference: Reference,
    /// Active ingredient indicator
    #[serde(rename = "isActive")]
    pub is_active: Option<BooleanType>,
    /// Extension element for the 'isActive' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isActive")]
    pub _is_active: Option<Element>,
    /// Quantity of ingredient present
    pub strength: Option<Ratio>,
}

impl Default for Medication {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            code: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            manufacturer: Default::default(),
            form: Default::default(),
            amount: Default::default(),
            ingredient: Default::default(),
            batch: Default::default(),
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
            item_codeable_concept: Default::default(),
            item_reference: Default::default(),
            is_active: Default::default(),
            _is_active: Default::default(),
            strength: Default::default(),
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
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
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

impl crate::traits::domain_resource::DomainResourceExistence for Medication {
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

impl crate::traits::medication::MedicationAccessors for Medication {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn status(&self) -> Option<MedicationStatus> {
        self.status.clone()
    }
    fn manufacturer(&self) -> Option<Reference> {
        self.manufacturer.clone()
    }
    fn form(&self) -> Option<CodeableConcept> {
        self.form.clone()
    }
    fn amount(&self) -> Option<Ratio> {
        self.amount.clone()
    }
    fn ingredient(&self) -> &[MedicationIngredient] {
        self.ingredient.as_deref().unwrap_or(&[])
    }
    fn batch(&self) -> Option<MedicationBatch> {
        self.batch.clone()
    }
}

impl crate::traits::medication::MedicationMutators for Medication {
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
    fn set_manufacturer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.manufacturer = Some(value);
        resource
    }
    fn set_form(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.form = Some(value);
        resource
    }
    fn set_amount(self, value: Ratio) -> Self {
        let mut resource = self.clone();
        resource.amount = Some(value);
        resource
    }
    fn set_ingredient(self, value: Vec<MedicationIngredient>) -> Self {
        let mut resource = self.clone();
        resource.ingredient = Some(value);
        resource
    }
    fn add_ingredient(self, item: MedicationIngredient) -> Self {
        let mut resource = self.clone();
        resource.ingredient.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_batch(self, value: MedicationBatch) -> Self {
        let mut resource = self.clone();
        resource.batch = Some(value);
        resource
    }
}

impl crate::traits::medication::MedicationExistence for Medication {
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
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.is_some()
    }
    fn has_form(&self) -> bool {
        self.form.is_some()
    }
    fn has_amount(&self) -> bool {
        self.amount.is_some()
    }
    fn has_ingredient(&self) -> bool {
        self.ingredient.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_batch(&self) -> bool {
        self.batch.is_some()
    }
}

impl crate::validation::ValidatableResource for Medication {
    fn resource_type(&self) -> &'static str {
        "Medication"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Medication")
    }
}
