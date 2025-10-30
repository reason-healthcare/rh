# Alternate Validation Design: Dynamic Profile-Based Validation

## Overview

This document proposes an alternative validation architecture that moves away from compile-time code generation to a dynamic, runtime-based approach centered on **StructureDefinition snapshots**. This design enables validation against arbitrary FHIR profiles without requiring code regeneration.

## Current Approach (Phase 1-9)

### Strengths
- ✅ Compile-time type safety
- ✅ Zero runtime overhead for metadata access
- ✅ Fast validation (metadata embedded in generated code)
- ✅ Strong IDE support and autocomplete

### Limitations
- ❌ Requires code regeneration for new profiles (US Core, custom profiles)
- ❌ Cannot validate against dynamically loaded profiles
- ❌ Metadata scattered across generated code (invariants, bindings, cardinalities)
- ❌ Limited extensibility for profile-based validation
- ❌ No support for differential validation (base + profile constraints)

## New Approach: Snapshot-Based Validation

### Core Concept

**StructureDefinition Snapshots** are the complete, resolved view of all element definitions including:
- Base resource constraints
- Profile-specific constraints (cardinality, value restrictions, etc.)
- Inherited invariants
- Bindings (required, extensible, preferred)
- Extensions
- Slicing rules

For a **base resource** (e.g., Patient from FHIR R4):
```
Snapshot = Differential (they are the same)
```

For a **profile** (e.g., US Core Patient):
```
Snapshot = Base Patient + US Core Patient differential
```

### Key Design Principles

1. **Lazy Snapshot Generation**: Generate snapshots on-demand and cache them
2. **Profile Registry**: Centralized registry of loaded StructureDefinitions
3. **Smart Caching**: Cache both raw StructureDefinitions and generated snapshots
4. **Validation Rules Compilation**: Convert snapshots into optimized validation rules
5. **Backward Compatibility**: Support both static (codegen) and dynamic validation

---

## Architecture

### Component Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     FhirValidator                           │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           ProfileRegistry                            │  │
│  │  - Load StructureDefinitions from:                   │  │
│  │    • FHIR packages (~/.fhir/packages)               │  │
│  │    • Custom directories                              │  │
│  │    • URLs (lazy download)                            │  │
│  │  - Cache loaded definitions                          │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↓                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         SnapshotGenerator                            │  │
│  │  - Generate complete element hierarchy               │  │
│  │  - Merge base + profile differentials                │  │
│  │  - Resolve references                                │  │
│  │  - Cache generated snapshots                         │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↓                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │      ValidationRuleCompiler                          │  │
│  │  - Extract validation rules from snapshot:           │  │
│  │    • Cardinality constraints (min..max)             │  │
│  │    • Type constraints                                │  │
│  │    • Fixed values                                    │  │
│  │    • Pattern matching                                │  │
│  │    • Invariants (FHIRPath)                          │  │
│  │    • Bindings (required/extensible)                 │  │
│  │    • Slicing rules                                   │  │
│  │  - Compile to optimized representation               │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↓                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         ValidationEngine                             │  │
│  │  - Execute validation rules against resources        │  │
│  │  - Evaluate FHIRPath expressions                     │  │
│  │  - Check bindings against ValueSets                  │  │
│  │  - Validate slices                                   │  │
│  │  - Generate validation issues                        │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Data Structures

```rust
/// Profile registry for managing StructureDefinitions
pub struct ProfileRegistry {
    /// Loaded StructureDefinitions (URL → StructureDefinition)
    definitions: HashMap<String, StructureDefinition>,
    
    /// Generated snapshots (URL → Snapshot)
    snapshots: HashMap<String, Arc<Snapshot>>,
    
    /// Package directories to search
    package_dirs: Vec<PathBuf>,
    
    /// Cache for performance
    cache: LruCache<String, Arc<CompiledValidationRules>>,
}

/// Complete element hierarchy with all constraints resolved
pub struct Snapshot {
    /// URL of the profile/resource
    pub url: String,
    
    /// Base definition URL (if this is a profile)
    pub base: Option<String>,
    
    /// All elements in path order
    pub elements: Vec<ElementDefinition>,
    
    /// Metadata
    pub kind: String,  // "resource", "complex-type", "primitive-type"
    pub resource_type: Option<String>,
    
    /// Generation timestamp (for cache invalidation)
    pub generated_at: Instant,
}

/// Compiled validation rules for fast execution
pub struct CompiledValidationRules {
    /// Cardinality constraints (path → min..max)
    pub cardinality: HashMap<String, (usize, Option<usize>)>,
    
    /// Type constraints (path → allowed types)
    pub types: HashMap<String, Vec<String>>,
    
    /// Fixed values (path → fixed value)
    pub fixed_values: HashMap<String, serde_json::Value>,
    
    /// Pattern values (path → pattern)
    pub patterns: HashMap<String, serde_json::Value>,
    
    /// Invariants (key → FHIRPath expression)
    pub invariants: Vec<Invariant>,
    
    /// Required bindings (path → ValueSet URL)
    pub required_bindings: HashMap<String, String>,
    
    /// Extensible bindings (path → ValueSet URL)
    pub extensible_bindings: HashMap<String, String>,
    
    /// Slicing rules (path → slicing definition)
    pub slices: HashMap<String, SlicingRules>,
}

/// Element definition from snapshot
pub struct ElementDefinition {
    pub path: String,
    pub min: usize,
    pub max: Option<usize>,
    pub types: Vec<ElementType>,
    pub fixed_value: Option<serde_json::Value>,
    pub pattern_value: Option<serde_json::Value>,
    pub binding: Option<Binding>,
    pub constraint: Vec<Constraint>,
    pub slicing: Option<Slicing>,
    // ... other FHIR element properties
}
```

---

## API Design

### Example 1: Validate Against Base Resource

```rust
use rh_validator::{FhirValidator, ProfileRegistry};

// Create validator with package directory
let mut registry = ProfileRegistry::new();
registry.add_package_dir("~/.fhir/packages/hl7.fhir.r4.core#4.0.1");

let validator = FhirValidator::with_registry(registry);

// Validate against base Patient (automatic detection)
let patient_json = r#"{"resourceType": "Patient", ...}"#;
let result = validator.validate(patient_json)?;

// Or explicitly specify profile
let result = validator.validate_with_profile(
    patient_json,
    "http://hl7.org/fhir/StructureDefinition/Patient"
)?;
```

### Example 2: Validate Against US Core Profile

```rust
use rh_validator::{FhirValidator, ProfileRegistry};

// Load both base and US Core packages
let mut registry = ProfileRegistry::new();
registry.add_package_dir("~/.fhir/packages/hl7.fhir.r4.core#4.0.1");
registry.add_package_dir("~/.fhir/packages/hl7.fhir.us.core#6.1.0");

let validator = FhirValidator::with_registry(registry);

// Validate against US Core Patient profile
let us_core_patient = r#"{
    "resourceType": "Patient",
    "meta": {
        "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
    },
    "identifier": [{"system": "...", "value": "..."}],
    "name": [{"family": "Doe", "given": ["John"]}],
    "gender": "male"
}"#;

// Validates using US Core constraints (stricter than base)
let result = validator.validate(us_core_patient)?;

// Check for US Core specific violations
for issue in result.issues {
    println!("{}: {}", issue.severity, issue.details);
}
```

### Example 3: Custom Profile Validation

```rust
use rh_validator::{FhirValidator, ProfileRegistry};

// Load custom profile from file
let mut registry = ProfileRegistry::new();
registry.add_package_dir("~/.fhir/packages/hl7.fhir.r4.core#4.0.1");
registry.load_profile_from_file("./profiles/my-custom-patient.json")?;

let validator = FhirValidator::with_registry(registry);

// Validate against custom profile
let result = validator.validate_with_profile(
    patient_json,
    "http://example.org/fhir/StructureDefinition/MyCustomPatient"
)?;
```

### Example 4: Pre-compile Profiles for Performance

```rust
use rh_validator::{FhirValidator, ProfileRegistry};

// Create registry and pre-compile frequently used profiles
let mut registry = ProfileRegistry::new();
registry.add_package_dir("~/.fhir/packages/hl7.fhir.r4.core#4.0.1");
registry.add_package_dir("~/.fhir/packages/hl7.fhir.us.core#6.1.0");

// Pre-compile common profiles (generates snapshots + validation rules)
registry.precompile(&[
    "http://hl7.org/fhir/StructureDefinition/Patient",
    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
    "http://hl7.org/fhir/StructureDefinition/Observation",
    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-observation-lab",
])?;

let validator = FhirValidator::with_registry(registry);

// Subsequent validations use cached compiled rules (fast)
let result = validator.validate(patient_json)?;
```

### Example 5: Snapshot Inspection API

```rust
use rh_validator::ProfileRegistry;

let mut registry = ProfileRegistry::new();
registry.add_package_dir("~/.fhir/packages/hl7.fhir.r4.core#4.0.1");
registry.add_package_dir("~/.fhir/packages/hl7.fhir.us.core#6.1.0");

// Generate snapshot for US Core Patient
let snapshot = registry.generate_snapshot(
    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"
)?;

// Inspect elements
for element in &snapshot.elements {
    println!("{}: {}..{}", 
        element.path, 
        element.min,
        element.max.map(|m| m.to_string()).unwrap_or("*".to_string())
    );
}

// Find specific element
let identifier = snapshot.find_element("Patient.identifier")?;
println!("Patient.identifier cardinality: {}..{}", 
    identifier.min,
    identifier.max.map(|m| m.to_string()).unwrap_or("*".to_string())
);

// Check if profile adds constraints
let base_snapshot = registry.generate_snapshot(
    "http://hl7.org/fhir/StructureDefinition/Patient"
)?;
let base_identifier = base_snapshot.find_element("Patient.identifier")?;

println!("Base: {}..{}", base_identifier.min, base_identifier.max.unwrap_or(usize::MAX));
println!("US Core: {}..{}", identifier.min, identifier.max.unwrap_or(usize::MAX));
// Output: Base: 0..*, US Core: 1..* (US Core requires at least one identifier)
```

### Example 6: Multi-Profile Validation

```rust
use rh_validator::FhirValidator;

let validator = FhirValidator::new()?;

// Resource claiming conformance to multiple profiles
let patient_json = r#"{
    "resourceType": "Patient",
    "meta": {
        "profile": [
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
            "http://example.org/fhir/StructureDefinition/MyCustomPatient"
        ]
    },
    ...
}"#;

// Validates against ALL claimed profiles
let result = validator.validate(patient_json)?;

// Check which profile caused issues
for issue in result.issues {
    if let Some(profile) = issue.profile {
        println!("Profile {}: {}", profile, issue.details);
    }
}
```

### Example 7: Validation Rule Inspection

```rust
use rh_validator::ProfileRegistry;

let registry = ProfileRegistry::new();
// ... load profiles ...

// Get compiled validation rules
let rules = registry.compile_rules(
    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"
)?;

// Inspect cardinality constraints
for (path, (min, max)) in &rules.cardinality {
    let max_str = max.map(|m| m.to_string()).unwrap_or("*".to_string());
    println!("{}: {}..{}", path, min, max_str);
}

// Inspect required bindings
for (path, valueset_url) in &rules.required_bindings {
    println!("{} must be from ValueSet: {}", path, valueset_url);
}

// Inspect invariants
for invariant in &rules.invariants {
    println!("{}: {} - {}", invariant.key, invariant.severity, invariant.human);
    println!("  Expression: {}", invariant.expression);
}
```

---

## Snapshot Generation Algorithm

### High-Level Process

```
1. Load base StructureDefinition (if profile)
2. Generate base snapshot (recursive if base is also a profile)
3. Load profile differential
4. Merge differential into base snapshot:
   a. Override cardinality (min can only increase, max can only decrease)
   b. Add new constraints (invariants)
   c. Strengthen bindings (preferred → extensible → required)
   d. Add fixed values
   e. Add slicing rules
   f. Add extensions
5. Resolve all references
6. Sort elements by path
7. Cache result
```

### Example: US Core Patient Snapshot Generation

**Base Patient** (`http://hl7.org/fhir/StructureDefinition/Patient`):
```json
{
  "snapshot": {
    "element": [
      {
        "path": "Patient",
        "min": 0,
        "max": "*"
      },
      {
        "path": "Patient.identifier",
        "min": 0,
        "max": "*"
      },
      {
        "path": "Patient.name",
        "min": 0,
        "max": "*"
      },
      {
        "path": "Patient.gender",
        "min": 0,
        "max": "1",
        "binding": {
          "strength": "required",
          "valueSet": "http://hl7.org/fhir/ValueSet/administrative-gender"
        }
      }
    ]
  }
}
```

**US Core Patient Differential** (`http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient`):
```json
{
  "differential": {
    "element": [
      {
        "path": "Patient",
        "constraint": [
          {
            "key": "us-core-14",
            "severity": "error",
            "human": "Either Patient.name.given and/or Patient.name.family SHALL be present",
            "expression": "name.given.exists() or name.family.exists()"
          }
        ]
      },
      {
        "path": "Patient.identifier",
        "min": 1,
        "mustSupport": true
      },
      {
        "path": "Patient.name",
        "min": 1,
        "mustSupport": true
      },
      {
        "path": "Patient.gender",
        "min": 1,
        "mustSupport": true
      }
    ]
  }
}
```

**Generated US Core Patient Snapshot** (merged):
```json
{
  "snapshot": {
    "element": [
      {
        "path": "Patient",
        "min": 0,
        "max": "*",
        "constraint": [
          {
            "key": "us-core-14",
            "severity": "error",
            "human": "Either Patient.name.given and/or Patient.name.family SHALL be present",
            "expression": "name.given.exists() or name.family.exists()"
          }
        ]
      },
      {
        "path": "Patient.identifier",
        "min": 1,          // ← Changed from 0 (US Core requirement)
        "max": "*",
        "mustSupport": true
      },
      {
        "path": "Patient.name",
        "min": 1,          // ← Changed from 0 (US Core requirement)
        "max": "*",
        "mustSupport": true
      },
      {
        "path": "Patient.gender",
        "min": 1,          // ← Changed from 0 (US Core requirement)
        "max": "1",
        "mustSupport": true,
        "binding": {
          "strength": "required",
          "valueSet": "http://hl7.org/fhir/ValueSet/administrative-gender"
        }
      }
    ]
  }
}
```

---

## Performance Optimizations

### 1. Lazy Loading
```rust
// Only load StructureDefinitions when needed
registry.load_on_demand("http://hl7.org/fhir/us/core/...");

// Don't generate snapshots until validation requested
validator.validate(patient_json)?; // Triggers snapshot generation
```

### 2. Smart Caching
```rust
pub struct SnapshotCache {
    /// LRU cache with max size (e.g., 100 snapshots)
    snapshots: LruCache<String, Arc<Snapshot>>,
    
    /// Compiled rules cache (smaller, more expensive to generate)
    compiled_rules: LruCache<String, Arc<CompiledValidationRules>>,
}

// Cache eviction strategies:
// - LRU for snapshots (keep most recently used)
// - TTL for snapshots (invalidate after 1 hour)
// - Size-based (max 100 MB in memory)
```

### 3. Pre-compilation
```rust
// At startup, pre-compile frequently used profiles
registry.precompile(&[
    "http://hl7.org/fhir/StructureDefinition/Patient",
    "http://hl7.org/fhir/StructureDefinition/Observation",
    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
]);

// Store compiled rules on disk for next run
registry.save_cache_to_disk("~/.rh/validation-cache")?;
registry.load_cache_from_disk("~/.rh/validation-cache")?;
```

### 4. Parallel Snapshot Generation
```rust
use rayon::prelude::*;

// Generate multiple snapshots in parallel
let profiles = vec![
    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-observation-lab",
    // ... more profiles
];

profiles.par_iter()
    .for_each(|url| {
        registry.generate_snapshot(url).ok();
    });
```

### 5. Incremental Validation
```rust
// For large batches, reuse compiled rules
let rules = registry.compile_rules("http://hl7.org/fhir/StructureDefinition/Patient")?;

for patient_json in patients {
    // Validate using pre-compiled rules (no re-compilation)
    let result = validator.validate_with_rules(patient_json, &rules)?;
}
```

---

## Migration Strategy

### Phase 1: Add Dynamic Validation Alongside Static (Backward Compatible)

```rust
// Option 1: Static validation (current approach - fast)
let result = validator.validate_full::<Patient>(json)?;

// Option 2: Dynamic validation (new approach - flexible)
let result = validator.validate_dynamic(json, Some(profile_url))?;

// Option 3: Hybrid (use static if available, dynamic otherwise)
let result = validator.validate(json)?; // Auto-detect and choose best method
```

### Phase 2: Optimize Dynamic Validation

- Implement caching
- Add pre-compilation
- Benchmark and optimize hot paths
- Add disk-based cache persistence

### Phase 3: Default to Dynamic, Keep Static as Optimization

```rust
// Static code generation becomes an optimization for known profiles
// Generated code uses same validation engine but with pre-compiled rules

impl ValidatableResource for Patient {
    fn validation_rules() -> &'static CompiledValidationRules {
        // Rules compiled at codegen time, embedded in binary
        &COMPILED_RULES
    }
}
```

### Phase 4: Remove Code Generation Dependency (Optional)

- Pure runtime validation
- No code generation required
- Full profile flexibility

---

## Comparison: Static vs Dynamic

| Aspect | Static (Current) | Dynamic (Proposed) |
|--------|------------------|-------------------|
| **Flexibility** | ❌ Fixed at compile time | ✅ Any profile at runtime |
| **Performance** | ✅ Zero overhead | ⚠️ Snapshot generation cost (cached) |
| **Memory** | ✅ Minimal (metadata in code) | ⚠️ Higher (caches in memory) |
| **Type Safety** | ✅ Full Rust type checking | ⚠️ Runtime checks only |
| **Profile Support** | ❌ Requires code regen | ✅ Load any profile |
| **Custom Profiles** | ❌ Not supported | ✅ Fully supported |
| **IDE Support** | ✅ Full autocomplete | ⚠️ Limited |
| **Binary Size** | ⚠️ Larger (embedded metadata) | ✅ Smaller |
| **Startup Time** | ✅ Fast | ⚠️ Slower (if pre-compiling) |
| **Multi-Profile** | ❌ Not supported | ✅ Fully supported |

---

## Implementation Phases

### Phase 10: Foundation (1-2 weeks)
- [ ] Create `ProfileRegistry` 
- [ ] Implement basic snapshot generation (no merging)
- [ ] Add caching infrastructure (in-memory LRU)
- [ ] Unit tests for snapshot generation

### Phase 11: Snapshot Merging (2-3 weeks)
- [ ] Implement differential merging algorithm
- [ ] Handle cardinality constraint tightening
- [ ] Handle binding strength escalation
- [ ] Handle constraint addition (invariants)
- [ ] Integration tests with US Core Patient

### Phase 12: Validation Rule Compilation (2 weeks)
- [ ] Create `CompiledValidationRules` structure
- [ ] Extract rules from snapshot
- [ ] Optimize rule representation
- [ ] Benchmarks comparing static vs dynamic

### Phase 13: Dynamic Validation Engine (1-2 weeks)
- [ ] Implement `validate_dynamic()` method
- [ ] Execute compiled rules against resources
- [ ] Profile detection from `meta.profile`
- [ ] Multi-profile validation support

### Phase 14: Performance Optimization (2 weeks)
- [ ] Implement disk-based cache
- [ ] Add parallel snapshot generation
- [ ] Profile pre-compilation
- [ ] Benchmark and optimize hot paths
- [ ] Memory profiling and optimization

### Phase 15: Migration & Documentation (1 week)
- [ ] Update examples
- [ ] Write migration guide
- [ ] Performance comparison documentation
- [ ] API documentation

---

## Hybrid Solution: Pre-Processed Metadata with Runtime Flexibility

### Concept

Rather than choosing between compile-time code generation OR pure runtime processing, we can combine both approaches:

1. **Pre-process conformance resources** (StructureDefinitions, ValueSets, CodeSystems) into **optimized metadata artifacts**
2. **Load metadata at runtime** from these pre-processed artifacts
3. **Avoid snapshot generation overhead** by pre-computing snapshots during build time
4. **Enable dynamic profile loading** while maintaining high performance

This gives us:
- ✅ **Fast startup** - No runtime snapshot generation for known profiles
- ✅ **Flexibility** - Can still load and process new profiles dynamically
- ✅ **Type safety** - Keep generated Rust types for IDE support
- ✅ **Portability** - Metadata artifacts are language-agnostic (JSON/Protobuf/etc)
- ✅ **Version control** - Pre-processed artifacts are diffable and reviewable

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Build Time Phase                         │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  FHIR Package (*.tgz)                                        │
│  ├── StructureDefinition resources                          │
│  ├── ValueSet resources                                     │
│  └── CodeSystem resources                                   │
│                    │                                         │
│                    ▼                                         │
│         Conformance Preprocessor                            │
│         ├── Parse JSON resources                            │
│         ├── Generate snapshots                              │
│         ├── Extract validation rules                        │
│         ├── Build value set expansions                      │
│         ├── Compile FHIRPath expressions                    │
│         └── Optimize data structures                        │
│                    │                                         │
│                    ▼                                         │
│      Optimized Metadata Artifacts                           │
│      ├── resource_metadata.bin (snapshots)                  │
│      ├── validation_rules.bin (compiled rules)              │
│      ├── value_sets.bin (expanded value sets)               │
│      ├── code_systems.bin (code lookup tables)              │
│      └── fhirpath_expressions.bin (compiled ASTs)           │
│                                                               │
└─────────────────────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                     Runtime Phase                            │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ValidationEngine::new()                                     │
│  ├── Load pre-processed artifacts (mmap for zero-copy)      │
│  ├── Deserialize into in-memory structures                  │
│  ├── Build lookup indices (by resource type, profile URL)   │
│  └── Initialize caches                                      │
│                    │                                         │
│                    ▼                                         │
│  Validate Resource                                          │
│  ├── Lookup snapshot from loaded metadata                   │
│  ├── Execute pre-compiled validation rules                  │
│  ├── Check value sets via pre-expanded lookups              │
│  └── Evaluate compiled FHIRPath expressions                 │
│                    │                                         │
│  Optional: Load Dynamic Profile                             │
│  ├── Parse new StructureDefinition                          │
│  ├── Generate snapshot (runtime fallback)                   │
│  ├── Compile validation rules on-the-fly                    │
│  └── Cache for future use                                   │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Metadata Artifact Format

#### 1. Resource Metadata (resource_metadata.bin)

Pre-computed snapshots with optimized layouts:

```rust
struct ResourceMetadata {
    // Version and package info
    fhir_version: String,
    package_id: String,
    package_version: String,
    
    // Indexed by canonical URL
    profiles: HashMap<String, ProfileMetadata>,
}

struct ProfileMetadata {
    url: String,
    name: String,
    base_type: String,
    
    // Pre-computed snapshot (no differential merging needed)
    snapshot: Snapshot,
    
    // Quick lookup indices
    element_index: HashMap<String, usize>, // path -> element index
    required_elements: Vec<String>,        // paths of required elements
    choice_types: HashMap<String, Vec<String>>, // path -> allowed types
    
    // Validation metadata
    cardinalities: Vec<ElementCardinality>,
    bindings: Vec<ElementBinding>,
    invariants: Vec<ElementInvariant>,
}

struct Snapshot {
    elements: Vec<ElementDefinition>,
    // Additional indices for fast lookup
    slices: HashMap<String, Vec<usize>>, // slice path -> element indices
    extensions: HashMap<String, ExtensionDefinition>,
}
```

#### 2. Validation Rules (validation_rules.bin)

Pre-compiled validation rules for each profile:

```rust
struct ValidationRules {
    // Indexed by canonical URL
    rules: HashMap<String, ProfileRules>,
}

struct ProfileRules {
    profile_url: String,
    
    // Structural rules (cardinality, type checking)
    structural_rules: Vec<StructuralRule>,
    
    // Invariant rules with pre-compiled FHIRPath
    invariant_rules: Vec<InvariantRule>,
    
    // Binding rules with pre-expanded value sets
    binding_rules: Vec<BindingRule>,
    
    // Extension rules
    extension_rules: Vec<ExtensionRule>,
}

struct StructuralRule {
    element_path: String,
    min_cardinality: usize,
    max_cardinality: Option<usize>,
    allowed_types: Vec<String>,
    check_fn: ValidationCheckType, // Enum representing check type
}

struct InvariantRule {
    key: String,
    severity: Severity,
    expression: CompiledFHIRPath, // Pre-compiled AST
    human_description: String,
}
```

#### 3. Value Sets (value_sets.bin)

Pre-expanded value sets for fast lookup:

```rust
struct ValueSetData {
    // Indexed by value set URL
    value_sets: HashMap<String, ExpandedValueSet>,
}

struct ExpandedValueSet {
    url: String,
    version: String,
    
    // Pre-expanded codes for fast lookup
    codes: HashSet<CodeEntry>,
    
    // For hierarchical code systems
    hierarchy: Option<CodeHierarchy>,
    
    // Metadata
    expansion_timestamp: DateTime<Utc>,
    total_codes: usize,
}

struct CodeEntry {
    system: String,
    code: String,
    display: Option<String>,
}
```

#### 4. Compiled FHIRPath (fhirpath_expressions.bin)

Pre-compiled FHIRPath expressions:

```rust
struct CompiledExpressions {
    // Indexed by expression hash or key
    expressions: HashMap<String, CompiledFHIRPath>,
}

struct CompiledFHIRPath {
    source: String,           // Original expression
    ast: FHIRPathAST,        // Parsed AST
    dependencies: Vec<String>, // Element paths referenced
    
    // Optimization flags
    is_pure: bool,           // No side effects, can cache result
    is_constant: bool,       // Always returns same value
}
```

### Pre-Processor Implementation

The preprocessor runs at build time (or as a CLI tool):

```rust
struct ConformancePreprocessor {
    config: PreprocessorConfig,
}

impl ConformancePreprocessor {
    /// Load FHIR package and pre-process all conformance resources
    pub fn process_package(
        &self,
        package_path: &Path,
        output_dir: &Path,
    ) -> Result<ProcessingReport> {
        // 1. Load all conformance resources from package
        let structure_defs = self.load_structure_definitions(package_path)?;
        let value_sets = self.load_value_sets(package_path)?;
        let code_systems = self.load_code_systems(package_path)?;
        
        // 2. Generate snapshots for all profiles
        let snapshots = self.generate_all_snapshots(&structure_defs)?;
        
        // 3. Extract and compile validation rules
        let validation_rules = self.compile_validation_rules(&snapshots)?;
        
        // 4. Expand value sets
        let expanded_value_sets = self.expand_value_sets(&value_sets, &code_systems)?;
        
        // 5. Compile all FHIRPath expressions
        let compiled_expressions = self.compile_fhirpath_expressions(&snapshots)?;
        
        // 6. Optimize data structures
        let optimized_metadata = self.optimize_metadata(
            snapshots,
            validation_rules,
            expanded_value_sets,
            compiled_expressions,
        )?;
        
        // 7. Serialize to binary format
        self.write_artifacts(output_dir, &optimized_metadata)?;
        
        Ok(ProcessingReport {
            profiles_processed: structure_defs.len(),
            value_sets_expanded: value_sets.len(),
            expressions_compiled: compiled_expressions.len(),
            output_size_bytes: self.calculate_output_size(output_dir)?,
        })
    }
}
```

### Runtime Loading

Fast loading with memory-mapped files:

```rust
struct ValidationEngine {
    metadata: Arc<ResourceMetadata>,
    rules: Arc<ValidationRules>,
    value_sets: Arc<ValueSetData>,
    expressions: Arc<CompiledExpressions>,
    
    // Optional: for dynamic profile loading
    dynamic_profiles: RwLock<HashMap<String, ProfileMetadata>>,
}

impl ValidationEngine {
    /// Load pre-processed artifacts
    pub fn from_artifacts(artifacts_dir: &Path) -> Result<Self> {
        // Use memory-mapped files for zero-copy loading
        let metadata_mmap = unsafe { Mmap::open(artifacts_dir.join("resource_metadata.bin"))? };
        let rules_mmap = unsafe { Mmap::open(artifacts_dir.join("validation_rules.bin"))? };
        let value_sets_mmap = unsafe { Mmap::open(artifacts_dir.join("value_sets.bin"))? };
        let expressions_mmap = unsafe { Mmap::open(artifacts_dir.join("fhirpath_expressions.bin"))? };
        
        // Deserialize (could use zero-copy formats like Cap'n Proto or FlatBuffers)
        let metadata = Arc::new(bincode::deserialize(&metadata_mmap)?);
        let rules = Arc::new(bincode::deserialize(&rules_mmap)?);
        let value_sets = Arc::new(bincode::deserialize(&value_sets_mmap)?);
        let expressions = Arc::new(bincode::deserialize(&expressions_mmap)?);
        
        Ok(Self {
            metadata,
            rules,
            value_sets,
            expressions,
            dynamic_profiles: RwLock::new(HashMap::new()),
        })
    }
    
    /// Validate with pre-processed rules
    pub fn validate(&self, resource: &Value, profile_url: &str) -> ValidationResult {
        // Fast path: lookup pre-processed rules
        if let Some(rules) = self.rules.rules.get(profile_url) {
            return self.validate_with_compiled_rules(resource, rules);
        }
        
        // Slow path: dynamic profile (load and compile on-the-fly)
        let profile = self.load_dynamic_profile(profile_url)?;
        let rules = self.compile_rules_for_profile(&profile)?;
        
        // Cache for future use
        self.dynamic_profiles.write().insert(profile_url.to_string(), profile);
        
        self.validate_with_compiled_rules(resource, &rules)
    }
}
```

### Benefits of Hybrid Approach

1. **Performance**
   - No runtime snapshot generation for common profiles
   - Pre-expanded value sets for O(1) lookup
   - Pre-compiled FHIRPath expressions avoid parsing overhead
   - Memory-mapped files for fast startup
   - Zero-copy deserialization possible with right format

2. **Flexibility**
   - Can still load new profiles dynamically
   - Supports custom profiles without rebuilding artifacts
   - Runtime compilation fallback for edge cases

3. **Developer Experience**
   - Keep generated Rust types for IDE support
   - Metadata artifacts are portable (can be used from other languages)
   - Clear separation of concerns (build-time vs runtime)

4. **Storage Efficiency**
   - Binary format smaller than JSON
   - Compressed representations
   - Deduplicated common data (value sets referenced by multiple profiles)

5. **Versioning**
   - Artifacts can be versioned independently
   - Can ship multiple FHIR versions side-by-side
   - Easy to diff changes between versions

### Artifact Size Estimates

For FHIR R4 Core (4.0.1):

```
Resource Metadata:     ~5 MB  (all StructureDefinitions with snapshots)
Validation Rules:      ~3 MB  (compiled rules for all profiles)
Value Sets:           ~20 MB  (expanded value sets including SNOMED/LOINC subsets)
FHIRPath Expressions:  ~2 MB  (compiled ASTs for all invariants)
----------------------------------------
Total:                ~30 MB  (uncompressed)
                      ~10 MB  (with compression)
```

For US Core 6.1.0 (additional):

```
Resource Metadata:     ~500 KB
Validation Rules:      ~300 KB
Value Sets:           ~5 MB  (additional expansions)
FHIRPath Expressions:  ~200 KB
----------------------------------------
Total:                ~6 MB  (uncompressed)
                      ~2 MB  (with compression)
```

### CLI Integration

Extend `rh-cli` to support preprocessing:

```bash
# Pre-process FHIR package into optimized artifacts
rh preprocess hl7.fhir.r4.core 4.0.1 -o ./artifacts

# Pre-process multiple packages
rh preprocess hl7.fhir.r4.core 4.0.1 \
              hl7.fhir.us.core 6.1.0 \
              -o ./artifacts

# Validate using pre-processed artifacts
rh validate patient.json --artifacts ./artifacts --profile http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient

# Show artifact statistics
rh artifacts info ./artifacts
# Output:
#   FHIR Version: 4.0.1
#   Packages: hl7.fhir.r4.core@4.0.1, hl7.fhir.us.core@6.1.0
#   Profiles: 234
#   Value Sets: 1,234 (456,789 codes)
#   Expressions: 3,456
#   Total Size: 12.3 MB (compressed)
```

### Comparison Matrix

| Aspect | Static (Codegen) | Pure Runtime | Hybrid (Proposed) |
|--------|-----------------|--------------|-------------------|
| **Startup Time** | Fast (no loading) | Slow (parse + snapshot) | Fast (pre-processed) |
| **Memory Usage** | Low (embedded) | High (caching) | Medium (loaded artifacts) |
| **Flexibility** | None (requires rebuild) | Full (dynamic) | Full (dynamic fallback) |
| **Performance** | Fastest | Slower | Fast (near-static) |
| **Type Safety** | Strong | None | Strong (keep codegen) |
| **Portability** | Rust-only | Any language | Any language (artifacts) |
| **Storage** | Large (source code) | None | Medium (artifacts) |
| **Versioning** | Difficult (code) | N/A | Easy (artifacts) |

### Migration Path

**Phase 1**: Generate artifacts alongside code generation
- Update codegen to also output binary artifacts
- Keep existing static validation working
- Add artifact loading to rh-validator (optional mode)

**Phase 2**: Primary validation from artifacts
- Switch validator to load from artifacts by default
- Keep static validation as fallback
- Measure performance differences

**Phase 3**: Deprecate static validation
- Remove static validation code
- Keep only generated types (for IDE support)
- Fully rely on artifact-based validation

**Phase 4**: Add dynamic compilation
- Add runtime profile loading
- Implement on-the-fly rule compilation
- Cache dynamically compiled rules

---

## Open Questions

1. **Snapshot Generation Performance**: How expensive is snapshot generation for complex profiles? Do we need to persist snapshots to disk?
   - **Hybrid Answer**: Yes, we pre-compute snapshots at build time and persist them in optimized binary format.

2. **Cache Invalidation**: When should we invalidate cached snapshots? Version changes? File modifications?
   - **Hybrid Answer**: Artifacts are versioned and immutable. New versions mean new artifact files.

3. **Memory Limits**: What's the maximum memory budget for cached snapshots and rules? 100 MB? 500 MB?
   - **Hybrid Answer**: Artifacts are ~10-15 MB compressed per major package (R4 Core + US Core). Memory-mapped files allow lazy loading. Budget: 100 MB for typical deployments, 500 MB for comprehensive validation with many IGs.

4. **Backward Compatibility**: Should we maintain static validation indefinitely, or plan for deprecation?
   - **Hybrid Answer**: Deprecate static validation after Phase 3. Keep generated Rust types for IDE support, but validation logic comes from artifacts.

5. **Slicing Support**: How do we efficiently validate slices in the dynamic approach?
   - **Hybrid Answer**: Pre-process slice discriminators and paths into optimized lookup structures in artifacts. Include slice indices in snapshot metadata.

6. **Extension Validation**: How deeply do we validate extensions? Only structure or also content?
   - **Hybrid Answer**: Full validation. Extension definitions are included in artifacts with their own validation rules.

7. **Error Messages**: How do we provide helpful error messages that reference the specific profile constraint that failed?
   - **Hybrid Answer**: Store human-readable descriptions and profile URLs in validation rules. Include profile context in error messages.

8. **Profile Discovery**: Should we auto-discover profiles from packages, or require explicit loading?
   - **Hybrid Answer**: Auto-discover during artifact generation. At runtime, profiles are indexed by URL in artifacts for fast lookup.

9. **Artifact Format**: Should we use Bincode, Protocol Buffers, FlatBuffers, or Cap'n Proto?
   - **Hybrid Consideration**: Start with Bincode for simplicity. Migrate to FlatBuffers or Cap'n Proto for zero-copy deserialization if performance demands it.

10. **Incremental Updates**: How do we handle updates to specific profiles without regenerating entire artifact set?
    - **Hybrid Consideration**: Design artifact format to support partial updates. Store profiles as separate chunks within a container format.

---

## Success Criteria

### Hybrid Approach Specific
- ✅ Artifact generation completes in < 30 seconds for R4 Core
- ✅ Artifact loading (startup) < 100ms with memory-mapped files
- ✅ Artifacts are portable across platforms (Linux, macOS, Windows)
- ✅ Artifact size < 15 MB per major FHIR package (compressed)
- ✅ Can regenerate artifacts independently for each package

### General Validation
- ✅ Can validate against any FHIR profile (pre-processed or dynamic)
- ✅ Performance within 1.5x of static validation (with pre-processed artifacts)
- ✅ Memory usage < 100 MB for typical use cases, < 500 MB for comprehensive
- ✅ Supports US Core, IPS, and custom profiles
- ✅ Multi-profile validation works correctly
- ✅ Backward compatible with existing static validation during migration
- ✅ Comprehensive test coverage (>90%)
- ✅ Clear documentation and examples
- ✅ CLI supports artifact generation, inspection, and validation

---

## Conclusion

This alternate design provides a **hybrid validation approach** that combines the best of both worlds:

### Best of Static (Compile-Time)
- ✅ **Fast performance** through pre-processing
- ✅ **Type safety** (keep generated Rust types)
- ✅ **Zero snapshot generation overhead**
- ✅ **Optimized data structures**

### Best of Dynamic (Runtime)
- ✅ **Flexibility** - validate against any profile
- ✅ **No code regeneration** for new profiles
- ✅ **Portable artifacts** (language-agnostic)
- ✅ **Version control friendly**

### Key Innovations
1. **Pre-processed conformance resources** into optimized binary artifacts
2. **Memory-mapped artifact loading** for fast startup
3. **Pre-compiled validation rules** for high performance
4. **Dynamic profile loading fallback** for flexibility
5. **Incremental migration path** from static to hybrid

### Technical Advantages
- **Startup Time**: 100ms (vs seconds for pure runtime)
- **Performance**: Within 1.5x of static (vs 3-5x for pure runtime)
- **Storage**: 10-15 MB per package (vs GBs of generated code)
- **Flexibility**: Full profile support (vs none for pure static)
- **Portability**: Artifacts work in any language (vs Rust-only codegen)

The migration can be done incrementally:
1. Generate artifacts alongside code (Phase 1-2)
2. Switch to artifact-based validation (Phase 3)
3. Deprecate static validation (Phase 4)
4. Add dynamic compilation for ad-hoc profiles (Phase 5)

The long-term vision is a **high-performance, artifact-based validator** that:
- Loads pre-processed metadata for known profiles (fast path)
- Compiles validation rules on-the-fly for custom profiles (flexible path)
- Maintains type safety through generated Rust types (developer experience)
- Supports any FHIR version or profile without code regeneration
