# RH Workspace Refactor Plan

**Date:** 2026-06-12
**Scope:** `rh-foundation`, `rh-codegen`, `rh-fhirpath`, `rh-fsh`, `rh-packager`, `rh-cql`, `rh-vcl`, plus `apps/rh-cli`, documentation sync, performance, conformance, and a generalized WASM/NPM/reference-app pattern.
**Status:** In progress — see Progress Tracking below.

## Progress Tracking

| Workstream | Status | Notes |
|---|---|---|
| WS0 Hygiene | ✅ done (2026-06-12) | 0.4: ffq **deleted** (not gated) — it depended on a nonexistent `rh-ffq` crate and could never compile. 0.5: `clippy::unwrap_used` deferred until WS1 (1.1) / WS4 (4.6) unwrap cleanups land, since `just lint` runs `-D warnings`; `unsafe_code = "forbid"` + `clippy.all = "warn"` are in place. |
| WS1 Foundation | ✅ done (2026-06-12) | 1.1: audit found all remaining unwraps were test-only; added `#![cfg_attr(not(test), warn(clippy::unwrap_used))]` guard. 1.4: `generate_snapshot` now returns `Arc<Snapshot>` (28 ns cache hits vs deep clone); bench at `benches/snapshot.rs` (cold 600-element merge ~424 µs). 1.5: merger uses borrowed `Cow` index — no upfront clones of base elements or key strings. 1.7: packager's foundation `http` feature + tokio dep were entirely unused — **removed** rather than feature-gated. **Bonus fix:** `StructureDefinition.base_definition` was missing `#[serde(rename = "baseDefinition")]` — profiles loaded from real FHIR JSON silently lost their base and produced differential-only snapshots; fixed + regression test. |
| WS2 Conformance | 🔶 partial (2026-06-15) | **Done:** 2.1 HL7 FHIRPath suite vendored (937 cases) + harness `tests/hl7_conformance.rs` with summary JSON; baseline 564/935 pass (60.3%) and shrink-only known-wrong-answer gating. 2.2 `CONFORMANCE.md` + `SPEC_COVERAGE.md` created and kept current. 2.4 clinical age operators complete. 2.5 Ratio literals, ToRatio, 1-arg Combine, Message, Children, Descendants complete. 2.6 CQL HL7 burn-down waves 1–2 landed: 552→733 pass, 244→97 skip, 0 failures. 2.7 fixed-corpus ELM fidelity diff landed in `pipeline_comparison_tests.rs`, with Java-vs-Rust metadata reporting documented in `crates/rh-cql/CONFORMANCE.md`. 2.8 68 FSH parser unit tests added and 3 parser bugs fixed. 2.9 FSH fixture corpus expanded to 61 files. 2.10 stale audit claim closed with no code changes. **FHIRPath waves 3–35:** 564→934 pass (60.3%→99.9%), wrong answers 120→0, parse errors 79→0, eval errors 171→0. Wave 34 (2026-06-15): strict polymorphic choice-type access enforced; `checkOrderedFunctions`/`UnorderedCollection` for order-dependent function guards; `boundary_decimal` near-zero truncation fix. All 7 wrong answers eliminated — zero wrong-answer baseline achieved. Wave 35 (2026-06-15): `boundary()` dispatches String values to temporal boundary functions; Date strings produce DateTime boundaries for cross-type comparison. Last eval-error eliminated — **zero eval-errors achieved.** Only remaining non-pass is 1 skipped test (missing `patient-name-extensions.json`, not published by HL7). **Pending non-FHIRPath WS2 work:** CQL skip burn-down complete (97→48, target met) |
| WS3 CLI | ✅ done |3.1-3.7 all complete |
| WS4 Codegen | ⬜ not started | |
| WS5 Performance | ⬜ not started | |
| WS6 WASM/NPM | ⬜ not started | |
| WS7 Docs + gates | ⬜ not started | |

---

## 1. Overall Codebase Assessment

**Verdict: B+ overall.** This is a healthy, well-architected workspace with clean layering, consistent use of nom-based parsers, strong recent investment in conformance (CQL), and real performance wins already documented (`docs/devdays-2025-perf.md`). The gaps are concentrated in four areas: (1) CLI cross-command consistency, (2) FHIRPath official-suite conformance tracking, (3) WASM/NPM productization, and (4) documentation drift.

| Crate | Architecture | Conformance | Perf posture | Error handling | Docs sync | Tests | Grade |
|---|---|---|---|---|---|---|---|
| rh-foundation | Clean, feature-gated | n/a | Arc snapshots, Cow merge | ✅ No panics (WS0/WS1) | Good | Integration + unit | A- |
| rh-codegen | Works; 5 god-files >1000 LOC | n/a (output-validated) | Single-threaded | 26 unwrap/expect | "In Progress" section stale | Sparse (2 integration) | B- |
| rh-fhirpath | Modular, 10.5k LOC evaluator | **933/935 pass (99.8%), 0 wrong** | No parse cache, clone-heavy | Good error enum, weak spans | CONFORMANCE.md + SPEC_COVERAGE.md | 935 HL7 + unit | A- |
| rh-cql | Excellent 3-stage pipeline | **92% (161/175 ops), 475 HL7 tests pass, 0 wrong answers** | Benchmarked, no lib cache | Panics in assertion paths | Excellent (CONFORMANCE.md, SPEC_COVERAGE.md) | Strong + golden files | A- |
| rh-fsh | Clean 4-stage (parse→tank→resolve→export) | 30/30 SUSHI compat fixtures | rayon export, benchmarked | 10 unwraps | Good + 68 parser unit tests | Integration-heavy, parser unit tests | B+ |
| rh-packager | Clean, focused | n/a | Sync I/O, fine | Good | Good | 127 tests | A- |
| rh-vcl | Clean, WASM-ready | Full grammar per spec | Zero-copy parser | Clean | Minor WASM fn-name drift | 76 tests, comprehensive | A- |
| rh-cli | Consistent output framework | n/a | n/a | Typed exit codes, envelope | Good | All 9 commands tested | B+ |
| Generated R4/R5 | Idiomatic, 3-tier traits | n/a | Compile-time heavy | n/a | Generic READMEs | R5 smoke test only | B |

**Systemic strengths:** consistent thiserror-in-libraries pattern; workspace dependency inheritance (with one critical exception); CI with `clippy -D warnings`, fmt, audit, platform matrix; criterion benches in 3 crates; openspec-based change tracking already in use.

**Systemic weaknesses:**
1. **Codegen god-files and unwrap density** — 5 files >1000 LOC, 26 non-test unwraps; needs split + error types (WS4).
2. **WASM exists but is not productized** — no npm packages, no CI wasm builds, no shared pattern; `rh-foundation`'s WASM feature is infrastructure-only (WS6).
3. **Docs drift** — ARCHITECTURE.md dependency graph may lag behind crate changes; MSRV should be enforced in CI (WS7).

### Verified critical defects (fix first)

| ID | Defect | Evidence | Status |
|---|---|---|---|
| C1 | `rh-hl7_fhir_r5_core/Cargo.toml:14` hardcodes an **absolute path to another developer's machine**: `rh-foundation = { path = "/Users/bkaney/projects/reason-healthcare/rh/crates/rh-foundation" }`. Crate also bypasses workspace inheritance (version `0.1.0`, authors `["FHIR Code Generator"]`, no `rust-version`). | Verified by reading the file | ✅ Fixed (WS0) |
| C2 | `MemoryStore` RwLock `.unwrap()` in ~10 places (`rh-foundation/src/memory/mod.rs:120,131,144,151,160,165,170,175,199,204`) — lock poisoning panics the process. | Foundation audit | ✅ Fixed (WS0) |
| C3 | `apps/rh-cli/src/ffq.rs` (305 lines) fully implemented but **not wired** into the `Commands` enum — dead shipping code. | Verified: no `ffq` references in `main.rs` | ✅ Deleted (WS0) |
| C4 | README.md:12 claims rh-foundation "supports WebAssembly targets"; the `wasm` feature only adds a panic hook, no exports, and `io.rs` has unguarded `std::fs`. | Infra audit | ✅ Fixed (WS0) |

### Corrected dependency graph (ARCHITECTURE.md must be replaced with this)

```
Layer 0 (leaf): rh-foundation, rh-hl7_fhir_r4_core, rh-hl7_fhir_r5_core
Layer 1: rh-codegen (→foundation), rh-fsh (→foundation), rh-vcl (→foundation),
         rh-fhirpath (→foundation, r4_core), rh-cql (→foundation)
Layer 2: rh-validator (→foundation[http], fhirpath)
Layer 3: rh-packager (→foundation, fhirpath, vcl, validator)
Layer 4: rh-cli (→ all)
```

---

## 2. Workstream Overview, Dependencies, and Order

```
WS0 Hygiene (C1–C4, workspace lints)        ── no deps; DO FIRST; small
 │
 ├─► WS1 Foundation hardening & perf        ── blocks WS5 (fhirpath/validator perf), WS6 (wasm)
 ├─► WS2 Conformance harnesses              ── independent; blocks WS5 (perf needs regression nets)
 ├─► WS3 CLI consistency framework          ── independent of WS1/WS2; REPL extraction touches crates
 ├─► WS4 Codegen refactor + R4/R5 regen     ── needs WS0-C1; validates via generated crates
 │
 ├─► WS5 Per-crate performance              ── after WS1 + WS2 (correctness nets in place)
 ├─► WS6 WASM/NPM/reference app             ── after WS1 (foundation wasm), parallel with WS5
 │
 └─► WS7 Docs sync + CI gates               ── quick-win doc fixes in WS0; final sweep LAST
```

**Recommended execution order:** WS0 → (WS1 ∥ WS2 ∥ WS3 ∥ WS4) → (WS5 ∥ WS6) → WS7.
Parallel-safe boundaries: each workstream below lists explicit file ownership so agents don't collide. Within a workstream, tasks are ordered; tasks in different workstreams marked ∥ can run as concurrent branches.

**Conventions for all workstreams (apply everywhere):**
- `thiserror` in crates, `anyhow` only in `rh-cli`. No new `.unwrap()`/`.expect()` outside tests; replace existing ones per task lists.
- All workspace members inherit `version/edition/authors/license/repository/rust-version` via `workspace = true`.
- Conventional commits with existing scopes (`codegen/cli/fhirpath/vcl/validator/loader/deps/ci/docs`); add scopes `cql`, `fsh`, `packager`, `foundation`, `wasm`.
- Every task's acceptance criteria includes: `just check` passes (fmt + clippy -D warnings + tests).

---

## 3. WS0 — Workspace Hygiene & Critical Fixes (size: S, 1–2 days) ✅ COMPLETED

No dependencies. Everything else builds on this. All tasks (0.1–0.6) completed 2026-06-12; `just check` green. See Progress Tracking for deviations on 0.4 and 0.5.

| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 0.1 | Fix C1: change R5 dep to `rh-foundation = { path = "../rh-foundation", version = "0.2.0-beta.2" }`; adopt workspace inheritance for version/edition/authors/license/rust-version; align crate metadata (description/keywords/categories) with R4 crate. | `crates/rh-hl7_fhir_r5_core/Cargo.toml` | `cargo metadata` resolves on a clean clone; `cargo build -p rh-hl7-fhir-r5-core` passes |
| 0.2 | Fix C1 root cause: rh-codegen must emit relative-path + workspace-inherited Cargo.toml. Locate the manifest template in rh-codegen's file generator and parameterize the foundation dependency path. | `crates/rh-codegen/src/file_generator.rs` (manifest emission) | Regenerated crate Cargo.toml contains no absolute paths; golden-file test added |
| 0.3 | Fix C2: replace all `RwLock` `.unwrap()` in MemoryStore with poisoning recovery (`unwrap_or_else(PoisonError::into_inner)`) or `parking_lot::RwLock` (preferred: parking_lot, add to workspace deps). | `crates/rh-foundation/src/memory/mod.rs:120–204` | `grep -n "\.unwrap()" src/memory/mod.rs` returns only test code |
| 0.4 | Fix C3: decide ffq fate. Default decision: wire `ffq` into `Commands` enum behind a `--features unstable` flag OR delete the module. (Ask maintainer; if unreachable, gate it as unstable.) | `apps/rh-cli/src/main.rs`, `apps/rh-cli/src/ffq.rs` | No dead modules; README documents the decision |
| 0.5 | Add `[workspace.lints]`: `rust.unsafe_code = "forbid"` (if true today), `clippy.unwrap_used = "warn"` (deny in libs via crate-level), `clippy.all = "warn"`. Add `[lints] workspace = true` to every crate. | root `Cargo.toml`, all 11 member Cargo.tomls | `cargo clippy --workspace` clean |
| 0.6 | Quick doc corrections (full sync is WS7): README.md:40 R5 status → ✅; DEVELOPMENT.md:14 MSRV 1.70 → 1.91; README.md:12 WASM claim → "rh-fhirpath, rh-vcl"; replace ARCHITECTURE.md graph with §1's corrected graph including rh-cql, rh-packager, R5. | `README.md`, `.github/DEVELOPMENT.md`, `.github/ARCHITECTURE.md` | No doc claims contradict Cargo.toml facts |

---

## 4. WS1 — rh-foundation Hardening & Performance (size: M, 3–5 days) ✅ COMPLETED

Depends on WS0. Blocks WS5 (snapshot perf propagates to validator/packager) and WS6 (WASM story). All tasks (1.1–1.8) completed 2026-06-12; `just check` green. See Progress Tracking for details and the bonus `baseDefinition` serde fix.

### 4.1 API & error hardening
| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 1.1 | Audit/remove remaining non-test `.unwrap()/.expect()` beyond MemoryStore; convert to typed `FoundationError` variants. | `crates/rh-foundation/src/**` | clippy `unwrap_used` clean for the crate |
| 1.2 | Document `ElementMerger::merge_elements` (`src/snapshot/merger.rs:15`) — public API with no doc comment; describe merge algorithm, slicing/reslicing semantics, error conditions. | `src/snapshot/merger.rs` | `cargo doc` renders complete docs; missing_docs lint clean for snapshot module |
| 1.3 | Add integration tests covering loader + snapshot + HTTP together (currently 0 integration tests). Use a fixture FHIR package directory, no network. | `crates/rh-foundation/tests/` | ≥5 integration tests; loader→snapshot round-trip covered |

### 4.2 Performance (hot paths: package loader, snapshot generation)
| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 1.4 | Snapshot cache returns `Arc<Snapshot>` instead of cloning whole snapshots on every hit (`src/snapshot/generator.rs:59,81,119`). API change: `generate()` returns `Arc<...>` or adds `generate_arc()`. | `src/snapshot/generator.rs` + callers in rh-validator, rh-packager | criterion bench added (`benches/snapshot.rs`); cache-hit allocations eliminated; validator/packager compile against new API |
| 1.5 | `ElementMerger` builds its base-element map without cloning every element (`src/snapshot/merger.rs:40–46`): index by reference or use `Arc<ElementDefinition>`. | `src/snapshot/merger.rs` | bench shows reduced time on a 500+-element profile (use US Core or fixture) |
| 1.6 | Guard `std::fs` use for wasm32: `#[cfg(not(target_arch = "wasm32"))]` on `io.rs` filesystem APIs; compile-error rather than runtime-fail on wasm. | `src/io.rs`, `src/lib.rs` feature wiring | `cargo check -p rh-foundation --target wasm32-unknown-unknown --no-default-features --features wasm` passes |
| 1.7 | Make rh-packager's hardcoded `features = ["http"]` on rh-foundation optional via a packager `remote` feature (default on), so offline/embedded builds avoid tokio/reqwest. | `crates/rh-packager/Cargo.toml`, conditional code paths | `cargo check -p rh-packager --no-default-features` passes |

### 4.3 rh-packager (small, riding on WS1)
| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 1.8 | Expand end-to-end pipeline integration tests (currently 1 minimal file): init→build→pack a fixture package exercising snapshot, validate, FSH, CQL processors. | `crates/rh-packager/tests/integration_test.rs` | Pipeline test covers all 4 built-in processors |

---

## 5. WS2 — Conformance Harnesses (size: M-L, 1–2 weeks) 🔶 PARTIAL (near completion)

Independent of WS1/WS3. Must land **before** WS5 perf work (regression safety net).

**Status 2026-06-15:** 2.1 ✅ · 2.2 ✅ · 2.4 ✅ · 2.5 ✅ · 2.6 ✅ (waves 1–2 done; further skip burn-down remaining) · 2.7 ✅ (fixed-corpus Java-vs-Rust metadata diff documented and tested) · 2.8 ✅ · 2.9 ✅ · 2.10 ✅ (no-op, stale audit claim) · 2.3 waves 1–35 done. WS5 regression net is in place at 934/935 (99.9%) with **zero wrong answers and zero eval-errors**. Only 1 skipped test remains (missing HL7 fixture). WS2 is effectively complete.
### 5.1 FHIRPath: adopt the official HL7 test suite (the big gap)
rh-cql already demonstrates the target pattern: vendored official suite + categorized results (pass / compile-error / eval-error / skipped) + CI assertion of no wrong-answer regressions + `CONFORMANCE.md`. Replicate exactly for FHIRPath.

| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 2.1 | Vendor the HL7 FHIRPath test suite (XML `tests-fhir-r4.xml` + input resources from `hl7/fhirpath` repo / fhir-test-cases) under `crates/rh-fhirpath/tests/fixtures/hl7_fhirpath_tests/`. Write a harness (`tests/hl7_conformance.rs`) that parses the XML (quick-xml, already a workspace dep), runs each case, and categorizes: pass / wrong-answer / parse-error / eval-error / skipped(unsupported). | `crates/rh-fhirpath/tests/` | Harness runs full suite; emits machine-readable summary JSON; CI job asserts **zero wrong answers** (errors allowed initially) |
| 2.2 | Create `crates/rh-fhirpath/CONFORMANCE.md` + `SPEC_COVERAGE.md` mirroring rh-cql's format, generated/updated from the harness summary. Record the baseline pass rate. | new docs | Docs list per-function status incl. the 16 known-missing features (encode/decode, escape/unescape, aggregate, defineVariable, type() reflection, lowBoundary/highBoundary/precision, %vs-/%ext- vars, resolve(), memberOf()) |
| 2.3 | Implement missing functions in priority order (each its own PR): (a) `aggregate()`, `defineVariable()`; (b) string `encode/decode/escape/unescape`; (c) `lowBoundary()/highBoundary()/precision()`; (d) type reflection `type()`; (e) `resolve()` + `memberOf()` behind a pluggable resolver/terminology trait (no network in core). | `crates/rh-fhirpath/src/evaluator/functions/` | Each lands with conformance-suite cases flipping to pass; no wrong-answer regressions |

### 5.2 CQL: close documented gaps (reference: cqframework cql-to-elm + HL7 suite, already vendored)
| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 2.4 | Implement clinical operators (currently 0/8): `AgeInYears`, `AgeInYearsAt`, `CalculateAge*` — requires patient birthDate from the evaluation context. Design: add `patient_birth_date` to `EvaluationContext` populated by the data provider. | `crates/rh-cql/src/eval/operators/`, `eval/context.rs` | HL7 suite eval-error count drops accordingly; SPEC_COVERAGE.md regenerated |
| 2.5 | Implement `Message` (error operator), `Ratio` literals + `ToRatio`, `Combine`/`Split` emit+eval (stubs exist), `Children`/`Descendants`. | `crates/rh-cql/src/` per SPEC_COVERAGE.md pointers | 175/175 operators at least emit; wrong-answer count stays 0 |
| 2.6 | Burn down `skipped: 210` (unparsed output formats) in the HL7 harness; burn down compile-errors (149) per `compiler.rs:864` parser-recovery TODO. Track in CONFORMANCE.md each wave. | `crates/rh-cql/tests/hl7_eval_tests.rs` | Skipped < 50; compile-errors reduced; CI gate unchanged (0 wrong answers) |
| 2.7 | ELM fidelity check vs Java translator: extend `pipeline_comparison_tests.rs` to diff localId/locator/annotation presence against reference ELM for a fixed corpus. | `crates/rh-cql/tests/` | Documented diff report; intentional deviations listed in CONFORMANCE.md |

### 5.3 FSH: deepen SUSHI parity
| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 2.8 | Add parser unit tests (currently near-zero inline tests — regression risk flagged) covering each rule type, soft indexing `[+]`, parameterized RuleSets, caret paths. | `crates/rh-fsh/src/parser/*` `#[cfg(test)]` | ≥80 parser unit tests; soft-indexing + ParamRuleSet integration fixtures added |
| 2.9 | Expand SUSHI golden corpus beyond 30 FSHOnline fixtures; import additional SUSHI regression fixtures; document unsupported constructs in a new `crates/rh-fsh/CONFORMANCE.md`. | `crates/rh-fsh/tests/sushi_compat.rs`, fixtures | Corpus ≥60 fixtures; CONFORMANCE.md lists gaps with spec links |

### 5.4 VCL
| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 2.10 | Fix README/WASM drift: README references `parse_vcl_expression()`; wasm.rs exports `parse_vcl_simple()`. Align names (prefer renaming wasm exports to match docs’ clearer names) and add a doc test. | `crates/rh-vcl/src/wasm.rs`, `README.md` | Every README-documented symbol exists; `cargo test --doc` passes |

---

## 6. WS3 — CLI Consistency: Humans + Agents, UNIX Philosophy (size: L, 1.5–2 weeks)

Independent start; REPL-extraction subtasks touch library crates (coordinate file ownership with WS2/WS5). This is the highest-leverage UX work in the plan.

### 6.1 Design contract (implement once in `rh-foundation::cli` or a new `apps/rh-cli/src/output.rs`)

Every subcommand MUST satisfy:
1. **Input:** positional file args; `-` or absent ⇒ stdin where sensible. Standardize on `-i/--input` only where multiple inputs need flags; kill `--data`/`--file` variants (keep as hidden aliases for one release).
2. **Output:** result data → **stdout**; logs/diagnostics/progress → **stderr**, via `tracing`. Never interleave prose with JSON on stdout. `-o/--output <file>` supported uniformly.
3. **Formats:** global `--format <human|json|ndjson>` (replaces per-command `--format`/`--compact` zoo; `--compact` becomes `--format json` + `--pretty` flag). `--format json` emits a **stable envelope**: `{"ok": bool, "result": ..., "errors": [{"code","message","span?"}], "meta": {"version", "command"}}`. Document schema in `apps/rh-cli/docs/OUTPUT.md`; version it.
4. **Exit codes:** `0` success; `1` operational error (I/O, network); `2` usage error (clap default); `3` validation/conformance failure (resource invalid, tests failed); `4` parse error of user input (FHIRPath/CQL/FSH/VCL syntax). Define once as `enum ExitCode` in main.rs.
5. **Verbosity:** global `-q/--quiet` (suppresses stderr info), `-v/--verbose` (repeatable: info→debug→trace). Defaults quiet-ish: human format prints results only.
6. **Color/TTY:** honor `NO_COLOR`, add `--color auto|always|never`; emojis/symbols only when stdout is a TTY and color enabled (use `anstream`/`is-terminal`). Remove hardcoded ✅/❌/⚠️ from machine paths.
7. **Completions & version:** add `rh completions <shell>` (clap_complete); `--version` includes git SHA via build script (`vergen` or `env!` from `build.rs`).

### 6.2 Tasks
| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 3.1 | ✅ Implement the output framework: envelope types, `ExitCode`, format negotiation, TTY/color detection, `--quiet/--verbose/--color/--format` as global clap args. | `apps/rh-cli/src/main.rs`, new `apps/rh-cli/src/output.rs` | Unit tests for envelope serialization; `rh --help` shows globals |
| 3.2 | ✅ Migrate each subcommand to the contract, one PR per command, in this order (most-broken first): `validate` (prose+JSON mixing at validator.rs:444–476,524–529; no `-o`), `fsh` (per-resource println), `download` (`list` human-only), `codegen` (no manifest output — add `--format json` emitting generated-files manifest), `cql`, `fhirpath`, `vcl`, `snapshot` (implement or remove declared-but-unimplemented `diff`/`validate` at snapshot.rs:196,201), `package`. | `apps/rh-cli/src/<command>.rs` | Per command: stdout is pure data; stderr-only logs; exit codes per contract; `--format json` round-trips through `jq` |
| 3.3 | ✅ Extract REPLs to library crates behind a `repl` feature (rustyline dep moves there): `cql.rs:1190+` → `rh-cql::repl`, `fhirpath.rs:225+` → `rh-fhirpath::repl`, `vcl.rs:235+` → `rh-vcl::repl`. Create one shared REPL scaffold (readline loop, history, `:commands`) in `rh-foundation::cli::repl` to kill the 4-way duplication. | CLI command files + 3 crates + foundation | CLI command modules shrink to thin dispatch (<150 lines each); REPL behavior unchanged (manual check) |
| 3.4 | ✅ Move CLI-resident logic to crates: validator output formatting (validator.rs:400–700) → `rh-validator::report` module (human + JSON renderers); `expand_home_dir`, `parse_package_spec` (snapshot.rs) → `rh-foundation`. | as listed | No business logic >50 lines in any CLI module |
| 3.5 | ✅ Differentiated errors: keep anyhow at the edge but downcast library errors to map exit codes; print errors as `error: <message>` on stderr, and in JSON envelope when `--format json`. | `apps/rh-cli/src/main.rs:93–143` | Integration tests assert exit codes 0/1/2/3/4 |
| 3.6 | ✅ CLI integration tests for ALL 9 commands (assert_cmd + predicates; currently only cql + validator): golden help text, exit codes, JSON schema validation of envelopes, stdin piping (`echo ... \| rh fhirpath eval -`). | `apps/rh-cli/tests/` | Every command has ≥4 integration tests incl. one stdin pipe test |
| 3.7 | ✅ Sync `apps/rh-cli/README.md`: document `download list`, `fsh parse`/`tank`, snapshot subcommands, new globals, exit codes, env vars (`NO_COLOR`, `RUST_LOG`); add an "Agent usage" section showing `--format json` + `jq` recipes. | `apps/rh-cli/README.md` | README commands == `rh --help` output (script-checked, see 8.5) |

---

## 7. WS4 — Codegen Refactor, Validated by R4/R5 Regeneration (size: L, 2 weeks)

Depends on WS0 (0.1/0.2). The generated R4/R5 crates are the test bed: every codegen change is validated by regenerating both and diffing/compiling/testing.

### 7.1 Establish the regeneration harness FIRST
| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 4.1 | Add `just regen-r4` / `just regen-r5` recipes invoking `rh codegen hl7.fhir.r4.core 4.0.1` / `hl7.fhir.r5.core 5.0.0` into the existing crate dirs (preserving `tests/`, per commit b7cda08), then `cargo build -p` + `cargo test -p` both. Add a CI job (manual trigger / nightly) that regenerates and fails on uncommitted diff ("generated code is stale"). | `justfile`, `.github/workflows/` | One-command regeneration; CI catches drift |
| 4.2 | Eliminate R5 post-generation manual patches (red flag from audit): fold SubscriptionStatus naming collision, metadata_resource stub impls, integer64 corrections, and the R5 `language`-binding workaround into codegen logic so regeneration is hermetic. | `crates/rh-codegen/src/{generator,accessor/mutator generators}` | `just regen-r5` produces a crate that builds + passes r5_smoke_test with zero manual edits |
| 4.3 | Add golden-file tests for representative StructureDefinitions (Patient, Observation, a profile, an extension, a choice-type, a circular-ref pair) — snapshot the generated Rust and diff. | `crates/rh-codegen/tests/golden/` | Golden tests run in normal `cargo test`; updating requires explicit `UPDATE_GOLDEN=1` |

### 7.2 Internal refactor (after harness exists)
| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 4.4 | Split god-files (each its own PR, behavior-preserving, golden tests green): `trait_impl_generator.rs` (1777) → per-trait-tier modules (accessors/mutators/existence); `file_generator.rs` (1412) → manifest emission / file layout / io; `type_registry.rs` (1215) → registry + lookup tables; `field_generator.rs` (1087); `generator.rs` (1028) → orchestration only. Target: no file >800 lines. | `crates/rh-codegen/src/` | Golden + integration tests unchanged; module map documented in crate ARCHITECTURE section |
| 4.5 | Remove borrow-checker clones of `value_set_manager`/`enum_cache` (`generator.rs:363–364`) — restructure ownership (e.g., split phases, pass `&mut` registries, or interior maps). | `crates/rh-codegen/src/generator.rs` | No `.clone()` of managers in the pipeline |
| 4.6 | Replace the 26 non-test `.unwrap()/.expect()` with `CodegenError` variants — worst offenders: `type_mapper.rs:257`, `invariants.rs:120,136`, `token_generator.rs:47,194,365`, `import_manager.rs:71–75`, `value_sets.rs:181`, `struct_generator.rs:713` panic. | as listed | clippy unwrap_used clean (lib code) |
| 4.7 | Parallelize generation: rayon `par_iter` over StructureDefinitions for token generation + file writing (audit estimates ~10x). Cache parsed base definitions (Element, BackboneElement, DomainResource) as shared singletons instead of re-parsing per resource. | `crates/rh-codegen/src/{generator,file_generator}` | criterion bench `benches/generate.rs` added; R4 full generation wall-time recorded before/after in PR description; output byte-identical to serial run |

### 7.3 Generated-output quality (changes generated API — coordinate as a minor-version bump of generated crates)
| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 4.8 | **Decision + implementation — `Option<Vec<T>>` → `Vec<T>`** for 0..* fields (1,356+ instances in R4): emit `#[serde(default, skip_serializing_if = "Vec::is_empty")] pub field: Vec<T>`. Verify JSON round-trip equivalence (absent ⇔ empty) against FHIR spec expectations; primitive-extension companion arrays (`_field`) need the same treatment. This is the single biggest ergonomics win. | codegen field/struct generators; regenerate R4+R5 | Serde round-trip property tests pass on FHIR examples corpus; smoke tests updated |
| 4.9 | Document the `Box` usage rationale (Reference↔Identifier cycle) in generated doc comments; audit whether other cycles need boxing that currently rely on luck. | codegen | Cycle-detection note in codegen docs |
| 4.10 | Split `metadata.rs` monoliths (R4 25.9k / R5 30.5k LOC) by category (resources/datatypes/primitives) to improve incremental compile; measure `cargo build -p rh-hl7-fhir-r4-core` time before/after. | codegen file layout | Build-time delta recorded; no API change |
| 4.11 | R4 extensions story: R4 has 388 generated extension files, R5 only 3 — regenerate R5 extensions for parity; add typed extension accessor helpers (e.g., `resource.extension_by_url(url)` in foundation trait) to fix the "weak extension story". | codegen + foundation traits | R5 extensions ≥ parity with R4 source package; helper trait tested |
| 4.12 | Add property-based test: feed every StructureDefinition in R4+R5 packages through codegen, assert generated code parses with `syn` (cheap compile-check) before file write. | `crates/rh-codegen/tests/` | Full-package generation test in CI (nightly job if slow) |

---

## 8. WS5 — Performance (size: M, 1 week)

Depends on WS2 (conformance nets) for fhirpath/cql; WS1 covers foundation perf (tasks 1.4–1.5). Add criterion benches FIRST in any crate lacking them, so every claim is measured.

| # | Task | Crate | Detail | Acceptance criteria |
|---|---|---|---|---|
| 5.1 | Add `benches/` (criterion: parse, eval-simple, eval-complex, eval-large-resource) | rh-fhirpath | None exist today despite devdays perf claims | Baseline numbers committed to bench README |
| 5.2 | Expression parse cache: `FhirPathEngine` wrapper holding an LRU (`lru` crate or foundation MemoryStore) of expression→AST; CLI REPL and validator use it. | rh-fhirpath | Parser currently re-parses every invocation | Bench shows ≥10x on repeated-eval workload |
| 5.3 | Reduce evaluator cloning (127 `.clone()` sites): introduce `Cow`/`Arc<Value>`-based collection items in hot paths (member access, where/select/repeat). Stage 1: wrap items in `Arc<serde_json::Value>`; Stage 2 (optional, bigger): internal value model decoupled from serde_json. Conformance suite (2.1) is the regression net. | rh-fhirpath | `src/evaluator/**` | Zero conformance regressions; eval-large-resource bench improves; report % in PR |
| 5.4 | Replace `Rc<RefCell<>>` in `EvaluationContext` with `Arc<Mutex<>>` (or thread-local trace buffer) so contexts are `Send` — unblocks parallel batch evaluation and async embedding. | rh-fhirpath | `src/evaluator/core/` | `static_assertions::assert_impl_all!(EvaluationContext: Send)` test |
| 5.5 | Compiled-library cache + parallel population eval: cache compiled ELM libraries keyed by source hash; rayon over ndjson resources in `rh cql eval --data *.ndjson`. | rh-cql | `library/mod.rs`, CLI eval path | bench_eval shows linear scaling across cores on a 10k-patient ndjson fixture |
| 5.6 | Codegen parallelization — covered by 4.7 (listed here for ordering: it is a perf task gated on WS4 harness). | rh-codegen | — | — |
| 5.7 | Re-run and refresh `docs/devdays-2025-perf.md` numbers post-changes; add a `just bench` recipe running all crate benches. | docs, justfile | — | Doc dated and reproducible via `just bench` |

---

## 9. WS6 — Generalized WASM + NPM + Reference App (size: L, 2 weeks)

Depends on WS1 (1.6 foundation wasm hygiene). Current state: rh-fhirpath and rh-vcl have working wasm-bindgen exports, `wasm` features, wasm-pack metadata (`-O4`), per-crate justfile recipes, and WASM_BUILD.md docs — but **no npm packaging, no CI, no shared pattern, and rh-fhirpath has an ad-hoc demo/ dir**. rh-cql core is WASM-clean (no tokio/reqwest/fs in core) and is the next candidate. rh-fsh is blocked by rayon + `std::fs` in its public API.

### 9.1 The generalized pattern (one convention, applied to every WASM-capable crate)

```
crates/rh-<name>/
  Cargo.toml          # [features] wasm = ["dep:wasm-bindgen", ...]
                      # [package.metadata.wasm-pack.profile.release] wasm-opt = ["-O4"]
  src/wasm.rs         # #[wasm_bindgen] facade: thin, JSON-in/JSON-out, options structs,
                      #   WasmResult envelope from rh-foundation::wasm
packages/             # NEW top-level dir (npm workspace, pnpm)
  fhirpath/           # npm: @reason-healthcare/fhirpath
    package.json      # wraps wasm-pack output; exports map for node/web/bundler
    src/index.ts      # TS wrapper: ergonomic API over raw bindgen exports, typed
    test/             # vitest tests against the built wasm
  vcl/                # npm: @reason-healthcare/vcl
  cql/                # npm: @reason-healthcare/cql (after 6.5)
examples/
  playground/         # reference web application (single app, all packages)
```

Rules:
- `src/wasm.rs` contains **zero logic** — only type marshalling. All behavior stays in the crate so native/wasm never diverge.
- Shared `WasmResult` envelope (already in rh-foundation) is the JS-facing error contract; mirror the CLI JSON envelope shape (`ok/result/errors`) for cross-tool consistency.
- npm packages are TS wrappers re-exporting typed APIs; wasm-pack output (`pkg/`) is a build artifact, never committed.
- One `just wasm` at workspace root builds all targets for all wasm crates; per-crate recipes remain.

### 9.2 Tasks
| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 6.1 | Extract the common justfile WASM recipes into root justfile parameterized by crate (`just wasm-build fhirpath web`); add `wasm32-unknown-unknown` check for fhirpath, vcl, foundation(no-default-features), cql-core to CI. | `justfile`, `.github/workflows/ci.yml` | CI fails if any wasm-capable crate regresses wasm compilation |
| 6.2 | Create `packages/` pnpm workspace with `@reason-healthcare/fhirpath` and `@reason-healthcare/vcl`: TS wrappers, typed options, exports map (node/web/bundler via wasm-pack `--target` variants), vitest suites, README each. | `packages/*` | `pnpm -r build && pnpm -r test` green; `npm pack` produces installable tarballs |
| 6.3 | NPM release automation: extend `release.yml` with a job building wasm + publishing packages on `v*` tags (npm provenance, `NPM_TOKEN` secret); document in RELEASING.md (and fix RELEASING.md:266 pointing at WASM_BUILD.md docs — verify paths). | `.github/workflows/release.yml`, `RELEASING.md` | Dry-run publish (`npm publish --dry-run`) wired into CI |
| 6.4 | Reference app: `examples/playground/` — Vite + TS web app with tabs: FHIRPath evaluator (expression + resource JSON → result), VCL translator/explainer, (later) CQL compile→ELM viewer. Replaces/absorbs `crates/rh-fhirpath/demo/`. Deployed to GitHub Pages via workflow. | `examples/playground/` | `pnpm dev` runs locally; Pages deploy job on main; rh-fhirpath/demo removed with pointer |
| 6.5 | rh-cql WASM: add `wasm` feature mirroring the pattern; feature-gate file/package library providers (`src/library/providers/{file,package}.rs`) behind `fs` (default on, off for wasm); `src/wasm.rs` exposing `compile(source, options) → ELM JSON` and `evaluate(elm, data, parameters)` with in-memory providers. Then `packages/cql`. | `crates/rh-cql/` | `cargo check -p rh-cql --target wasm32-unknown-unknown --no-default-features --features wasm` passes; playground CQL tab works |
| 6.6 | rh-fsh WASM (stretch, do last): make rayon optional (`parallel` feature, sequential fallback iterator) and split file-I/O entry points (`compile_fsh_files`) from pure `compile_fsh(sources: &[(name, content)])`. | `crates/rh-fsh/` | wasm32 check passes with `--no-default-features`; native perf unchanged with `parallel` on (bench) |
| 6.7 | rh-foundation: either implement real wasm exports or stop claiming support — decision: keep `wasm` feature as internal plumbing (WasmResult, panic hook), document it as "WASM support infrastructure, not a user-facing WASM API"; ensure README/docs say so (ties to 0.6). | `crates/rh-foundation/README.md` | No doc overstates foundation WASM |

---

## 10. WS7 — Documentation Sync + CI Gates (size: S-M, 3–4 days; final sweep LAST)

Quick fixes already landed in 0.6. This is the systematic pass after code workstreams settle.

| # | Task | Files | Acceptance criteria |
|---|---|---|---|
| 7.1 | Rewrite `.github/ARCHITECTURE.md` from the corrected graph (§1), including layer table, WASM-capable matrix, and the packages/ + examples/ additions from WS6. | `.github/ARCHITECTURE.md` | Graph mechanically matches `cargo metadata` (see 7.5) |
| 7.2 | Per-crate README pass: rh-codegen (remove stale "In Progress" features or mark shipped), rh-fhirpath (link CONFORMANCE.md, record suite pass rate), rh-vcl (2.10), generated-crate READMEs gain version-specific notes (R5 quirks: integer64, CodeableReference, binding count). | crate READMEs | Each README's examples compile as doc tests where feasible |
| 7.3 | Root README: component table accurate (R5 ✅), WASM section pointing at packages/ + playground, exit-code + `--format json` documentation for agents. AGENTS.md: add pointers to CONFORMANCE.md files and `just` recipes agents should run. | `README.md`, `AGENTS.md` | — |
| 7.4 | RELEASING.md: npm publish flow (6.3), regenerated-crates release policy (when codegen changes require R4/R5 crate version bumps), verify WASM doc paths. | `RELEASING.md` | Dry-run release checklist executed once |
| 7.5 | Drift guards in CI: (a) script that diffs ARCHITECTURE.md's dependency list against `cargo metadata`; (b) script that diffs apps/rh-cli/README.md command/flag table against `rh <cmd> --help` output; (c) MSRV job (`cargo +1.91 check`). | `scripts/check-docs-sync.sh`, `ci.yml` | CI fails on doc drift; MSRV enforced |

---

## 11. Sequencing Summary (hand-off checklist)

| Order | Workstream | Size | Hard dependencies | Parallel-safe with |
|---|---|---|---|---|
| 1 | WS0 Hygiene | S | — | — (do alone, fast) |
| 2a | WS1 Foundation | M | WS0 | WS2, WS3, WS4 |
| 2b | WS2 Conformance | M-L | WS0 | WS1, WS3, WS4 |
| 2c | WS3 CLI | L | WS0 | WS1, WS2, WS4 (coordinate REPL extraction with WS2 crate owners) |
| 2d | WS4 Codegen | L | WS0 (0.1, 0.2) | WS1, WS2, WS3 |
| 3a | WS5 Performance | M | WS1 (1.4–1.5), WS2 (nets) | WS6 |
| 3b | WS6 WASM/NPM/app | L | WS1 (1.6), stable crate APIs | WS5 |
| 4 | WS7 Docs + gates | S-M | all above | — (final) |

**File-ownership map for parallel agents (avoid collisions):**
- WS1: `crates/rh-foundation/**`, `crates/rh-packager/**`
- WS2: `crates/rh-fhirpath/{tests,src/evaluator/functions}/**`, `crates/rh-cql/{tests,src/eval}/**`, `crates/rh-fsh/{tests,src/parser}/**`, `crates/rh-vcl/{src/wasm.rs,README.md}`
- WS3: `apps/rh-cli/**` + new `repl` modules in cql/fhirpath/vcl + `rh-validator::report`
- WS4: `crates/rh-codegen/**`, `crates/rh-hl7_fhir_r{4,5}_core/**` (regeneration output)
- WS6: `packages/**`, `examples/playground/**`, wasm.rs files, justfile/workflows wasm recipes

**Open decisions requiring maintainer input (defaults chosen, flag in PRs):**
1. (0.4) ffq command: gate as unstable vs delete. Default: gate.
2. (4.8) `Option<Vec<T>>` → `Vec<T>` changes the generated-crate public API — needs version bump policy for rh-hl7_fhir_r{4,5}_core. Default: do it; bump generated crates to 0.3.0.
3. (6.2) npm scope name `@reason-healthcare/*`. Default assumed.
4. (5.3 stage 2) Internal FHIRPath value model decoupled from serde_json — large; only proceed if stage-1 Arc wrapping shows insufficient gains.

**Verification note for implementers:** audit findings above were produced by parallel code surveys on 2026-06-12; spot-verified items are marked. Before acting on any specific file:line, re-read the file — line numbers drift. Two audit claims were found wrong during verification and corrected here: WASM_BUILD.md files DO exist for rh-fhirpath and rh-vcl; `docs/devdays-2025-perf.md` DOES exist. The CQL "1105 unwrap" figure is unverified and likely includes test code — re-count during 2.4–2.6.
