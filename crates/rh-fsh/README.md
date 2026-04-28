# rh-fsh

FHIR Shorthand (FSH) compiler for the `rh` toolkit.

Transforms FSH source files into FHIR R4 JSON packages including StructureDefinitions,
ValueSets, CodeSystems, and Instances. Designed for high throughput — see
[Performance](#performance) below.

## Usage

```rust
use rh_fsh::compile_fsh;

let fsh = r#"
Profile: MyPatient
Parent: Patient
Title: "My Patient Profile"
Description: "A custom patient profile"
* name 1..*
"#;

let package = compile_fsh(fsh, "example.fsh")?;
for resource in &package.resources {
    println!("{}", serde_json::to_string_pretty(resource)?);
}
```

### Multi-file compilation

```rust
use rh_fsh::compile_fsh_files;
use std::path::PathBuf;

let paths = vec![
    PathBuf::from("profiles.fsh"),
    PathBuf::from("valuesets.fsh"),
    PathBuf::from("instances.fsh"),
];
let package = compile_fsh_files(&paths)?;
println!("Compiled {} resources", package.resources.len());
```

### Using FshCompiler with configuration

```rust
use rh_fsh::{FshCompiler, CompilerOptions, FshConfig};

let compiler = FshCompiler::new(CompilerOptions {
    pretty_print: true,
    config: FshConfig {
        canonical: Some("http://example.org/fhir".into()),
        fhir_version: Some("4.0.1".into()),
        id: Some("my-ig".into()),
        name: Some("MyIG".into()),
        status: Some("active".into()),
        publisher: Some("My Organization".into()),
        version: Some("1.0.0".into()),
    },
});
let package = compiler.compile(fsh_input, "my-ig.fsh")?;
```

## CLI Usage

```bash
# Compile FSH files to FHIR JSON output directory
rh fsh compile profiles/*.fsh --output output/

# Print AST for debugging
rh fsh parse myprofile.fsh

# Inspect parsed entity tank
rh fsh tank myprofile.fsh
```

See the [CLI FSH docs](../../apps/rh-cli/docs/FSH.md) for full CLI reference.

## FSH Entities Supported

- `Profile` — StructureDefinition with derivation=constraint
- `Extension` — Extension StructureDefinition
- `Logical` — Logical model StructureDefinition
- `Resource` — Resource StructureDefinition
- `Instance` — FHIR resource instance
- `ValueSet` — FHIR ValueSet
- `CodeSystem` — FHIR CodeSystem
- `Invariant` — FHIRPath invariant (embedded in element constraints)
- `Mapping` — Mapping declarations (embedded in StructureDefinition)
- `RuleSet` / `ParamRuleSet` — Reusable rule sets (inlined at compile time)
- `Alias` — URL aliases (expanded at compile time)

## Error Handling

Non-fatal errors (e.g. unresolvable parent SDs) are collected in `FhirPackage::errors`
rather than aborting compilation. This mirrors sushi's behavior of producing maximum
output even under partial failures:

```rust
let package = compile_fsh(fsh, "example.fsh")?;
if !package.errors.is_empty() {
    for err in &package.errors {
        eprintln!("Warning: {err}");
    }
}
// package.resources contains whatever compiled successfully
```

## Performance

`rh-fsh` uses a nom-based parser and rayon parallel export, delivering significantly
faster compile times than the reference sushi TypeScript implementation:

| Input Size | rh-fsh | sushi | Speedup |
|------------|--------|-------|---------|
| 1 profile | ~105 µs | ~3.9 s | ~37,000× |
| 10 profiles | ~306 µs | ~4.0 s | ~13,000× |
| 1,000 profiles | ~2.0 ms | ~9.8 s | ~4,900× |

> Benchmarks run on Apple M-series hardware. Sushi startup overhead dominates small inputs.

Key performance properties:
- Nom parser: direct-to-AST, no ANTLR runtime or tree allocation
- Parallel export: rayon `par_iter()` over each entity type
- O(1) element lookups: pre-indexed `HashMap<path, ElementSummary>` (vs sushi's `unfold()`)
- No GC pauses: Rust ownership model, `Arc<FhirDefs>` shared read-only

## Compatibility

Passes 30/30 non-skipped [FSHOnline sushi compatibility tests](https://github.com/FHIR/FSHOnline-Examples).

```bash
cargo test -p rh-fsh --test sushi_compat -- --include-ignored
```

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for a detailed description of the compiler pipeline:
parse → tank → resolve → export.

## License

MIT OR Apache-2.0
