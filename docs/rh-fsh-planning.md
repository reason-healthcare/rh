# rh-fsh: FHIR Shorthand Compiler for Rust

## Executive Summary

This document outlines the planning approach for implementing `rh-fsh`, a FHIR Shorthand (FSH) compiler in Rust. FSH is a domain-specific language for defining HL7 FHIR Implementation Guides, with SUSHI serving as the reference implementation.

**Key Statistics:**
- 600+ Implementation Guide projects using FSH
- 1000+ repositories indexed by FSH Finder
- 250,000+ npm package downloads for SUSHI
- FSH 3.0.0 is the current specification version (Mixed Normative/Trial Use)

## Project Goals

1. **High-performance FSH compilation** - Leverage Rust's performance characteristics
2. **SUSHI compatibility** - Produce identical FHIR artifacts for valid FSH input
3. **Integration with rh ecosystem** - Reuse existing crates (rh-loader, rh-validator, etc.) and expose functionality via `rh-cli`
4. **WASM compilation target** - Enable browser-based FSH tooling

## SUSHI Architecture Reference

The reference implementation (SUSHI) follows this structure:

```
SUSHI Compiler Pipeline
========================

┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   FSH Files │───▶│   Parser    │───▶│   FSHTank   │───▶│   Exporter  │
│   (.fsh)    │    │   (ANTLR4)  │    │   (AST)     │    │   (FHIR)    │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                          │                  │                  │
                          ▼                  ▼                  ▼
                   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
                   │ FSHImporter │    │  fshtypes/  │    │ fsh-generated/
                   │ (91KB TS)   │    │  entities   │    │  (JSON)     │
                   └─────────────┘    └─────────────┘    └─────────────┘
```

### SUSHI Source Modules

| Module | Purpose | Size |
|--------|---------|------|
| `src/import/` | Parsing, AST construction | FSHImporter.ts (91KB) |
| `src/fshtypes/` | FSH entity definitions | Profile, Extension, Instance, etc. |
| `src/fhirtypes/` | FHIR type representations | ElementDefinition, StructureDefinition |
| `src/export/` | FHIR JSON generation | Profile, ValueSet, CodeSystem exporters |
| `src/fhirdefs/` | FHIR definition loading | Package cache, dependency resolution |

## FSH Language Overview

### Entity Types (11 total)

| Entity | Description | Output |
|--------|-------------|--------|
| **Alias** | URL/URN shortcuts | (internal) |
| **Profile** | Resource constraints | StructureDefinition |
| **Extension** | Custom element definitions | StructureDefinition |
| **Logical** | Abstract data models | StructureDefinition |
| **Resource** | Custom resource types | StructureDefinition |
| **Instance** | Concrete examples | Any FHIR resource |
| **ValueSet** | Code collections | ValueSet |
| **CodeSystem** | Code definitions | CodeSystem |
| **Invariant** | FHIRPath constraints | (embedded) |
| **RuleSet** | Reusable rule groups | (internal) |
| **Mapping** | Cross-standard maps | ConceptMap |

### Rule Types (16 total)

1. **Add Element** - New elements in Logical/Resource
2. **Assignment** - Value setting (`* status = #active`)
3. **Binding** - ValueSet binding (`* code from MyVS (required)`)
4. **Cardinality** - Occurrence constraints (`* name 1..1`)
5. **Contains** - Slicing definition
6. **Exclude** - ValueSet exclusion
7. **Flag** - Element markers (MS, SU, ?!, N, TU, D)
8. **Include** - ValueSet inclusion
9. **Insert** - RuleSet application
10. **Local Code** - CodeSystem code definition
11. **Mapping** - Element mapping (`* code -> "SCT#123"`)
12. **Obeys** - Invariant application
13. **Only** - Type restriction (`* value[x] only string`)
14. **Path** - Metadata via caret syntax
15. **Type** - Element type constraint

## Proposed rh-fsh Architecture

```
rh-fsh Crate Structure
======================

crates/rh-fsh/
├── src/
│   ├── lib.rs              # Public API
│   ├── parser/
│   │   ├── mod.rs
│   │   ├── lexer.rs        # Token definitions
│   │   ├── grammar.pest    # OR grammar.lalrpop
│   │   └── ast.rs          # Parse tree types
│   ├── fshtypes/
│   │   ├── mod.rs
│   │   ├── profile.rs
│   │   ├── extension.rs
│   │   ├── instance.rs
│   │   ├── valueset.rs
│   │   ├── codesystem.rs
│   │   ├── logical.rs
│   │   ├── resource.rs
│   │   ├── invariant.rs
│   │   ├── ruleset.rs
│   │   ├── mapping.rs
│   │   └── rules/          # Rule type definitions
│   ├── resolver/
│   │   ├── mod.rs
│   │   ├── alias.rs        # Alias resolution
│   │   ├── ruleset.rs      # RuleSet expansion
│   │   └── reference.rs    # Name resolution
│   ├── exporter/
│   │   ├── mod.rs
│   │   ├── profile.rs      # StructureDefinition generation
│   │   ├── valueset.rs
│   │   ├── codesystem.rs
│   │   └── instance.rs
│   └── error.rs            # Diagnostic types
├── tests/
│   ├── parser_tests.rs
│   ├── exporter_tests.rs
│   └── regression/         # SUSHI compatibility tests
└── Cargo.toml
```

### Integration with Existing rh Crates

```
Dependency Graph
================

rh-fsh
├── rh-loader        # FHIR package resolution
├── rh-snapshot      # StructureDefinition snapshots
├── rh-validator     # Output validation (optional)
├── rh-fhirpath      # FHIRPath in invariants
└── hl7_fhir_r4_core # Generated FHIR types
```

## Implementation Phases

### Phase 1: Parser Foundation
- Grammar implementation (pest or LALRPOP)
- AST type definitions
- Basic error reporting with source locations

### Phase 2: Core Entity Support
- Profile and Extension compilation
- Basic rule processing (cardinality, flags, binding)
- Alias and name resolution

### Phase 3: Full Entity Coverage
- ValueSet and CodeSystem
- Instance generation
- Logical and Resource types
- Invariant processing (FHIRPath integration)

### Phase 4: Advanced Features
- RuleSet expansion (including parameterized)
- Mapping support
- Slicing and discriminators
- Trial Use features (Context, Characteristics)

### Phase 5: Compatibility and Testing
- SUSHI regression test suite integration
- FSH Finder repository testing
- Performance benchmarking

## Key Technical Decisions

### Decision 1: Parser Strategy
See [rh-fsh-parser-strategy.md](./rh-fsh-parser-strategy.md) for detailed analysis.

**Options:**
1. Port ANTLR grammar to antlr4rust
2. Rewrite grammar for pest (PEG)
3. Rewrite grammar for LALRPOP (LR(1))
4. Hand-written recursive descent

### Decision 2: FHIR Type Representation
**Options:**
1. Use `hl7_fhir_r4_core` generated types directly
2. Create FSH-specific intermediate types
3. Work with `serde_json::Value` dynamically

**Recommendation:** Hybrid approach - FSH-specific types for compilation, convert to `hl7_fhir_r4_core` for output.

### Decision 3: Testing Strategy
See the regression testing section for the recommended approach based on SUSHI's practices.

## Related Documents

- [rh-fsh-parser-strategy.md](./rh-fsh-parser-strategy.md) - Parser implementation analysis
- [task-fsh-spec-extraction.md](./task-fsh-spec-extraction.md) - Specification extraction task
- [rh-fsh-regression-testing.md](./rh-fsh-regression-testing.md) - Testing strategy

## References

- [FHIR Shorthand Specification](https://build.fhir.org/ig/HL7/fhir-shorthand/)
- [SUSHI Repository](https://github.com/FHIR/sushi)
- [FSH Finder](https://fshschool.github.io/fsh-finder/)
- [FSH School](https://fshschool.org/)
