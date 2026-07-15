## Why

`rh-fsh` now produces every expected resource identity for the pinned SUSHI
corpus, but 179 resources still first differ in JSON shape. These differences
are concentrated in recursive extension, Bundle, Parameters, and repeating
BackboneElement paths, where a shared schema-driven writer can remove entire
classes of project-specific mismatches.

## What Changes

- Make recursive schema metadata the single source of array/scalar and datatype
  shape decisions for normal, contained, inline, Bundle, and Parameters
  instance assignments.
- Define deterministic precedence between explicit assignments, local profile
  differentials, dependency-derived defaults, and core FHIR defaults.
- Complete JSON serialization for nested extensions and primitive shadow fields
  without changing resource identities or emitting unused defaults.
- Add reviewed SUSHI fixtures and conformance diagnostics for the representative
  shapes corrected in this wave.

## Capabilities

### New Capabilities

- `fsh-json-shape-conformance`: Regression coverage and diagnostics for
  schema-shaped instance JSON parity with SUSHI.

### Modified Capabilities

- `fsh-export`: Instance export will apply recursive FHIR and resolved-profile
  metadata consistently across all embedding contexts and precedence layers.

## Impact

- Affected code: `crates/rh-fsh/src/export/instance.rs`, schema and definition
  resolution, SUSHI compatibility fixtures, and project comparison reporting.
- Affected behavior: JSON structure of exported instances; no public CLI or FSH
  syntax changes are expected.
- Validation: targeted reviewed SUSHI goldens, focused project comparisons, and
  two deterministic full comparison runs before any threshold is reduced.
