use crate::bindings::group_type::GroupType;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Group
///
/// Represents a defined collection of entities that may be discussed or acted upon collectively but which are not expected to act collectively, and are not formally or legally recognized; i.e. a collection of entities that isn't an Organization.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Group
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Group
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique id
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this group's record is in active use
    pub active: Option<BooleanType>,
    /// Extension element for the 'active' primitive field. Contains metadata and extensions.
    pub _active: Option<Element>,
    /// person | animal | practitioner | device | medication | substance
    #[serde(rename = "type")]
    pub type_: GroupType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Descriptive or actual
    pub actual: BooleanType,
    /// Extension element for the 'actual' primitive field. Contains metadata and extensions.
    pub _actual: Option<Element>,
    /// Kind of Group members
    ///
    /// Binding: example (Kind of particular resource; e.g. cow, syringe, lake, etc.)
    pub code: Option<CodeableConcept>,
    /// Label for Group
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Number of members
    pub quantity: Option<UnsignedIntType>,
    /// Extension element for the 'quantity' primitive field. Contains metadata and extensions.
    pub _quantity: Option<Element>,
    /// Entity that is the custodian of the Group's definition
    #[serde(rename = "managingEntity")]
    pub managing_entity: Option<Reference>,
    /// Include / Exclude group members by Trait
    pub characteristic: Option<Vec<GroupCharacteristic>>,
    /// Who or what is in group
    pub member: Option<Vec<GroupMember>>,
}
/// Group nested structure for the 'member' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reference to the group member
    pub entity: Reference,
    /// Period member belonged to the group
    pub period: Option<Period>,
    /// If member is no longer in group
    pub inactive: Option<BooleanType>,
    /// Extension element for the 'inactive' primitive field. Contains metadata and extensions.
    pub _inactive: Option<Element>,
}
/// Group nested structure for the 'characteristic' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupCharacteristic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Kind of characteristic
    ///
    /// Binding: example (List of characteristics used to describe group members; e.g. gender, age, owner, location, etc.)
    pub code: CodeableConcept,
    /// Value held by characteristic (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// Value held by characteristic (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Value held by characteristic (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Value held by characteristic (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// Value held by characteristic (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Reference,
    /// Group includes or excludes
    pub exclude: BooleanType,
    /// Extension element for the 'exclude' primitive field. Contains metadata and extensions.
    pub _exclude: Option<Element>,
    /// Period over which characteristic is tested
    pub period: Option<Period>,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            active: Default::default(),
            _active: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            actual: BooleanType::default(),
            _actual: Default::default(),
            code: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            quantity: Default::default(),
            _quantity: Default::default(),
            managing_entity: Default::default(),
            characteristic: Default::default(),
            member: Default::default(),
        }
    }
}

impl Default for GroupMember {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            entity: Reference::default(),
            period: Default::default(),
            inactive: Default::default(),
            _inactive: Default::default(),
        }
    }
}

impl Default for GroupCharacteristic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: CodeableConcept::default(),
            value_codeable_concept: Default::default(),
            value_boolean: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
            value_reference: Default::default(),
            exclude: BooleanType::default(),
            _exclude: Default::default(),
            period: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Group {
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

impl crate::traits::resource::ResourceMutators for Group {
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

impl crate::traits::resource::ResourceExistence for Group {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Group {
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

impl crate::traits::domain_resource::DomainResourceMutators for Group {
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

impl crate::traits::domain_resource::DomainResourceExistence for Group {
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

impl crate::traits::group::GroupAccessors for Group {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn active(&self) -> Option<BooleanType> {
        self.active
    }
    fn type_(&self) -> GroupType {
        self.type_.clone()
    }
    fn actual(&self) -> BooleanType {
        self.actual
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn quantity(&self) -> Option<UnsignedIntType> {
        self.quantity
    }
    fn managing_entity(&self) -> Option<Reference> {
        self.managing_entity.clone()
    }
    fn characteristic(&self) -> &[GroupCharacteristic] {
        self.characteristic.as_deref().unwrap_or(&[])
    }
    fn member(&self) -> &[GroupMember] {
        self.member.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::group::GroupMutators for Group {
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
    fn set_active(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.active = Some(value);
        resource
    }
    fn set_type_(self, value: GroupType) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn set_actual(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.actual = value;
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_quantity(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.quantity = Some(value);
        resource
    }
    fn set_managing_entity(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.managing_entity = Some(value);
        resource
    }
    fn set_characteristic(self, value: Vec<GroupCharacteristic>) -> Self {
        let mut resource = self.clone();
        resource.characteristic = Some(value);
        resource
    }
    fn add_characteristic(self, item: GroupCharacteristic) -> Self {
        let mut resource = self.clone();
        resource
            .characteristic
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_member(self, value: Vec<GroupMember>) -> Self {
        let mut resource = self.clone();
        resource.member = Some(value);
        resource
    }
    fn add_member(self, item: GroupMember) -> Self {
        let mut resource = self.clone();
        resource.member.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::group::GroupExistence for Group {
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
    fn has_active(&self) -> bool {
        self.active.is_some()
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_actual(&self) -> bool {
        true
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_quantity(&self) -> bool {
        self.quantity.is_some()
    }
    fn has_managing_entity(&self) -> bool {
        self.managing_entity.is_some()
    }
    fn has_characteristic(&self) -> bool {
        self.characteristic.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_member(&self) -> bool {
        self.member.as_ref().is_some_and(|v| !v.is_empty())
    }
}
