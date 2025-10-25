use crate::resources::family_member_history::FamilyMemberHistory;
use serde::{Deserialize, Serialize};
/// Family member history for genetics analysis
///
/// Adds additional information to a family member history supporting both the capture of mother/father relationships as well as additional observations necessary to enable genetics-based risk analysis for patients
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/familymemberhistory-genetic
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: FamilyMemberHistory
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FamilymemberhistoryGenetic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: FamilyMemberHistory,
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
    rh_foundation::Invariant::new("fhs-1", rh_foundation::Severity::Error, "Can have age[x] or born[x], but not both", "age.empty() or born.empty()").with_xpath("not (*[starts-with(local-name(.), 'age')] and *[starts-with(local-name(.), 'birth')])"),
    rh_foundation::Invariant::new("fhs-2", rh_foundation::Severity::Error, "Can only have estimatedAge if age[x] is present", "age.exists() or estimatedAge.empty()").with_xpath("exists(*[starts-with(local-name(.), 'age')]) or not(exists(f:estimatedAge))"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "FamilyMemberHistory.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/history-status|4.0.1",
        )
        .with_description("A code that identifies the status of the family history record.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("FamilyMemberHistory", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.contained", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.extension", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.extension", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.extension", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.identifier", 0, None),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.instantiatesCanonical",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.instantiatesUri", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.dataAbsentReason",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.patient", 1, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.relationship", 1, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.sex", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.born[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.age[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.estimatedAge", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.deceased[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.reasonCode", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.reasonReference", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.note", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.condition", 0, None),
            rh_foundation::ElementCardinality::new("FamilyMemberHistory.condition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.outcome",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.contributedToDeath",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.onset[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "FamilyMemberHistory.condition.note",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for FamilymemberhistoryGenetic {
    fn id(&self) -> Option<String> {
        self.base.id()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.meta()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.implicit_rules()
    }
    fn language(&self) -> Option<String> {
        self.base.language()
    }
}

impl crate::traits::resource::ResourceMutators for FamilymemberhistoryGenetic {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_id(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_meta(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_implicit_rules(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_language(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for FamilymemberhistoryGenetic {
    fn has_id(&self) -> bool {
        self.base.has_id()
    }
    fn has_meta(&self) -> bool {
        self.base.has_meta()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.has_implicit_rules()
    }
    fn has_language(&self) -> bool {
        self.base.has_language()
    }
}

impl crate::traits::familymemberhistory_genetic::FamilymemberhistoryGeneticMutators
    for FamilymemberhistoryGenetic
{
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for FamilymemberhistoryGenetic {
    fn resource_type(&self) -> &'static str {
        "FamilyMemberHistory"
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
        Some("http://hl7.org/fhir/StructureDefinition/familymemberhistory-genetic")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::familymemberhistory_genetic::{
    FamilymemberhistoryGeneticAccessors, FamilymemberhistoryGeneticExistence,
    FamilymemberhistoryGeneticMutators,
};
