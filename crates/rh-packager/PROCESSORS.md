# rh-packager Processor Guide

Processors are the extension point of the rh-packager pipeline. They run at defined lifecycle
stages (before/after build, before/after pack) and can read and mutate the in-memory FHIR
resource map before it is written to disk.

There are two kinds of processors:

| Kind | When to use |
|---|---|
| **Shell processor** | Any language: bash, Python, Node.js, Ruby, or any CLI tool |
| **Native Rust processor** | High-performance or deep integration with rh crate internals |

Both kinds are referenced by name in `packager.toml` and share the same lifecycle model. The
sections below cover each in depth.

---

## 1. Shell Processors

Shell processors let you run any external command — a bash one-liner, a Python script, a Node
tool, or anything that has a CLI — as a named pipeline stage.

### Configuration

Declare a shell processor under `[processors.<name>]` in `packager.toml`:

```toml
[processors.enrich]
command     = "python scripts/enrich.py"
working_dir = "."           # optional; relative to source dir (default: source dir)
timeout_secs = 120          # optional; reserved — accepted but not yet enforced

[processors.enrich.env]     # optional; extra environment variables
STRICT = "1"
API_KEY = "abc123"
```

Then reference the processor name in a hook stage list:

```toml
[hooks]
before_build = ["snapshot", "enrich", "validate"]
after_build  = ["audit-log"]
```

Multiple processors may be declared and used across different stages:

```toml
[processors.lint]
command = "./scripts/lint.sh"

[processors.audit-log]
command = "node scripts/audit.mjs"

[hooks]
before_build = ["lint"]
after_build  = ["audit-log"]
```

### Execution environment

When a shell processor runs, the packager:

1. Creates a **temporary working directory** (`$PACKAGER_WORKDIR`).
2. Writes each in-memory FHIR resource as a JSON file into `$PACKAGER_WORKDIR/resources/`.
3. Sets [environment variables](#environment-variables) and launches the command.
4. Waits for the command to exit.
5. If the exit code is **zero**: reads back all JSON files from `$PACKAGER_WORKDIR/resources/`
   and upserts them into the resource map.
6. If the exit code is **non-zero**: the pipeline aborts with an error.
7. The temporary directory is deleted after the processor completes (success or failure).

#### Working directory

The command runs with its current directory set to:

- `<source_dir>/<working_dir>` — if `working_dir` is set in config
- `<source_dir>` — otherwise

`<source_dir>` is the directory passed to `rh publish build` (the one containing
`package.json`).

#### Command execution

The command string is passed to the OS shell:

- **Unix/macOS**: `sh -c "<command>"`
- **Windows**: `cmd /C "<command>"`

This means you can use shell features: pipes, redirects, environment variable expansion,
conditionals, etc.

#### Environment variables

The following variables are set for every shell processor invocation:

| Variable | Value |
|---|---|
| `PACKAGER_SOURCE_DIR` | Absolute path to the source directory |
| `PACKAGER_OUTPUT_DIR` | Absolute path to the configured output directory |
| `PACKAGER_WORKDIR` | Absolute path to the processor's temporary working directory |
| `PACKAGER_PACKAGE_NAME` | `name` field from `package.json` |
| `PACKAGER_PACKAGE_VERSION` | `version` field from `package.json` |
| `PACKAGER_FHIR_VERSIONS` | Comma-separated FHIR versions from `package.json` |

Variables defined under `[processors.<name>.env]` are applied on top of these and take
precedence. The processor also inherits the parent process environment (`PATH`, `HOME`, etc.).

### Resource exchange protocol

The mechanism by which a shell processor reads and modifies FHIR resources:

```
Before invocation                     After invocation
─────────────────                     ────────────────
$PACKAGER_WORKDIR/                    $PACKAGER_WORKDIR/
  resources/                            resources/
    StructureDefinition-foo.json          StructureDefinition-foo.json  ← may be modified
    ValueSet-bar.json                     ValueSet-bar.json
    ImplementationGuide.json              ImplementationGuide.json
                                          ValueSet-new.json             ← new resource added
```

- **Read**: The JSON files written to `resources/` before invocation represent the current
  resource map entering this stage. Filenames are `<stem>.json` where `<stem>` is the key
  used internally (e.g. `StructureDefinition-foo` for `StructureDefinition-foo.json`).

- **Modify**: Overwrite any file in `resources/` to update the resource for the rest of the
  pipeline.

- **Add**: Write a new `.json` file to `resources/` to inject a resource into the map.

- **Delete**: Not supported — removing a file from `resources/` has no effect. Resources
  cannot be removed from the pipeline by a shell processor.

- **Malformed JSON**: Files that cannot be parsed as JSON are skipped with a warning; the
  corresponding resource retains its previous value.

### stdout and stderr

- **stdout** lines are emitted as `INFO`-level log entries labelled with the processor name.
- **stderr** lines are emitted as `WARN`-level log entries labelled with the processor name.

Run `rh publish build` with `RUST_LOG=rh_packager=debug` to see per-processor resource sync
counts and timing.

### Examples

#### Bash: stamp a build date onto all StructureDefinitions

```bash
#!/usr/bin/env bash
# scripts/stamp-date.sh
set -euo pipefail

DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
for f in "$PACKAGER_WORKDIR/resources"/StructureDefinition-*.json; do
  python3 -c "
import json, sys
r = json.load(open('$f'))
r['date'] = '$DATE'
json.dump(r, open('$f', 'w'), indent=2)
"
done
echo "Stamped $(ls "$PACKAGER_WORKDIR/resources"/StructureDefinition-*.json | wc -l) resources"
```

```toml
# packager.toml
[processors.stamp-date]
command = "bash scripts/stamp-date.sh"

[hooks]
before_build = ["stamp-date", "snapshot", "validate"]
```

#### Python: add a custom extension to every resource

```python
#!/usr/bin/env python3
# scripts/add-extension.py
import json, os, glob

workdir = os.environ["PACKAGER_WORKDIR"]
pkg_name = os.environ["PACKAGER_PACKAGE_NAME"]

for path in glob.glob(f"{workdir}/resources/*.json"):
    with open(path) as f:
        resource = json.load(f)

    resource.setdefault("extension", [])
    # Avoid duplicates
    if not any(e.get("url") == "http://example.org/source-package" for e in resource["extension"]):
        resource["extension"].append({
            "url": "http://example.org/source-package",
            "valueString": pkg_name,
        })

    with open(path, "w") as f:
        json.dump(resource, f, indent=2)

print(f"Processed {len(glob.glob(f'{workdir}/resources/*.json'))} resources")
```

```toml
[processors.add-extension]
command = "python3 scripts/add-extension.py"

[hooks]
before_build = ["add-extension"]
```

#### Node.js: validate against a custom schema

```js
// scripts/validate-custom.mjs
import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join } from "path";

const workdir = process.env.PACKAGER_WORKDIR;
const resources = readdirSync(join(workdir, "resources"))
  .filter((f) => f.endsWith(".json"))
  .map((f) => JSON.parse(readFileSync(join(workdir, "resources", f), "utf8")));

const errors = [];
for (const r of resources) {
  if (!r.id) errors.push(`${r.resourceType}: missing 'id'`);
}

if (errors.length > 0) {
  console.error(errors.join("\n"));
  process.exit(1);
}

console.log(`Validated ${resources.length} resources — OK`);
```

```toml
[processors.custom-validate]
command = "node scripts/validate-custom.mjs"

[hooks]
before_build = ["snapshot", "custom-validate", "validate"]
```

#### Shell: use an external CLI tool

```toml
[processors.fhir-lint]
command = "fhirlint check $PACKAGER_SOURCE_DIR"

[hooks]
before_build = ["fhir-lint"]
```

### Error handling

A non-zero exit code causes the pipeline to abort immediately with an error that includes:

- The processor name
- The exit code (or `signal` if the process was killed by a signal)
- The first line of stderr

Any processors declared after the failed one in the same stage are not run.

---

## 2. Native Rust Processors

For scenarios requiring tight integration with rh crate internals, direct access to the
in-memory resource map, or maximum performance, you can implement the `HookProcessor` trait
directly in Rust.

### Implement the trait

Add `rh-packager` as a dependency and implement [`hooks::HookProcessor`]:

```rust
use rh_packager::{
    context::PublishContext,
    hooks::HookProcessor,
    Result,
};

pub struct TimestampProcessor;

impl HookProcessor for TimestampProcessor {
    fn name(&self) -> &str {
        // Must match the name used in packager.toml hook lists.
        "timestamp"
    }

    fn run(&self, ctx: &mut PublishContext) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();

        // ctx.resources is a HashMap<String, serde_json::Value>
        // keyed by filename stem (e.g. "StructureDefinition-foo").
        // Processors may insert, modify, or read entries.
        for resource in ctx.resources.values_mut() {
            let rt = resource
                .get("resourceType")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if rt == "StructureDefinition" {
                resource["date"] = serde_json::Value::String(now.clone());
            }
        }

        Ok(())
    }
}
```

#### `PublishContext` fields available to processors

| Field | Type | Description |
|---|---|---|
| `source_dir` | `PathBuf` | Absolute path to the source directory |
| `output_dir` | `PathBuf` | Absolute path to the output directory |
| `package_json` | `PackageJson` | Parsed `package.json` manifest |
| `resources` | `HashMap<String, Value>` | Mutable FHIR resource map (filename stem → JSON) |
| `config` | `PublisherConfig` | Full parsed `packager.toml` configuration |
| `standalone_markdown` | `Vec<PathBuf>` | Markdown files without a matching resource |

Processors **may** mutate `ctx.resources` freely. They **must not** write files to disk
directly — the pipeline owns all output and performs a single write pass after all stages.

### Register and run

Build a `ProcessorRegistry`, register your processor, then call the pipeline:

```rust
use rh_packager::{
    hooks::{build_registry, ProcessorRegistry},
    pipeline::build,
};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    // Start with all built-in processors (snapshot, validate, cql).
    let mut registry = build_registry();

    // Register your custom processor.
    registry.register(TimestampProcessor);

    // The pipeline resolves hook names from packager.toml against this registry.
    // build() now accepts a pre-built registry as its third argument.
    let tgz = build(Path::new("my-package"), Path::new("output"), registry)?;
    println!("Package written to {}", tgz.display());
    Ok(())
}
```

To bypass all built-ins:

```rust
let mut registry = ProcessorRegistry::new(); // empty — no built-ins
registry.register(TimestampProcessor);
```

### Wire to `packager.toml`

Native processors registered programmatically are referenced by name in `packager.toml` the
same way as built-ins and shell processors:

```toml
[hooks]
before_build = ["snapshot", "timestamp", "validate"]
```

The name in `packager.toml` must exactly match the string returned by
`HookProcessor::name()`. An unregistered name causes an immediate startup error before any
files are touched.

---

## 3. Built-in Processors

| Name | Typical stage | Description |
|---|---|---|
| `snapshot` | `before_build` | Generates missing `snapshot.element` arrays for `StructureDefinition` resources by loading base definitions from dependency packages. |
| `validate` | `before_build` or `after_build` | Validates all FHIR resources using `rh-validator`. Fails the pipeline on any ERROR-severity issue. |
| `cql` | `before_build` | Compiles `.cql` source files to ELM JSON and embeds source + ELM into `Library.content[]`. Auto-creates a minimal `Library` resource if none exists (with a warning). |

Built-in processors are configured via their own sections in `packager.toml`
(`[validate]`, `[cql]`). See [packager.toml Reference](README.md#packagertoml-reference) for
the full field list.

---

## 4. Processor Ordering and Isolation

- Processors within a stage run **sequentially in list order**.
- The **first failure aborts** the pipeline; later processors in the same stage are skipped.
- Each processor receives the resource map **as left by the previous processor** — mutations
  accumulate across the stage.
- Shell processor invocations are **isolated**: each gets its own temporary directory and
  cannot observe another processor's workdir.
- Native Rust processors share `&mut PublishContext` with no isolation boundary — they are
  responsible for not corrupting state.

---

## 5. Common Patterns

### Read-only audit / reporting

Run a processor that inspects resources but makes no changes. Shell processors that exit 0
without modifying `resources/` files are safe read-only auditors.

```bash
#!/usr/bin/env bash
# Count resources by type
jq -r '.resourceType' "$PACKAGER_WORKDIR"/resources/*.json | sort | uniq -c
```

### Conditional enrichment

Only modify resources matching a certain criterion:

```python
for path in glob.glob(f"{workdir}/resources/ValueSet-*.json"):
    vs = json.load(open(path))
    if vs.get("status") == "draft":
        vs["status"] = "active"
        json.dump(vs, open(path, "w"), indent=2)
```

### Calling an external FHIR server

```bash
curl -sf -X POST https://fhir.example.org/fhir/\$validate \
  -H 'Content-Type: application/json' \
  -d @"$PACKAGER_WORKDIR/resources/StructureDefinition-foo.json"
```

### Generating resources from a template

Write new files to `$PACKAGER_WORKDIR/resources/` and they will be injected into the
pipeline:

```bash
python3 generate.py --template templates/ValueSet.json.j2 \
  --output "$PACKAGER_WORKDIR/resources/"
```
