# Changelog

All notable user-facing changes to Reason Health are recorded here.

This project follows semantic versioning while the public API is still in the
0.x series. Release dates use `YYYY-MM-DD`.

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

[0.2.4]: https://github.com/reason-healthcare/rh/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/reason-healthcare/rh/compare/v0.2.2...v0.2.3
