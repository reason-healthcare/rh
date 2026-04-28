# rh fsh — FHIR Shorthand Compiler

The `rh fsh` command compiles [FHIR Shorthand (FSH)](https://build.fhir.org/ig/HL7/fhir-shorthand/) source files into FHIR R4 JSON resources. It is powered by [`rh-fsh`](../../../crates/rh-fsh/README.md), a Rust-native FSH compiler that is significantly faster than the reference [sushi](https://github.com/FHIR/sushi) implementation.

## Subcommands

| Subcommand | Description |
|------------|-------------|
| `rh fsh compile` | Compile FSH files to FHIR JSON |
| `rh fsh parse` | Parse a FSH file and print the AST as JSON |
| `rh fsh tank` | Parse FSH files and print a summary of the entity tank |

---

## `rh fsh compile`

Compile one or more FSH files into FHIR JSON resources.

### Usage

```bash
rh fsh compile <FILES...> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `<FILES...>` | One or more FSH input files (globs supported via shell) |

### Options

| Option | Description |
|--------|-------------|
| `-o, --output <DIR>` | Write resources to this directory (one file per resource). Defaults to stdout. |
| `--compact` | Output compact JSON (no indentation). Default is pretty-printed. |

### Examples

```bash
# Compile a single FSH file to stdout
rh fsh compile myprofile.fsh

# Compile multiple FSH files to an output directory
rh fsh compile profiles/*.fsh --output output/

# Compact JSON output
rh fsh compile myprofile.fsh --compact
```

### Output

Each compiled resource is written as a FHIR JSON file named `{ResourceType}-{id}.json`. For example:

```
output/
  StructureDefinition-MyPatient.json
  ValueSet-MyValueSet.json
  CodeSystem-MyCodes.json
```

Non-fatal errors (e.g. unresolvable parent SDs) are printed to stderr as warnings. The compiler continues and outputs as many resources as possible.

### Metadata via CLI flags

You can supply package metadata as CLI flags. These populate the `meta.profile`, `url`, and related fields that sushi would read from `sushi-config.yml`:

| Flag | Description |
|------|-------------|
| `--canonical <URL>` | Canonical base URL for all resources |
| `--status <STATUS>` | Publication status (`draft`, `active`, `retired`, `unknown`). Default: `draft` |

> **Note**: Full `sushi-config.yml` support is planned. For now, metadata can be encoded directly in FSH using `^` caret-value rules (e.g. `* ^url = "http://example.org/fhir/StructureDefinition/MyProfile"`).

---

## `rh fsh parse`

Parse a FSH file and emit the AST (Abstract Syntax Tree) as JSON. Useful for debugging or tooling.

### Usage

```bash
rh fsh parse <FILE>
```

### Example

```bash
rh fsh parse myprofile.fsh
```

```json
{
  "source_name": "myprofile.fsh",
  "entities": [
    {
      "inner": {
        "Profile": {
          "name": "MyPatient",
          "metadata": { "parent": "Patient", "title": "My Patient Profile" },
          "rules": [ ... ]
        }
      },
      "loc": { "start": { "line": 1, "col": 1 }, "end": { "line": 6, "col": 1 } }
    }
  ]
}
```

---

## `rh fsh tank`

Parse one or more FSH files, load them into the entity tank, and print a summary. Useful for verifying what entities were detected before compiling.

### Usage

```bash
rh fsh tank <FILE>
```

### Example

```bash
rh fsh tank myprofile.fsh
```

```
Tank summary:
  Profiles:     3
  Extensions:   1
  Instances:    2
  ValueSets:    1
  CodeSystems:  0
```

---

## FSH Entities Supported

`rh fsh compile` supports the full core FSH entity set:

| Entity | FHIR Output |
|--------|-------------|
| `Profile` | `StructureDefinition` (derivation: constraint) |
| `Extension` | `StructureDefinition` (type: Extension) |
| `Logical` | `StructureDefinition` (kind: logical) |
| `Resource` | `StructureDefinition` (kind: resource) |
| `Instance` | FHIR resource matching `instanceOf` type |
| `ValueSet` | `ValueSet` |
| `CodeSystem` | `CodeSystem` |
| `Invariant` | Embedded in StructureDefinition element constraints |
| `Mapping` | Embedded in StructureDefinition mapping declarations |
| `RuleSet` | Inlined at compile time (not a standalone resource) |
| `Alias` | Expanded at compile time (not a standalone resource) |

---

## FSH Rule Types Supported

| Rule | Syntax | Example |
|------|--------|---------|
| Cardinality | `* path M..N` | `* name 1..*` |
| Flags | `* path FLAG` | `* name MS` |
| Binding | `* path from VS (strength)` | `* status from http://... (required)` |
| Assignment | `* path = value` | `* status = #active` |
| Contains (slicing) | `* path contains Name M..N` | `* category contains mySlice 1..1` |
| Type constraint | `* path only Type` | `* subject only Reference(Patient)` |
| Obeys | `* path obeys inv-id` | `* obeys my-inv-1` |
| Caret-value | `* path ^prop = value` | `* ^url = "http://..."` |
| Insert RuleSet | `* insert RuleSetName` | `* insert CommonRules` |
| Add Element | `* path M..N type "short"` | `* myElem 0..1 string "My field"` |

---

## Performance

`rh fsh` is built on a Rust-native nom parser and rayon parallel export, delivering substantially faster compile times than the Node.js-based sushi compiler:

| Input Size | rh-fsh | sushi | Speedup |
|------------|--------|-------|---------|
| 1 profile | ~105 µs | ~3.9 s | ~37,000× |
| 10 profiles | ~306 µs | ~4.0 s | ~13,000× |
| 1,000 profiles | ~2.0 ms | ~9.8 s | ~4,900× |

> Benchmarks run on Apple M-series hardware. Sushi startup overhead dominates small inputs; rh-fsh scales linearly.

---

## Related

- [`rh-fsh` crate README](../../../crates/rh-fsh/README.md)
- [`rh-fsh` Architecture](../../../crates/rh-fsh/ARCHITECTURE.md)
- [FSH Specification](https://build.fhir.org/ig/HL7/fhir-shorthand/)
- [FSH Online Examples](https://github.com/FHIR/FSHOnline-Examples)
