# rh CLI - Publish Commands

## Overview

The `rh package` commands let you scaffold, build, and distribute conformant FHIR Packages.
Given a source directory of FHIR JSON resources, optional FHIR Shorthand (FSH), CQL libraries,
and markdown narrative, `rh package build` produces a spec-conformant `.tgz` tarball and
expanded output directory ready for registry upload or distribution.

The tool follows the [FHIR Package Specification](https://build.fhir.org/packages.html): every
package is a gzip tarball containing a `package/` folder with `package.json`, `.index.json`,
definitional FHIR resources at the top level, and examples in `package/examples/`.

See [`crates/rh-packager/README.md`](../../../crates/rh-packager/README.md) for the library
reference and [`crates/rh-packager/PROCESSORS.md`](../../../crates/rh-packager/PROCESSORS.md)
for the custom processor API.

---

## Commands

### `rh package init`

Scaffold a new FHIR Package source directory with `packager.toml` and a
minimal `ImplementationGuide.json`.

**Usage:**
```bash
rh package init [DIR] --canonical <URL> [--name <NAME>] [OPTIONS]
```

**Arguments:**
- `[DIR]` — Target directory (default: current directory)

**Required options:**
- `-c, --canonical <URL>` — Canonical URL base for resources (e.g. `https://example.org/fhir`)

**Optional options:**
- `-n, --name <NAME>` — Package name (e.g. `hl7.fhir.us.core`). Inferred from `--canonical` when absent — see naming convention below.
- `--version <VERSION>` — Package version [default: `0.1.0`]
- `--fhir-version <VERSION>` — FHIR version [default: `4.0.1`]
- `--title <TITLE>` — Human-readable title (default: PascalCase of name)
- `--description <TEXT>` — Package description
- `--author <NAME>` — Author or publisher name
- `--license <SPDX>` — SPDX license identifier [default: `CC0-1.0`]
- `--status <STATUS>` — IG resource status (`draft`|`active`|`retired`) [default: `draft`]

#### Package naming convention

FHIR package names follow the pattern `<domain>.<path-segments>`, derived from the canonical URL:

| Canonical URL | Package name |
|---|---|
| `http://hl7.org/fhir/us/core` | `hl7.fhir.us.core` |
| `http://hl7.org/fhir/uv/extensions` | `hl7.fhir.uv.extensions` |
| `https://example.org/fhir` | `example.fhir` |
| `https://example.org/fhir/my-ig` | `example.fhir.my-ig` |

The second-level domain (e.g. `hl7` from `hl7.org`) becomes the first segment; remaining path
segments follow. When `--name` is omitted, `rh package init` derives the name automatically
and prints it so you can confirm or override.

The `ImplementationGuide.packageId` field is always set to the package name, keeping the
IG and `package.json` in sync.

**Examples:**
```bash
# Name inferred from canonical — prints "Name derived from canonical: example.fhir"
rh package init my-package \
  --canonical https://example.org/fhir \
  --title "Example FHIR Package" \
  --author "Example Organization"

# Explicit name override
rh package init my-package \
  --canonical https://example.org/fhir \
  --name com.example.fhir \
  --title "Example FHIR Package"

# R4B package
rh package init my-r4b-package \
  --canonical https://example.org/fhir \
  --fhir-version 4.3.0
```

**Generated files:**

| File | Description |
|------|-------------|
| `packager.toml` | Package metadata and hook configuration skeleton |
| `ImplementationGuide.json` | Minimal IG with `id`, `name`, `url`, `packageId`, and `dependsOn` |

`package.json` is **not** created here — it is generated into `output/package/` during `rh package build`.
Returns an error if `packager.toml` already exists in the target directory.

---

### `rh package build`

Build a FHIR Package from a source directory.

**Usage:**
```bash
rh package build [OPTIONS] <DIR>
```

**Arguments:**
- `<DIR>` — Path to the source directory containing `packager.toml` and FHIR resources

**Options:**
- `-o, --out <PATH>` — Output directory for the expanded package (default: `<DIR>/output`)

**Examples:**
```bash
# Build with default output location (DIR/output/)
rh package build my-package/

# Build with a custom output directory
rh package build my-package/ --out /tmp/build
```

The build pipeline:
1. Runs `before_build` hook processors (from `packager.toml`)
2. Processes markdown narrative (embeds `.md` files into matching resource `.text`)
3. Syncs and validates `ImplementationGuide.json` against `packager.toml`
4. Auto-populates `ImplementationGuide` `dependsOn`, `definition.resource`, and `definition.page`
5. Applies canonical pinning from `fhir-lock.json` (if present)
6. Generates `package/.index.json` and `package/examples/.index.json`
7. Runs `after_build` hook processors
8. Writes the expanded output directory (including generated `package.json`)
9. Creates the `.tgz` tarball

---

### `rh package lock`

Resolve canonical references in source resources and write `fhir-lock.json`.

**Usage:**
```bash
rh package lock <DIR>
```

**Arguments:**
- `<DIR>` — Path to the source directory

**Example:**
```bash
rh package lock my-package/
```

Scans all FHIR resources for unversioned canonical references in fields of FHIR type `canonical`
(e.g. `baseDefinition`, `valueSet`, `profile`, `targetProfile`). Resolves each against:
1. Source resources in the same package (own canonicals)
2. Locally cached dependency packages (`~/.fhir/packages/`)

Writes a `fhir-lock.json` recording resolved `url → version` mappings. Run `rh download` first
to ensure dependencies are cached locally.

Well-known external code systems (SNOMED CT, LOINC, RxNorm, ICD-10/11) and base FHIR core
definitions are excluded from pinning.

---

### `rh package lock-check`

Report canonical reference pinning status without modifying any files.

**Usage:**
```bash
rh package lock-check <DIR>
```

**Arguments:**
- `<DIR>` — Path to the source directory

**Example:**
```bash
rh package lock-check my-package/
```

Scans all FHIR resources for canonical-typed fields and reports which references are already
pinned (have `|version`) and which are unversioned. Useful for auditing a package before
publishing or after adding new resources.

**Sample output:**
```
PINNED (2):
  http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient|7.0.0 (in: StructureDefinition-bp, field: baseDefinition)
  http://example.org/fhir/ValueSet/bp-status|1.0.0 (in: StructureDefinition-bp, field: snapshot.element[0].binding.valueSet)

UNPINNED (1):
  http://example.org/fhir/StructureDefinition/base (in: StructureDefinition-child, field: baseDefinition)

Run `rh package lock my-package/` to pin all unversioned references.
```

### `rh package check`

Validate source directory structure without building any output.

**Usage:**
```bash
rh package check <DIR>
```

**Arguments:**
- `<DIR>` — Path to the source directory

**Example:**
```bash
rh package check my-package/
```

Checks that `packager.toml` and `ImplementationGuide.json` are present and consistent, and that
`fhir-lock.json` is up to date if present. No output is written.

---

### `rh package pack`

Pack an already-built expanded output directory into a `.tgz` tarball.

**Usage:**
```bash
rh package pack [OPTIONS] <DIR>
```

**Arguments:**
- `<DIR>` — Path to the expanded output directory (must contain a `package/` subdirectory)

**Options:**
- `-o, --out <PATH>` — Output `.tgz` path (default: `<DIR>/../<name>-<version>.tgz`)

**Example:**
```bash
rh package pack my-package/output/
```

---

## Source Directory Layout

The source directory can be organised however you like. By convention:

```
my-package/
  packager.toml                 # Package metadata and hook configuration (required)
  ImplementationGuide.json      # ImplementationGuide resource (required)
  StructureDefinition-foo.json  # Definitional FHIR resources
  ValueSet-bar.json
  StructureDefinition-foo.md    # Narrative embedded into matching resource
  fsh/
    MyProfile.fsh               # FSH source (compiled by fsh processor)
  cql/
    MyLibrary.cql               # CQL source (compiled by cql processor)
  examples/
    Patient-example.json        # Example resources — published to package/examples/
  docs/
    overview.md                 # Standalone narrative (copied to package/other/)
    introduction.md
  fhir-lock.json                # Generated by rh package lock
```

**Source → published output mapping:**

| Source | Published location |
|--------|--------------------|
| `*.json` (FHIR resources) | `package/*.json` |
| `examples/**/*.json` | `package/examples/*.json` |
| `*.md` matching a resource stem | Embedded as `resource.text` |
| `docs/*.md` (standalone narrative) | `package/other/*.md` |

---

## Published Package Layout

Per the FHIR Package Specification, the output is a gzip tarball containing:

```
package/
  package.json              # Manifest
  .index.json               # Resource index (auto-generated)
  ImplementationGuide.json
  StructureDefinition-foo.json
  ValueSet-bar.json
  examples/
    .index.json             # Example resource index (auto-generated)
    Patient-example.json
  other/
    overview.md             # Standalone narrative
```

---

## `packager.toml` Configuration

`packager.toml` is the **sole source of truth** for all package metadata and hook configuration.
All metadata you would previously put in `package.json` goes here. `package.json` is **generated**
into the output directory during `rh package build` — never into the source directory.

```toml
# Package / IG metadata
id           = "example.fhir.my-package"   # required (package name)
version      = "1.0.0"                     # required
fhir_version = "4.0.1"                     # required
canonical    = "https://example.org/fhir"  # required
description  = "My FHIR Package"           # optional
author       = "Example Organization"      # optional
license      = "CC0-1.0"                   # optional

# Shared FHIR packages cache (default: ~/.fhir/packages)
# packages_dir = "~/.fhir/packages"

# Package / IG display metadata
# name         = "MyPackage"               # default: PascalCase of id
# status       = "draft"                   # default: "draft"
# publisher    = "My Organization"         # optional

# Dependency packages (used to generate package.json dependencies)
[dependencies]
"hl7.fhir.r4.core" = "4.0.1"

[hooks]
# Processors run in order before the build stage.
# Built-in processors: "fsh", "snapshot", "cql", "validate"
before_build = ["fsh", "snapshot", "cql", "validate"]
after_build  = []
before_pack  = []
after_pack   = []

[validate]
# terminology_server = "https://tx.fhir.org/r4"
# skip_invariants = false
# skip_bindings = false

[cql]
# packages_dir = "~/.fhir/packages"

[fsh]
# Future FSH-specific options go here.

# Custom shell processor
[processors.my-script]
command = "./scripts/my_script.sh"
# working_dir = "."
# timeout_secs = 60
# [processors.my-script.env]
# MY_VAR = "value"
```

Shell processors receive these environment variables:

| Variable | Description |
|---|---|
| `PACKAGER_SOURCE_DIR` | Absolute path to the source directory |
| `PACKAGER_OUTPUT_DIR` | Absolute path to the output directory being assembled |
| `PACKAGER_WORKDIR` | Temporary directory for resource exchange (see below) |
| `PACKAGER_PACKAGE_NAME` | Package name from `packager.toml` |
| `PACKAGER_PACKAGE_VERSION` | Package version from `packager.toml` |
| `PACKAGER_FHIR_VERSIONS` | Comma-separated FHIR versions from `packager.toml` |

To read or modify FHIR resources from within a shell processor, use `$PACKAGER_WORKDIR/resources/`.
Files are named `<ResourceType>-<id>.json`. Changes written back to this directory are picked up
by the packager after the script exits successfully.

---

## Built-in Processors

The packager ships four built-in processors. Reference them by name in `packager.toml` hook
lists and configure them with their own `[<name>]` section.

---

### `fsh` — FHIR Shorthand compiler

Compiles FHIR Shorthand (`*.fsh`) files into FHIR resources and injects them into the build
context before the core build stage runs.

**Typical stage:** `before_build`

Package/IG metadata fields (`canonical`, `fhir_version`, `id`, `name`, `status`, `publisher`,
`version`) are **root-level** `packager.toml` fields shared by all processors. Set them once at
the top of `packager.toml` — all processors read them from there.

```toml
# Root-level — not under [fsh]
canonical    = "https://example.org/fhir"  # canonical base URL
fhir_version = "4.0.1"                    # FHIR version string
id           = "my.package"               # package id
name         = "MyPackage"                # human-readable name
version      = "1.0.0"                    # version string
status       = "draft"                    # default: "draft"
publisher    = "My Organization"          # optional

[fsh]
# Future FSH-specific options go here.
```

| Field | Default | Description |
|---|---|---|
| `canonical` | — | Canonical base URL for generated resources |
| `fhir_version` | — | FHIR version string (e.g. `"4.0.1"`) |
| `id` | — | Package id embedded in generated resources |
| `name` | — | Human-readable package name |
| `version` | — | Version string for generated resources |
| `status` | `"draft"` | Resource status for all generated resources |
| `publisher` | — | Publisher name embedded in generated resources |

**Behavior:**

- Scans the source directory **recursively** for `*.fsh` files and compiles them in a single pass (definitions in one file may reference definitions in another).
- Each compiled resource is inserted into the build context as `<ResourceType>-<id>`.
- If a resource with the same key already exists (e.g. a pre-compiled JSON file), the FSH version overwrites it with a warning. Use FSH _or_ JSON for a given resource, not both.
- Non-fatal compilation warnings are logged; fatal parse or resolve errors abort the pipeline.

---

### `snapshot` — StructureDefinition snapshot expander

Generates missing `snapshot.element` arrays for any `StructureDefinition` resource that has a
`differential` but no `snapshot`. Loads base definitions from locally cached dependency packages.

**Typical stage:** `before_build` (run after `fsh`, before `validate`)

**`packager.toml` configuration:**

```toml
[snapshot]
# Override the shared packages cache for snapshot resolution only.
# packages_dir = "~/.fhir/packages"
```

| Field | Default | Description |
|---|---|---|
| `packages_dir` | top-level `packages_dir` or `~/.fhir/packages` | Path to locally installed FHIR packages used to resolve base definitions |

**Behavior:**

- Skips `StructureDefinition` resources that already have a `snapshot.element` array.
- Resolves the base profile from the `baseDefinition` URL against dependency packages.
- Fails the pipeline if a base definition cannot be found in the local cache.
- Run `rh download <package>#<version>` first to ensure all dependency packages are available.

---

### `validate` — FHIR resource validator

Validates all FHIR resources in the build context using `rh-validator`, checking conformance
against the base FHIR specification and any declared profiles.

**Typical stage:** `before_build` (after `snapshot` so profiles are fully expanded) or `after_build`

**`packager.toml` configuration:**

```toml
[validate]
# packages_dir = "~/.fhir/packages"

# Reserved for future use:
# skip_invariants    = false
# skip_bindings      = false
# terminology_server = "https://tx.fhir.org/r4"
```

| Field | Default | Description |
|---|---|---|
| `packages_dir` | top-level `packages_dir` or `~/.fhir/packages` | Packages cache used when loading dependency packages for validation |
| `skip_invariants` | `false` | **Reserved.** Will skip FHIRPath invariant evaluation when implemented |
| `skip_bindings` | `false` | **Reserved.** Will skip terminology binding checks when implemented |
| `terminology_server` | — | **Reserved.** FHIR terminology server URL for binding validation when implemented |

**Behavior:**

- Fails the pipeline if any resource has an ERROR-severity validation issue.
- Validation warnings are logged but do not abort the pipeline.
- Placing `validate` after `snapshot` in `before_build` ensures profiles are fully expanded before checking.

---

### `cql` — CQL to ELM compiler

Compiles CQL (Clinical Quality Language) source files to ELM JSON and embeds both the source
and compiled ELM into matching `Library` resources as `content[]` attachments.

**Typical stage:** `before_build`

**`packager.toml` configuration:**

```toml
[cql]
# packages_dir = "~/.fhir/packages"
# model_info = "fhir"
```

| Field | Default | Description |
|---|---|---|
| `packages_dir` | top-level `packages_dir` or `~/.fhir/packages` | Packages cache for resolving CQL library dependencies from FHIR packages |
| `model_info` | `"fhir"` | CQL model info identifier passed to the compiler (e.g. `"fhir"`, `"qicore"`) |

**Behavior:**

- Scans the source directory recursively for `*.cql` files.
- Matches each CQL library to a `Library-<name>.json` resource by the CQL library name.
- If no matching `Library` resource exists, the processor auto-creates a minimal one and logs a warning recommending you check it into source.
- Embeds the base64-encoded CQL source and compiled ELM JSON as `content[]` attachments on the Library resource.
- Fails the pipeline on any CQL syntax or compilation error.

**CQL ↔ Library naming:**

| CQL file | Matched Library resource |
|---|---|
| `BpCheck.cql` | `Library-BpCheck.json` |

---

## Shell Processors (Custom Hooks)

In addition to the built-in processors above, you can run any external command as a named
pipeline stage using `[processors.<name>]` in `packager.toml`. The command can be a shell
one-liner, a script in any language, or any CLI tool.

See [`crates/rh-packager/PROCESSORS.md`](../../../crates/rh-packager/PROCESSORS.md) for the
full custom processor reference, including:

- Environment variables reference (`PACKAGER_SOURCE_DIR`, `PACKAGER_WORKDIR`, etc.)
- Resource exchange protocol (reading, modifying, and adding FHIR resources via `$PACKAGER_WORKDIR/resources/`)
- Worked examples in bash, Python, and Node.js
- Native Rust `HookProcessor` trait for embedding processors directly in Rust code
- Error handling and processor ordering semantics

---

## Step-by-Step Guide

This guide walks through building a complete FHIR Package from scratch using all built-in
processors. See [PROCESSORS.md](../../../crates/rh-packager/PROCESSORS.md) for the full
processor API reference.

### Prerequisites

- `rh` installed (see [README.md](../README.md))
- FHIR R4 core package cached: `rh download hl7.fhir.r4.core#4.0.1`

---

### Step 1 — Initialize the package

```bash
rh package init bp-profiles \
  --canonical http://example.org/fhir/bp-profiles \
  --title "Blood Pressure Profiles" \
  --author "Example Organization"
# → Name derived from canonical: example.fhir.bp-profiles
```

This creates:
```
bp-profiles/
  packager.toml
  ImplementationGuide.json
```

Review and edit these files as needed. The `ImplementationGuide.json` has a minimal skeleton —
`definition.resource`, `dependsOn`, and `definition.page` are all **auto-populated at build time**
from the scanned resources and `packager.toml` dependencies, so you do not need to maintain them
manually.

`package.json` is **generated** into `bp-profiles/output/package/` during `rh package build`.

---

### Step 2 — Configure the pipeline

Edit `bp-profiles/packager.toml` to enable the built-in processors:

```toml
[hooks]
before_build = ["fsh", "snapshot", "validate", "cql"]
```

> **Processor order matters.** `fsh` compiles source to FHIR JSON, `snapshot` expands
> differentials so `validate` can check fully-resolved resources, and `cql` can reference
> the validated profiles.

---

### Step 3 — Author a profile with FHIR Shorthand

Create `bp-profiles/fsh/BpObservation.fsh`:

```fsh
Alias: $LNC = http://loinc.org

Profile: BpObservation
Parent: Observation
Id: bp-observation
Title: "Blood Pressure Observation"
Description: "A blood pressure panel with systolic and diastolic components."

* status MS
* code = $LNC#55284-4 "Blood pressure systolic and diastolic"
* subject 1..1 MS
* subject only Reference(Patient)
* component ^slicing.discriminator.type = #pattern
* component ^slicing.discriminator.path = "code"
* component ^slicing.rules = #open

* component contains
    systolic 1..1 MS and
    diastolic 1..1 MS

* component[systolic].code = $LNC#8480-6 "Systolic blood pressure"
* component[systolic].value[x] only Quantity
* component[systolic].valueQuantity.unit = "mmHg"
* component[systolic].valueQuantity.system = "http://unitsofmeasure.org"
* component[systolic].valueQuantity.code = #mm[Hg]

* component[diastolic].code = $LNC#8462-4 "Diastolic blood pressure"
* component[diastolic].value[x] only Quantity
* component[diastolic].valueQuantity.unit = "mmHg"
* component[diastolic].valueQuantity.system = "http://unitsofmeasure.org"
* component[diastolic].valueQuantity.code = #mm[Hg]
```

The `fsh` processor compiles this to `StructureDefinition-bp-observation.json` during the build.

---

### Step 4 — Add narrative

Place a markdown file next to any resource (matching its filename stem) to embed it as
`resource.text` during the build. Create `bp-profiles/StructureDefinition-bp-observation.md`:

```markdown
## Blood Pressure Observation

This profile constrains the base FHIR `Observation` resource to represent a blood pressure
measurement with systolic and diastolic component values.

### Usage

Use this profile when recording blood pressure readings from a patient encounter. Both
`component[systolic]` and `component[diastolic]` are mandatory.
```

For standalone narrative (overview pages, introduction docs) that are not tied to a specific
resource, place them in `docs/`. Create `bp-profiles/docs/overview.md`:

```markdown
# Blood Pressure Profiles

This package defines FHIR profiles for recording blood pressure observations.
```

During the build, `StructureDefinition-bp-observation.md` is converted to XHTML and embedded
as `resource.text`, while `docs/overview.md` is copied to `package/other/overview.md` and
registered in `ImplementationGuide.definition.page` as a child page with `nameUrl: "other/overview.md"`.
The page `title` is parsed from the first `# Heading` in the file — in this case `"Blood Pressure Profiles"`.

---

### Step 5 — Add CQL clinical logic

Create `bp-profiles/cql/BpCheck.cql`:

```cql
library BpCheck version '0.1.0'

using FHIR version '4.0.1'

include FHIRHelpers version '4.0.1' called FHIRHelpers

codesystem "LOINC": 'http://loinc.org'

code "BP Panel": '55284-4' from "LOINC" display 'Blood pressure systolic and diastolic'

context Patient

define "Blood Pressure Observations":
  [Observation: "BP Panel"] O
    where O.status in { 'final', 'amended' }

define "High Systolic Readings":
  "Blood Pressure Observations" O
    where exists (
      O.component C
        where C.code ~ "BP Panel"
          and (C.value as Quantity).value > 140
    )

define "Has Hypertensive Reading":
  exists "High Systolic Readings"
```

The `cql` processor compiles this and upserts `Library-BpCheck.json` into the package with
the ELM and base64-encoded CQL source embedded as `content` attachments.

---

### Step 6 — Add example resources

Create `bp-profiles/examples/Observation-bp-example.json`:

```json
{
  "resourceType": "Observation",
  "id": "bp-example",
  "meta": {
    "profile": ["http://example.org/fhir/bp-profiles/StructureDefinition/bp-observation"]
  },
  "status": "final",
  "code": {
    "coding": [{ "system": "http://loinc.org", "code": "55284-4" }]
  },
  "subject": { "reference": "Patient/example" },
  "component": [
    {
      "code": { "coding": [{ "system": "http://loinc.org", "code": "8480-6" }] },
      "valueQuantity": { "value": 120, "unit": "mmHg", "system": "http://unitsofmeasure.org" }
    },
    {
      "code": { "coding": [{ "system": "http://loinc.org", "code": "8462-4" }] },
      "valueQuantity": { "value": 80, "unit": "mmHg", "system": "http://unitsofmeasure.org" }
    }
  ]
}
```

Resources in the `examples/` subdirectory are published to `package/examples/` with their
own `.index.json`.

---

### Step 7 — Build the package

```bash
rh package build bp-profiles/
```

Expected output:
```
[fsh]       Compiling 1 FSH file(s)...
[fsh]       ✓ StructureDefinition/bp-observation
[snapshot]  Expanding 1 StructureDefinition(s)...
[snapshot]  ✓ StructureDefinition/bp-observation
[validate]  Validating 3 resource(s)...
[validate]  ✓ All resources valid.
[cql]       Compiling 1 CQL file(s)...
[cql]       ✓ Library/BpCheck
Building package...
✓ Wrote output/package/
✓ Packed example.fhir.bp-profiles-0.1.0.tgz
```

Output layout:
```
bp-profiles/
  output/
    package/
      package.json
      .index.json
      ImplementationGuide-example.fhir.bp-profiles.json
      StructureDefinition-bp-observation.json
      Library-BpCheck.json
      examples/
        .index.json
        Observation-bp-example.json
  example.fhir.bp-profiles-0.1.0.tgz
```

---

### Step 8 — Lock canonical references

```bash
rh package lock bp-profiles/
```

This scans all FHIR resources for unversioned canonical references (in canonical-typed fields
such as `baseDefinition`, `valueSet`, `profile`, `targetProfile`), resolves them against source
resources in the same package and locally cached dependency packages, and writes `fhir-lock.json`.
Run `rh download` first for any dependencies not yet cached.

To audit pinning status without modifying any files:

```bash
rh package lock-check bp-profiles/
```

### Extending the Pipeline — Custom Hook Scripts

Shell processors can be written in any language. This example uses Python, but the same logic
works in bash, Ruby, Node.js, or any other scripting language — only the `command` in
`packager.toml` changes.

#### `scripts/stamp_date.py` — inject build timestamp

```python
#!/usr/bin/env python3
"""Stamp a build-date tag onto every FHIR resource in the pipeline."""
import json, os, sys
from datetime import datetime, timezone
from pathlib import Path

resources_dir = Path(os.environ["PACKAGER_WORKDIR"]) / "resources"
if not resources_dir.exists():
    sys.exit(0)

build_date = datetime.now(timezone.utc).strftime("%Y-%m-%d")
tag = {"system": "http://example.org/tags", "code": "build-date", "display": build_date}

for path in resources_dir.glob("*.json"):
    resource = json.loads(path.read_text())
    meta = resource.setdefault("meta", {})
    tags = [t for t in meta.setdefault("tag", [])
            if not (t.get("system") == tag["system"] and t.get("code") == "build-date")]
    tags.append(tag)
    meta["tag"] = tags
    path.write_text(json.dumps(resource, indent=2))
    print(f"stamp_date: stamped {path.name} with build-date={build_date}")
```

#### `scripts/check_date.py` — validate build timestamp is present

```python
#!/usr/bin/env python3
"""Validate that every FHIR resource carries the build-date tag."""
import json, os, sys
from pathlib import Path

resources_dir = Path(os.environ["PACKAGER_WORKDIR"]) / "resources"
if not resources_dir.exists():
    sys.exit(0)

errors = []
for path in resources_dir.glob("*.json"):
    resource = json.loads(path.read_text())
    tags = resource.get("meta", {}).get("tag", [])
    if not any(t.get("system") == "http://example.org/tags" and t.get("code") == "build-date"
               for t in tags):
        errors.append(path.name)

if errors:
    print("check_date: ERROR — missing build-date tag on:", file=sys.stderr)
    for name in errors:
        print(f"  - {name}", file=sys.stderr)
    sys.exit(1)

print(f"check_date: all {len(list(resources_dir.glob('*.json')))} resource(s) have build-date tag ✓")
```

#### Wire in `packager.toml`

```toml
[hooks]
before_build = ["fsh", "snapshot", "stamp-date", "validate", "cql"]
after_build  = ["check-date"]

[processors.stamp-date]
command = "python3 scripts/stamp_date.py"

[processors.check-date]
command = "python3 scripts/check_date.py"
```

`stamp-date` runs before `validate` so the tag is present when resources are checked.
`check-date` runs after the build as a final quality gate — if any resource is missing the
tag (e.g. because `stamp-date` was accidentally removed), the pipeline aborts:

```
[processor:check-date] check_date: ERROR — missing build-date tag on:
[processor:check-date]   - StructureDefinition-bp-observation.json
error: processor 'check-date' exited with code 1 — pipeline aborted
```

---

## Related Documentation

- [Publisher library README](../../../crates/rh-packager/README.md) — full configuration reference
- [Processor API](../../../crates/rh-packager/PROCESSORS.md) — custom Rust and shell processor contracts
- [FHIR Package Specification](https://build.fhir.org/packages.html)
