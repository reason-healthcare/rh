# Changelog

All notable user-facing changes to Reason Health are recorded here.

This project follows semantic versioning while the public API is still in the
0.x series. Release dates use `YYYY-MM-DD`.

## [0.2.7] - 2026-07-15

### Added

- Added schema-informed, profile-aware FSH compilation and export, including
  typed instance shaping, recursive instance resolution, dependency-backed
  extension slices, nested caret paths, and CLI project workflow documentation.
- Expanded FSH conformance coverage to 63 reviewed fixtures and six real-world
  implementation guide projects, with `rh-fsh` and SUSHI now emitting the same
  920 resource identities across the project corpus.

### Changed

- Improved CQL syntax diagnostics for malformed operators and declarations
  while preserving the CLI syntax-error exit code and structured JSON envelope.
- Expanded generated R4 and R5 metadata used for schema navigation and
  profile-aware FSH export.
- Clarified the project overview around healthcare interoperability, AI agent
  workflows, and the supported FHIR toolchain.

### Fixed

- Fixed CQL parsing, ELM emission, and evaluation for canonical null and Boolean
  tests, logical operator precedence, and zero-offset temporal relationships.
- Fixed validator unknown-property checks for invalid choice-field variants and
  recursively nested backbone elements.
- Fixed FSH output for required extension identity, local profile lineage,
  extension context arrays, repeating root caret fields, and recursive resource
  shapes.

## [0.2.6] - 2026-07-10

### Added

- Added SQL-on-FHIR relational projection support to `rh cql`, including CLI
  documentation, integration coverage, and the supporting `rh-cql` analytics
  module.
- Expanded the CQL WebAssembly and TypeScript package helpers with node, web,
  and bundler entry points, tests, and playground integration.
- Added canonical URL handling for package build and validation workflows,
  including manifest support and integration tests.

### Changed

- Consolidated duplicated `rh cql compile` and `rh cql validate` CLI handling
  while preserving command behavior.
- Reorganized `rh-cql` architecture documentation and cleaned up validator
  documentation to reflect the current conformance workflow.

## [0.2.5] - 2026-07-04

### Changed

- Improved `rh-validator` warmed US Core Patient validation performance by
  caching repeated ValueSet and CodeSystem membership checks, caching parsed
  invariant expressions, adding native fast paths for common core invariants,
  and precomputing unknown-property lookup plans.
- Added a durable `rh-validator` smoke benchmark and refreshed the validator
  performance report with full Criterion benchmark results through 500-resource
  batches.

## [0.2.4] - 2026-07-02

### Changed

- Improved `rh-validator` agreement with Java validator behavior across R4
  conformance fixtures, including bundle references, contained resources,
  terminology, StructureDefinition, Questionnaire, Measure, and invariant
  handling.
- Expanded `rh-fsh` SUSHI conformance coverage and refactored supporting
  conformance test infrastructure.
- Removed unsupported Nix flake, release-hash helper, CI job, and related
  release/documentation references.

### Fixed

- Fixed validator slicing metadata preservation, versioned profile URL lookup,
  and value/profile discriminator matching.
- Fixed validator conformance gaps for FHIR string size limits, terminology
  filters backed by registered CodeSystems, choice bindings, contained resource
  id validation, narrative fragment references, and profile reference target
  types.
- Made validator CI independent of the local package cache.

## [0.2.3] - 2026-06-23

### Added

- Started maintaining this changelog as the canonical release summary.
- Added a consolidated `rh-cql` conformance summary covering spec status and
  Java/JavaScript reference implementation comparison.
- Added CQL conformance reference tooling for matrix runs, corpus auditing,
  and source-validity tracking.

### Changed

- Improved FHIRPath temporal value handling by normalizing `time` output and
  using canonical temporal string serialization across evaluator operations,
  conversions, comparisons, boundary functions, and CLI output.
- Improved CQL compiler, emitter, modelinfo, include resolution, and evaluator
  behavior across the conformance workstream.
- Simplified `rh-cql` conformance documentation around the crate README,
  `CONFORMANCE.md`, `SPEC_COVERAGE.md`, and the conformance runbook.

### Fixed

- Fixed FHIRPath `ToTime()` and time component behavior so converted and
  evaluated time values omit the literal `T` prefix in output.
- Fixed CQL `Time` literal evaluation for hour-precision values such as
  `@T14`.
- Fixed `rh cql compile --source-map` so it honors `--lib-path` and package
  include resolution consistently with normal compilation.
- Fixed CQL conformance expected-string handling to avoid double-unescaping.

[0.2.7]: https://github.com/reason-healthcare/rh/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/reason-healthcare/rh/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/reason-healthcare/rh/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/reason-healthcare/rh/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/reason-healthcare/rh/compare/v0.2.2...v0.2.3
