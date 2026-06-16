# rh — Unified CLI for FHIR Tools

The `rh` CLI is a unified command-line interface for working with FHIR data. It provides FHIR code generation, FHIRPath evaluation, CQL compilation, FSH compilation, validation, and package management in a single tool.

## Installation

### Pre-built binaries (recommended)

Download the latest release binary for your platform from the [GitHub Releases page](https://github.com/reason-healthcare/rh/releases).

**macOS (Apple Silicon)**
```bash
curl -L https://github.com/reason-healthcare/rh/releases/latest/download/rh-aarch64-apple-darwin.tar.gz | tar xz
sudo mv rh /usr/local/bin/
```

**macOS (Intel)**
```bash
curl -L https://github.com/reason-healthcare/rh/releases/latest/download/rh-x86_64-apple-darwin.tar.gz | tar xz
sudo mv rh /usr/local/bin/
```

**Linux x86_64 (static binary)**
```bash
curl -L https://github.com/reason-healthcare/rh/releases/latest/download/rh-x86_64-unknown-linux-musl.tar.gz | tar xz
sudo mv rh /usr/local/bin/
```

**Windows x86_64**

Download [`rh-x86_64-pc-windows-msvc.zip`](https://github.com/reason-healthcare/rh/releases/latest/download/rh-x86_64-pc-windows-msvc.zip), extract it, and place `rh.exe` somewhere on your `PATH`.

### Build from source

```bash
cargo install --path apps/rh-cli
```

### From crates.io

```bash
cargo install rh-cli
```

### Docker

A pre-built Docker image is available from the GitHub Container Registry:

```bash
# Pull the latest image
docker pull ghcr.io/reason-healthcare/rh:latest

# Show available commands
docker run --rm ghcr.io/reason-healthcare/rh:latest --help

# Run rh against files in the current directory
docker run --rm \
  -v "$(pwd)":/work \
  -w /work \
  ghcr.io/reason-healthcare/rh:latest \
  validate resource --input patient.json
```

You can also pin to a specific version:

```bash
docker run --rm ghcr.io/reason-healthcare/rh:v0.1.0-beta.1 --version
```

## Quick Start

```bash
# See all available commands
rh --help

# Download a FHIR package
rh download package hl7.fhir.r4.core 4.0.1

# List available versions for a package
rh download list hl7.fhir.r4.core

# Generate a Rust crate from a FHIR package
rh codegen hl7.fhir.r4.core 4.0.1 --output crates/rh-hl7_fhir_r4_core

# Evaluate a FHIRPath expression (--data is optional; omit for context-free eval)
rh fhirpath eval "Patient.name.family" --data patient.json

# Validate a FHIR resource
rh validate resource --input patient.json

# Compile CQL to ELM
rh cql compile library.cql --output library.elm.json

# Compile FSH to FHIR JSON
rh fsh compile profiles/*.fsh --output output/

# Parse FSH and inspect the AST
rh fsh parse profiles/MyProfile.fsh

# Show FSH Tank summary (profiles, extensions, instances, …)
rh fsh tank profiles/MyProfile.fsh

# Generate a StructureDefinition snapshot
rh snapshot generate http://hl7.org/fhir/StructureDefinition/Patient \
  --package hl7.fhir.r4.core@4.0.1

# Scaffold a new FHIR Package
rh package init --canonical https://example.org/fhir --name com.example.fhir
```

## Command Reference

<!-- docs-sync:cli-commands:start -->
| Command | Description | Docs |
|---------|-------------|------|
| `rh codegen` | Generate organized Rust crates from FHIR Packages | [CODEGEN.md](docs/CODEGEN.md) |
| `rh cql` | Compile CQL (Clinical Quality Language) to ELM | [CQL.md](docs/CQL.md) |
| `rh download` | Download and install FHIR packages from npm-style registries | [DOWNLOAD.md](docs/DOWNLOAD.md) |
| `rh fhirpath` | Parse and evaluate FHIRPath expressions | [FHIRPATH.md](docs/FHIRPATH.md) |
| `rh vcl` | Parse and translate VCL (ValueSet Compose Language) expressions | [VCL.md](docs/VCL.md) |
| `rh fsh` | Compile and work with FHIR Shorthand (FSH) files | [FSH.md](docs/FSH.md) |
| `rh snapshot` | Generate and manage StructureDefinition snapshots | — |
| `rh package` | Assemble a conformant FHIR Package from a source directory | [PACKAGER.md](docs/PACKAGER.md) |
| `rh validate` | Validate FHIR resources | [VALIDATOR.md](docs/VALIDATOR.md) |
| `rh help` | Print this message or the help of the given subcommand(s) | — |
<!-- docs-sync:cli-commands:end -->

### `rh package` subcommands

```
rh package init  [DIR]   Scaffold a new FHIR Package source directory
rh package build <dir>   Build a FHIR Package from a source directory
rh package lock  <dir>   Resolve canonical references and write fhir-lock.json
rh package check <dir>   Validate source (no output written)
rh package pack  <dir>   Pack an already-built output directory into a .tgz

Options (init):
  -n, --name <NAME>         Package name (e.g. com.example.fhir)   [required]
  -c, --canonical <URL>     Canonical URL base                      [required]
      --version <VERSION>   Package version    [default: 0.1.0]
      --fhir-version <VER>  FHIR version       [default: 4.0.1]
      --title <TITLE>       Human-readable title
      --description <TEXT>  Package description
      --author <NAME>       Author / publisher name
      --license <SPDX>      License identifier [default: CC0-1.0]
      --status <STATUS>     IG status          [default: draft]

Options (build/pack):
  -o, --out <path>   Output directory (build) or output file path (pack)
```

See [docs/PACKAGER.md](docs/PACKAGER.md) for the full documentation including the step-by-step guide and `packager.toml` configuration reference.

## Global Options

These options apply to every `rh` subcommand:

<!-- docs-sync:cli-options:start -->
| Flag | Short | Description |
|------|-------|-------------|
| `--format <FORMAT>` | | Output format: `human` (default), `json`, `ndjson` |
| `--quiet` | `-q` | Suppress informational output (stderr) |
| `--verbose` | `-v` | Increase verbosity; repeat for more detail (`-v` info, `-vv` debug, `-vvv` trace) |
| `--color <WHEN>` | | Color output policy: `auto` (default), `always`, `never` |
| `--help` | `-h` | Print help |
| `--version` | `-V` | Print version |
<!-- docs-sync:cli-options:end -->

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Operational error (I/O failure, network error, missing file) |
| `2` | Usage error (invalid arguments — set by clap) |
| `3` | Validation / conformance failure (resource invalid, tests failed) |
| `4` | Parse error of user input (FHIRPath / CQL / FSH / VCL syntax) |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `NO_COLOR` | Disable ANSI color output (see <https://no-color.org>) |
| `RUST_LOG` | Override log level filter (e.g. `debug`, `rh=trace`) |
| `RH_REGISTRY_TOKEN` | Bearer token for private FHIR package registries |

## Agent / Scripting Usage

Use `--format json` to get machine-readable output. Every command emits a
stable envelope `{"ok": bool, "result": ..., "errors": [...], "meta": {...}}`.

```bash
# Evaluate a FHIRPath expression and extract the result with jq
rh fhirpath eval 'name.given' --format json | jq '.result'

# Compile CQL and check whether compilation succeeded
rh cql compile patient.cql --format json | jq '.ok'

# Validate a resource — exit 3 on failure, pipe issues to jq
rh validate resource --input patient.json --format json \
  | jq '.result[].issues[] | select(.severity=="error") | .message'

# Download package versions as a JSON array
rh download list hl7.fhir.r4.core --format json | jq '.result[]'

# Translate a VCL expression and extract the FHIR ValueSet compose
rh vcl translate '(http://snomed.info/sct)123456' --format json \
  | jq '.result'

# Check output is ok=true in a script
if rh fhirpath eval 'true' --format json | jq -e '.ok' > /dev/null; then
  echo "success"
fi
```

### Error envelopes

When an error occurs and `--format json` is active, `rh` always emits a
parseable error envelope on stdout _and_ prints `error: <message>` on stderr:

```json
{
  "ok": false,
  "errors": [{ "code": "operational_error", "message": "..." }],
  "meta": { "version": "0.2.0", "command": "rh" }
}
```

## Related Documentation

- [Code Generation Library](../../crates/rh-codegen/README.md)
- [FHIRPath Library](../../crates/rh-fhirpath/README.md)
- [FSH Library](../../crates/rh-fsh/README.md)
- [CQL Library](../../crates/rh-cql/README.md)
- [Publisher Library](../../crates/rh-packager/README.md)
- [Validator Library](../../crates/rh-validator/README.md)
- [VCL Library](../../crates/rh-vcl/README.md)
- [Foundation Library](../../crates/rh-foundation/README.md)
- [Workspace Overview](../../README.md)

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for development guidelines.
