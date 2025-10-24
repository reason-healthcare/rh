use crate::bindings::list_mode::ListMode;
use crate::bindings::list_status::ListStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// List
///
/// A list is a curated collection of resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/List
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: List
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier
    pub identifier: Option<Vec<Identifier>>,
    /// current | retired | entered-in-error
    pub status: ListStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// working | snapshot | changes
    pub mode: ListMode,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// Descriptive name for the list
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// What the purpose of this list is
    ///
    /// Binding: example (What the purpose of a list is.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/list-example-codes
    pub code: Option<CodeableConcept>,
    /// If all resources have the same subject
    pub subject: Option<Reference>,
    /// Context in which list created
    pub encounter: Option<Reference>,
    /// When the list was prepared
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Who and/or what defined the list contents (aka Author)
    pub source: Option<Reference>,
    /// What order the list has
    ///
    /// Binding: preferred (What order applies to the items in a list.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/list-order
    #[serde(rename = "orderedBy")]
    pub ordered_by: Option<CodeableConcept>,
    /// Comments about the list
    pub note: Option<Vec<Annotation>>,
    /// Entries in the list
    pub entry: Option<Vec<ListEntry>>,
    /// Why list is empty
    ///
    /// Binding: preferred (If a list is empty, why it is empty.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/list-empty-reason
    #[serde(rename = "emptyReason")]
    pub empty_reason: Option<CodeableConcept>,
}
/// List nested structure for the 'entry' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEntry {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Status/Workflow information about this item
    ///
    /// Binding: example (Codes that provide further information about the reason and meaning of the item in the list.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/list-item-flag
    pub flag: Option<CodeableConcept>,
    /// If this item is actually marked as deleted
    pub deleted: Option<BooleanType>,
    /// Extension element for the 'deleted' primitive field. Contains metadata and extensions.
    pub _deleted: Option<Element>,
    /// When item added to list
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Actual entry
    pub item: Reference,
}

impl Default for List {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: ListStatus::default(),
            _status: Default::default(),
            mode: ListMode::default(),
            _mode: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            code: Default::default(),
            subject: Default::default(),
            encounter: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            source: Default::default(),
            ordered_by: Default::default(),
            note: Default::default(),
            entry: Default::default(),
            empty_reason: Default::default(),
        }
    }
}

impl Default for ListEntry {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            flag: Default::default(),
            deleted: Default::default(),
            _deleted: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            item: Reference::default(),
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
    rh_foundation::Invariant::new("lst-1", rh_foundation::Severity::Error, "A list can only have an emptyReason if it is empty", "emptyReason.empty() or entry.empty()").with_xpath("not(exists(f:emptyReason) and exists(f:entry))"),
    rh_foundation::Invariant::new("lst-2", rh_foundation::Severity::Error, "The deleted flag can only be used if the mode of the list is \"changes\"", "mode = 'changes' or entry.deleted.empty()").with_xpath("(f:mode/@value = 'changes') or not(exists(f:entry/f:deleted))"),
    rh_foundation::Invariant::new("lst-3", rh_foundation::Severity::Error, "An entry date can only be used if the mode of the list is \"working\"", "mode = 'working' or entry.date.empty()").with_xpath("(f:mode/@value = 'working') or not(exists(f:entry/f:date))"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for List {
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

impl crate::traits::resource::ResourceMutators for List {
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

impl crate::traits::resource::ResourceExistence for List {
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

impl crate::traits::domain_resource::DomainResourceAccessors for List {
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

impl crate::traits::domain_resource::DomainResourceMutators for List {
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

impl crate::traits::domain_resource::DomainResourceExistence for List {
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

impl crate::traits::list::ListAccessors for List {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ListStatus {
        self.status.clone()
    }
    fn mode(&self) -> ListMode {
        self.mode.clone()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn source(&self) -> Option<Reference> {
        self.source.clone()
    }
    fn ordered_by(&self) -> Option<CodeableConcept> {
        self.ordered_by.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn entry(&self) -> &[ListEntry] {
        self.entry.as_deref().unwrap_or(&[])
    }
    fn empty_reason(&self) -> Option<CodeableConcept> {
        self.empty_reason.clone()
    }
}

impl crate::traits::list::ListMutators for List {
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
    fn set_status(self, value: ListStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_mode(self, value: ListMode) -> Self {
        let mut resource = self.clone();
        resource.mode = value;
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_source(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.source = Some(value);
        resource
    }
    fn set_ordered_by(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.ordered_by = Some(value);
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
    fn set_entry(self, value: Vec<ListEntry>) -> Self {
        let mut resource = self.clone();
        resource.entry = Some(value);
        resource
    }
    fn add_entry(self, item: ListEntry) -> Self {
        let mut resource = self.clone();
        resource.entry.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_empty_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.empty_reason = Some(value);
        resource
    }
}

impl crate::traits::list::ListExistence for List {
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
    fn has_status(&self) -> bool {
        true
    }
    fn has_mode(&self) -> bool {
        true
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_source(&self) -> bool {
        self.source.is_some()
    }
    fn has_ordered_by(&self) -> bool {
        self.ordered_by.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_entry(&self) -> bool {
        self.entry.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_empty_reason(&self) -> bool {
        self.empty_reason.is_some()
    }
}

impl crate::validation::ValidatableResource for List {
    fn resource_type(&self) -> &'static str {
        "List"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/List")
    }
}
