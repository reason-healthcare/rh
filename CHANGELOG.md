# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **rh-foundation**: New CLI utilities module with input/output helpers, format parsing, and result printing
- **rh-foundation**: WASM utilities module for WebAssembly support
- **rh-validator**: Core validation types (ValidationResult, ValidationIssue, Severity, IssueCode, Invariant, ValidatorError)
- **rh-validator**: Benchmark suite for validation performance testing
- **rh-validator**: Comprehensive unit tests for all core types (8 tests)
- **rh-validator**: DESIGN.md architecture document describing two-layer validation approach
- **rh-validator**: TODO.md with 7-phase implementation roadmap
- **rust-analyzer.toml**: Project-wide rust-analyzer configuration
- Comprehensive documentation for CLI utilities in rh-foundation README

### Changed
- **README.md**: Updated workspace structure to show rh-foundation instead of rh-common/rh-core
- **rh-foundation/README.md**: Added CLI utilities section with examples and module structure
- **rh-cli**: Refactored validator.rs, vcl.rs to use rh-foundation CLI utilities
- **rh-foundation/error.rs**: Added `with_context()` method and `Serde` variant for better error handling
- **rh-validator**: Complete rewrite - removed all existing validation code for clean redesign
- **rh-validator/Cargo.toml**: Minimized dependencies, added criterion for benchmarks

### Removed
- **rh-cli/src/ffq.rs**: Removed experimental FFQ (FHIR Filter Query) feature
- **rh-validator**: Removed all previous validator implementation code (fresh start for Phase 0)

### Fixed
- **rh-foundation/src/wasm.rs**: Fixed doctest to use getter method for private field
- **rh-validator**: Fixed Severity enum ordering to ensure Error > Warning > Information

## [0.1.0] - 2025-07-24

### Added
- Initial release of the Rust monorepo template
