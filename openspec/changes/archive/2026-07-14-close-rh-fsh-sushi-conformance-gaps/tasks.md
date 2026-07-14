## 1. Conformance Baseline and Reporting

- [x] 1.1 Extend `run_sushi_comparison.py` to categorize differences into resource identity, JSON shape, StructureDefinition, metadata, terminology, and IG-generation buckets.
- [x] 1.2 Add per-project threshold configuration to the runner and include pass/fail threshold status in JSON and Markdown reports.
- [x] 1.3 Add a small fixture or smoke mode that exercises dependency/profile resource identity without requiring full mCODE/DTR runs.
- [x] 1.4 Update `CONFORMANCE.md` with categorized baseline counts before implementation waves begin.
- [x] 1.5 Add opt-in retained raw and normalized pairwise artifacts for semantic diff analysis.

## 2. Dependency and Profile Resolution

- [x] 2.1 Parse or ingest `sushi-config.yaml` dependency declarations for project compiles without breaking library callers that provide `FshConfig` directly.
- [x] 2.2 Reuse package cache/loading utilities to load dependency package StructureDefinitions by package id and version.
- [x] 2.3 Build a definition index keyed by FSH name, id, canonical URL, and alias-expanded references for local and dependency StructureDefinitions.
- [x] 2.4 Replace external profile name heuristics in instance export with dependency-backed base resource resolution.
- [x] 2.5 Add regression tests for US Core, CRD, SDC, genomics, and mCODE profile instances resolving to `Patient`, `Practitioner`, `Questionnaire`, `ServiceRequest`, `Coverage`, `Observation`, and `DiagnosticReport`.
- [x] 2.6 Rerun the project comparison and record the missing/extra reduction for mCODE and DTR.

## 3. Instance JSON Shape Fidelity

- [x] 3.1 Refactor path assignment into a shared writer that uses FHIR field metadata and resolved profile element metadata for array/scalar decisions.
- [x] 3.2 Fix CodeableConcept, Coding, code, canonical, Reference, and Quantity conversion to match SUSHI output for known target field types.
- [x] 3.3 Implement primitive extension shadow-field output for `_field` arrays and scalar primitive companions.
- [x] 3.4 Implement nested extension writing with required `extension[]`, `url`, and `value[x]` wrappers.
- [x] 3.5 Fix contained resource and Bundle entry instance embedding so embedded resource `resourceType`, references, `fullUrl`, and ids match SUSHI.
- [x] 3.6 Add regression fixtures for representative DTR `Parameters`, `Questionnaire`, `QuestionnaireResponse`, `Bundle`, and mCODE medication dosage examples.
- [x] 3.7 Rerun project comparison and update `CONFORMANCE.md` with JSON shape gap changes.

## 4. StructureDefinition Export Fidelity

- [x] 4.1 Emit root differential elements with SUSHI-compatible min/max, constraints, metadata, and invariant source fields.
- [x] 4.2 Export `obeys` rules as full ElementDefinition `constraint` entries with key, severity, human, expression, xpath when present, and source.
- [x] 4.3 Export Extension `context` and context-invariant caret metadata in SUSHI-compatible shape.
- [x] 4.4 Fix baseDefinition/type/derivation for profiles, extensions, logical models, resources, and dependency-derived parents.
- [x] 4.5 Add regression tests for DTR `DTR-QPackageBundle`, `DTR-QRBundle`, `DTRMetricData`, and extension context examples.
- [x] 4.6 Rerun project comparison and update StructureDefinition mismatch counts.

## 5. Project Metadata, Terminology, and CLI Output

- [x] 5.1 Generate `ImplementationGuide/<id>` resources from normalized project configuration when compiling an IG project.
- [x] 5.2 Emit CodeSystem `count` values matching SUSHI for flat and hierarchical concept systems.
- [x] 5.3 Fix canonical `url`, `version`, `status`, `publisher`, `title`, and definition-instance metadata where SUSHI derives values from project config.
- [x] 5.4 Sanitize `rh fsh compile --output` filenames so resource ids or canonical URLs containing slashes cannot cause write failures.
- [x] 5.5 Include non-fatal compile/export diagnostics in `--format json` output so conformance runs can compare partial output with warnings.
- [x] 5.6 Add CLI tests for safe filename generation and stable JSON envelope output.

## 6. Verification and Acceptance

- [x] 6.1 Run `cargo test -p rh-fsh` and targeted `rh-cli` FSH integration tests after each implementation wave.
- [x] 6.2 Run `python3 crates/rh-fsh/conformance/scripts/run_sushi_comparison.py --project davinci-dtr --timeout-seconds 600` until DTR reaches the configured interim threshold.
- [x] 6.3 Run full project comparison for all configured projects and update `CONFORMANCE.md` with latest results and remaining categorized gaps.
- [x] 6.4 Add a non-blocking CI artifact upload for the project comparison summary report.
- [x] 6.5 Tighten thresholds only after DTR and mCODE results are deterministic across two consecutive local runs.
