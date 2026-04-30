# rh-packager

FHIR Package assembler — builds conformant [FHIR Packages](https://hl7.org/fhir/packages.html)
from a flat source directory of resources, markdown narrative, and CQL files.

Used by `rh package` subcommands in the `rh` CLI. For the end-user guide and CLI reference,
see [`apps/rh-cli/docs/PACKAGER.md`](../../apps/rh-cli/docs/PACKAGER.md).

## Source Directory Layout

```
my-package/
  packager.toml             # Required: package metadata + hook/processor configuration
  ImplementationGuide.json  # Required: IG resource (must sync with packager.toml)
  StructureDefinition-foo.json
  ValueSet-bar.json
  Library-MyLib.json        # Optional: created/updated by cql processor
  MyLib.cql                 # Optional: CQL source (processed by cql hook)
  MyProfile.fsh             # Optional: FSH source (processed by fsh hook)
  StructureDefinition-foo.md   # Narrative: embedded into matching resource .text
  docs/
    overview.md             # Standalone narrative: copied to package/other/
  fhir-lock.json            # Canonical version pins
```

`package.json` is **not** a source file — it is **generated** into the output directory
(`output/package/package.json`) during the build. All package metadata (`id`, `version`,
`canonical`, `fhir_version`, `dependencies`, etc.) lives in `packager.toml`.

---

## packager.toml Reference

`packager.toml` lives at the root of your source directory alongside `package.json`. All
sections are optional; absent sections use their defaults.

### Top-level fields

```toml
# Package / IG metadata — required for building.
id           = "my.package"                  # package name / IG packageId
version      = "1.0.0"                       # package version
fhir_version = "4.0.1"                       # FHIR version
canonical    = "https://example.org/fhir"    # canonical base URL
description  = "My FHIR Package"             # optional
author       = "My Organization"             # optional
license      = "CC0-1.0"                     # optional

# Shared FHIR packages cache used by all processors that need package resolution.
# Overridden per-processor with a section-level packages_dir.
# Default: ~/.fhir/packages
packages_dir = "/path/to/.fhir/packages"

# Package / IG display metadata
name         = "MyPackage"                   # default: PascalCase of id
status       = "active"                      # default: "draft"
publisher    = "My Organization"             # optional

# Dependency packages (maps directly to package.json dependencies)
[dependencies]
"hl7.fhir.r4.core" = "4.0.1"
```

| Field | Type | Default | Description |
|---|---|---|---|
| `id` | string | — | Package identifier (used as `package.json` `name` and IG `packageId`). |
| `version` | string | `"0.1.0"` | Package version. |
| `fhir_version` | string | — | FHIR version (e.g. `"4.0.1"`). |
| `canonical` | string | — | Canonical base URL for generated resources. |
| `description` | string | — | Package description written to `package.json`. |
| `author` | string | — | Author name written to `package.json`. |
| `license` | string | — | SPDX license identifier written to `package.json`. |
| `packages_dir` | string | `~/.fhir/packages` | Filesystem path to locally installed FHIR packages. Applied to every processor unless overridden in its own section. |
| `name` | string | PascalCase of `id` | Human-readable package name. |
| `version` | string | `"0.1.0"` | Version string for generated resources. |
| `status` | string | `"draft"` | Resource status (`active`, `draft`, `retired`, `unknown`). |
| `publisher` | string | — | Publisher name embedded in generated resources. |
| `[dependencies]` | map | `{}` | Dependency package map written to `package.json` `dependencies`. Keys must be quoted in TOML (e.g. `"hl7.fhir.r4.core" = "4.0.1"`). |

---

### `[hooks]`

Controls which processors run at each pipeline stage and in what order.

```toml
[hooks]
before_build = ["snapshot", "cql", "validate"]
after_build  = []
before_pack  = []
after_pack   = []
```

| Field | Type | Default | Description |
|---|---|---|---|
| `before_build` | list of strings | `[]` | Processors that run before the core build stage (narrative embedding, IG sync, index generation, lock application). |
| `after_build` | list of strings | `[]` | Processors that run after the build stage, before output is written to disk. |
| `before_pack` | list of strings | `[]` | Processors that run before the directory is packed into a `.tgz`. |
| `after_pack` | list of strings | `[]` | Processors that run after the `.tgz` is written. |

Each entry is a processor name. Unknown names cause an immediate error at startup, before any
files are touched. Processors execute in list order; the first failure aborts the pipeline.

#### Pipeline stage order

```
build
  ├─ [before_build]   ← processors declared here
  ├─ narrative        (embed .md into resource .text)
  ├─ ig_sync          (validate IG ↔ package.json)
  ├─ index            (generate .index.json)
  ├─ lock             (apply canonical pins from fhir-lock.json)
  ├─ [after_build]    ← processors declared here
  └─ write output dir

pack
  ├─ [before_pack]    ← processors declared here
  ├─ tarball          (.tgz written)
  └─ [after_pack]     ← processors declared here
```

> **Note:** `before_pack`/`after_pack` are only invoked by the `build` function (which runs pack
> as a final step). Calling `pack_dir()` directly on an already-built directory skips hooks because
> no source `packager.toml` is in scope.

---

### `[validate]`

Configuration for the built-in `validate` processor, which runs `rh-validator` over every
FHIR resource in the source directory.

```toml
[validate]
# Override the shared packages_dir for validation only.
# Default: inherits top-level packages_dir (or ~/.fhir/packages)
packages_dir = "/path/to/.fhir/packages"

# Reserved for future use — accepted but currently has no effect:
# skip_invariants    = false
# skip_bindings      = false
# terminology_server = "https://tx.fhir.org/r4"
```

| Field | Type | Default | Description |
|---|---|---|---|
| `packages_dir` | string | (top-level or `~/.fhir/packages`) | Packages cache used when loading dependency packages for validation. |
| `skip_invariants` | bool | `false` | **Reserved.** Will skip FHIRPath invariant evaluation when implemented. |
| `skip_bindings` | bool | `false` | **Reserved.** Will skip terminology binding checks when implemented. |
| `terminology_server` | string | — | **Reserved.** FHIR terminology server base URL for binding validation when implemented. |

The processor fails the pipeline if any resource has ERROR-severity issues.

---

### `[cql]`

Configuration for the built-in `cql` processor, which compiles `.cql` files to ELM JSON and
embeds them in matching `Library` resources.

```toml
[cql]
# Override the shared packages_dir for CQL package resolution only.
# Default: inherits top-level packages_dir (or ~/.fhir/packages)
packages_dir = "/path/to/.fhir/packages"

# Model info to use when compiling CQL.
# Default: "fhir"
model_info = "fhir"
```

| Field | Type | Default | Description |
|---|---|---|---|
| `packages_dir` | string | (top-level or `~/.fhir/packages`) | Packages cache used to resolve CQL library dependencies from FHIR packages. |
| `model_info` | string | `"fhir"` | CQL model info identifier passed to the compiler (e.g. `"fhir"`, `"qicore"`). |

The processor fails the pipeline on any CQL syntax or compilation error.

---

### `[fsh]`

Configuration for the built-in `fsh` processor, which compiles FHIR Shorthand (`*.fsh`) files
and injects the resulting resources into the build context.

Package/IG metadata (`canonical`, `fhir_version`, `id`, `name`, `status`, `publisher`,
`version`) are **root-level** `packager.toml` fields (see [Top-level fields](#top-level-fields))
and apply to all processors. The `[fsh]` section is reserved for future FSH-specific settings.

```toml
[fsh]
# Future FSH-specific options go here.
```

The processor scans the source directory **recursively** for `*.fsh` files, compiles them all in
one pass, and upserts each result into the resource map using the key `<ResourceType>-<id>`.
Non-fatal compilation warnings are logged but do not abort the pipeline. Fatal parse or resolve
errors abort the pipeline.

---

### `packages_dir` resolution order

For any processor that loads packages, the directory is chosen in this order:

1. Processor-level `packages_dir` (e.g. `[validate] packages_dir`)
2. Top-level `packages_dir` in `packager.toml`
3. `~/.fhir/packages`

---

### Built-in processors

| Name | When to use | Description |
|---|---|---|
| `snapshot` | `before_build` | Generates missing `snapshot.element` arrays for `StructureDefinition` resources. Loads base definitions from dependency packages. |
| `validate` | `before_build` or `after_build` | Validates all FHIR resources using `rh-validator`. Fails on any ERROR-severity issue. |
| `cql` | `before_build` | Compiles `.cql` files to ELM JSON, embeds source + ELM into `Library.content[]`. Auto-creates a minimal Library resource if none exists. |
| `fsh` | `before_build` | Compiles FHIR Shorthand (`*.fsh`) files into FHIR resources and injects them into the build context. All metadata config comes from root-level `packager.toml` fields. |

---

## Processors

The build pipeline supports **shell processors** (bash, Python, Node.js, any CLI) and
**native Rust processors**. See **[PROCESSORS.md](PROCESSORS.md)** for the complete guide,
including:

- Shell processor configuration and full execution contract
- Environment variables reference (`PACKAGER_SOURCE_DIR`, `PACKAGER_WORKDIR`, etc.)
- Resource exchange protocol (reading, modifying, and adding FHIR resources)
- Worked examples in bash, Python, and Node.js
- Native Rust `HookProcessor` trait implementation and registry API
- Built-in processor details (`snapshot`, `validate`, `cql`, `fsh`)

---

## fhir-lock.json Format

Records resolved canonical versions for reproducible builds. Written by the lock pipeline step.

```json
{
  "generated": "2025-01-01T00:00:00Z",
  "pinMode": "pin-all",
  "canonicals": [
    {
      "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
      "resolvedVersion": "6.1.0",
      "resolvedPackage": "hl7.fhir.us.core#6.1.0",
      "pinned": true
    }
  ]
}
```

Well-known external code systems (SNOMED CT, LOINC, RxNorm, ICD-10/11) are never pinned.

## package.json ↔ ImplementationGuide Sync

`package.json` is generated from `packager.toml` at build time. These fields must match
between the generated `package.json` and the `ImplementationGuide` resource:

| `packager.toml` | `package.json` | IG field |
|---|---|---|
| `id` | `name` | `packageId` |
| `version` | `version` | `version` |
| `fhir_version` | `fhirVersions[0]` | `fhirVersion` |
| `canonical` | `url` | `url` prefix |

## CQL ↔ Library Naming Convention

| CQL file | Matched Library resource |
|---|---|
| `MyLibrary.cql` | `Library-MyLibrary.json` |

If no matching Library exists, the `cql` processor auto-creates a minimal one and emits a
warning suggesting you check it in to source.
