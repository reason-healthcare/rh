## Context

The project-level SUSHI runner currently compares `rh-fsh` against real IG source for CARIN Blue Button, mCODE, Da Vinci CRD, Da Vinci DTR, Da Vinci PAS, and IPS. The latest baseline shows:

- CARIN Blue Button: 134 SUSHI resources vs 133 `rh-fsh` resources; 23 missing, 22 extra, 111 mismatched.
- mCODE: 350 SUSHI resources vs 349 `rh-fsh` resources; 1 missing, 0 extra, 328 mismatched.
- Da Vinci CRD: 85 SUSHI resources vs 85 `rh-fsh` resources; 8 missing, 8 extra, 73 mismatched.
- Da Vinci DTR: 75 SUSHI resources vs 79 `rh-fsh` resources; 1 missing, 5 extra, 63 mismatched.
- Da Vinci PAS: 158 SUSHI resources vs 160 `rh-fsh` resources; 15 missing, 17 extra, 141 mismatched.
- IPS: 118 SUSHI resources vs 214 `rh-fsh` resources; 1 missing, 97 extra, 116 mismatched.

The initial parser blocker is closed: all mCODE/DTR FSH files parse individually and the full compile reaches export. Remaining gaps cluster around dependency profile resolution, resource identity, JSON shape fidelity, StructureDefinition detail, and project metadata/IG output.

## Goals / Non-Goals

**Goals:**

- Reduce mCODE and DTR missing/extra resource identity gaps by resolving profile base resources from local and dependency StructureDefinitions.
- Bring instance JSON shape closer to SUSHI for arrays, complex datatypes, primitive extension shadow fields, nested extensions, bundles, contained resources, quantities, codings, references, and canonicals.
- Bring StructureDefinition export closer to SUSHI for root elements, constraints, extension contexts, base definitions, differential metadata, and caret-derived metadata.
- Generate the project-level metadata resources SUSHI emits where they are required for comparison, especially `ImplementationGuide`.
- Maintain a reproducible conformance report with pass/fail thresholds and categorized gap summaries.

**Non-Goals:**

- Full IG Publisher parity. This plan targets SUSHI FSH output, not publisher rendering, validation, snapshots, or narrative generation.
- Arbitrary FHIR version support beyond the current R4-focused compiler path.
- Blocking CI on complete project parity immediately. Thresholds should tighten as gap waves land.
- Replacing the existing parser/resolver/exporter architecture wholesale.

## Decisions

### 1. Use SUSHI comparison as the primary planning metric

The project runner is the only tool currently measuring the real production gap. Keep it as the source of truth, but enhance reports with categorized differences so implementation work can be prioritized by root cause.

Alternative considered: rely only on focused Rust unit tests. Unit tests are still required for each fix, but they do not reveal cross-file, dependency, and project metadata interactions.

### 2. Load dependency StructureDefinitions instead of expanding heuristics

Current external profile resolution uses name heuristics such as recognizing `profile-servicerequest` as `ServiceRequest`. That reduces some missing/extra pairs but is not reliable enough for US Core, CRD, SDC, genomics, and mCODE dependency profiles.

The tank/compiler should resolve dependency packages declared in `sushi-config.yaml` or passed through compiler config, then index StructureDefinitions by name, id, URL, and type. Instance export should use those definitions to derive the true base resource type and canonical profile metadata.

Alternative considered: add more string heuristics. This is faster short term but creates fragile false positives and will not fix metadata, targetProfile, or baseDefinition mismatches.

### 3. Centralize FHIR JSON shape decisions

Many mismatches are arrays vs scalars, Coding vs CodeableConcept, Quantity field shape, primitive extension shadow fields, and nested extension paths. These should be handled through a shared path/type application layer backed by FHIR metadata and profile element summaries, not ad hoc per-resource patches.

Alternative considered: special-case common resources such as CapabilityStatement and Parameters. This can be useful as a stopgap for isolated bugs, but broad project parity needs a consistent path writer.

### 4. Stage implementation by gap category

The tasks should run in waves:

1. Categorize and stabilize the report.
2. Dependency/profile identity resolution.
3. Instance JSON shape fidelity.
4. StructureDefinition fidelity.
5. Metadata/IG/terminology polish.
6. CI thresholds and docs.

This keeps each wave measurable and avoids mixing structural fixes with lower-priority metadata differences.

## Risks / Trade-offs

- Dependency package loading may introduce network/cache variability -> Use local FHIR package cache first, pin versions from `sushi-config.yaml`, and report missing packages explicitly.
- Richer path writing may change existing fixture output -> Keep existing fixture tests and add targeted SUSHI-derived regression fixtures before changing shared writers.
- Complete SUSHI parity is broad -> Define interim acceptance thresholds by project and category instead of requiring zero differences immediately.
- SUSHI emits project metadata from `sushi-config.yaml` -> `rh-fsh` currently accepts limited config, so config ingestion must be designed carefully and remain optional for library callers.
- Report normalization can hide real bugs -> Keep normalization limited to volatile fields and document every ignored path.

## Migration Plan

1. Keep the existing fixture tests as the fast local guard.
2. Add categorized project-report output without changing compiler behavior.
3. Land each implementation wave with before/after conformance numbers in `CONFORMANCE.md`.
4. Only tighten CI or local gate thresholds after the project runner is stable and dependency resolution is deterministic.
5. If a wave regresses fixture output unexpectedly, revert that wave and keep the categorized report for diagnosis.

## Open Questions

- Should `rh-fsh` read `sushi-config.yaml` directly, or should config parsing live in CLI/packager and pass normalized `FshConfig` into the compiler?
- Which package cache abstraction should `rh-fsh` reuse: `rh-foundation` package loading directly or a narrower adapter to avoid a heavy dependency path?
- What exact interim thresholds are acceptable for CI: DTR-only first, mCODE smoke first, or non-blocking report artifact only?
