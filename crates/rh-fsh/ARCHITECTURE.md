# rh-fsh Architecture

This document describes the architectural design of the `rh-fsh` FSH-to-FHIR compiler,
highlighting key design choices and their rationale.

## Overview

`rh-fsh` transforms FHIR Shorthand (FSH) source text into FHIR R4 JSON resources.
The architecture is a four-stage pipeline — parse, index, resolve, export — with
parallel export via rayon. Validation is explicitly out of scope; structural errors
are delegated to `rh-validator`.

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                          FSH source text(s)                                  │
└──────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│                 Stage 1 — Parser  (nom combinators)                          │
│         parser/span.rs · parser/lexer.rs · parser/entity.rs                 │
│                         parser/rules.rs · parser/ast.rs                     │
└──────────────────────────────────────────────────────────────────────────────┘
                                    │  FshDocument  (parser/ast.rs)
                                    ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│                   Stage 2 — Tank  (FshTank)                                  │
│                          tank/mod.rs                                         │
└──────────────────────────────────────────────────────────────────────────────┘
                                    │  FshTank  (indexed by entity name)
                                    ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│                  Stage 3 — Resolver  (FshResolver)                           │
│                        resolver/mod.rs                                       │
│  Pass 1: alias expansion → Pass 2: RuleSet graph + topo-sort + inline        │
└──────────────────────────────────────────────────────────────────────────────┘
                                    │  FshTank (resolved)
                                    ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│             Stage 4 — Export  (FshExporter, rayon parallel)                  │
│   export/structure_def.rs · export/value_set.rs · export/code_system.rs     │
│            export/instance.rs · export/mapping.rs · export/mod.rs           │
└──────────────────────────────────────────────────────────────────────────────┘
                                    │
                         ┌──────────┴───────────┐
                         │  FhirDefs (Arc<>)     │
                         │  fhirdefs/mod.rs      │
                         └──────────────────────┘
                                    │
                                    ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│                      FhirPackage  (FHIR R4 JSON)                             │
│             resources: Vec<serde_json::Value> + errors: Vec<FshError>        │
└──────────────────────────────────────────────────────────────────────────────┘
```

## Design Principles

### 1. nom-Based Parser with `Span<'a>`

The parser uses [nom](https://docs.rs/nom) combinators directly over a custom
`Span<'a>` type (in `parser/span.rs`) that tracks `offset`, `line`, and `column`
alongside the input slice. This is the same approach as `rh-cql` and `rh-fhirpath`.

```rust
// Every parser function takes and returns Span<'a>
fn parse_card_rule(input: Span<'_>) -> IResult<Span<'_>, Spanned<CardRule>> {
    let (input, path) = fsh_path(input)?;
    let (input, _) = ws(input)?;
    let (input, (min, max)) = cardinal(input)?;
    // ...
}
```

**Why not a parser generator (logos, lalrpop, pest)?**

FSH is line-oriented with a simple grammar — every entity starts at a keyword
at line-start; every rule line starts with `*`. This structure maps naturally to
hand-written nom combinators. Avoiding a codegen step keeps the build fast, makes
error recovery explicit, and follows the established rh crate pattern.

**Error recovery:** when a `*` rule line cannot be parsed by any specific rule
parser, it is skipped with a warning rather than aborting the whole document.
This mirrors FSH tooling expectations — a malformed rule should not prevent
valid surrounding entities from compiling.

### 2. `FshTank` as the Central Index

After parsing, all entities live in a `FshTank` — a struct of `IndexMap<String, T>`
collections, one per entity type. `IndexMap` is used instead of `HashMap` to
preserve declaration order in output, matching sushi's behaviour.

```rust
pub struct FshTank {
    pub profiles:      IndexMap<String, Profile>,
    pub extensions:    IndexMap<String, Extension>,
    pub instances:     IndexMap<String, Instance>,
    pub value_sets:    IndexMap<String, ValueSet>,
    pub rule_sets:     IndexMap<String, RuleSet>,
    pub aliases:       HashMap<String, String>,
    // ...
}
```

`FshTank::fish(name)` searches all collections by name in a single call — the
same interface as sushi's `FSHTank.fish()`. This makes the resolver and exporter
agnostic to which entity type they are looking up.

Multiple `FshDocument` values (one per input file) can be added to the same tank,
enabling multi-file compilation with cross-file reference resolution.

### 3. Four-Pass Resolver

`FshResolver::resolve` mutates the tank in place through four ordered passes:

| Pass | What it does |
|------|-------------|
| **Alias expansion** | Replaces alias names (e.g. `$SCT`) with canonical system URLs across all rule values |
| **RuleSet dependency graph** | Builds a directed graph of which RuleSets `insert` which others |
| **Topological sort + inline** | DFS topo-sort; inlines rule bodies in sorted order; detects cycles → `FshError::RuleSetCycle` |
| **ParamRuleSet expansion** | Clones parameterised rule bodies and substitutes argument values |

Passes are strictly sequential because each depends on the result of the previous.

### 4. Parallel Export with rayon

`FshExporter::export` collects all entities from the resolved tank and dispatches
to per-type exporter functions in parallel using `rayon::par_iter()`. Each
exporter receives a shared `Arc<FhirDefs>` for read-only element lookups.

```rust
// Profiles exported in parallel
let profile_results: Vec<_> = tank
    .profiles
    .values()
    .collect::<Vec<_>>()
    .into_par_iter()
    .map(|p| structure_def::export_profile(p, &defs))
    .collect();
```

Non-fatal export errors (e.g. unresolvable parent SD) are collected into
`FhirPackage::errors` rather than aborting compilation. The caller decides
whether to treat them as hard failures.

### 5. Differential-Only StructureDefinitions

The `structure_def` exporter produces FHIR `StructureDefinition` resources
containing only a `differential` — the minimal set of element overrides derived
from the FSH rules. It does **not** generate a `snapshot`. Snapshot generation,
if required, is delegated to `rh-foundation`'s snapshot utilities or the FHIR
validator.

This matches the output of sushi and keeps the exporter fast: computing a full
snapshot requires walking the entire parent SD hierarchy, which is expensive
and only needed for validation.

### 6. `FhirDefs` — Pluggable Definition Registry

`FhirDefs` is an `Arc`-wrapped registry of FHIR StructureDefinition summaries
used by the exporter for:

- Resolving `instanceOf` resource types
- Looking up element cardinality / type when building differential entries
- Inheriting parent SD metadata (URL, kind, baseDefinition)

```rust
// Thread-safe: Arc cloned into each rayon closure
pub fn export(tank: &FshTank, defs: Arc<FhirDefs>) -> FhirPackage
```

`FhirDefs::r4()` currently returns a registry pre-populated with a representative
set of common R4 resource names. **Full element-level indexing from
`rh-hl7_fhir_r4_core` is a planned follow-on.** Exporters handle a missing
SD lookup gracefully by producing the element differential without type
constraints, keeping compilation non-fatal.

## Performance vs. sushi

| Bottleneck in sushi | rh-fsh approach |
|---|---|
| ANTLR4 token stream + CST allocation | nom direct-to-AST, zero intermediate tree |
| Serial file and entity processing | `rayon::par_iter()` across entity collections |
| `unfold()` re-walks SD tree per rule | `FhirDefs` pre-indexed `HashMap` for O(1) element lookup |
| SD re-fetch per instance assignment | `Arc<FhirDefs>` shared across all rayon threads |
| `validateValueAtPath` per assignment | Delegated to `rh-validator` — not done during compilation |

## Module Structure

```
src/
├── lib.rs                  # Public API: FshCompiler, compile_fsh(), compile_fsh_files()
├── error.rs                # FshError enum with source location context
├── parser/
│   ├── mod.rs              # FshParser::parse() entry point
│   ├── span.rs             # Span<'a> with SourceLocation / SourceRange tracking
│   ├── lexer.rs            # Keyword constants, ws/identifier/path nom helpers
│   ├── ast.rs              # All FSH AST types (FshEntity, rule structs, FshValue)
│   ├── entity.rs           # Top-level entity parsers (parse_document, parse_profile, …)
│   └── rules.rs            # Rule-line parsers (parse_sd_rule, parse_instance_rule, …)
├── tank/
│   └── mod.rs              # FshTank, FshEntityRef, fish() lookup
├── resolver/
│   └── mod.rs              # FshResolver, alias expansion, RuleSet inlining
├── fhirdefs/
│   └── mod.rs              # FhirDefs, SdSummary, ElementSummary
├── export/
│   ├── mod.rs              # FshExporter, FhirPackage, rayon dispatch
│   ├── structure_def.rs    # Profile/Extension/Logical/Resource → StructureDefinition JSON
│   ├── value_set.rs        # ValueSet → FHIR ValueSet JSON
│   ├── code_system.rs      # CodeSystem → FHIR CodeSystem JSON
│   ├── instance.rs         # Instance → FHIR resource JSON
│   └── mapping.rs          # Mapping → FHIR ConceptMap JSON
└── benches/
    └── compile.rs          # criterion benchmarks (small/medium/large)
```

## Supported FSH Entity Types

| Entity | Exporter | Output resource |
|---|---|---|
| `Profile` | `structure_def` | `StructureDefinition` (kind: resource) |
| `Extension` | `structure_def` | `StructureDefinition` (type: Extension) |
| `Logical` | `structure_def` | `StructureDefinition` (kind: logical) |
| `Resource` | `structure_def` | `StructureDefinition` (kind: resource, derivation: specialization) |
| `Instance` | `instance` | Any FHIR resource type |
| `ValueSet` | `value_set` | `ValueSet` |
| `CodeSystem` | `code_system` | `CodeSystem` |
| `Mapping` | `mapping` | `ConceptMap` |
| `Invariant` | (embedded) | Injected into `StructureDefinition.constraint[]` |
| `RuleSet` | (inlined) | Rules inlined into referencing entities |
| `ParamRuleSet` | (expanded) | Rules expanded with substituted parameters |

`Alias` is consumed entirely during resolution and produces no output resource.

## Error Handling

`FshError` carries a `SourceLocation` (line, column) on every parse error,
enabling precise diagnostics:

```rust
pub enum FshError {
    Parse           { message: String, location: SourceLocation },
    DuplicateEntity { name: String,    location: SourceLocation },
    UnresolvedAlias { name: String,    location: SourceLocation },
    RuleSetCycle    { cycle: String },
    Export          { entity: String,  message: String },
    // ...
}
```

The compiler distinguishes **fatal errors** (parse failures, RuleSet cycles)
that abort compilation from **non-fatal errors** (unknown parent SD, export
warnings) that are collected in `FhirPackage::errors`. This matches the
ergonomics of sushi's error model.

## Extension Points

### Adding a new entity type

1. Add variant to `parser/ast.rs` `FshEntity` enum
2. Add parser in `parser/entity.rs`
3. Add collection to `FshTank` and register in `fish()`
4. Add exporter in `export/`
5. Wire into `FshExporter::export()`

### Populating FhirDefs from rh-hl7_fhir_r4_core

The planned follow-on to make element lookups complete:

1. Use the `rh-hl7_fhir_r4_core` `metadata` module to enumerate all R4 element paths
2. Populate `FhirDefs` with `ElementSummary` entries at startup
3. `FhirDefs::get_element(sd_name, path)` will return full cardinality + type info

### Supporting FHIR packages beyond R4

`FhirDefs::new()` accepts any externally-loaded SDs. To support IGs or R5:

1. Load IG package JSON (e.g. from a FHIR package cache)
2. Parse `StructureDefinition` resources into `SdSummary` / `ElementSummary`
3. Pass the populated `Arc<FhirDefs>` to `FshExporter`

### Snapshot generation

Pass the output `FhirPackage` to `rh-foundation`'s snapshot utilities, which
walk the differential and parent SD hierarchy to compute the full `snapshot`.
This is intentionally separate from compilation.

---

## Sushi Compatibility Tests

The `tests/sushi_compat.rs` integration test compares rh-fsh output against
golden FHIR JSON files generated by [SUSHI](https://fshschool.org/docs/sushi/).

### Fixtures

FSH source files from [FSHOnline-Examples](https://github.com/FHIR/FSHOnline-Examples/tree/main/Examples)
are stored in `tests/fixtures/` (preserving the upstream directory structure).

### Goldens

Golden JSON files are sushi's compiled output for each fixture, stored at:
`tests/goldens/<category>/<fixture-stem>/<ResourceType-id>.json`

Each fixture has its own subdirectory to prevent collisions between fixtures
that define resources with the same name.

### Generating / regenerating goldens

Requires Node.js and npx:
```sh
./scripts/generate-fsh-goldens.sh
```

Goldens are committed to the repo so CI does not require Node.js or sushi.

### Running

```sh
# All compatibility tests:
cargo test -p rh-fsh --test sushi_compat -- --include-ignored

# A single category:
cargo test -p rh-fsh --test sushi_compat test_value_sets -- --include-ignored
cargo test -p rh-fsh --test sushi_compat test_instances -- --include-ignored
cargo test -p rh-fsh --test sushi_compat test_profiles -- --include-ignored
cargo test -p rh-fsh --test sushi_compat test_extensions -- --include-ignored
cargo test -p rh-fsh --test sushi_compat test_code_systems -- --include-ignored
```

### Comparison strategy

The test runner matches rh-fsh resources to golden resources by `(resourceType, id)`.
Comparison is field-selective to account for known intentional differences:

- `snapshot` is **not** compared — rh-fsh outputs only `differential`
- `meta`, `text`, `version`, and package metadata are skipped
- For `StructureDefinition`: `differential.element[*].path` is compared bidirectionally
- For `ValueSet`: `compose` (include/exclude, normalised by system URL) is compared
- For `CodeSystem`: `concept` (code + display) is compared
- For instances: all top-level fields except skipped metadata are compared
