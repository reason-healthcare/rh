# Changelog

All notable user-facing changes to Reason Health are recorded here.

This project follows semantic versioning while the public API is still in the
0.x series. Release dates use `YYYY-MM-DD`.

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

[0.2.3]: https://github.com/reason-healthcare/rh/compare/v0.2.2...v0.2.3
