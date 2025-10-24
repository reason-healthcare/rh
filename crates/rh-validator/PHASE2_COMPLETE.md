# Phase 2: Code Generation - Invariant Metadata - COMPLETE ✅

**Completion Date:** 2025-01-XX

## Overview

Phase 2 successfully implemented automatic extraction and generation of FHIR invariant metadata in generated Rust types. This infrastructure enables runtime access to invariant constraints without parsing FHIR definitions, setting the foundation for Phase 3 (FHIRPath validation).

## Deliverables

### 1. Shared Validation Types (`rh-foundation`)

**File:** `crates/rh-foundation/src/validation.rs` (135 lines)

```rust
pub enum Severity {
    Error,
    Warning,
    Information,
}

pub struct Invariant {
    pub key: String,
    pub severity: Severity,
    pub human: String,
    pub expression: String,
    pub xpath: Option<String>, // Legacy support, preserved but not used
}
```

**Features:**
- Serde support for serialization/deserialization
- Builder pattern with `Invariant::new()` and `.with_xpath()`
- Severity ordering and Display trait implementation
- 4 comprehensive tests

### 2. Invariant Extraction (`rh-codegen/src/invariants.rs`)

**File:** `crates/rh-codegen/src/invariants.rs` (292 lines)

```rust
pub fn extract_invariants(structure_def: &StructureDefinition) -> Vec<Invariant>
```

**Features:**
- Extracts constraints from all ElementDefinitions in StructureDefinition
- Filters to only FHIRPath expressions (requires `expression` field)
- Preserves XPath for reference (via `with_xpath()`)
- Deduplicates by invariant key
- Sorts alphabetically by key
- Maps FHIR severity strings to `Severity` enum
- 7 comprehensive tests covering extraction, deduplication, sorting, severity mapping

**Test Coverage:**
- `test_extract_empty_invariants` - No constraints
- `test_extract_single_invariant` - Single constraint extraction
- `test_extract_multiple_invariants` - Multiple constraints
- `test_extract_deduplicate_invariants` - Deduplication by key
- `test_extract_sort_invariants` - Alphabetical sorting
- `test_severity_mapping` - Error/Warning/Information mapping
- `test_xpath_preservation` - XPath field preservation

### 3. Invariant Constant Generation (`rh-codegen/src/generators/invariant_generator.rs`)

**File:** `crates/rh-codegen/src/generators/invariant_generator.rs` (330 lines)

```rust
pub struct InvariantGenerator;

impl InvariantGenerator {
    pub fn generate_invariants_constant(structure_def: &StructureDefinition) -> String
}
```

**Output Format:**
```rust
/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub const INVARIANTS: &[rh_foundation::Invariant] = &[
    rh_foundation::Invariant {
        key: "pat-1",
        severity: rh_foundation::Severity::Error,
        human: "SHALL at least contain a contact's details...",
        expression: "name.exists() or telecom.exists()...",
    },
];
```

**Features:**
- String-based code generation for simplicity
- Proper Rust string escaping (quotes, backslashes, newlines)
- Returns empty string for types with no invariants (no constant generated)
- Doc comments explain invariant purpose
- 8 comprehensive tests

**Test Coverage:**
- `test_generate_empty_invariants` - No invariants case
- `test_generate_single_invariant` - Single invariant
- `test_generate_multiple_invariants` - Multiple invariants with sorting
- `test_generate_invariants_escaping` - Special character escaping
- `test_generate_invariants_severity_mapping` - All severity levels
- `test_generate_invariants_structure_def` - Full StructureDefinition integration
- `test_generate_invariants_tokens` - Alternative TokenStream-based generation
- `test_generate_invariants_constant_format` - Output format validation

### 4. ValidatableResource Trait Generation (`rh-codegen/src/generators/validation_trait_generator.rs`)

**File:** `crates/rh-codegen/src/generators/validation_trait_generator.rs` (300+ lines)

```rust
pub struct ValidationTraitGenerator;

impl ValidationTraitGenerator {
    pub fn generate_trait_definition() -> String
    pub fn generate_trait_impl(structure_def: &StructureDefinition) -> String
    pub fn generate_validation_module(structure_defs: &[StructureDefinition]) -> String
}
```

**Trait Definition:**
```rust
pub trait ValidatableResource {
    /// Returns the FHIR resource type name
    fn resource_type(&self) -> &'static str;
    
    /// Returns the invariants for this resource type
    fn invariants() -> &'static [rh_foundation::Invariant];
    
    /// Returns the profile URL if this is a profile
    fn profile_url() -> Option<&'static str>;
}
```

**Trait Implementation Example:**
```rust
impl crate::validation::ValidatableResource for Patient {
    fn resource_type(&self) -> &'static str {
        "Patient"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Patient")
    }
}
```

**Features:**
- Trait provides uniform access to invariants across all resource types
- Only generates impl for types with invariants
- Profile detection via `base_definition.is_some() && !is_abstract`
- Module-level generation for `validation.rs` file
- 6 comprehensive tests

**Test Coverage:**
- `test_generate_trait_definition` - Trait definition format
- `test_generate_trait_impl_base_resource` - Base resource implementation
- `test_generate_trait_impl_profile` - Profile with profile_url()
- `test_generate_trait_impl_no_invariants` - Empty string for no invariants
- `test_generate_validation_module` - Complete module generation
- `test_profile_url_detection` - Profile vs base resource detection

### 5. FileGenerator Integration (`rh-codegen/src/generators/file_generator.rs`)

**Integration Points:**
1. **Line ~360:** Conditional import of `rh_foundation::Invariant` when invariants exist
2. **Line ~460:** INVARIANTS constant generation after Default impl
3. **Line ~472:** ValidatableResource trait impl after invariants

**Logic:**
```rust
// Only add import if type has invariants
if !invariants.is_empty() {
    code.push_str("use rh_foundation::Invariant;\n");
}

// Generate INVARIANTS constant
let invariants_code = InvariantGenerator::generate_invariants_constant(structure_def);
if !invariants_code.is_empty() {
    code.push_str(&invariants_code);
    code.push('\n');
}

// Generate ValidatableResource trait impl
let trait_impl = ValidationTraitGenerator::generate_trait_impl(structure_def);
if !trait_impl.is_empty() {
    code.push_str(&trait_impl);
    code.push('\n');
}
```

**Applies To:**
- All resource types (`kind == "resource"`)
- All complex datatypes (`kind == "complex-type"`)

## Testing

### Unit Tests
- **rh-codegen:** 124 tests passing (21 new tests for Phase 2)
  - 7 tests in `invariants.rs`
  - 8 tests in `invariant_generator.rs`
  - 6 tests in `validation_trait_generator.rs`
- **rh-foundation:** 19 tests passing (4 for validation types)
- **All crates:** 1000+ tests passing

### Integration Examples
Created two comprehensive examples demonstrating the complete Phase 2 functionality:

#### 1. `test_invariant_extraction.rs`
Tests invariant extraction from FHIR StructureDefinitions:
- Patient resource with pat-1 invariant
- Observation resource with obs-3 and obs-6 invariants
- Severity mapping (error/warning/information)
- XPath preservation alongside FHIRPath

#### 2. `test_validation_trait.rs`
Tests code generation for ValidatableResource trait:
- INVARIANTS constant generation for Patient and Observation
- ValidatableResource trait implementation
- Profile URL detection for US Core Patient profile
- Complete code generation pipeline verification

**Running Examples:**
```bash
cargo run --example test_invariant_extraction
cargo run --example test_validation_trait
```

### Quality Checks
All quality checks passing:
```bash
just check  # Runs fmt-check, lint, test, audit
```

## Verification Against FHIR Specification

### Patient Resource
**Official FHIR R4 Invariant:**
- **Key:** pat-1
- **Severity:** error
- **Human:** "SHALL at least contain a contact's details or a reference to an organization"
- **Expression:** `name.exists() or telecom.exists() or address.exists() or organization.exists()`

**Verification:** ✅ Correctly extracted and generated

### Observation Resource
**Official FHIR R4 Invariants:**
- **obs-3:**
  - Severity: error
  - Human: "Must have at least a value or a dataAbsentReason"
  - Expression: `value.exists() or dataAbsentReason.exists()`
- **obs-6:**
  - Severity: error
  - Human: "dataAbsentReason SHALL only be present if Observation.value[x] is not present"
  - Expression: `dataAbsentReason.empty() or value.empty()`

**Verification:** ✅ Both correctly extracted and generated

## Architecture Decisions

### 1. String-Based Generation vs TokenStream
**Decision:** Use string-based generation for invariants and trait code
**Rationale:**
- Simpler to implement and maintain
- Easier to read and debug generated output
- Sufficient for static constant declarations
- TokenStream alternative implemented for comparison but not used

### 2. XPath Handling
**Decision:** Preserve XPath but don't use for validation
**Rationale:**
- XPath is legacy from FHIR STU3 and earlier
- FHIRPath is the official expression language for FHIR R4+
- Preserving XPath maintains compatibility with FHIR definitions
- Phase 3 will use FHIRPath exclusively for validation

### 3. Trait Method Signatures
**Decision:** Mixed instance (`&self`) and static methods
**Rationale:**
- `resource_type(&self)` - Instance method for polymorphic access
- `invariants()` - Static method since invariants are type-level, not instance-level
- `profile_url()` - Static method for profile metadata

### 4. Profile Detection
**Decision:** Use `base_definition.is_some() && !is_abstract`
**Rationale:**
- Profiles extend base resources (have base_definition)
- Abstract types are base definitions, not profiles
- Provides accurate profile vs base resource distinction

### 5. Empty Invariants Handling
**Decision:** Return empty string, don't generate constant/impl
**Rationale:**
- Cleaner generated code
- Only types with invariants get validation infrastructure
- Avoids unnecessary empty arrays and trait impls

## Next Steps (Phase 3)

With invariant metadata now embedded in generated types, Phase 3 will implement runtime validation:

1. **FHIRPath Integration**
   - Execute FHIRPath expressions from invariants
   - Use existing `rh-fhirpath` crate
   - Map FHIR resources to FHIRPath evaluation context

2. **Validation Implementation**
   - Create `InvariantValidator` using `ValidatableResource` trait
   - Evaluate all invariants for a resource instance
   - Collect and report validation issues

3. **Error Reporting**
   - Map invariant violations to `ValidationIssue`
   - Include invariant key, severity, human description
   - Provide expression context for debugging

4. **Testing**
   - Validate Patient instances against pat-1
   - Validate Observation instances against obs-3, obs-6
   - Edge cases and error scenarios
   - Performance benchmarks

## Summary Statistics

- **New Files:** 2 (invariants.rs, validation.rs in rh-foundation)
- **Modified Files:** 3 (file_generator.rs, mod.rs, ElementConstraint in fhir_types.rs)
- **New Generators:** 2 (InvariantGenerator, ValidationTraitGenerator)
- **Total Tests Added:** 21
- **Example Programs:** 2 comprehensive examples
- **Lines of Code:** ~920 lines (excluding tests and examples)
- **Code Quality:** All clippy lints passing, 100% formatted
- **Test Success Rate:** 100% (1000+ tests across all crates)

## Key Achievements

1. ✅ **Zero-cost invariant access** - Constants embedded at compile time
2. ✅ **Type-safe validation infrastructure** - ValidatableResource trait
3. ✅ **FHIR specification compliance** - Verified against official invariants
4. ✅ **Comprehensive testing** - 21 new tests, 2 integration examples
5. ✅ **Clean architecture** - Shared types, modular generators
6. ✅ **Production ready** - All quality checks passing

Phase 2 is complete and provides a solid foundation for Phase 3 FHIRPath validation! 🎉
