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

Scaffold a new FHIR Package source directory with `package.json`, `packager.toml`, and a
minimal `ImplementationGuide.json`.

**Usage:**
```bash
rh package init [DIR] --name <NAME> --canonical <URL> [OPTIONS]
```

**Arguments:**
- `[DIR]` — Target directory (default: current directory)

**Required options:**
- `-n, --name <NAME>` — Package identifier in reverse-DNS NPM style (e.g. `com.example.fhir`)
- `-c, --canonical <URL>` — Canonical URL base for resources (e.g. `https://example.org/fhir`)

**Optional options:**
- `--version <VERSION>` — Package version [default: `0.1.0`]
- `--fhir-version <VERSION>` — FHIR version [default: `4.0.1`]
- `--title <TITLE>` — Human-readable title (default: PascalCase of name)
- `--description <TEXT>` — Package description
- `--author <NAME>` — Author or publisher name
- `--license <SPDX>` — SPDX license identifier [default: `CC0-1.0`]
- `--status <STATUS>` — IG resource status (`draft`|`active`|`retired`) [default: `draft`]

**Examples:**
```bash
# Scaffold a package in a new directory
rh package init my-package \
  --name com.example.bp-profiles \
  --canonical https://example.org/fhir \
  --title "Blood Pressure Profiles" \
  --author "Example Organization"

# Scaffold in the current directory
rh package init \
  --name com.example.fhir \
  --canonical https://example.org/fhir

# R4B package
rh package init my-r4b-package \
  --name com.example.r4b \
  --canonical https://example.org/fhir \
  --fhir-version 4.3.0
```

**Generated files:**

| File | Description |
|------|-------------|
| `package.json` | FHIR package manifest with name, version, FHIR version, and base dependency |
| `packager.toml` | Hook configuration skeleton (all stages empty, built-ins commented) |
| `ImplementationGuide.json` | Minimal IG with `id`, `name`, `url`, `packageId`, and `dependsOn` |

Returns an error if `package.json` already exists in the target directory.

---

### `rh package build`

Build a FHIR Package from a source directory.

**Usage:**
```bash
rh package build [OPTIONS] <DIR>
```

**Arguments:**
- `<DIR>` — Path to the source directory containing `package.json` and FHIR resources

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
3. Syncs and validates `ImplementationGuide.json` against `package.json`
4. Auto-populates `ImplementationGuide` `dependsOn`, `definition.resource`, and `definition.page`
5. Applies canonical pinning from `fhir-lock.json` (if present)
6. Generates `package/.index.json` and `package/examples/.index.json`
7. Runs `after_build` hook processors
8. Writes the expanded output directory
9. Creates the `.tgz` tarball

---

### `rh package lock`

Resolve external canonical references in source resources and write `fhir-lock.json`.

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

Scans all FHIR resources for unversioned canonical references, resolves each against locally
cached dependency packages (`~/.fhir/packages/`), and writes a `fhir-lock.json` file
recording resolved `url → version` mappings. Run `rh download` first to ensure dependencies
are cached locally.

---

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

Checks that `package.json` and `ImplementationGuide.json` are present and consistent, and that
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
  package.json                  # FHIR package manifest (required)
  packager.toml                 # Hook configuration (optional)
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

```toml
# Shared FHIR packages cache (default: ~/.fhir/packages)
# packages_dir = "~/.fhir/packages"

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
# canonical = "https://example.org/fhir"  # inferred from package.json when absent

# Custom shell processor
[processors.my-script]
command = "python3 scripts/my_script.py"
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
| `PACKAGER_PACKAGE_NAME` | Package name from `package.json` |
| `PACKAGER_PACKAGE_VERSION` | Package version from `package.json` |
| `PACKAGER_FHIR_VERSIONS` | Comma-separated FHIR versions from `package.json` |

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

**`packager.toml` configuration** (all fields optional — inferred from `package.json` when absent):

```toml
[fsh]
canonical    = "https://example.org/fhir"  # inferred from package.json url
fhir_version = "4.0.1"                    # inferred from package.json fhirVersions[0]
id           = "my.package"               # inferred from package.json name
name         = "MyPackage"                # inferred from package.json name
version      = "1.0.0"                    # inferred from package.json version
status       = "draft"                    # default: "draft"
publisher    = "My Organization"          # optional
```

| Field | Default | Description |
|---|---|---|
| `canonical` | `package.json` `url` | Canonical base URL for generated resources |
| `fhir_version` | `package.json` `fhirVersions[0]` | FHIR version string (e.g. `"4.0.1"`) |
| `id` | `package.json` `name` | Package id embedded in generated resources |
| `name` | `package.json` `name` | Human-readable package name |
| `version` | `package.json` `version` | Version string for generated resources |
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
  --name example.bp-profiles \
  --canonical http://example.org/fhir/bp-profiles \
  --title "Blood Pressure Profiles" \
  --author "Example Organization"
```

This creates:
```
bp-profiles/
  package.json
  packager.toml
  ImplementationGuide.json
```

Review and edit these files as needed. The `ImplementationGuide.json` has a minimal skeleton —
`definition.resource`, `dependsOn`, and `definition.page` are all **auto-populated at build time**
from the scanned resources and `package.json` dependencies, so you do not need to maintain them
manually.

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
✓ Packed example.bp-profiles-0.1.0.tgz
```

Output layout:
```
bp-profiles/
  output/
    package/
      package.json
      .index.json
      ImplementationGuide-example.bp-profiles.json
      StructureDefinition-bp-observation.json
      Library-BpCheck.json
      examples/
        .index.json
        Observation-bp-example.json
  example.bp-profiles-0.1.0.tgz
```

---

### Step 8 — Lock canonical references

```bash
rh package lock bp-profiles/
```

This scans all resources for unversioned canonical references, resolves them against locally
cached packages, and writes `fhir-lock.json`. Run `rh download` first for any dependencies
not yet cached.

---

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
