# rh CLI - Validation Commands

## Overview

The `rh validate` command validates FHIR JSON resources using `rh-validator`.

Current subcommands:

- `rh validate resource` for one or more JSON resources
- `rh validate batch` for NDJSON input

The validator always runs in FHIR R4 mode today.

## `rh validate resource`

Validate a single resource from a file, multiple files, a glob, or standard input.

```bash
rh validate resource [OPTIONS]
```

### Options

- `-i, --input <INPUT>` - Input file path(s) or glob pattern(s). If omitted, reads JSON from stdin.
- `-f, --format <FORMAT>` - Output format: `text`, `json`, or `operationoutcome`. Default: `text`.
- `--strict` - Exit non-zero if any issues are reported, including warnings.
- `--security-checks` - Enable stricter HTML and script-content checks.
- `--terminology-server <URL>` - Use a FHIR terminology server such as `https://tx.fhir.org/r4`.
- `--skip-invariants` - Accepted by the CLI but not currently implemented.
- `--skip-bindings` - Accepted by the CLI but not currently implemented.

### Examples

```bash
# Validate one file
rh validate resource --input patient.json

# Validate multiple files matched by a glob
rh validate resource --input 'examples/**/*.json'

# Emit OperationOutcome JSON
rh validate resource --input patient.json --format operationoutcome

# Read from stdin
cat patient.json | rh validate resource

# Enable remote terminology validation
rh validate resource \
  --input observation.json \
  --terminology-server https://tx.fhir.org/r4
```

## `rh validate batch`

Validate newline-delimited JSON (NDJSON) resources from one or more files or from stdin.

```bash
rh validate batch [OPTIONS]
```

### Options

- `-i, --input <INPUT>` - NDJSON file path(s) or glob pattern(s). If omitted, reads from stdin.
- `-f, --format <FORMAT>` - Output format: `text`, `json`, or `operationoutcome`. Default: `text`.
- `--summary-only` - Hide per-resource issue details in text output.
- `--strict` - Exit non-zero if any issues are reported, including warnings.
- `--security-checks` - Enable stricter HTML and script-content checks.
- `--terminology-server <URL>` - Use a FHIR terminology server such as `https://tx.fhir.org/r4`.
- `--threads <N>` - Accepted by the CLI and defaults to `4`; current validation still runs on the blocking worker path without per-record parallelism.
- `--skip-invariants` - Accepted by the CLI but not currently implemented.
- `--skip-bindings` - Accepted by the CLI but not currently implemented.

### Examples

```bash
# Validate NDJSON from a file
rh validate batch --input bundle.ndjson

# Validate NDJSON from stdin
cat bundle.ndjson | rh validate batch --summary-only

# Emit JSON results for multiple files
rh validate batch --input 'fixtures/*.ndjson' --format json
```

## Exit behavior

- Invalid resources always cause a non-zero exit code.
- `--strict` also treats warnings and informational issues as failure conditions.
- Empty input is reported and returns non-zero only when `--strict` is set.

## Related docs

- [`crates/rh-validator/README.md`](../../../crates/rh-validator/README.md)
- [`openspec/planning/rh-validator/TODO.md`](../../../openspec/planning/rh-validator/TODO.md)
- [`openspec/planning/rh-validator/PHASE_15_ANALYSIS.md`](../../../openspec/planning/rh-validator/PHASE_15_ANALYSIS.md)
