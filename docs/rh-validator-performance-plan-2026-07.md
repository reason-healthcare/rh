# rh-validator Performance Improvement Plan

- **Date:** 2026-07-03
- **Primary goal:** Move `rh-validator` toward the targets in `crates/rh-validator/PERFORMANCE.md`.
- **Scope guardrail:** Changes may touch `rh-validator`, `rh-fhirpath`, and `rh-foundation` only when they measurably improve `rh-validator` validation latency or throughput.
- **Explicit non-goal:** Do not implement parallel batch validation in this plan. Single-resource latency must improve first.

## Current Baseline

Use `crates/rh-validator/PERFORMANCE.md` as the baseline source of truth.

| Metric | Current | Target |
|--------|---------|--------|
| Explicit simple US Core Patient | 82.21 ms | < 5 ms |
| Auto-detected US Core Patient | 163.88 ms | < 5 ms |
| Complex US Core Patient | 635.67 ms | Track down, no current target |
| Warmed batch throughput | 12.05-12.31 resources/sec | > 500 resources/sec |
| Warm cache hit rate | 100.0% | > 90% |

The batch throughput target cannot be reached by parallelism alone. Even perfect 10-core scaling from 82 ms/resource would only reach about 120 resources/sec. The first milestone is to reduce explicit simple-patient latency to under 15 ms, then under 5 ms.

## Progress Log

### 2026-07-03

Started branch `codex/rh-validator-performance-plan` from `origin/main` with
`crates/rh-validator/ARCHITECTURE.md`,
`crates/rh-validator/PERFORMANCE.md`, and this plan carried over.

Completed:

- Gate 0: added `crates/rh-validator/benches/perf_smoke.rs` and registered the
  `perf_smoke` bench target.
- A1: changed `ProfileRegistry` snapshot cache to store shared
  `Arc<StructureDefinition>` values.
- A2: changed `RuleCompiler` rule cache to store shared
  `Arc<CompiledValidationRules>` values and avoid holding the rule cache mutex
  while compiling.

Validation:

- `cargo test -p rh-validator` passed after Gate 0.
- `cargo test -p rh-validator` passed after A1.
- `cargo test -p rh-validator` passed after A2.

Smoke benchmark, Gate 0 plus A1:

| Benchmark | Mean |
|-----------|------|
| `validate_simple_patient` | 84.125 ms |
| `validate_auto_detect` | 166.55 ms |
| `validate_complex_patient` | 640.71 ms |
| `warmed_batch_validation/10` | 828.48 ms |
| `warmed_batch_validation/50` | 3.9096 s |

Smoke benchmark, after A2:

| Benchmark | Mean | Criterion change |
|-----------|------|------------------|
| `validate_simple_patient` | 74.311 ms | -10.387% |
| `validate_auto_detect` | 149.39 ms | -7.2423% |
| `validate_complex_patient` | 611.96 ms | -4.4865% |
| `warmed_batch_validation/10` | 758.86 ms | -8.4037% |
| `warmed_batch_validation/50` | 3.7392 s | -4.3580% |

Warm cache metrics in the post-A2 smoke run:

```text
Profile Cache: 6/6 hits (100.0% hit rate)
Rule Cache: 1/1 hits (100.0% hit rate)
```

## Operating Rules For Implementing Agents

1. Preserve validation behavior unless a task explicitly says otherwise.
2. Do not loosen validation rules to improve benchmarks.
3. Keep every change small enough to benchmark independently.
4. Run the measurement gate before and after every task.
5. Record the before/after numbers in this file or in the PR description.
6. If a change touches `rh-fhirpath`, run the FHIRPath conformance suite.
7. If a change touches `rh-foundation` snapshot code, run the foundation snapshot benchmark.
8. Do not start batch parallelism until the explicit simple-patient benchmark is below 15 ms.

## Measurement Gate

### Gate 0: Add Durable Fast Performance Harness First

The existing validator Criterion suite is useful but too slow for repeated work because the full batch group takes a long time at current timings. Before optimization work, add a fast, durable benchmark harness.

Create `crates/rh-validator/benches/perf_smoke.rs` with:

- explicit simple US Core Patient validation
- auto-detected simple US Core Patient validation
- complex US Core Patient validation
- warmed batch sizes 10 and 50 only
- cache hit metrics printed once after warmup
- `Criterion::default().sample_size(10)` or equivalent local configuration
- no 500-resource batch in the smoke harness

Keep the existing `benches/validation.rs` for full reports.

Recommended commands after the smoke harness exists:

```bash
cargo bench -p rh-validator --bench perf_smoke
cargo bench -p rh-validator --bench perf_smoke -- validate_simple_patient
cargo test -p rh-validator
```

For slower milestone reports:

```bash
cargo bench -p rh-validator --bench validation -- validate_simple_patient
cargo bench -p rh-validator --bench validation -- validate_auto_detect
cargo bench -p rh-validator --bench validation -- validate_complex_patient
```

If touching FHIRPath:

```bash
cargo test -p rh-fhirpath
cargo test -p rh-fhirpath --test hl7_conformance
```

If touching snapshot generation:

```bash
cargo test -p rh-foundation
cargo bench -p rh-foundation --bench snapshot
```

### Gate 1: Add Timing Breakdown Instrumentation

Before changing hot-path code, add optional timing breakdowns so improvements can be attributed.

Suggested implementation:

- Add an internal `ValidationTimings` struct in `crates/rh-validator/src/validator.rs` or a new `src/perf.rs`.
- Keep it behind `#[cfg(feature = "perf-timings")]` or `#[cfg(test)]` plus a benchmark-only helper.
- Do not expose it as a stable public API unless needed.
- Track at least:
  - base validation
  - profile snapshot lookup
  - rule compilation/cache lookup
  - unknown-property validation
  - cardinality
  - type validation
  - fixed/pattern validation
  - binding validation
  - invariant validation
  - extension validation
  - slicing validation
  - known-code/terminology local checks

Acceptance criteria:

- `cargo test -p rh-validator` passes.
- Smoke benchmark can emit timing breakdowns in a controlled way.
- Timing instrumentation is off by default and has no effect on normal users.

## Workstream A: Compile More, Interpret Less

### A1. Return Shared Cached Snapshots

Problem:

`ProfileRegistry::get_snapshot()` clones a cached `StructureDefinition` on every cache hit. Current code returns `Option<StructureDefinition>`.

Files:

- `crates/rh-validator/src/profile.rs`
- `crates/rh-validator/src/validator.rs`
- any tests depending on the concrete return type

Plan:

1. Change `snapshot_cache` from `LruCache<String, StructureDefinition>` to `LruCache<String, Arc<StructureDefinition>>`.
2. Change `get_snapshot()` to return `Result<Option<Arc<StructureDefinition>>>`.
3. When generating a snapshot, build the `StructureDefinition` once, wrap it in `Arc`, store an `Arc` in the cache, and return `Arc::clone`.
4. Keep dynamic profiles working by wrapping registered definitions in `Arc` on cache insert.
5. Update validator call sites to accept `&snapshot` through `Arc` deref.

Measure:

- Run `cargo bench -p rh-validator --bench perf_smoke -- validate_simple_patient`.
- Expected impact: small to medium. This removes large clone work and prepares later plan caching.

Acceptance criteria:

- `cargo test -p rh-validator` passes.
- Profile cache metrics remain correct.
- No public behavior changes.
- Explicit simple-patient benchmark does not regress.

### A2. Return Shared Compiled Validation Rules

Problem:

`RuleCompiler::compile()` clones `CompiledValidationRules` on every cache hit. The compiled rule graph contains many strings, vectors, and JSON values.

Files:

- `crates/rh-validator/src/rules.rs`
- `crates/rh-validator/src/validator.rs`

Plan:

1. Change `RuleCompiler.cache` from `LruCache<String, CompiledValidationRules>` to `LruCache<String, Arc<CompiledValidationRules>>`.
2. Change `compile()` to return `Result<Arc<CompiledValidationRules>>`.
3. Store `Arc::new(rules)` after compilation.
4. Return `Arc::clone(cached)` on cache hit.
5. Avoid holding the mutex while doing expensive compilation.
   - First check cache under lock.
   - Drop lock.
   - Compile.
   - Reacquire lock to insert.

Measure:

- Run smoke benchmark before and after.
- Record rule cache hit rate and explicit simple-patient latency.

Acceptance criteria:

- `cargo test -p rh-validator` passes.
- Rule cache metrics remain correct.
- Explicit simple-patient benchmark improves or is neutral.

### A3. Compile Path Strings Into Path Specs

Problem:

Validation repeatedly splits and joins rule paths in helpers such as `get_values_at_path`, `should_validate_path`, cardinality, slicing, fixed/pattern, and binding validation.

Files:

- `crates/rh-validator/src/rules.rs`
- `crates/rh-validator/src/validator.rs`
- optionally new `crates/rh-validator/src/path.rs`

Plan:

1. Introduce a `PathSpec` type:

   ```rust
   #[derive(Debug, Clone)]
   pub struct PathSpec {
       pub original: String,
       pub segments: Vec<PathSegment>,
       pub parent_segments: Vec<PathSegment>,
       pub terminal: String,
       pub contains_choice: bool,
       pub contains_contained: bool,
   }

   #[derive(Debug, Clone)]
   pub enum PathSegment {
       Field(String),
       Choice { prefix: String },
   }
   ```

2. Add `PathSpec::parse(path: &str) -> PathSpec`.
3. Replace each rule's `path: String` with either:
   - `path: PathSpec`, or
   - `path: String` plus `path_spec: PathSpec` during migration.
4. Update helper functions to accept `&PathSpec`.
5. Keep original path strings for issue messages.
6. Do not rewrite every helper in one PR. Start with:
   - `should_validate_path`
   - `get_values_at_path`
   - `validate_type_at_path`
   - `validate_fixed_pattern_at_path`
7. Then migrate cardinality, binding, extension, and slicing helpers.

Measure:

- Run smoke benchmark after each helper migration.
- Use timing breakdown to confirm path navigation and rule-loop sections improve.

Acceptance criteria:

- `cargo test -p rh-validator` passes after each migration.
- No issue path text changes unless tests are updated deliberately.
- Explicit simple-patient benchmark improves or is neutral after each subtask.

### A4. Precompute Unknown-Property Maps

Problem:

`validate_unknown_properties_against_rules()` rebuilds `allowed_children` and choice-prefix maps every validation from `rules.element_paths`.

Files:

- `crates/rh-validator/src/rules.rs`
- `crates/rh-validator/src/validator.rs`

Plan:

1. Add to `CompiledValidationRules`:

   ```rust
   pub unknown_properties: UnknownPropertyPlan,
   ```

2. Define:

   ```rust
   pub struct UnknownPropertyPlan {
       pub resource_roots: HashSet<String>,
       pub allowed_children: HashMap<String, HashSet<String>>,
       pub allowed_choice_prefixes: HashMap<String, Vec<String>>,
   }
   ```

3. Build `UnknownPropertyPlan` once inside `RuleCompiler::compile`.
4. Change `validate_unknown_properties_against_rules()` to use the precomputed plan.
5. Keep the same recursion and issue behavior initially.

Measure:

- Run smoke benchmark.
- Timing breakdown should show unknown-property validation shrink.

Acceptance criteria:

- `cargo test -p rh-validator` passes.
- Existing unknown-property tests pass unchanged.
- Explicit simple-patient benchmark improves or is neutral.

## Workstream B: Invariant Validation Hot Path

This is likely the highest-impact workstream. US Core Patient currently has 102 snapshot constraints, including 77 repeats of:

```text
hasValue() or (children().count() > id.count())
```

Current validator behavior parses each invariant expression during every validation and clones the full JSON resource for each FHIRPath context.

### B1. Compile FHIRPath Invariants Once

Problem:

`validate_invariant()` normalizes and parses each invariant expression on every validation.

Files:

- `crates/rh-validator/src/rules.rs`
- `crates/rh-validator/src/validator.rs`
- `crates/rh-fhirpath/src/parser.rs` only if needed for type visibility

Plan:

1. Add a new rule type:

   ```rust
   pub struct CompiledInvariantRule {
       pub path: PathSpec,
       pub key: String,
       pub severity: String,
       pub human: String,
       pub expression_text: String,
       pub kind: InvariantKind,
   }

   pub enum InvariantKind {
       Native(NativeInvariant),
       FhirPath(Arc<rh_fhirpath::FhirPathExpression>),
       ParseError(String),
   }
   ```

2. During `RuleCompiler::compile`, normalize expression once.
3. Use one parser instance to parse expressions.
4. Dedupe identical normalized expressions with `HashMap<String, Arc<FhirPathExpression>>`.
5. Store parse errors as `InvariantKind::ParseError`, preserving current warning behavior at validation time.
6. Change validator invariant loop to use `CompiledInvariantRule`.

Measure:

- Run smoke benchmark before and after.
- Timing breakdown should show invariant time drop.

Acceptance criteria:

- `cargo test -p rh-validator` passes.
- Invariant parse warning behavior is preserved.
- Explicit simple-patient benchmark improves.

### B2. Add Native Fast Paths For Common Invariants

Problem:

Several FHIR core invariants are simple and repeated. Running the full FHIRPath engine for them is expensive.

Files:

- `crates/rh-validator/src/rules.rs`
- `crates/rh-validator/src/validator.rs`
- optionally new `crates/rh-validator/src/invariants.rs`

Start with these native invariants:

| Key or expression | Native behavior |
|-------------------|-----------------|
| `ele-1` / `hasValue() or (children().count() > id.count())` | For primitive/object element: valid if it has a JSON value or any child other than `id`; preserve extension handling. |
| `ext-1` / `extension.exists() != value.exists()` | Extension cannot have both nested `extension` and `value[x]`, and must have one. |
| `per-1` | If both `start` and `end` exist on Period, `start <= end`. |
| `dom-2` | Contained resource cannot itself contain contained resources. |
| `dom-3` | Preserve existing special-case `contained_resources_are_referenced`. |
| `dom-4` | Contained resource cannot have `meta.versionId` or `meta.lastUpdated`. |
| `dom-5` | Contained resource cannot have `meta.security`. |
| `dom-6` | Resource should have narrative text where applicable, warning severity as profile defines. |

Plan:

1. Add `NativeInvariant` enum.
2. Add classifier in rule compilation:

   ```rust
   fn classify_native_invariant(key: &str, normalized_expression: &str) -> Option<NativeInvariant>
   ```

3. Implement native evaluators against `&serde_json::Value` and `&PathSpec`.
4. Keep fallback to compiled FHIRPath for all other expressions.
5. Add focused tests comparing native and old FHIRPath behavior for representative resources.

Measure:

- Run smoke benchmark.
- Run complex patient benchmark because extensions exercise `ext-1`.
- Expected impact: large.

Acceptance criteria:

- `cargo test -p rh-validator` passes.
- No invariant-related conformance regressions.
- Explicit simple-patient benchmark improves materially.

### B3. Reduce FHIRPath EvaluationContext Cloning

Problem:

`EvaluationContext::new(resource.clone())` clones full JSON. Element-level invariants then call `.with_current((*element).clone())`, cloning again. This lives in `rh-fhirpath`, but validator invariants amplify the cost.

Files:

- `crates/rh-fhirpath/src/evaluator/core/context.rs`
- `crates/rh-fhirpath/src/evaluator/core/evaluator.rs`
- `crates/rh-validator/src/validator.rs`

Plan:

Stage 1, low-risk:

1. Add an alternate context constructor that takes `Arc<Value>`:

   ```rust
   pub fn from_arc(resource: Arc<Value>) -> Self
   ```

2. Internally change `EvaluationContext.root` and `current` to shared storage only if this can be done without widespread churn. If not, skip Stage 1 and do Stage 2 as a dedicated FHIRPath refactor.

Stage 2, larger:

1. Change `EvaluationContext` to store:

   ```rust
   pub root: Arc<Value>
   pub current: Arc<Value>
   ```

2. Adjust `FhirPathValue::from_json` call sites to borrow or cheaply wrap current values.
3. Replace `Rc<RefCell<Vec<TraceLog>>>` with `Arc<Mutex<Vec<TraceLog>>>` only if required by the new structure; parallelism is still not part of this plan.
4. Run full FHIRPath tests and conformance.

Measure:

- Add or use `rh-fhirpath` benches for repeated evaluation.
- Run validator smoke benchmark, especially after B1/B2.

Acceptance criteria:

- `cargo test -p rh-fhirpath` passes.
- `cargo test -p rh-fhirpath --test hl7_conformance` passes at the existing baseline.
- `cargo test -p rh-validator` passes.
- Validator explicit simple-patient benchmark improves or is neutral.

## Workstream C: Validate A Resource Once, Reuse The Work

### C1. Add `ValidationSession`

Problem:

`validate()` traverses the same JSON many times for base rules, coding checks, string checks, extensions, canonical URLs, and resource-specific checks. Profile validation then navigates paths repeatedly.

Files:

- `crates/rh-validator/src/validator.rs`
- optionally new `crates/rh-validator/src/session.rs`

Plan:

1. Introduce:

   ```rust
   pub(crate) struct ValidationSession<'a> {
       pub resource: &'a Value,
       pub resource_type: &'a str,
       pub issues: Vec<ValidationIssue>,
       pub path_cache: HashMap<PathId, Vec<&'a Value>>,
       pub extensions: Vec<ExtensionInfo>,
       pub codings: Vec<CodingInfo<'a>>,
       pub ucum_codings: Vec<UcumCodingInfo<'a>>,
   }
   ```

2. Start small:
   - Store `resource` and `resource_type`.
   - Move issue accumulation into `session.push_issue`.
   - Do not add indexing yet.
3. Update `validate()` and `validate_profile_rules_inner()` to accept a session internally.
4. Keep public APIs unchanged.

Measure:

- This initial extraction may be neutral. Run smoke benchmark to ensure no regression.

Acceptance criteria:

- `cargo test -p rh-validator` passes.
- Public API unchanged.
- Benchmark does not regress by more than 2%.

### C2. Single-Pass Base Scan

Problem:

Base validation currently uses multiple recursive functions that each walk the JSON tree and allocate path strings.

Plan:

1. Add a single recursive scanner:

   ```rust
   fn scan_resource(session: &mut ValidationSession, value: &Value, path: &mut PathBuffer)
   ```

2. During the scan, perform or collect data for:
   - string length checks
   - empty array checks
   - base64 field candidates
   - attachment size candidates
   - extension URLs
   - codings
   - UCUM codings
   - signature placeholders
   - coding system URI candidates
3. Keep specialized Bundle, Questionnaire, Measure, and StructureDefinition validators separate at first.
4. Introduce `PathBuffer` to avoid repeated `format!("{path}.{key}")` allocation except when creating issues.

Measure:

- Timing breakdown should show base validation shrink.
- Smoke benchmark should improve.

Acceptance criteria:

- Existing structural, extension, coding, and security tests pass.
- Issue paths remain stable.
- Explicit simple-patient benchmark improves.

### C3. Path Lookup Cache For Profile Rules

Problem:

Cardinality, type, fixed/pattern, binding, and invariant checks often ask for the same paths.

Plan:

1. Assign each compiled `PathSpec` a `PathId`.
2. Add `ValidationSession::values_at_path(path_id, &PathSpec) -> &[&Value]`.
3. Cache the result for the duration of one resource validation.
4. Use this in type, fixed/pattern, binding, and invariant validation first.
5. Migrate cardinality after because it has count-specific logic.

Measure:

- Timing breakdown should show type/fixed/binding/invariant navigation shrink.
- Smoke benchmark should improve.

Acceptance criteria:

- `cargo test -p rh-validator` passes.
- No issue path regressions.
- Explicit simple-patient benchmark improves.

## Workstream D: Avoid Duplicate Profile Work

### D1. Build A Profile Validation Plan Per Resource

Problem:

`validate_auto()` validates declared profiles and then separately validates the base profile. This aligns with the observed roughly 2x auto-detect cost.

Files:

- `crates/rh-validator/src/validator.rs`
- `crates/rh-validator/src/profile.rs`
- `crates/rh-validator/src/rules.rs`

Plan:

1. Introduce internal:

   ```rust
   struct ProfileValidationPlan {
       profiles: Vec<Arc<CompiledValidationRules>>,
       canonical_urls: HashSet<String>,
   }
   ```

2. Resolve declared profiles and base profile into one plan.
3. Dedupe by canonical URL.
4. If a declared profile's snapshot already includes base resource rules, do not separately apply the core base profile unless tests prove it is required for missing rules.
5. If unknown custom profile cannot resolve, preserve current warning and still apply base validation.
6. Add tests for:
   - no `meta.profile`
   - one known US Core profile
   - duplicate profile URLs with version suffix
   - unknown profile plus base resource type

Measure:

- Run `validate_auto_detect` smoke benchmark.
- Expected impact: high for auto-detect; little to no explicit-profile effect.

Acceptance criteria:

- `cargo test -p rh-validator` passes.
- Auto-detect benchmark improves materially.
- Explicit profile benchmark does not regress.

### D2. Separate Base Structural Validation From Profile Validation

Problem:

`validate_with_profile()` always calls `validate()` first, then profile rules. `validate_with_profiles()` also calls `validate()` once and applies profile rules. This is correct externally but makes it hard to share session data.

Plan:

1. Internally split:

   ```rust
   fn validate_base_into(&self, session: &mut ValidationSession) -> Result<()>
   fn validate_profiles_into(&self, session: &mut ValidationSession, plan: &ProfileValidationPlan) -> Result<()>
   ```

2. Public `validate`, `validate_with_profile`, `validate_with_profiles`, and `validate_auto` become thin wrappers.
3. Ensure `validate_auto` uses one session and one profile plan.

Measure:

- Smoke benchmark should be neutral or improved.
- Timing breakdown should be easier to interpret.

Acceptance criteria:

- `cargo test -p rh-validator` passes.
- Public API unchanged.
- Benchmark does not regress.

## Workstream E: rh-foundation Snapshot Generator Checks

Snapshot generation may not dominate warmed validation because cache hit rate is 100%, but cold validation and profile loading still matter.

### E1. Confirm Snapshot Cache Hit Is Not Re-Cloning Downstream

Problem:

`rh-foundation` already returns `Arc<Snapshot>` from `generate_snapshot`, but `rh-validator` unwraps/clones the snapshot into a `StructureDefinition` before caching its own copy.

Files:

- `crates/rh-validator/src/profile.rs`
- `crates/rh-foundation/src/snapshot/*` only if API adjustment is needed

Plan:

1. After A1, confirm `profile_with_snapshot.snapshot = Some(...)` still requires a deep snapshot clone.
2. If yes, consider one of:
   - storing `Arc<Snapshot>` in validator-specific cached profile wrapper, or
   - adding a foundation wrapper type that pairs `StructureDefinition` metadata with `Arc<Snapshot>`.
3. Avoid changing foundation public APIs unless necessary.

Measure:

- Cold first-validation timing.
- `cargo bench -p rh-foundation --bench snapshot`.

Acceptance criteria:

- Warm validation does not regress.
- Cold first validation improves or is neutral.

## Workstream F: rh-fhirpath Focused Improvements For Validator

Do not make broad FHIRPath rewrites unless they improve validator benchmarks.

### F1. Add FHIRPath Repeated-Eval Benchmarks

Files:

- `crates/rh-fhirpath/Cargo.toml`
- `crates/rh-fhirpath/benches/eval.rs`

Benchmarks:

- parse `hasValue() or (children().count() > id.count())`
- evaluate that expression against primitive-like and object-like JSON
- evaluate `extension.exists() != value.exists()`
- evaluate a resource-level `dom-3`-style expression if still used after native fast paths

Measure:

```bash
cargo bench -p rh-fhirpath --bench eval
```

Acceptance criteria:

- Benchmarks compile and run.
- Baseline numbers are recorded before FHIRPath changes.

### F2. Add An Optional Expression Cache API

Problem:

Other callers may benefit from repeated expression parse caching. Validator-specific compiled invariant rules may solve validator parsing, but a reusable FHIRPath cache still helps CLI and future work.

Plan:

1. Add `FhirPathEngine` or `CompiledExpressionCache` in `rh-fhirpath`.
2. Use `LruCache<String, Arc<FhirPathExpression>>`.
3. Keep existing parser/evaluator APIs unchanged.
4. Migrate validator only if it simplifies B1. Otherwise leave validator on its own compiled invariant plan.

Measure:

- `rh-fhirpath` repeated parse/eval benchmark.
- Validator smoke benchmark if validator uses it.

Acceptance criteria:

- `cargo test -p rh-fhirpath` passes.
- No public API break.

## Workstream G: Clean Architecture Extraction

These tasks help less powerful agents work safely, but should follow the high-impact measurement and invariant work unless file size blocks progress.

### G1. Split Validator Modules Along Hot-Path Boundaries

Current `crates/rh-validator/src/validator.rs` is very large. Split only when it supports the performance work.

Suggested modules:

- `src/session.rs`: `ValidationSession`, `PathBuffer`, per-resource scan/index
- `src/path.rs`: `PathSpec`, path navigation, path cache
- `src/invariants.rs`: native invariant classification/evaluation
- `src/profile_plan.rs`: profile resolution and dedupe
- `src/base_rules.rs`: base structural validation scanner
- `src/profile_rules.rs`: applying compiled profile rules

Plan:

1. Move code without behavior changes.
2. Run tests after each module extraction.
3. Do not mix extraction with optimization in the same commit unless the optimization is trivial.

Measure:

- Benchmark should be neutral.

Acceptance criteria:

- `cargo test -p rh-validator` passes after each extraction.
- No file move changes public API.
- Benchmark does not regress by more than 2%.

## Milestones

### Milestone 1: Reproducible Fast Measurement

Tasks:

- Gate 0
- Gate 1

Exit criteria:

- `cargo bench -p rh-validator --bench perf_smoke` is the standard quick measurement.
- Timing breakdown exists for attribution.
- Baseline is recorded in this file or in `PERFORMANCE.md`.

### Milestone 2: Cache Clones Removed

Tasks:

- A1
- A2

Exit criteria:

- Snapshot and rule cache hits return `Arc`.
- Warm explicit simple-patient validation improves or is neutral.

### Milestone 3: Invariant Hot Path Fixed

Tasks:

- B1
- B2
- B3 only if needed after B1/B2

Exit criteria:

- Explicit simple-patient validation is below 30 ms.
- Complex patient validation materially improves.
- No invariant regression tests fail.

### Milestone 4: Compiled Paths And Unknown Properties

Tasks:

- A3
- A4
- C3

Exit criteria:

- Explicit simple-patient validation is below 15 ms.
- Auto-detect is no more than 1.5x explicit profile validation.

### Milestone 5: Single-Session Validation And Auto-Dedup

Tasks:

- C1
- C2
- D1
- D2

Exit criteria:

- Explicit simple-patient validation approaches or beats 5 ms.
- Auto-detect approaches explicit profile validation.
- Warm batch throughput improves as a direct result of lower per-resource latency, without parallelism.

## Required Progress Log Format

Add a row after each completed task.

| Date | Task | Simple explicit | Auto-detect | Complex | Batch 50 throughput | Notes |
|------|------|-----------------|-------------|---------|---------------------|-------|
| 2026-07-03 | Baseline from `PERFORMANCE.md` | 82.21 ms | 163.88 ms | 635.67 ms | 12.31/sec | Apple M1 Max, macOS 26.4 |

## Suggested First Implementation Sequence

This is the safest path for a less powerful agent:

1. Add `perf_smoke.rs`.
2. Add timing breakdown instrumentation.
3. Change rule cache to return `Arc<CompiledValidationRules>`.
4. Change snapshot cache to return `Arc<StructureDefinition>`.
5. Compile invariant FHIRPath expressions once.
6. Add native fast path for `ele-1`.
7. Add native fast path for `ext-1`.
8. Add native fast path for `per-1`.
9. Precompute unknown-property maps.
10. Introduce `PathSpec` and migrate `should_validate_path`.
11. Migrate `get_values_at_path` to `PathSpec`.
12. Add `ValidationSession` with no indexing.
13. Add path lookup cache to `ValidationSession`.
14. Dedupe `validate_auto` profile plan.
15. Refresh `crates/rh-validator/PERFORMANCE.md`.

## When To Stop And Reassess

Stop and reassess if any of these happen:

- A task improves simple explicit validation by less than 1% and adds meaningful complexity.
- Any conformance or validator test fails in a way that requires loosening validation behavior.
- FHIRPath API changes start cascading into unrelated crates.
- The simple explicit benchmark drops below 5 ms before all tasks are complete. At that point, update `PERFORMANCE.md`, then reconsider whether remaining work is still worth the complexity.
