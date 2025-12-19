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

## Exporter Pipeline (Post-Parsing)

After parsing FSH files into an AST (FSHTank), the compiler executes a multi-stage export pipeline to transform FSH entities into FHIR JSON resources and write them to disk.

### Pipeline Overview

```
Export Pipeline
===============

┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   FSHTank   │───▶│ FHIRExporter│───▶│   Package   │───▶│ Serializer  │───▶│ fsh-generated/
│   (AST)     │    │ (Orchestrator)   │  (In-Memory)│    │ (toJSON)    │    │  resources/ │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                          │
          ┌───────────────┼───────────────┐
          │               │               │
          ▼               ▼               ▼
   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
   │StructureDef │ │  ValueSet   │ │  Instance   │
   │  Exporter   │ │  Exporter   │ │  Exporter   │
   └─────────────┘ └─────────────┘ └─────────────┘
```

### Stage 1: Insert Rule Expansion

Before export, all RuleSet references are expanded in-place:

```
applyInsertRules()
├── Expand RuleSets in Invariants
└── Expand RuleSets in StructureDefinitions (Profile, Extension, Logical, Resource)
    ├── Handle parameterized RuleSets with variable substitution
    └── Recursively expand nested RuleSet references
```

### Stage 2: Entity Export (FHIRExporter Orchestration)

The `FHIRExporter` coordinates five specialized exporters in a specific order:

```typescript
// Export order matters for cross-references
1. structureDefinitionExporter.export()  // Profiles, Extensions, Logicals, Resources
2. codeSystemExporter.export()           // CodeSystem resources
3. valueSetExporter.export()             // ValueSet resources  
4. instanceExporter.export()             // Instance resources (any FHIR type)
5. structureDefinitionExporter.applyDeferredRules()  // Handle circular refs
6. mappingExporter.export()              // ConceptMap resources
```

### Stage 3: StructureDefinition Export (Profiles/Extensions/Logicals/Resources)

The most complex exporter, handling constraint application:

```
exportStructDef(fshDefinition)
├── getStructureDefinition()           // Clone parent SD, set baseDefinition/url/type
├── resetParentElements()              // Adjust element ids/paths for Logical/Resource
├── setMetadata()                      // id, name, title, description, status, etc.
├── preprocessStructureDefinition()    // Infer extension value[x]/extension constraints
├── setRules()                         // Apply each FSH rule to elements
│   ├── AddElementRule                 // New elements (Logical/Resource only)
│   ├── CardRule                       // Cardinality constraints
│   ├── AssignmentRule                 // Fixed/pattern values
│   ├── FlagRule                       // MS, SU, ?!, N, TU, D
│   ├── OnlyRule                       // Type constraints
│   ├── BindingRule                    // ValueSet bindings
│   ├── ContainsRule                   // Slicing (extensions or other arrays)
│   ├── CaretValueRule                 // Direct element/SD property assignment
│   └── ObeysRule                      // Invariant application
├── setContext() (Extensions only)     // Extension context constraints
├── cleanResource()                    // Remove internal properties
└── validate()                         // Check for errors
```

### Stage 4: Package Population

Each exporter populates typed arrays in the `Package` object:

```typescript
class Package {
  profiles: StructureDefinition[]      // Profile SDs
  extensions: StructureDefinition[]    // Extension SDs
  logicals: StructureDefinition[]      // Logical Model SDs
  resources: StructureDefinition[]     // Custom Resource SDs
  instances: InstanceDefinition[]      // Any FHIR resource instance
  valueSets: ValueSet[]                // ValueSet resources
  codeSystems: CodeSystem[]            // CodeSystem resources
  
  fshMap: Map<string, SourceInfo>      // Tracks FSH source → output file mapping
}
```

### Stage 5: JSON Serialization

Each FHIR type class implements serialization methods:

```typescript
interface Exportable {
  getFileName(): string;    // e.g., "StructureDefinition-MyProfile.json"
  toJSON(snapshot?: boolean): any;  // FHIR-compliant JSON representation
}
```

**StructureDefinition.toJSON(snapshot)**:
- Generates `differential` (always) - only changed elements
- Optionally generates `snapshot` (full element tree)
- Orders properties per FHIR specification
- Removes internal tracking properties

**ValueSet/CodeSystem.toJSON()**:
- Deep clones the object with property ordering
- Removes undefined/null properties

**InstanceDefinition.toJSON()**:
- Returns clean FHIR resource JSON
- Removes `_instanceMeta` tracking object

### Stage 6: File Output (writeFHIRResources)

The final stage writes all resources to disk:

```
writeFHIRResources(outDir, package, defs, snapshot)
├── Create output directory: {outDir}/fsh-generated/resources/
├── Skip predefined resources (avoid duplicates)
├── For each resource category:
│   ├── profiles[]
│   ├── extensions[]
│   ├── logicals[]
│   ├── resources[]
│   ├── valueSets[]
│   ├── codeSystems[]
│   └── instances[] (excluding Inline usage)
│       ├── checkNullValuesOnArray()     // Warn about sparse arrays
│       ├── resource.getFileName()        // Generate filename
│       ├── resource.toJSON(snapshot)     // Serialize to JSON
│       └── fs.outputJSONSync()           // Write with 2-space indent
└── Generate index files:
    ├── fsh-generated/fsh-index.txt       // Human-readable mapping
    └── fsh-generated/data/fsh-index.json // Machine-readable mapping
```

### Output Directory Structure

```
{project}/
├── input/
│   └── fsh/
│       └── *.fsh                         # Source FSH files
└── fsh-generated/
    ├── resources/
    │   ├── StructureDefinition-*.json    # Profiles, Extensions, Logicals, Resources
    │   ├── ValueSet-*.json               # ValueSets
    │   ├── CodeSystem-*.json             # CodeSystems
    │   └── {ResourceType}-*.json         # Instances
    ├── fsh-index.txt                     # Source mapping (text)
    └── data/
        └── fsh-index.json                # Source mapping (JSON)
```

### File Naming Convention

| Resource Type | Filename Pattern |
|---------------|------------------|
| StructureDefinition | `StructureDefinition-{id}.json` |
| ValueSet | `ValueSet-{id}.json` |
| CodeSystem | `CodeSystem-{id}.json` |
| Instance | `{resourceType}-{id}.json` |

### FSH Index Format

The `fsh-index.json` maps outputs back to FSH sources:

```json
[
  {
    "outputFile": "StructureDefinition-MyProfile.json",
    "fshName": "MyProfile",
    "fshType": "Profile",
    "fshFile": "profiles.fsh",
    "startLine": 1,
    "endLine": 15
  }
]
```

### Deferred Rule Processing

Some rules require deferred processing due to circular dependencies:

1. **Instance Assignments** - Instances may reference SDs still being exported
2. **Inline ValueSet Bindings** - Contained ValueSets processed after instances
3. **Caret Rules on Contained Resources** - Must wait for container to exist

```
applyDeferredRules()
├── Process deferred CaretValueRules
│   ├── Fish for Instance definitions
│   ├── Apply instance assignments
│   └── Re-clean modified contained resources
└── Resolve inline ValueSet bindings
    ├── Find contained ValueSet by id
    └── Update binding to relative reference (#id)
```

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

## rh-fsh Exporter Design Considerations

### FHIR Type Representations

For the Rust implementation, we need representations for:

| FHIR Type | rh-fsh Approach |
|-----------|-----------------|
| StructureDefinition | Custom struct with element tree management |
| ElementDefinition | Custom struct supporting all constraint methods |
| ValueSet | Use `hl7_fhir_r4_core::ValueSet` with extension trait |
| CodeSystem | Use `hl7_fhir_r4_core::CodeSystem` with extension trait |
| Instance | `serde_json::Value` with metadata wrapper |

### Serialization Strategy

```rust
// Each exportable type implements:
trait FhirExportable {
    fn get_filename(&self) -> String;
    fn to_json(&self, include_snapshot: bool) -> Result<serde_json::Value>;
}

// StructureDefinition needs special handling for:
// - Differential generation (compare to captured original)
// - Optional snapshot generation
// - Element ordering per FHIR spec
// - Removal of internal tracking fields
```

### Output Writer Module

```rust
// Proposed module structure
mod output {
    pub struct FhirWriter {
        output_dir: PathBuf,
        predefined_resources: HashSet<ResourceKey>,
    }
    
    impl FhirWriter {
        pub fn write_package(&self, pkg: &Package) -> Result<WriteStats>;
        pub fn write_resource<T: FhirExportable>(&self, resource: &T) -> Result<()>;
        pub fn write_fsh_index(&self, pkg: &Package, input_dir: &Path) -> Result<()>;
    }
}
```

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

### Decision 3: Differential/Snapshot Generation

For StructureDefinition export, we must generate:

1. **Differential** (required) - Only elements that differ from parent
2. **Snapshot** (optional) - Complete element tree with inherited elements

**Options:**
1. Port SUSHI's element capture/comparison approach
2. Leverage `rh-snapshot` crate for snapshot generation
3. Implement dedicated diff algorithm

**Recommendation:** Use `rh-snapshot` for snapshot generation, implement lightweight differential tracking during rule application.

### Decision 4: Testing Strategy
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
