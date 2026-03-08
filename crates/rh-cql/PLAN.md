# rh-cql PLAN

Status: ✅ Restructured into three-stage pipeline (cql-multi-stage-pipeline-refactor)

## 1) Objective

Build a new Rust crate, `rh-cql`, that supports:

1. **Parse** CQL source to a typed AST
2. **Compile** CQL to ELM (JSON first; XML optional)
3. **Explain** CQL (parser + semantic/translation explanation output)
4. **Evaluate** ELM (primary runtime target; CQL evaluation compiles to ELM then executes)

Cross-cutting requirement: translation and evaluation must produce and preserve
source maps suitable for debugger-style traces and structured logging.

This plan is designed to be split into follow-up OpenSpec changes.

---

## 2) Spec Scope Deep-Dive (What is in scope)

Based on HL7 CQL 2.0.0-ballot continuous build and related pages:

- **Normative anchors**:
  - Grammar (ANTLR; CQL extends FHIRPath grammar)
  - Translation semantics (CQL↔ELM mappings)
  - ELM schema definitions (`expression.xsd`, `clinicalexpression.xsd`, `library.xsd`, annotations)
- **Informative but critical**:
  - Official test ecosystem (`cqframework/cql-tests`)
  - Reference implementations (Java translator, JS execution engines)

### Practical interpretation for rh-cql

- We should treat **CQL→ELM** as the canonical compile path.
- Runtime should execute **ELM IR**, not raw CQL AST.
- "Explain" should expose at least:
  - parse tree / AST summary
  - symbol/type resolution notes
  - translation mapping notes (CQL form → ELM operator)
  - diagnostics with source spans and message codes

---

## 3) Scope Boundaries

## In scope (initial product)

- CQL library parsing with key declarations:
  - `library`, `using`, `include`, `parameter`, `define`, `define function`, `context`
- Core expression families needed to unlock practical content:
  - literals/selectors, logical/comparison/arithmetic/string/date-time/list basics
  - query subset with `from/let/where/return/sort`
- CQL→ELM translation with deterministic output
- ELM JSON loader + evaluator for translated expressions
- Explain output for parser/translator/evaluator stages
- Foundational diagnostics model (error/warn/info with codes)

## Explicitly out of scope (phase-gated)

- Full parity with all trial-use features on day 1
- Complete terminology service stack (VSAC integration, remote expansion, etc.)
- Full modelinfo ecosystem parity and every external model immediately
- Full ELM XML write path in initial MVP
- Bit-for-bit match to Java translator formatting/annotation output

---

## 4) Architecture Direction

```text
                  ┌───────────────────────────┐
CQL source         │        rh-cql parser      │
───────────▶       │ lexer + grammar + AST     │
                  └─────────────┬─────────────┘
                                │
                                ▼
                  ┌───────────────────────────┐
                  │    semantic pipeline       │
                  │ symbols, types, options    │
                  └─────────────┬─────────────┘
                                │
                                ▼
                  ┌───────────────────────────┐
                  │   CQL → ELM translator     │
                  │   (canonical IR target)    │
                  └─────────────┬─────────────┘
                                │
                  ┌─────────────┴─────────────┐
                  ▼                           ▼
       ┌─────────────────────┐      ┌─────────────────────┐
       │ explain renderer     │      │ ELM evaluator       │
       │ (human diagnostics)  │      │ (runtime execution) │
       └─────────────────────┘      └─────────────────────┘
```

### Internal module plan

- `syntax/` — tokenizer + parser + AST + source spans
- `semantics/` — symbol tables, type inference/checking, conversion rules
- `elm/` — ELM Rust model + serde serialization/deserialization
- `translate/` — CQL AST/semantic IR → ELM
- `eval/` — ELM evaluator core
- `explain/` — explanation trees, trace records, human-readable output
- `diagnostics/` — structured diagnostics and message catalog
- `sourcemap/` — CQL span ↔ ELM node ↔ runtime step correlation data

### Alignment with `rh-fhirpath` decisions

`rh-cql` should follow the same parser/grammar strategy and engineering posture used in `rh-fhirpath` unless there is a documented, evidence-based reason to diverge.

Required alignment points:

- Prefer a pure Rust parser-combinator approach compatible with existing workspace patterns (avoid introducing generator-heavy build steps unless clearly necessary).
- Keep grammar handling and AST construction style consistent with `rh-fhirpath` to reduce maintenance and onboarding cost.
- Reuse existing expression parsing conventions where CQL inherits FHIRPath concepts (operator precedence, invocation/indexing semantics, temporal/unit literal treatment).
- Reuse diagnostics conventions established by `rh-fhirpath` (clear, user-facing errors with stable structure and source location details).
- Match test philosophy from `rh-fhirpath`: focused unit tests, integration-style examples, and deterministic behavior in parser/evaluator outputs.

If any of these constraints are intentionally broken, the change should include an ADR-style note in the OpenSpec design artifact documenting:

1. Why alignment is insufficient
2. Alternatives considered
3. Long-term maintenance impact
4. Migration path if we need to converge later

---

## 5) Capability Phases (OpenSpec-friendly slicing)

## Phase A — Parser Foundation ✅ (Restructured)

**Goal**: Parse CQL libraries into stable AST with spans and recoverable errors.

Deliverables:
- Grammar strategy selected and documented, with `rh-fhirpath` alignment as default baseline (pure Rust combinator parser unless justified otherwise)
- AST definitions for declarations/statements/expressions
- Parser tests for representative grammar categories
- Error recovery strategy (continue after syntax errors where feasible)

Exit criteria:
- Can parse canonical examples from spec pages + selected real-world snippets
- Diagnostics include file/line/column + error code

## Phase B — Semantic Analysis ✅ (Restructured — moved to `semantics/`)

**Goal**: Build symbol/type pipeline needed for meaningful compile and explain.

Deliverables:
- Symbol table for declarations + includes
- Type model (System.* primitives, list/interval/tuple basics)
- Implicit conversion and list-promotion policy framework (option-controlled)
- Semantic diagnostics (undefined symbol, overload mismatch, invalid conversion)

Exit criteria:
- Semantic pass stable for Phase A expression set
- Deterministic diagnostic ordering

## Phase C — CQL→ELM Compile ✅ (Restructured — three-stage pipeline, `emit/` modules)

**Goal**: Produce valid ELM JSON for supported CQL subset.

Deliverables:
- ELM data model structs for supported operators
- Mapping layer for declarations/types/literals/operators/queries
- Translation option support (at least the four highlighted in spec)
- Source-map emission strategy (needed by explain, debugger traces, and round-trip aids)
  - CQL source span → ELM element id/path mapping
  - Stable element identifiers to support deterministic logs
  - Serialization format for map metadata (JSON sidecar and/or embedded annotations)

Exit criteria:
- Translation golden tests pass (CQL input → expected ELM JSON shape)
- Output is consistent and round-trippable for supported subset

## Phase D — Explain Mode ✅ (Implemented)

**Goal**: Human-oriented "what happened" output for parse/compile/eval pipelines.

Deliverables:
- `explain parse`: AST summary + precedence/grouping details
- `explain compile`: mapping narrative (e.g., `A != B` → `Not(Equal(A,B))`)
- `explain eval`: step trace of ELM execution with intermediate values and source-map links back to original CQL
- Stable machine-readable explain format (JSON)

Exit criteria:
- Explain output usable for debugging translator/evaluator issues
- Message catalog references spec section tags where possible

## Phase E — ELM Evaluation Engine ✅ (Implemented)

**Goal**: Execute supported ELM reliably and deterministically.

Deliverables:
- Runtime value model and three-valued logic/null semantics
- Core operator execution for supported subset
- Query execution primitives (filter/sort/projection/aggregation subset)
- Timezone/precision policy + deterministic clock injection for tests
- Runtime trace correlation using source maps:
  - Every evaluable step should carry ELM node id/path
  - When available, include original CQL span in logs and debug events
  - Structured trace events suitable for CLI verbose logging and future debugger hooks

Exit criteria:
- Passes selected cql-tests corpus for implemented feature groups
- Handles CQL-compiled ELM and direct ELM input equivalently

## Phase F — Integration + Conformance Expansion 🔄 (In Progress)

**Goal**: Connect crate to CLI/workflows and broaden operator coverage.

Deliverables:
- CLI plumbing (parse/compile/explain/eval commands)
- Compatibility matrix documenting supported CQL/ELM features
- Incremental expansion across operator families and clinical operators
- Performance baselines and regressions checks

Exit criteria:
- End-to-end user flows functional in `rh` CLI
- Published status page for coverage and known gaps

---

## 6) API Surface (Draft)

```rust
// parse
pub fn parse_library(source: &str) -> Result<CqlLibraryAst, Diagnostics>;

// compile
pub fn compile_to_elm(source: &str, opts: CompileOptions) -> Result<ElmLibrary, Diagnostics>;
pub fn compile_to_elm_with_sourcemap(source: &str, opts: CompileOptions) -> Result<(ElmLibrary, SourceMap), Diagnostics>;

// explain
pub fn explain_parse(source: &str, opts: ExplainOptions) -> Result<ExplainReport, Diagnostics>;
pub fn explain_compile(source: &str, opts: CompileOptions) -> Result<ExplainReport, Diagnostics>;
pub fn explain_eval(elm: &ElmLibrary, input: &EvaluationInput, opts: EvalOptions) -> Result<ExplainReport, Diagnostics>;

// evaluate
pub fn evaluate_elm(elm: &ElmLibrary, input: &EvaluationInput, opts: EvalOptions) -> Result<EvaluationResult, Diagnostics>;
pub fn evaluate_elm_with_trace(elm: &ElmLibrary, input: &EvaluationInput, opts: EvalOptions) -> Result<(EvaluationResult, EvaluationTrace), Diagnostics>;
```

Note: API signatures are planning placeholders and may evolve.

---

## 6.1) Source-Map Schema Sketch (Draft)

Purpose: correlate CQL source text, translated ELM nodes, and runtime evaluation
events for debugging, explain output, and structured logs.

### Core requirements

- Deterministic IDs across repeated compilation of unchanged source.
- Many-to-many mapping support:
  - One CQL span may map to multiple ELM nodes.
  - One ELM node may originate from synthetic translation of multiple CQL spans.
- Runtime trace events must reference ELM node IDs, then resolve to CQL spans via map.
- Compact enough for CLI verbose mode, but rich enough for machine tooling.

### Proposed logical model

```text
SourceMap
├── version: string
├── source_documents: [SourceDocument]
├── elm_nodes: [ElmNodeMeta]
├── mappings: [SourceElmMapping]
└── options_fingerprint: string

SourceDocument
├── doc_id: string
├── uri: string
├── checksum: string
└── line_index: optional compressed offsets

ElmNodeMeta
├── elm_node_id: string
├── elm_path: string            // JSON pointer-like path in ELM document
├── elm_kind: string            // Equal, Query, FunctionRef, etc.
└── parent_id: optional string

SourceElmMapping
├── mapping_id: string
├── doc_id: string
├── span: {start_line,start_col,end_line,end_col,start_byte,end_byte}
├── role: string                // direct | implicit-conversion | desugared | synthetic
├── elm_node_ids: [string]
├── confidence: string          // exact | approximate
└── note: optional string
```

### Stable ID strategy (draft)

- `doc_id`: hash of normalized library identifier + version + canonical URI
- `elm_node_id`: hash of (`elm_kind`, canonical `elm_path`, normalized operator signature)
- `mapping_id`: hash of (`doc_id`, span bytes, sorted `elm_node_ids`, role)

Normalization rules should be documented and versioned so IDs remain stable across
platforms and formatting-only source changes (where feasible).

### Example map payload (JSON)

```json
{
  "version": "rh-cql-sourcemap/v1",
  "source_documents": [
    {
      "doc_id": "doc:8f8e4b",
      "uri": "library:MyLogic:1.0.0",
      "checksum": "sha256:..."
    }
  ],
  "elm_nodes": [
    {
      "elm_node_id": "elm:2ab913",
      "elm_path": "/statements/def[ObservationFlag]/expression/operand[0]",
      "elm_kind": "Equal"
    }
  ],
  "mappings": [
    {
      "mapping_id": "map:5c11d1",
      "doc_id": "doc:8f8e4b",
      "span": {
        "start_line": 14,
        "start_col": 9,
        "end_line": 14,
        "end_col": 24,
        "start_byte": 312,
        "end_byte": 327
      },
      "role": "direct",
      "elm_node_ids": ["elm:2ab913"],
      "confidence": "exact"
    }
  ],
  "options_fingerprint": "compileopts:7a09e2"
}
```

### Runtime trace correlation sketch

Each evaluation step event should include:

- `event_id`
- `elm_node_id`
- `op` (operator kind)
- `inputs` (redaction-capable)
- `output`
- `timestamp` / monotonic step index

Optional enriched fields (resolved via source-map lookup):

- `source.doc_id`
- `source.uri`
- `source.span`
- `source.snippet` (bounded length)

This allows logs to stay lightweight by default (ELM IDs only), while debug modes
can hydrate source details on demand.

### Serialization strategy

- Primary: sidecar JSON (`*.elm.json` + `*.elm.sourcemap.json`)
- Optional: embedded minimal mapping metadata in ELM annotations for portability
- Explain/eval APIs should accept either embedded or sidecar map sources

### Compatibility & evolution

- Version the schema (`rh-cql-sourcemap/vN`) and avoid breaking changes inside a major version.
- Additive fields only in minor revisions.
- Include migration adapters for persisted map artifacts if schema evolves.

---

## 7) Testing & Validation Strategy

## Test sources

- Unit tests for parser, semantics, translator, evaluator
- Golden tests:
  - CQL → AST snapshots
  - CQL → ELM JSON snapshots
  - CQL → ELM source-map snapshots
  - Explain output snapshots
- Conformance tests from `cqframework/cql-tests` (subset first, expand per phase)

## Determinism constraints

- Stable diagnostic ordering
- Stable JSON field ordering where possible for snapshots
- Injectable clock/timezone for date-time tests

## Quality gates (phase-dependent)

- Parse success/failure precision targets
- Translation correctness for declared operator families
- Runtime equivalence tests for CQL-compiled vs direct ELM paths

---

## 8) Key Risks and Mitigations

1. **Spec breadth and optionality**
   - Mitigation: capability matrix + explicit feature flags + phased conformance

2. **Implicit conversion complexity**
   - Mitigation: central conversion engine with option toggles and explain traces

3. **Null/temporal semantics subtlety**
   - Mitigation: focused semantic tests and deterministic runtime configuration

4. **Model/terminology integration complexity**
   - Mitigation: decouple base evaluator from pluggable terminology/model providers

5. **Round-trip expectations (CQL↔ELM)**
   - Mitigation: preserve source annotations and document non-round-trip equivalence cases

---

## 9) OpenSpec Change Backlog Proposal

Recommended split into independent changes:

1. `cql-parser-core`
2. `cql-semantics-core`
3. `cql-compile-to-elm`
4. `cql-explain-mode`
5. `elm-evaluator-core`
6. `cql-cli-integration`
7. `cql-conformance-expansion`

Each change should include:
- proposal.md (scope + risks)
- design.md (data model and pipeline)
- tasks.md (phased implementation checklist)
- specs/ deltas for user-visible behavior

---

## 10) Suggested Immediate Next Step

Start with **Phase A + thin Phase C slice**:
- Parse a constrained subset
- Compile just enough operators to produce executable ELM for a few canonical examples
- Wire explain output for parser and translation mapping on that subset

This gives the team early end-to-end value without committing to full-language scope up front.

---

## 11) External Implementations Analysis (Design Inputs)

This section captures what to learn from selected CQL implementations and how it
should influence `rh-cql`.

### A) `cqframework/cql-execution` (TypeScript ELM runtime)

Observed characteristics:
- Strongly ELM-centric runtime model: classes map closely to ELM expression kinds.
- Practical library/context constructs (`Library`, `Context`, patient/unfiltered contexts).
- Heavy use of ELM `annotation` / `localId` metadata in test fixtures and execution behaviors.
- Extensive test corpus organized around ELM feature groups.

Adopt for `rh-cql`:
- Keep evaluator ELM-first and expression-node-oriented.
- Preserve translation metadata needed for debugging (our source-map strategy covers this).
- Build tests around representative ELM fixture families, not only parser unit tests.

Avoid for `rh-cql`:
- Avoid dynamic/loosely-typed runtime internals where Rust can enforce stronger compile-time guarantees.

### B) `google/cql` (Go parser+interpreter with explainability)

Observed characteristics:
- Clear split: parser does validation + implicit conversion insertion; interpreter assumes well-typed model.
- Explicit interfaces for retriever and terminology provider.
- Built-in explainability using source-expression/source-value trees.
- Strong testing strategy: parser tests + engine tests from CQL strings + imported spec tests + benchmarks.
- Documented limitations and staged feature coverage.

Adopt for `rh-cql`:
- Keep conversion/overload resolution centralized in semantic/translation phases, not in evaluator hot path.
- Keep runtime integration points pluggable (`Retriever`, terminology provider analogs).
- Make explain traces first-class artifacts, not ad-hoc logs.
- Use layered testing: parser tests, engine tests, imported conformance tests, benchmarks.
- Publish capability matrix and known limitations per release.

### C) `samply/blaze` (Production FHIR server CQL pipeline)

Observed characteristics:
- Explicit multi-stage pipeline: CQL→ELM, compile, evaluate, resolve refs, optimize, cache/attach filters, parallel patient evaluation.
- Operational focus: performance docs, query patterns, metrics around compile/evaluate latency.
- Uses proven external translator for CQL→ELM in production path.

Adopt for `rh-cql`:
- Treat compile and eval as separate optimization stages with measurable boundaries.
- Add structured metrics early (compile latency, eval latency, throughput, cache hit/miss where applicable).
- Design for parallel evaluation and workload-scale workflows as a first-order concern.
- Keep optimization passes explicit and inspectable in explain mode.

### D) `cqframework/cqf-ruler` (Clinical reasoning integration)

Observed characteristics:
- Emphasis on FHIR operation integration (`$cql`, `$evaluate`, CDS Hooks, measure workflows).
- Strong parameterization and request-context mapping concerns.
- Debug handling exists but is not universally surfaced as a stable developer trace contract.

Adopt for `rh-cql`:
- Keep API boundaries friendly for future operation-level integration (subject context, parameters, expression targeting).
- Ensure structured error payloads can map cleanly to operation-style responses later.
- Support both identifier-based evaluation (definition names) and expression-based evaluation paths.

### E) `FirelyTeam/firely-cql-sdk` (.NET translator + compiler toolchain)

Observed characteristics:
- ANTLR grammar-based translator pipeline with DI/toolkit abstractions.
- Strong packaging/tooling story (CQL→ELM→codegen/runtime artifacts).
- Explicit disclaimer that native CQL→ELM translation is early/non-production in parts.
- Useful debug-symbol concepts (PDB workflow) and locator-focused debugging utilities.

Adopt for `rh-cql`:
- Keep clear separations between translator, evaluator, and packaging/integration layers.
- Borrow the “debug symbol artifact” mindset for source maps (our sidecar map + stable IDs).
- Keep implementation maturity transparent in docs and capability matrix.

Avoid for `rh-cql`:
- Do not commit to heavyweight codegen/JIT strategy in MVP; prioritize deterministic interpreter path first.

---

## 12) Consolidated Decisions for `rh-cql`

From the above comparison, these decisions are added to scope direction:

1. **ELM-first runtime contract**
  - `evaluate(cql)` always compiles to ELM then executes ELM.

2. **Parser/semantic responsibility split**
  - Parser + semantics resolve overloads/conversions and emit explicit IR.
  - Evaluator minimizes dynamic type negotiation during execution.

3. **Source-map + explainability as core, not optional**
  - Every translation and evaluation path must be traceable to source spans and ELM node IDs.

4. **Pluggable external interfaces**
  - Data retrieval and terminology access are abstract interfaces with deterministic local test doubles.

5. **Performance and scale baked in**
  - Compile/eval metrics, benchmark suites, and parallel-friendly evaluation architecture are required.

6. **Conformance strategy with explicit maturity labels**
  - Ship in slices, track unsupported areas publicly, and validate against imported spec tests incrementally.

7. **Integration-ready API shape**
  - Preserve request-context inputs (subject, parameters, execution timestamp) for eventual operation-level use.
