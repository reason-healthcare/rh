# rh CLI - FHIRPath Commands

## Overview

The `rh fhirpath` command provides tools for parsing and evaluating [FHIRPath](https://hl7.org/fhirpath/) expressions — the standard path language used throughout FHIR for navigation, filtering, and data extraction.

## Commands

### `rh fhirpath parse`

Parse a FHIRPath expression and display its abstract syntax tree (AST).

**Usage:**
```bash
rh fhirpath parse [OPTIONS] <EXPRESSION>
```

**Arguments:**
- `<EXPRESSION>` - FHIRPath expression to parse

**Options:**
- `-f, --format <FORMAT>` - Output format: `pretty` (default), `json`, `debug`
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Parse and display AST
rh fhirpath parse "Patient.name.family"

# Output as JSON
rh fhirpath parse --format json "Patient.name.where(use = 'official')"
```

---

### `rh fhirpath eval`

Evaluate a FHIRPath expression against FHIR data.

**Usage:**
```bash
rh fhirpath eval [OPTIONS] <EXPRESSION>
```

**Arguments:**
- `<EXPRESSION>` - FHIRPath expression to evaluate

**Options:**
- `-d, --data <DATA>` - Path to a JSON file containing FHIR data (reads from stdin if not provided)
- `-f, --format <FORMAT>` - Output format: `pretty` (default), `json`, `debug`
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Evaluate against a FHIR resource file
rh fhirpath eval "Patient.name.family" --data patient.json

# Pipe FHIR data from stdin
cat patient.json | rh fhirpath eval "Patient.name.family"

# Use a filtering expression
rh fhirpath eval "Patient.name.where(use = 'official').family" --data patient.json

# Output as JSON for downstream processing
rh fhirpath eval "Patient.birthDate" --data patient.json --format json
```

---

### `rh fhirpath repl`

Start an interactive REPL for evaluating FHIRPath expressions.

**Usage:**
```bash
rh fhirpath repl [OPTIONS]
```

**Options:**
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Start REPL
rh fhirpath repl
```

In the REPL, type FHIRPath expressions and press Enter to evaluate them against any loaded context.

---

### `rh fhirpath test`

Run FHIRPath test cases from a file, reporting pass/fail results.

**Usage:**
```bash
rh fhirpath test [OPTIONS] --file <FILE>
```

**Options:**
- `-f, --file <FILE>` - Path to a file containing test cases (required)
- `-d, --data <DATA>` - Path to a JSON file containing FHIR data
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Run test cases from a file
rh fhirpath test --file tests/fhirpath-cases.yaml

# Run tests with associated FHIR data
rh fhirpath test --file tests/fhirpath-cases.yaml --data patient.json
```

## FHIRPath Resources

- [FHIRPath Specification (HL7)](https://hl7.org/fhirpath/)
- [FHIRPath in FHIR R4](https://www.hl7.org/fhir/fhirpath.html)
