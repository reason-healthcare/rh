## Why

`rh-fsh` now parses the mCODE and Da Vinci DTR FSH corpora end-to-end, but project-level SUSHI comparison still shows material output differences. Closing these gaps will move `rh-fsh` from fixture compatibility toward practical IG compilation parity and provide a repeatable conformance baseline for future work.

## What Changes

- Add a tracked project-level SUSHI conformance capability with stable comparison metrics, gap categorization, and documented acceptance thresholds.
- Improve instance export so profile instances consistently emit the underlying FHIR base resource type and correct reference targets, including profiles from dependency packages.
- Improve FHIR JSON shape fidelity for arrays, primitives, complex datatypes, nested extensions, contained resources, bundles, and examples.
- Improve StructureDefinition export for constraints, extension contexts, base definitions, root cardinality, and differential shape.
- Improve terminology and metadata export for CodeSystem `count`, canonical URLs, definition-instance metadata, and generated `ImplementationGuide` resources.
- Extend tank/package resolution so external package StructureDefinitions are available for profile parent resolution rather than relying on name heuristics.
- Keep project comparisons non-blocking until explicit thresholds are met; no breaking CLI/API changes are intended.

## Capabilities

### New Capabilities
- `fsh-sushi-conformance`: Project-level SUSHI comparison, reporting, gap categorization, and acceptance thresholds for real IG corpora.

### Modified Capabilities
- `fsh-export`: Export requirements expand from fixture-level output to SUSHI-compatible resource identity, metadata, StructureDefinition, datatype, extension, and instance JSON shapes for project corpora.
- `fsh-tank`: Resolution requirements expand to include dependency package StructureDefinition lookup for profile parent/base resource resolution.
- `fsh-cli`: CLI behavior requirements expand to support conformance-friendly compile output and safe deterministic file writing for generated resources.

## Impact

- Affected crates: `crates/rh-fsh`, `apps/rh-cli`, and possibly `crates/rh-foundation` package loading utilities.
- Affected docs/tests: `crates/rh-fsh/CONFORMANCE.md`, `crates/rh-fsh/conformance/**`, OpenSpec FSH specs, and focused Rust regression tests.
- External tooling: project comparison requires `git`, Node.js, `npx`, and `fsh-sushi`.
- Risk: dependency package loading and richer JSON shape handling may touch shared compile/export paths, so work should be staged with project-level and fixture-level regression checks after each wave.
