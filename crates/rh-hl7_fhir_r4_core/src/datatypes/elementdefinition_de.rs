use crate::datatypes::element_definition::ElementDefinition;
use serde::{Deserialize, Serialize};
/// DataElement constraint on ElementDefinition data type
///
/// Identifies how the ElementDefinition data type is used when it appears within a data element
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-de
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: ElementDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ElementDefinition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionDe {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: ElementDefinition,
}

impl Default for ElementdefinitionDe {
    fn default() -> Self {
        Self {
            base: ElementDefinition::default(),
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
    rh_foundation::Invariant::new("eld-1", rh_foundation::Severity::Error, "If there are no discriminators, there must be a definition", "discriminator.exists() or description.exists()").with_xpath("(f:discriminator) or (f:description)"),
    rh_foundation::Invariant::new("eld-11", rh_foundation::Severity::Error, "Binding can only be present for coded elements, string, and uri", "binding.empty() or type.code.empty() or type.select((code = 'code') or (code = 'Coding') or (code='CodeableConcept') or (code = 'Quantity') or (code = 'string') or (code = 'uri')).exists()").with_xpath("not(exists(f:binding)) or (count(f:type/f:code) = 0) or  f:type/f:code/@value=('code','Coding','CodeableConcept','Quantity','string', 'uri')"),
    rh_foundation::Invariant::new("eld-12", rh_foundation::Severity::Error, "ValueSet SHALL start with http:// or https:// or urn:", "valueSet.exists() implies (valueSet.startsWith('http:') or valueSet.startsWith('https') or valueSet.startsWith('urn:'))").with_xpath("(starts-with(string(f:valueSet/@value), 'http:') or starts-with(string(f:valueSet/@value), 'https:') or starts-with(string(f:valueSet/@value), 'urn:'))"),
    rh_foundation::Invariant::new("eld-13", rh_foundation::Severity::Error, "Types must be unique by code", "type.select(code).isDistinct()").with_xpath("not(exists(for $type in f:type return $type/preceding-sibling::f:type[f:code/@value=$type/f:code/@value]))"),
    rh_foundation::Invariant::new("eld-14", rh_foundation::Severity::Error, "Constraints must be unique by key", "constraint.select(key).isDistinct()").with_xpath("count(f:constraint) = count(distinct-values(f:constraint/f:key/@value))"),
    rh_foundation::Invariant::new("eld-15", rh_foundation::Severity::Error, "default value and meaningWhenMissing are mutually exclusive", "defaultValue.empty() or meaningWhenMissing.empty()").with_xpath("not(exists(f:*[starts-with(local-name(.), 'fixed')])) or not(exists(f:meaningWhenMissing))"),
    rh_foundation::Invariant::new("eld-16", rh_foundation::Severity::Error, "sliceName must be composed of proper tokens separated by \"/\"", "sliceName.empty() or sliceName.matches('^[a-zA-Z0-9\\\\/\\\\-_\\\\[\\\\]\\\\@]+$')").with_xpath("not(exists(f:sliceName/@value)) or matches(f:sliceName/@value, '^[a-zA-Z0-9\\/\\-_\\[\\]\\@]+$')"),
    rh_foundation::Invariant::new("eld-17", rh_foundation::Severity::Error, "targetProfile is only allowed if the type is Reference or canonical", "(code='Reference' or code = 'canonical') or targetProfile.empty()").with_xpath("not(exists(f:targetProfile)) or (f:code/@value = 'Reference')"),
    rh_foundation::Invariant::new("eld-18", rh_foundation::Severity::Error, "Must have a modifier reason if isModifier = true", "(isModifier.exists() and isModifier) implies isModifierReason.exists()").with_xpath("not(f:isModifier/@value = 'true') or exists(f:isModifierReason)"),
    rh_foundation::Invariant::new("eld-19", rh_foundation::Severity::Error, "Element names cannot include some special characters", "path.matches('[^\\\\s\\\\.,:;\\\\\\'\"\\\\/|?!@#$%&*()\\\\[\\\\]{}]{1,64}(\\\\.[^\\\\s\\\\.,:;\\\\\\'\"\\\\/|?!@#$%&*()\\\\[\\\\]{}]{1,64}(\\\\[x\\\\])?(\\\\:[^\\\\s\\\\.]+)?)*')").with_xpath("matches(path/@value, '[^\\s\\.,:;\\'&quot;\\/|?!@#$%&amp;*()\\[\\]{}]{1,64}(\\.[^\\s\\.,:;\\'&quot;\\/|?!@#$%&amp;*()\\[\\]{}]{1,64}(\\[x\\])?(\\:[^\\s\\.]+)?)*')"),
    rh_foundation::Invariant::new("eld-2", rh_foundation::Severity::Error, "Min <= Max", "min.empty() or max.empty() or (max = '*') or iif(max != '*', min <= max.toInteger())").with_xpath("not(exists(f:min)) or not(exists(f:max)) or (not(f:max/@value) and not(f:min/@value)) or (f:max/@value = '*') or (number(f:max/@value) >= f:min/@value)"),
    rh_foundation::Invariant::new("eld-20", rh_foundation::Severity::Warning, "Element names should be simple alphanumerics with a max of 64 characters, or code generation tools may be broken", "path.matches('[A-Za-z][A-Za-z0-9]*(\\\\.[a-z][A-Za-z0-9]*(\\\\[x])?)*')").with_xpath("matches(path/@value, '[A-Za-z][A-Za-z0-9]*(\\.[a-z][A-Za-z0-9]*(\\[x])?)*')"),
    rh_foundation::Invariant::new("eld-21", rh_foundation::Severity::Warning, "Constraints should have an expression or else validators will not be able to enforce them", "expression.exists()").with_xpath("exists(f:expression/@value)"),
    rh_foundation::Invariant::new("eld-22", rh_foundation::Severity::Error, "sliceIsConstraining can only appear if slicename is present", "sliceIsConstraining.exists() implies sliceName.exists()").with_xpath("exists(f:sliceName) or not(exists(f:sliceIsConstraining))"),
    rh_foundation::Invariant::new("eld-3", rh_foundation::Severity::Error, "Max SHALL be a number or \"*\"", "empty() or ($this = '*') or (toInteger() >= 0)").with_xpath("@value='*' or (normalize-space(@value)!='' and normalize-space(translate(@value, '0123456789',''))='')"),
    rh_foundation::Invariant::new("eld-4", rh_foundation::Severity::Error, "Aggregation may only be specified if one of the allowed types for the element is a reference", "aggregation.empty() or (code = 'Reference') or (code = 'canonical')").with_xpath("not(exists(f:aggregation)) or exists(f:code[@value = 'Reference']) or exists(f:code[@value = 'canonical'])"),
    rh_foundation::Invariant::new("eld-5", rh_foundation::Severity::Error, "if the element definition has a contentReference, it cannot have type, defaultValue, fixed, pattern, example, minValue, maxValue, maxLength, or binding", "contentReference.empty() or (type.empty() and defaultValue.empty() and fixed.empty() and pattern.empty() and example.empty() and minValue.empty() and maxValue.empty() and maxLength.empty() and binding.empty())").with_xpath("not(exists(f:contentReference) and (exists(f:type) or exists(f:*[starts-with(local-name(.), 'value')]) or exists(f:*[starts-with(local-name(.), 'defaultValue')])  or exists(f:*[starts-with(local-name(.), 'fixed')]) or exists(f:*[starts-with(local-name(.), 'pattern')]) or exists(f:*[starts-with(local-name(.), 'example')]) or exists(f:*[starts-with(local-name(.), 'f:minValue')]) or exists(f:*[starts-with(local-name(.), 'f:maxValue')]) or exists(f:maxLength) or exists(f:binding)))"),
    rh_foundation::Invariant::new("eld-6", rh_foundation::Severity::Error, "Fixed value may only be specified if there is one type", "fixed.empty() or (type.count()  <= 1)").with_xpath("not(exists(f:*[starts-with(local-name(.), 'fixed')])) or (count(f:type)<=1)"),
    rh_foundation::Invariant::new("eld-7", rh_foundation::Severity::Error, "Pattern may only be specified if there is one type", "pattern.empty() or (type.count() <= 1)").with_xpath("not(exists(f:*[starts-with(local-name(.), 'pattern')])) or (count(f:type)<=1)"),
    rh_foundation::Invariant::new("eld-8", rh_foundation::Severity::Error, "Pattern and fixed are mutually exclusive", "pattern.empty() or fixed.empty()").with_xpath("not(exists(f:*[starts-with(local-name(.), 'pattern')])) or not(exists(f:*[starts-with(local-name(.), 'fixed')]))"),
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
    rh_foundation::ElementBinding::new("ElementDefinition.binding.strength", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/binding-strength|4.0.1").with_description("Indication of the degree of conformance expectations associated with a binding."),
    rh_foundation::ElementBinding::new("ElementDefinition.constraint.severity", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/constraint-severity|4.0.1").with_description("SHALL applications comply with this constraint?"),
    rh_foundation::ElementBinding::new("ElementDefinition.extension.value[x]", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/ucum-units|4.0.1").with_description("Units of measure allowed for an element."),
    rh_foundation::ElementBinding::new("ElementDefinition.mapping.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/mimetypes|4.0.1").with_description("The mime type of an attachment. Any valid mime type is allowed."),
    rh_foundation::ElementBinding::new("ElementDefinition.representation", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/property-representation|4.0.1").with_description("How a property is represented when serialized."),
    rh_foundation::ElementBinding::new("ElementDefinition.slicing.discriminator.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/discriminator-type|4.0.1").with_description("How an element value is interpreted when discrimination is evaluated."),
    rh_foundation::ElementBinding::new("ElementDefinition.slicing.rules", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/resource-slicing-rules|4.0.1").with_description("How slices are interpreted when evaluating an instance."),
    rh_foundation::ElementBinding::new("ElementDefinition.type.aggregation", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/resource-aggregation-mode|4.0.1").with_description("How resource references can be aggregated."),
    rh_foundation::ElementBinding::new("ElementDefinition.type.versioning", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/reference-version-rules|4.0.1").with_description("Whether a reference needs to be version specific or version independent, or whether either can be used."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ElementDefinition", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.extension.extension",
                0,
                Some(0),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.extension.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.extension", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.extension.extension",
                0,
                Some(0),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.extension.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.path", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.representation", 0, Some(0)),
            rh_foundation::ElementCardinality::new("ElementDefinition.sliceName", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.sliceIsConstraining",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.label", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.code", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.slicing", 0, Some(0)),
            rh_foundation::ElementCardinality::new("ElementDefinition.slicing.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.slicing.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.slicing.discriminator",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.slicing.discriminator.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.slicing.discriminator.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.slicing.discriminator.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.slicing.discriminator.path",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.slicing.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.slicing.ordered", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.slicing.rules", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.short", 0, Some(0)),
            rh_foundation::ElementCardinality::new("ElementDefinition.definition", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.comment", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.requirements", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.alias", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.min", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.max", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.base", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.base.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.base.extension", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.base.path", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.base.min", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.base.max", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.contentReference",
                0,
                Some(0),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.type", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.extension", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.profile", 0, Some(0)),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.targetProfile", 0, None),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.type.aggregation",
                0,
                Some(0),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.versioning", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.defaultValue[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.meaningWhenMissing",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.orderMeaning", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.fixed[x]", 0, Some(0)),
            rh_foundation::ElementCardinality::new("ElementDefinition.pattern[x]", 0, Some(0)),
            rh_foundation::ElementCardinality::new("ElementDefinition.example", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.example.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.example.extension", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.example.label", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.example.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.minValue[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.maxValue[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.maxLength", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.condition", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.constraint", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.constraint.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.constraint.key", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.requirements",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.severity",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.human",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.xpath",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.source",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.mustSupport", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.isModifier", 0, Some(0)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.isModifierReason",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.isSummary", 0, Some(0)),
            rh_foundation::ElementCardinality::new("ElementDefinition.binding", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.binding.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.binding.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.binding.strength",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.binding.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.binding.valueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.mapping", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.mapping.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.mapping.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.mapping.identity",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.mapping.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.mapping.map", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.mapping.comment", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for ElementdefinitionDe {
    fn resource_type(&self) -> &'static str {
        "ElementDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/elementdefinition-de")
    }
}
