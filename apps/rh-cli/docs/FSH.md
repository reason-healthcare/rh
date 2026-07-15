# rh fsh — FHIR Shorthand Compiler

The `rh fsh` command compiles [FHIR Shorthand (FSH)](https://build.fhir.org/ig/HL7/fhir-shorthand/) source files into FHIR R4 JSON resources. It is powered by [`rh-fsh`](../../../crates/rh-fsh/README.md), a Rust-native FSH compiler. It can run directly on individual files, as part of an RH package build, or alongside an existing [SUSHI](https://github.com/FHIR/sushi) project.

## Subcommands

| Subcommand | Description |
|------------|-------------|
| `rh fsh compile` | Compile FSH files to FHIR JSON |
| `rh fsh parse` | Parse a FSH file and print the AST as JSON |
| `rh fsh tank` | Parse FSH files and print a summary of the entity tank |

---

## Quick workflows

### Start a new RH project

`rh package init` creates a package source tree with `input/fsh/` for FSH files.
Enable the built-in `fsh` processor in the generated `packager.toml`, then check
and build the package:

```bash
# 1. Scaffold the project.
rh package init my-ig \
  --canonical https://example.org/fhir/my-ig \
  --name example.fhir.my-ig \
  --title "Example IG"

# 2. Add "fsh" to the generated hook list in my-ig/packager.toml:
# [hooks]
# before_build = ["fsh"]

# 3. Create FSH source under the generated input/fsh directory.
cat > my-ig/input/fsh/MyPatient.fsh <<'FSH'
Profile: MyPatient
Parent: Patient
Id: my-patient
Title: "My Patient"
* name 1..* MS
FSH

# 4. Inspect, validate the project configuration, and build it.
rh fsh parse my-ig/input/fsh/MyPatient.fsh
rh fsh tank my-ig/input/fsh/MyPatient.fsh
rh package check my-ig
rh package build my-ig
```

The package build reads canonical URL, version, status, package ID, publisher,
and FHIR version from `packager.toml`. The FSH processor recursively compiles
`*.fsh` below `input/fsh/` and injects the generated resources into the package.
The expanded package is written to `my-ig/output/` and a `.tgz` is created next
to it.

For a quick compiler-only iteration without running the package pipeline:

```bash
rh fsh compile my-ig/input/fsh/*.fsh --output my-ig/fsh-preview
```

That direct command does not read `packager.toml`; use caret rules in the FSH or
the full `rh package build` workflow when package metadata matters.

### Add FSH to an existing RH project

For a project that already contains `packager.toml`, add an `input/fsh/`
directory and ensure the configured build hooks include `fsh` before processors
that consume generated resources:

```toml
[hooks]
before_build = ["fsh", "snapshot", "validate"]
```

If the project uses a custom source layout, configure it explicitly:

```toml
[input]
dir = "input"
fsh_dir = "shorthand"
```

Then run:

```bash
rh package check existing-project
rh package build existing-project
```

FSH-generated resources with the same resource identity as an existing JSON
resource replace that resource during the build, with a warning. Keep one
authoritative source for each `(resourceType, id)` to avoid accidental
overrides.

### Use rh-fsh in an existing SUSHI project

Point `rh fsh compile` at the FSH files below the SUSHI project's `input/fsh/`
directory. The compiler walks upward from the first input file and automatically
loads the nearest `sushi-config.yaml` or `sushi-config.yml`:

```bash
cd my-sushi-ig

# Install dependencies declared by sushi-config.yaml when they are not already
# present in ~/.fhir/packages.
rh download package hl7.fhir.r4.core 4.0.1

# Compile every FSH file recursively into a separate preview directory.
rh fsh compile $(find input/fsh -type f -name '*.fsh' | sort) \
  --output rh-generated
```

Use a separate output directory such as `rh-generated/`; do not point this at
SUSHI's `fsh-generated/` directory unless replacing its contents is intentional.
SUSHI remains the reference tool for a complete IG Publisher workflow. rh-fsh
is useful as a fast compile/inspection step and currently tracks remaining
content differences in [`rh-fsh/CONFORMANCE.md`](../../../crates/rh-fsh/CONFORMANCE.md).

For CI or scripts, use the global JSON envelope and inspect `.ok`, `.result`,
and `.errors`:

```bash
rh --format json fsh compile \
  $(find input/fsh -type f -name '*.fsh' | sort) > rh-fsh-result.json

jq -e '.ok == true' rh-fsh-result.json
jq '.result.diagnostics' rh-fsh-result.json
```

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
| `<FILES...>` | One or more FSH input files. Shell globs are expanded by the shell; directories are not recursively expanded by the command. |

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

# Recursively compile a conventional IG source tree
rh fsh compile $(find input/fsh -type f -name '*.fsh' | sort) --output output/
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

### Project configuration

When an input file is below a SUSHI project, `rh fsh compile` discovers the
nearest `sushi-config.yaml` or `sushi-config.yml` by walking up its ancestor
directories. The first input file with a discoverable config selects the project
configuration, so do not mix files from unrelated projects in one invocation.

The supported configuration mapping includes core package/IG metadata,
dependencies, contacts, publisher information, and FHIR version. Dependency
StructureDefinitions are loaded from the local FHIR package cache. Install a
missing dependency with:

```bash
rh download package <package-id> <version>
```

When no SUSHI config is found, compiler defaults are used. Direct compilation
does not read `packager.toml`; the RH package build's FSH processor supplies
that project metadata instead.

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

`rh fsh` is built on a Rust-native nom parser, compiled schema views, and rayon
parallel export. This makes it suitable for editor feedback, CI checks, and
package build pipelines without a Node.js startup cost.

| Operation | rh-fsh | SUSHI | Notes |
|-----------|--------|-------|-------|
| 1,000 simple instances | ~4.5 ms | — | Local Criterion benchmark |
| Core schema field lookup | ~65 ns | — | Precompiled metadata lookup |
| Compiled profile-view lookup | ~25 ns | — | Shared immutable schema view |

> Representative Apple M-series measurements; exact results vary by machine and input corpus.

---

## Related

- [`rh-fsh` crate README](../../../crates/rh-fsh/README.md)
- [`rh-fsh` Architecture](../../../crates/rh-fsh/ARCHITECTURE.md)
- [FSH Specification](https://build.fhir.org/ig/HL7/fhir-shorthand/)
- [FSH Online Examples](https://github.com/FHIR/FSHOnline-Examples)
