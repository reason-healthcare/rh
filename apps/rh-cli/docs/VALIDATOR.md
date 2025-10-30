# rh CLI - Validator Commands

## Overview

The `rh` CLI provides comprehensive FHIR resource validation capabilities through the `validate` subcommands. These commands support both single resource and batch validation workflows, with flexible input/output options suitable for both interactive use and CI/CD pipelines.

## Commands

### `rh validate resource`

Validate a single FHIR resource.

**Usage:**
```bash
rh validate resource [OPTIONS]
```

**Options:**
- `-i, --input <FILE>` - Input file path (reads from stdin if not provided)
- `-f, --format <FORMAT>` - Output format: `text` (default) or `json`
- `--skip-invariants` - Skip invariant validation (not yet implemented)
- `--skip-bindings` - Skip binding validation (not yet implemented)
- `--strict` - Exit with non-zero code if validation produces any issues (warnings count as failure)

**Exit Codes:**
- `0` - Validation successful
- `1` - Validation failed (errors found) or warnings in strict mode

**Examples:**

```bash
# Validate from stdin
echo '{"resourceType": "Patient", ...}' | rh validate resource

# Validate from file
rh validate resource --input patient.json

# JSON output for parsing/automation
rh validate resource --format json < patient.json

# Strict mode (treat warnings as errors)
rh validate resource --strict < patient.json

# Use in CI/CD
rh validate resource --input patient.json && echo "Valid!" || echo "Invalid!"
```

**Output (Text Format):**
```
✅ FHIR resource is valid (Patient)
⚠️  2 warning(s)
ℹ️  1 informational message(s)

Issues:
  1. ⚠️  Binding strength is 'example' - validation skipped
     Path: Patient.gender
  2. ℹ️  Profile auto-detected from meta.profile
```

**Output (JSON Format):**
```json
{
  "resourceType": "Patient",
  "valid": true,
  "errors": 0,
  "warnings": 2,
  "issues": [
    {
      "severity": "warning",
      "code": "code-invalid",
      "message": "Binding strength is 'example' - validation skipped",
      "path": "Patient.gender",
      "location": null
    }
  ]
}
```

---

### `rh validate batch`

Validate multiple FHIR resources from NDJSON (newline-delimited JSON).

**Usage:**
```bash
rh validate batch [OPTIONS]
```

**Options:**
- `-i, --input <FILE>` - Input NDJSON file path (reads from stdin if not provided)
- `-f, --format <FORMAT>` - Output format: `text` (default) or `json`
- `--threads <N>` - Number of threads for parallel validation (not yet implemented)
- `--skip-invariants` - Skip invariant validation (not yet implemented)
- `--skip-bindings` - Skip binding validation (not yet implemented)
- `--summary-only` - Show summary only (hide individual issues)
- `--strict` - Exit with non-zero code if validation produces any issues

**Exit Codes:**
- `0` - All resources valid
- `1` - One or more resources invalid

**Examples:**

```bash
# Validate NDJSON from stdin
cat resources.ndjson | rh validate batch

# Validate from file
rh validate batch --input resources.ndjson

# Summary only for large batches
rh validate batch --summary-only --input large-batch.ndjson

# JSON output
rh validate batch --format json < resources.ndjson

# CI/CD usage
rh validate batch --input batch.ndjson --strict && echo "All valid!"
```

**Output (Text Format):**
```
📋 Batch Validation Summary:
  Total resources: 150
  ✅ Valid: 145
  ❌ Invalid: 5
  Total errors: 8
  Total warnings: 23

❌ Invalid resources:
  Line 42 (Patient): 2 error(s), 1 warning(s)
    ❌ Missing required field 'name'
       at Patient.name
  Line 89 (Observation): 1 error(s), 0 warning(s)
    ❌ Required field 'status' is missing
       at Observation.status
```

**Output (JSON Format):**
```json
{
  "summary": {
    "total": 150,
    "valid": 145,
    "invalid": 5
  },
  "results": [
    {
      "line": 1,
      "resourceType": "Patient",
      "valid": true,
      "errors": 0,
      "warnings": 1,
      "issues": [...]
    },
    ...
  ]
}
```

---

## Input Formats

### Single Resource (JSON)

Standard FHIR JSON format:

```json
{
  "resourceType": "Patient",
  "id": "example",
  "name": [{
    "family": "Smith",
    "given": ["John"]
  }],
  "gender": "male",
  "birthDate": "1970-01-01"
}
```

### Batch Resources (NDJSON)

Newline-delimited JSON (one resource per line):

```
{"resourceType": "Patient", "id": "1", "name": [{"family": "Smith"}]}
{"resourceType": "Observation", "id": "2", "status": "final", "code": {"text": "Test"}}
{"resourceType": "Organization", "id": "3", "name": "Test Org"}
```

Empty lines are ignored.

---

## Validation Features

### Automatic Profile Detection

The validator automatically detects profiles from resource metadata:

```json
{
  "resourceType": "Patient",
  "meta": {
    "profile": [
      "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"
    ]
  },
  ...
}
```

If no profile is specified, validation falls back to base FHIR resource validation.

### Validation Types

1. **Structural Validation**
   - Resource type exists
   - Required fields present
   - JSON structure matches FHIR spec

2. **Cardinality Validation**
   - Min/max occurrences
   - Required vs. optional fields
   - Array constraints

3. **Type Validation**
   - Field types match spec
   - Reference types valid
   - Choice types correct

4. **Binding Validation**
   - Code values in allowed ValueSets
   - Binding strength enforcement (required, extensible, preferred, example)

5. **Invariant Validation**
   - FHIRPath constraints evaluated
   - Business rules checked

6. **Extension Validation**
   - Extension URLs valid
   - Required extensions present
   - Extension cardinality

7. **Slicing Validation**
   - Discriminator values match
   - Slice cardinality correct
   - Required slices present

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Validate FHIR Resources

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install rh CLI
        run: cargo install --path apps/rh-cli
      
      - name: Validate resources
        run: |
          rh validate batch --input resources.ndjson --strict
```

### Exit Code Usage

```bash
#!/bin/bash
set -e  # Exit on error

# Validate all JSON files
for file in resources/*.json; do
  echo "Validating $file..."
  rh validate resource --input "$file" --strict || exit 1
done

echo "✅ All resources valid!"
```

### JSON Parsing Example

```bash
# Extract validation summary
rh validate batch --format json --input batch.ndjson | jq '.summary'

# Count errors
rh validate batch --format json --input batch.ndjson | jq '.results | map(.errors) | add'

# Filter invalid resources
rh validate batch --format json --input batch.ndjson | jq '.results[] | select(.valid == false)'
```

---

## Performance

- **Single resource:** ~4ms average validation time
- **Batch processing:** ~252 resources/second (sequential)
- **Cache hit rate:** 100% for repeated profiles
- **Memory:** <10MB overhead for typical workloads

See `PERFORMANCE.md` for detailed benchmarks.

---

## Troubleshooting

### "Profile not found" error

If you see this error, the validator cannot find the profile definition. This typically happens when:

1. FHIR packages are not installed
2. The profile URL is incorrect
3. The package directory is not configured

**Solution:** Ensure FHIR packages are installed in `~/.fhir/packages/` or configure a custom package directory.

### Empty input warnings

The validator warns when input is empty but continues successfully. Use `--strict` to treat empty input as an error.

### Performance for large batches

For very large NDJSON files (>1000 resources), consider:

1. Using `--summary-only` to reduce output
2. Processing in chunks with shell scripts
3. Using JSON output for automated processing

---

## Future Enhancements

Planned features for future releases:

- [ ] Parallel batch validation (waiting for FhirPathEvaluator thread-safety)
- [ ] `--skip-invariants` and `--skip-bindings` implementation
- [ ] Profile management commands (`rh profile list/show/search`)
- [ ] Package management commands (`rh package install/list`)
- [ ] Custom profile directory configuration
- [ ] Streaming validation for very large files
- [ ] HTML report generation

---

## See Also

- [Validator README](../../crates/rh-validator/README.md) - Library documentation
- [Performance Report](../../crates/rh-validator/PERFORMANCE.md) - Benchmarks and optimization
- [Validator TODO](../../crates/rh-validator/TODO.md) - Implementation roadmap
