# rh-validator Architecture

`rh-validator` is the workspace validator crate for FHIR R4 JSON resources. It
combines fast structural checks over `serde_json::Value` with profile-aware
validation from StructureDefinition snapshots, FHIRPath invariant evaluation,
local ValueSet/CodeSystem checks, optional terminology service calls, and
QuestionnaireResponse validation.

The crate sits above `rh-foundation` and `rh-fhirpath`:

```text
FHIR packages / registered resources
        |
        v
rh-foundation snapshot loader/generator
        |
        v
ProfileRegistry -> RuleCompiler -> FhirValidator
        |              |              |
        |              |              +-> base JSON/FHIR checks
        |              |              +-> profile rule walkers
        |              |              +-> FHIRPath invariants
        |              |              +-> ValueSet/terminology checks
        |              |              +-> QuestionnaireResponse checks
        |              |
        |              +-> CompiledValidationRules
        |
        +-> cached StructureDefinition snapshots
```

## Public API Shape

The main API is `FhirValidator` from `src/validator.rs`.

- `validate(&Value)` runs base resource validation only.
- `validate_auto(&Value)` extracts `meta.profile`, validates against those
  profiles, and also applies the base profile for the resource type.
- `validate_with_profile(&Value, profile_url)` runs base validation plus one
  explicit profile.
- `validate_with_profiles(&Value, &[String])` runs base validation plus several
  explicit profiles and annotates profile-originated issues.

The CLI uses `validate_auto()` for normal validation so resources receive both
structural checks and applicable profile checks.

Results are represented by `ValidationResult` and `ValidationIssue` in
`src/types.rs`. `ValidationResult` keeps a boolean `valid` flag and issue list;
adding an error issue flips the result invalid. It can also be rendered as a
FHIR `OperationOutcome`.

## Main Modules

| Module | Responsibility |
|---|---|
| `validator.rs` | Main orchestrator, base validators, profile-rule execution, FHIRPath invariants, bundle/reference checks, terminology hooks, registration APIs. |
| `profile.rs` | Loads StructureDefinitions from the default FHIR package directory and optional package directory, generates snapshots, caches snapshots, supports dynamic profile registration. |
| `rules.rs` | Compiles StructureDefinition snapshot elements into typed rule vectors for cardinality, types, references, bindings, fixed/pattern values, invariants, extensions, slicing, and unknown-property plans. |
| `valueset.rs` | Loads and caches ValueSet resources from packages or runtime registration, caches ValueSet/CodeSystem membership checks, resolves simple expansion/compose membership, and checks local CodeSystem contents. |
| `terminology.rs` | Defines the `TerminologyService` trait, mock implementation, blocking HTTP implementation, and cached wrapper with optional disk persistence. |
| `questionnaire.rs` | Loads and caches Questionnaires, builds a `linkId` map, and validates QuestionnaireResponse items, answer cardinality, answer types, options, and value sets. |
| `report.rs` | CLI-oriented text, JSON, batch, and OperationOutcome renderers. |
| `fhir_version.rs` | Version-specific constants. The crate currently exposes R4 / 4.0.1. |

## Construction Flow

`FhirValidator::with_options` builds a validator in one pass:

1. Resolve package directories from the explicit `packages_dir` plus the default
   FHIR package location for `hl7.fhir.r4.core#4.0.1/package` when present.
2. Build an optional `Arc<dyn TerminologyService>` from `TerminologyConfig`.
3. Create a `ProfileRegistry`, which loads StructureDefinitions into a
   `HashMap` and into `rh-foundation`'s `SnapshotGenerator`.
4. Create reusable helpers: `RuleCompiler`, `ValueSetLoader`,
   `QuestionnaireLoader`, `FhirPathParser`, a parsed invariant-expression LRU,
   and `FhirPathEvaluator`.
5. Initialize mutable runtime registries for supplements, CodeSystems, and
   Measures behind `RwLock`s.

Runtime registration APIs (`register_profile`, `register_valueset`,
`register_questionnaire`, `register_codesystem`, and `register_measure`) let
test runners or callers preload resources that are not available from package
files.

## Validation Control Flow

### Base Validation

`validate()` is deliberately front-loaded with cheap structural checks. It:

1. Ensures the input is a JSON object and has a `resourceType`.
2. Validates common FHIR JSON rules such as id format, contained ids, string
   length, empty arrays, base64 fields, attachment size consistency, canonical
   URL shape, coding system URI shape, signature placeholders, and optional
   string security checks.
3. Runs resource-specific validators for Bundle, Parameters, Questionnaire,
   Measure, QuestionnaireResponse, MeasureReport, and StructureDefinition.
4. Resolves and checks unknown extensions through the profile registry.
5. Performs terminology-related passes:
   local terminology definition reference checks, optional display validation
   through `TerminologyService`, UCUM checks, known local CodeSystem checks, and
   selected conformance quick-win checks.
6. Returns a merged `ValidationResult`.

These checks mostly use recursive pattern matching over `serde_json::Value`.
They avoid deserializing the full resource into generated FHIR structs, which
keeps the validator tolerant of partial/invalid resources and lets it report
multiple issues in one pass.

### Automatic Profile Validation

`validate_auto()` decides which profiles to apply:

1. Extract profile canonical URLs from `meta.profile`.
2. If no profiles are declared and `resourceType` is available, validate against
   `http://hl7.org/fhir/StructureDefinition/{resourceType}`.
3. If profiles are declared, call `validate_with_profiles()`.
4. Always add the base resource profile unless it was already declared.

This means custom profile validation does not replace base validation; it layers
on top of it.

### Profile Rule Validation

`validate_profile_rules_inner()` is the profile engine:

1. Canonicalize versioned URLs by dropping the `|version` suffix.
2. Track visited profiles to avoid recursion cycles.
3. Ask `ProfileRegistry` for a snapshot. If a profile is missing, emit a warning
   and still run unknown extension checks.
4. Recursively apply non-core `baseDefinition` profile rules before the derived
   profile.
5. Ask `RuleCompiler` for compiled rules.
6. Execute rule vectors in a fixed order:
   unknown properties, cardinality, types, reference target profiles,
   fixed/pattern values, bindings, invariants, datatype-derived invariants,
   expression datatype checks, extension rules, then slicing rules.

Rule execution uses path helpers such as `get_values_at_path()` and
`should_validate_path()` to traverse only relevant subtrees. Choice elements
like `value[x]` are handled by prefix matching against concrete JSON property
names.

### Rule Compilation

`RuleCompiler::compile()` converts a snapshot into `CompiledValidationRules`.
It walks `snapshot.element` once to collect:

- element paths for unknown property checks
- cardinality rules from `min`/`max`
- type rules and reference target profiles
- ValueSet binding rules
- fixed and pattern constraints
- FHIRPath invariant rules
- extension slice requirements
- slicing discriminators and slice definitions
- precomputed allowed-child and choice-prefix maps for unknown-property checks

It also supplements snapshot-derived bindings with differential bindings when
needed. If a StructureDefinition has no snapshot, it falls back to limited
differential-derived cardinality and binding rules.

This compilation step turns a large, nested FHIR StructureDefinition into small
typed vectors that can be walked repeatedly without rediscovering rule metadata.

### FHIRPath Invariants

Invariant rules are evaluated through `rh-fhirpath`:

1. Normalize the invariant expression.
2. Try native Rust fast paths for common core invariants (`ele-1`, `ext-1`, and
   `per-1`).
3. For remaining expressions, parse through a validator-level LRU keyed by the
   normalized expression.
4. Evaluate resource-level invariants against the whole resource.
5. Evaluate element-level invariants once per element found at the rule path,
   using the full resource as root and the matched element as current context.
6. Interpret boolean, empty, and singleton collection results according to the
   validator's invariant semantics.

The parser/evaluator are stored on `FhirValidator` and reused. Parsed FHIRPath
expressions are cached with parse errors as well as successful parses so
repeated invalid profile constraints do not reparse on every resource.

### Terminology and ValueSet Flow

Bindings first try local ValueSet resolution through `ValueSetLoader`:

1. Strip any canonical `|version` suffix.
2. Check the in-memory ValueSet resource LRU cache.
3. Scan configured package directories for matching JSON ValueSet resources.
4. Check the membership-result LRU for repeated `(ValueSet, system, code)`
   requests.
5. Decide membership from expansions or simple compose includes.
6. For compose includes that reference complete CodeSystems, check the
   `(CodeSystem, code)` membership-result LRU before scanning package files.
7. For required bindings that cannot be decided locally, optionally fall back to
   the configured `TerminologyService`.

Terminology service calls are trait-based:

- `MockTerminologyService` supports deterministic tests with common FHIR codes.
- `HttpTerminologyService` calls FHIR `$validate-code` and `$lookup` endpoints.
- `CachedTerminologyService` wraps any service and caches CodeSystem,
  ValueSet, and lookup results in memory, with optional disk persistence.

## Performance Optimizations

The validator is optimized around repeated validation of resources against a
stable set of packages and profiles.

| Optimization | Where | Effect |
|---|---|---|
| Snapshot LRU cache | `ProfileRegistry` | Avoids repeated snapshot generation for the same profile URL. |
| Compiled rule LRU cache | `RuleCompiler` | Avoids rebuilding rule vectors from snapshots on repeated profile validation. |
| ValueSet LRU cache | `ValueSetLoader` | Avoids repeated package directory scans and JSON parsing for ValueSets. |
| ValueSet membership LRU cache | `ValueSetLoader` | Avoids repeated expansion/compose walks for the same binding code checks. |
| CodeSystem membership LRU cache | `ValueSetLoader` | Avoids repeated package CodeSystem scans for known code checks. |
| Parsed invariant-expression LRU cache | `FhirValidator` | Avoids reparsing repeated FHIRPath invariant expressions, including parse failures. |
| Native core invariant fast paths | `validator.rs` | Evaluates common `ele-1`, `ext-1`, and `per-1` constraints without invoking the FHIRPath engine. |
| Precomputed unknown-property plan | `CompiledValidationRules` | Reuses allowed-child and choice-prefix maps instead of rebuilding them per resource. |
| Questionnaire LRU cache | `QuestionnaireLoader` | Avoids repeated Questionnaire package lookups and parsing. |
| Terminology cache | `CachedTerminologyService` | Avoids repeated remote terminology calls and can persist results across runs. |
| Borrowed JSON traversal | `validator.rs` helpers | Most path lookups return `&Value`, reducing cloning while walking resources. |
| Typed compiled rules | `rules.rs` | Moves StructureDefinition interpretation out of the hot validation path. |
| Canonical URL normalization | profile/value set helpers | Improves cache reuse across versioned and unversioned canonical references. |

`PERFORMANCE.md` records the current benchmark profile from the 0.2.5
performance work: warmed simple US Core Patient validation around 170 us,
complex Patient validation around 393 us, auto-detect around 245 us, and
warmed batch throughput around 5,800 resources per second with 100 percent
profile/rule cache hit rates in the benchmark workload.

## Rust Features Used For Performance and Safety

- `serde_json::Value` lets the validator inspect malformed or incomplete FHIR
  JSON without requiring full typed deserialization.
- `HashMap` and `HashSet` are used for fast lookup of profiles, registered
  resources, bundle references, allowed children, duplicate detection, and
  Questionnaire `linkId` indexes.
- `lru::LruCache` bounds memory for snapshots, compiled rules, ValueSets, and
  Questionnaires while preserving hot entries.
- `RwLock` protects mostly-read runtime stores such as snapshot caches, dynamic
  profiles, registered CodeSystems, supplements, Measures, Questionnaire cache,
  and terminology caches.
- `Mutex` protects `LruCache` instances that require mutable access even for
  cache hits.
- `Arc<dyn TerminologyService>` allows the validator to share an optional
  terminology service behind a `Send + Sync` trait object.
- `thiserror` and `anyhow` split domain errors from orchestration context while
  keeping validation APIs simple.
- Iterator adapters and `filter_map` are used throughout rule compilation and
  resource traversal to keep intermediate allocations localized.
- `NonZeroUsize` enforces valid LRU capacities at construction time.

## Current Limits and Future Performance Work

- `rh-validator` does not currently expose a parallel `validate_batch()` API.
  The CLI batch path loops over input resources sequentially. The `rayon`
  dependency is present for future batch work.
- ValueSet and CodeSystem package lookup still scans package JSON files on cache
  misses. Workloads with many cold ValueSet references would benefit from an
  indexed package resource catalog.
- Generic FHIRPath invariant evaluation still clones the root resource and
  current element into `EvaluationContext`; native invariant fast paths avoid
  this for the common core constraints implemented so far.
- Dynamic profiles registered at runtime are stored as provided; they are not
  loaded into `SnapshotGenerator`, so callers should provide profiles with the
  snapshot data needed for full profile validation.
