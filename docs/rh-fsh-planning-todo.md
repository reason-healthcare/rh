# rh-fsh Planning: Open Items

This document tracks areas that require further research and planning before or during implementation.

## High Priority

### 1. sushi-config.yaml Processing

The project configuration file that drives FSH compilation. Needs analysis of:

- [ ] Full schema of supported configuration options
- [ ] FHIR version selection (R4, R4B, R5) and implications
- [ ] Dependency declaration and version resolution
- [ ] IG metadata fields and how they map to output
- [ ] Parameter overrides and their effects
- [ ] Menu/page/resource configuration for IG Publisher

**Reference:** SUSHI's `src/import/YAMLConfiguration.ts` and `YAMLschema.json`

### 2. Snapshot Generation

The process of generating StructureDefinition snapshots from differentials. This is complex and bug-prone.

- [ ] Parent chain resolution (Profile → Profile → Resource → Base)
- [ ] Differential merging algorithm
- [ ] Element inheritance rules
- [ ] Slicing in snapshots
- [ ] Extension handling in snapshots
- [ ] Integration strategy with `rh-snapshot` crate
- [ ] Performance considerations for deep inheritance chains

**Reference:** SUSHI's `src/fhirtypes/` and IG Publisher snapshot generation

### 3. Slicing Deep-Dive

FHIR slicing has significant complexity beyond basic rule parsing.

- [ ] Discriminator types: value, pattern, type, profile, exists
- [ ] Discriminator path resolution
- [ ] Ordered vs unordered slices
- [ ] Open vs closed slicing
- [ ] Re-slicing mechanics
- [ ] Default discriminator inference
- [ ] Slice matching during instance validation

**Reference:** FHIR spec on slicing, SUSHI's slicing implementation

### 4. Multi-FHIR Version Support

SUSHI supports R4, R4B, and R5. Strategy needed for rh-fsh.

- [ ] Architecture decision: feature flags vs separate crates vs runtime selection
- [ ] Version-specific base resources and types
- [ ] Version-specific features (CodeableReference, etc.)
- [ ] Cross-version compatibility considerations
- [ ] Testing across versions

## Medium Priority

### 5. IG Publisher Integration

SUSHI output must integrate with the IG Publisher pipeline.

- [ ] Output directory structure (`fsh-generated/`)
- [ ] Resource file naming conventions
- [ ] Package manifest generation
- [ ] ImplementationGuide resource generation
- [ ] Pre-processing hook compatibility

### 6. Error Message Quality

Critical for adoption and developer experience.

- [ ] Source location tracking through all compilation phases
- [ ] Error message style guide (format, verbosity)
- [ ] Warning vs error severity classification
- [ ] Contextual suggestions ("Did you mean X?")
- [ ] Error recovery strategies in parser
- [ ] Aggregate error reporting (don't stop at first error)

### 7. FHIRPath Integration Details

Beyond basic `rh-fhirpath` integration.

- [ ] Compile-time validation of FHIRPath expressions in invariants
- [ ] FHIRPath context binding (element type awareness)
- [ ] XPath support for R4 compatibility
- [ ] Expression embedding in generated StructureDefinitions

## Lower Priority

### 8. IDE/Editor Support

For real-world adoption beyond CLI usage.

- [ ] Language Server Protocol (LSP) implementation feasibility
- [ ] Syntax highlighting grammar (TextMate, VS Code)
- [ ] Go-to-definition, hover, completion
- [ ] Real-time diagnostics
- [ ] Relationship to existing vscode-language-fsh extension

### 9. WASM Target Specifics

Details for browser-based FSH tooling.

- [ ] Which crates need WASM compatibility
- [ ] Browser-specific I/O handling
- [ ] Package loading in browser context
- [ ] FSH Online-like functionality scope
- [ ] Bundle size considerations

### 10. Performance and Incremental Compilation

For large IG projects.

- [ ] Baseline performance targets (vs SUSHI)
- [ ] Profiling strategy
- [ ] Incremental compilation feasibility
- [ ] Caching strategies (parsed AST, resolved definitions)
- [ ] Parallel compilation opportunities

### 11. GoFSH Consideration

The reverse compiler (FHIR JSON → FSH).

- [ ] In scope or out of scope for rh-fsh?
- [ ] If in scope, separate crate or integrated?
- [ ] Round-trip fidelity requirements

## Research Questions

These need investigation before detailed planning:

1. **How does SUSHI handle circular dependencies in RuleSets?**
2. **What are the exact rules for Extension context validation?**
3. **How are contained resources handled in Instance generation?**
4. **What preprocessing steps does SUSHI apply before parsing?**
5. **How does SUSHI resolve ambiguous name references?**

## Completed

Items move here once adequately covered in planning documents.

- [x] Parser strategy analysis → `rh-fsh-parser-strategy.md`
- [x] Regression testing approach → `rh-fsh-regression-testing.md`
- [x] Specification extraction task → `task-fsh-spec-extraction.md`
- [x] Overall architecture → `rh-fsh-planning.md`
