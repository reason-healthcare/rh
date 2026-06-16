# Agent Instructions for RH Repository

This file provides guidance for AI agents working with code in this repository.

## Project Overview

**Reason Health (rh)** is a high-performance FHIR (Fast Healthcare Interoperability Resources) toolkit in Rust. Monorepo workspace with library crates in `/crates/` and the CLI application in `/apps/rh-cli/`.

## Documentation Structure

This repository uses modular documentation. See the following files for specific guidance:

- **Architecture & Design**: See @.github/ARCHITECTURE.md
- **Development Workflow**: See @.github/DEVELOPMENT.md
- **Coding Standards**: See @.github/CODING_STANDARDS.md
- **CLI behavior for agents**: See @apps/rh-cli/README.md, especially global `--format json`, exit codes, and agent/scripting examples.
- **Conformance status**:
  - FHIRPath: @crates/rh-fhirpath/CONFORMANCE.md and @crates/rh-fhirpath/SPEC_COVERAGE.md
  - CQL: @crates/rh-cql/CONFORMANCE.md and @crates/rh-cql/SPEC_COVERAGE.md
  - FSH: @crates/rh-fsh/CONFORMANCE.md

## Useful Just Recipes

- `just check` — full local gate before commits.
- `just docs-sync` — checks architecture dependency docs and CLI command docs against `cargo metadata` / `rh --help`.
- `just wasm-check` — compile-checks WASM-capable Rust crates.
- `just wasm-build <fhirpath|vcl|cql> <node|web|bundler|all>` — builds wasm-pack artifacts used by npm packages.
- `just regen-r4`, `just regen-r5`, `just regen-check` — regenerate and drift-check committed generated FHIR crates.
