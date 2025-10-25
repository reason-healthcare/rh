use crate::resources::observation::Observation;
use serde::{Deserialize, Serialize};
/// Example Lipid Profile
///
/// HDL Cholesterol Result
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/hdlcholesterol
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Hdlcholesterol {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Observation,
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
    rh_foundation::Invariant::new("obs-3", rh_foundation::Severity::Error, "Must have at least a low or a high or text", "low.exists() or high.exists() or text.exists()").with_xpath("(exists(f:low) or exists(f:high)or exists(f:text))"),
    rh_foundation::Invariant::new("obs-6", rh_foundation::Severity::Error, "dataAbsentReason SHALL only be present if Observation.value[x] is not present", "dataAbsentReason.empty() or value.empty()").with_xpath("not(exists(f:dataAbsentReason)) or (not(exists(*[starts-with(local-name(.), 'value')])))"),
    rh_foundation::Invariant::new("obs-7", rh_foundation::Severity::Error, "If Observation.code is the same as an Observation.component.code then the value element associated with the code SHALL NOT be present", "value.empty() or component.code.where(coding.intersect(%resource.code.coding).exists()).empty()").with_xpath("not(f:*[starts-with(local-name(.), 'value')] and (for $coding in f:code/f:coding return f:component/f:code/f:coding[f:code/@value=$coding/f:code/@value] [f:system/@value=$coding/f:system/@value]))"),
    rh_foundation::Invariant::new("qty-3", rh_foundation::Severity::Error, "If a code for the unit is present, the system SHALL also be present", "code.empty() or system.exists()").with_xpath("not(exists(f:code)) or exists(f:system)"),
    rh_foundation::Invariant::new("sqty-1", rh_foundation::Severity::Error, "The comparator is not used on a SimpleQuantity", "comparator.empty()").with_xpath("not(exists(f:comparator))"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "Observation.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/observation-status|4.0.1",
        )
        .with_description("Codes providing the status of an observation.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Observation", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.contained", 0, None),
            rh_foundation::ElementCardinality::new("Observation.extension", 0, None),
            rh_foundation::ElementCardinality::new("Observation.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Observation.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Observation.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("Observation.partOf", 0, None),
            rh_foundation::ElementCardinality::new("Observation.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.category", 0, None),
            rh_foundation::ElementCardinality::new("Observation.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.focus", 0, None),
            rh_foundation::ElementCardinality::new("Observation.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.effective[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.issued", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.performer", 0, None),
            rh_foundation::ElementCardinality::new("Observation.value[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.value[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.dataAbsentReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.interpretation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.note", 0, None),
            rh_foundation::ElementCardinality::new("Observation.bodySite", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.method", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.specimen", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.device", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.referenceRange", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Observation.referenceRange.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.low", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.high", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.type", 0, Some(0)),
            rh_foundation::ElementCardinality::new(
                "Observation.referenceRange.appliesTo",
                0,
                Some(0),
            ),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.age", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Observation.referenceRange.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.hasMember", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Observation.derivedFrom", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Observation.component", 0, None),
            rh_foundation::ElementCardinality::new("Observation.component.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.component.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Observation.component.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Observation.component.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Observation.component.value[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Observation.component.dataAbsentReason",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Observation.component.interpretation", 0, None),
            rh_foundation::ElementCardinality::new("Observation.component.referenceRange", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Hdlcholesterol {
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

impl crate::traits::resource::ResourceMutators for Hdlcholesterol {
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

impl crate::traits::resource::ResourceExistence for Hdlcholesterol {
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

impl crate::traits::hdlcholesterol::HdlcholesterolMutators for Hdlcholesterol {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Hdlcholesterol {
    fn resource_type(&self) -> &'static str {
        "Observation"
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
        Some("http://hl7.org/fhir/StructureDefinition/hdlcholesterol")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::hdlcholesterol::{
    HdlcholesterolAccessors, HdlcholesterolExistence, HdlcholesterolMutators,
};
