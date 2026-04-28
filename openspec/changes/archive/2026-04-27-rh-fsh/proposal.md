## Why

FHIR Shorthand (FSH) is the standard authoring language for FHIR implementation guides, but the reference compiler (sushi) is a Node.js/ANTLR4 tool that becomes painfully slow on large IGs — processing hundreds of profiles and thousands of instance rules can take minutes. `rh` already provides high-performance FHIR tooling in Rust; adding a native FSH → FHIR JSON compiler makes the full IG authoring pipeline fast and self-contained.

## What Changes

- **New crate** `crates/rh-fsh`: FSH text → FHIR JSON compiler
  - nom-based parser (Span + lexer helpers, following the CQL crate pattern)
  - FSH AST covering all 11 entity types and 15+ rule types
  - `FshTank`: indexed in-memory store for multi-file FSH workspaces
  - `FshResolver`: alias expansion + RuleSet inlining (topologically sorted)
  - Parallel exporters (rayon) for StructureDefinition, ValueSet, CodeSystem, Instance, ConceptMap
  - `FhirDefs`: pre-indexed FHIR R4 base definitions (O(1) element lookup via `rh-hl7_fhir_r4_core`)
- **New CLI subcommand** `rh fsh`: `compile`, `parse`, `tank` subcommands in `apps/rh-cli`
- **New workspace dependency**: `rayon = "1.10"` added to root `Cargo.toml`

## Capabilities

### New Capabilities

- `fsh-parser`: Lex and parse FSH source text into a typed AST (`FshDocument`, `FshEntity`, all rule types)
- `fsh-tank`: Collect parsed FSH entities from multiple files into an indexed `FshTank`; resolve aliases and expand RuleSets
- `fsh-export`: Convert a resolved `FshTank` into FHIR JSON resources (StructureDefinition, ValueSet, CodeSystem, resource instances, ConceptMap)
- `fsh-cli`: `rh fsh` CLI subcommand for compiling FSH files to FHIR JSON

### Modified Capabilities

## Impact

- New crate `crates/rh-fsh` (no changes to existing crates)
- `apps/rh-cli`: new `fsh.rs` module + clap subcommand wired into `main.rs`
- Root `Cargo.toml`: add `rayon` as a workspace dependency
- Depends on `rh-hl7_fhir_r4_core` for base FHIR R4 StructureDefinition data
- Depends on `rh-foundation` for shared error/foundation types
- No breaking changes to any existing crate public APIs
