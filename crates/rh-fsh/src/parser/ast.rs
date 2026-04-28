//! FSH Abstract Syntax Tree types

use crate::parser::span::SourceLocation;
use serde::{Deserialize, Serialize};

/// A complete FSH document containing all parsed entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FshDocument {
    pub entities: Vec<Spanned<FshEntity>>,
    pub source_name: String,
}

/// A value with its source location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spanned<T> {
    pub value: T,
    pub location: SourceLocation,
}

impl<T> Spanned<T> {
    pub fn new(value: T, location: SourceLocation) -> Self {
        Self { value, location }
    }
}

/// Top-level FSH entity discriminant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FshEntity {
    Profile(Profile),
    Extension(Extension),
    Logical(Logical),
    Resource(ResourceDef),
    Instance(Instance),
    ValueSet(ValueSet),
    CodeSystem(CodeSystem),
    Invariant(Invariant),
    Mapping(Mapping),
    RuleSet(RuleSet),
    ParamRuleSet(ParamRuleSet),
    Alias(Alias),
}

// ============================================================================
// Shared StructureDefinition metadata
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdMetadata {
    pub name: String,
    pub parent: Option<String>,
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    /// FSH Characteristics (only used for Logical models, e.g. `#can-be-target`)
    pub characteristics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub metadata: SdMetadata,
    pub rules: Vec<Spanned<SdRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub metadata: SdMetadata,
    pub rules: Vec<Spanned<SdRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logical {
    pub metadata: SdMetadata,
    pub rules: Vec<Spanned<SdRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDef {
    pub metadata: SdMetadata,
    pub rules: Vec<Spanned<SdRule>>,
}

// ============================================================================
// Instance
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceMetadata {
    pub name: String,
    pub instance_of: String,
    pub usage: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub metadata: InstanceMetadata,
    pub rules: Vec<Spanned<InstanceRule>>,
}

// ============================================================================
// ValueSet
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VsMetadata {
    pub name: String,
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSet {
    pub metadata: VsMetadata,
    pub components: Vec<Spanned<VsComponentRule>>,
    pub caret_rules: Vec<Spanned<CaretValueRule>>,
}

// ============================================================================
// CodeSystem
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsMetadata {
    pub name: String,
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSystem {
    pub metadata: CsMetadata,
    pub concepts: Vec<Spanned<ConceptRule>>,
    pub caret_rules: Vec<Spanned<CaretValueRule>>,
}

// ============================================================================
// Other entities
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invariant {
    pub name: String,
    pub description: Option<String>,
    pub expression: Option<String>,
    pub severity: Option<String>,
    pub xpath: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingMetadata {
    pub name: String,
    pub source: Option<String>,
    pub target: Option<String>,
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub metadata: MappingMetadata,
    pub rules: Vec<Spanned<MappingRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    pub name: String,
    pub rules: Vec<Spanned<SdRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamRuleSet {
    pub name: String,
    pub params: Vec<String>,
    pub rules: Vec<Spanned<SdRule>>,
    /// Raw text of each rule line (including leading `* `), used for parameter substitution
    pub raw_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alias {
    pub name: String,
    pub value: String,
}

// ============================================================================
// SD Rules
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SdRule {
    Card(CardRule),
    Flag(FlagRule),
    Binding(BindingRule),
    Assignment(AssignmentRule),
    Contains(ContainsRule),
    Only(OnlyRule),
    Obeys(ObeysRule),
    CaretValue(CaretValueRule),
    Insert(InsertRule),
    AddElement(AddElementRule),
    Path(PathRule),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstanceRule {
    Assignment(AssignmentRule),
    Insert(InsertRule),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardRule {
    pub path: FshPath,
    pub min: Option<u32>,
    pub max: Option<String>,
    pub flags: Vec<FshFlag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagRule {
    pub paths: Vec<FshPath>,
    pub flags: Vec<FshFlag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingRule {
    pub path: FshPath,
    pub value_set: String,
    pub strength: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentRule {
    pub path: FshPath,
    pub value: FshValue,
    pub exactly: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainsRule {
    pub path: FshPath,
    pub items: Vec<ContainsItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainsItem {
    pub name: String,
    pub alias: Option<String>,
    pub min: Option<u32>,
    pub max: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlyRule {
    pub path: FshPath,
    pub types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObeysRule {
    pub path: Option<FshPath>,
    pub invariants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaretValueRule {
    pub path: Option<FshPath>,
    pub caret_path: String,
    pub value: FshValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertRule {
    pub rule_set: String,
    pub params: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddElementRule {
    pub path: FshPath,
    pub min: u32,
    pub max: String,
    pub types: Vec<String>,
    pub short: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathRule {
    pub path: FshPath,
}

// ============================================================================
// Concept / VS / Mapping rules
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRule {
    pub code: String,
    pub display: Option<String>,
    pub definition: Option<String>,
    pub hierarchy: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VsComponentRule {
    pub inclusion: bool,
    pub system: Option<String>,
    pub from_vs: Vec<String>,
    pub concepts: Vec<VsConceptRef>,
    pub filters: Vec<VsFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VsConceptRef {
    pub code: String,
    pub system: Option<String>,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VsFilter {
    pub property: String,
    pub op: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingRule {
    pub path: Option<FshPath>,
    pub map: String,
    pub comment: Option<String>,
    pub language: Option<String>,
}

// ============================================================================
// Values
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FshValue {
    Str(String),
    Bool(bool),
    Integer(i64),
    Decimal(f64),
    Code {
        system: Option<String>,
        code: String,
        display: Option<String>,
    },
    Quantity {
        value: f64,
        unit: String,
    },
    Ratio {
        numerator: Box<FshValue>,
        denominator: Box<FshValue>,
    },
    Reference(String),
    Canonical(String),
    /// A date value like 1960-04-25
    Date(String),
    /// A dateTime value like 1960-04-25T10:00:00Z
    DateTime(String),
    /// An inline instance reference: `contained[+] = myInstance`
    InstanceRef(String),
}

// ============================================================================
// Paths
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FshPath {
    pub segments: Vec<FshPathSegment>,
}

impl FshPath {
    /// Join path segments with '.', rendering slices as `element[slice]`.
    pub fn to_dot_string(&self) -> String {
        self.segments
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(".")
    }

    /// Build FHIR SD `path` value: `base.element.subelement` (slice names stripped).
    pub fn to_fhir_path(&self, base: &str) -> String {
        if self.segments.is_empty() {
            return base.to_string();
        }
        let parts: Vec<String> = self.segments.iter().map(|s| s.to_path_part()).collect();
        format!("{}.{}", base, parts.join("."))
    }

    /// Build FHIR SD `id` value: `base.element:slice.subelement` (colon-separated slices).
    pub fn to_fhir_id(&self, base: &str) -> String {
        if self.segments.is_empty() {
            return base.to_string();
        }
        let mut parts: Vec<String> = Vec::new();
        for seg in &self.segments {
            match seg {
                FshPathSegment::Slice { element, slice } => {
                    parts.push(format!("{element}:{slice}"));
                }
                FshPathSegment::Name(n) => {
                    // Normalize choice-type resolved names (e.g. valueString → value[x]) in ids too
                    parts.push(normalize_choice_type_name(n).unwrap_or_else(|| n.clone()));
                }
                other => parts.push(other.to_string()),
            }
        }
        format!("{}.{}", base, parts.join("."))
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}

impl std::fmt::Display for FshPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_dot_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FshPathSegment {
    Name(String),
    Index(u32),
    /// Sliced element: `identifier[AccessionIdentifier]`
    Slice {
        element: String,
        slice: String,
    },
    ChoiceType(String),
    Extension(String),
}

/// Known FHIR R4 choice element base names (the part before `[x]`).
/// These are the element bases that can appear with type suffixes (e.g., `value` → `valueString`, `valueQuantity`).
/// Only names in this list will be normalized back to the `[x]` form.
static FHIR_CHOICE_BASES: &[&str] = &[
    "value",         // Extension.value[x], Observation.value[x], many others
    "deceased",      // Patient.deceased[x]
    "effective",     // Observation.effective[x], DiagnosticReport.effective[x]
    "onset",         // Condition.onset[x]
    "abatement",     // Condition.abatement[x]
    "multipleBirth", // Patient.multipleBirth[x]
    "occurrence",    // Immunization.occurrence[x], MedicationAdministration.occurrence[x]
    "performed",     // Procedure.performed[x]
    "serviced",      // ExplanationOfBenefit.serviced[x]
    "scheduled",     // CarePlan.activity.detail.scheduled[x]
    "asNeeded",      // Dosage.asNeeded[x]
    "dose",          // Dosage.doseAndRate.dose[x]
    "rate",          // Dosage.doseAndRate.rate[x]
    "age",           // FamilyMemberHistory.age[x]
    "born",          // FamilyMemberHistory.born[x]
    "due",           // Goal.due[x]
    "answer",        // QuestionnaireResponse.item.answer.value[x]
    "timing",        // PlanDefinition.action.timing[x]
    "allowed",       // CoverageEligibilityResponse benefit.allowed[x]
    "used",          // CoverageEligibilityResponse benefit.used[x]
    "fastingStatus", // ServiceRequest.fastingStatus[x]
    "diagnosis",     // Claim.diagnosis.diagnosis[x]
    "collected",     // Specimen.collection.collected[x]
    "quantity",      // Substance.ingredient.quantity[x]
];

/// FHIR type names that can appear as capitalized suffixes in choice-type path names.
static FHIR_TYPE_SUFFIXES: &[&str] = &[
    // Complex types (longer first to match greedily)
    "CodeableConcept",
    "SampledData",
    "BackboneElement",
    "RelatedArtifact",
    "ParameterDefinition",
    "TriggerDefinition",
    "ContactDetail",
    "DataRequirement",
    "UsageContext",
    "SimpleQuantity",
    "Contributor",
    "Attachment",
    "Expression",
    "HumanName",
    "Identifier",
    "Annotation",
    "Signature",
    "Canonical",
    "Quantity",
    "Distance",
    "Duration",
    "Timing",
    "Period",
    "Range",
    "Ratio",
    "Count",
    "Money",
    "Coding",
    "Address",
    "ContactPoint",
    "Reference",
    "Age",
    // Primitive types
    "Base64Binary",
    "PositiveInt",
    "UnsignedInt",
    "DateTime",
    "Instant",
    "Boolean",
    "Integer",
    "Decimal",
    "String",
    "Xhtml",
    "Date",
    "Time",
    "Code",
    "Uuid",
    "Oid",
    "Uri",
    "Url",
    "Id",
];

/// Attempt to normalize a choice-type resolved name back to its `[x]` form.
/// `valueString` → `value[x]`, `deceasedBoolean` → `deceased[x]`, `effectiveDateTime` → `effective[x]`
/// Returns `None` if the name doesn't match a known FHIR choice element pattern.
pub fn normalize_choice_type_name(name: &str) -> Option<String> {
    for suffix in FHIR_TYPE_SUFFIXES {
        if let Some(base) = name.strip_suffix(suffix) {
            // Only normalize when the base is a known FHIR choice element name
            if FHIR_CHOICE_BASES.contains(&base) {
                return Some(format!("{base}[x]"));
            }
        }
    }
    None
}

impl FshPathSegment {
    /// Render just the element name (no slice suffix) — used for FHIR SD `path`.
    /// Choice-type resolved names (e.g., `valueString`, `deceasedBoolean`) are normalized to `[x]` form.
    pub fn to_path_part(&self) -> String {
        match self {
            FshPathSegment::Name(n) => normalize_choice_type_name(n).unwrap_or_else(|| n.clone()),
            FshPathSegment::Index(i) => format!("[{i}]"),
            FshPathSegment::Slice { element, .. } => element.clone(),
            FshPathSegment::ChoiceType(t) => format!("{t}[x]"),
            FshPathSegment::Extension(e) => format!("extension({e})"),
        }
    }
}

impl std::fmt::Display for FshPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FshPathSegment::Name(n) => write!(f, "{n}"),
            FshPathSegment::Index(i) => write!(f, "[{i}]"),
            FshPathSegment::Slice { element, slice } => write!(f, "{element}[{slice}]"),
            FshPathSegment::ChoiceType(t) => write!(f, "{t}[x]"),
            FshPathSegment::Extension(e) => write!(f, "extension({e})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FshFlag {
    MustSupport,
    SummaryElement,
    Modifier,
    Normative,
    TrialUse,
    Draft,
}
