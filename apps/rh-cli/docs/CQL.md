# rh CLI - CQL Commands

## Overview

The `rh cql` command provides tools for working with CQL (Clinical Quality Language), compiling CQL source files to ELM (Expression Logical Model) JSON, evaluating expressions, and inspecting compilation details.

See the [CQL crate README](../../../crates/rh-cql/README.md) for library-level documentation.

## Commands

### `rh cql compile`

Compile a CQL source file to ELM JSON.

**Usage:**
```bash
rh cql compile [OPTIONS] <FILE>
```

**Arguments:**
- `<FILE>` - Path to a CQL file, or `-` to read from stdin

**Options:**
- `-o, --output <OUTPUT>` - Output file path (defaults to stdout)
- `--compact` - Output compact JSON (no pretty-printing)
- `--debug` - Enable debug mode (includes annotations, locators, and result types in output)
- `--result-types` - Include result type metadata in output
- `--strict` - Enable strict mode (disable implicit conversions)
- `--signatures` - Include all signatures in output
- `--source-map` - Also emit a source-map sidecar file alongside the ELM output
- `--source-map-output <PATH>` - Path for source-map output (defaults to `<output>.sourcemap.json` or stderr)
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Compile a CQL file to ELM JSON
rh cql compile library.cql

# Write output to a file
rh cql compile library.cql --output library.elm.json

# Compile with debug annotations
rh cql compile library.cql --debug

# Compile with ELM result type metadata
rh cql compile library.cql --result-types

# Compile in strict mode
rh cql compile library.cql --strict

# Emit compact JSON with a source map
rh cql compile library.cql --compact --source-map --output library.elm.json

# Compile from stdin
echo 'library Test version '"'"'1.0'"'"' ...' | rh cql compile -
```

---

### `rh cql validate`

Validate CQL source without generating ELM output.

**Usage:**
```bash
rh cql validate [OPTIONS] <FILE>
```

**Arguments:**
- `<FILE>` - Path to a CQL file, or `-` to read from stdin

**Options:**
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Validate a CQL file
rh cql validate library.cql

# Validate from stdin
cat library.cql | rh cql validate -
```

---

### `rh cql info`

Parse a CQL file and display library information.

**Usage:**
```bash
rh cql info [OPTIONS] <FILE>
```

**Arguments:**
- `<FILE>` - Path to a CQL file, or `-` to read from stdin

**Options:**
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Show library information
rh cql info library.cql
```

---

### `rh cql elm`

Inspect compiled ELM output without writing the full ELM JSON.

**Usage:**
```bash
rh cql elm inspect [OPTIONS] <FILE>
rh cql elm deps [OPTIONS] <FILE>
```

**Arguments:**
- `<FILE>` - Path to a CQL file, or `-` to read from stdin

**Options:**
- `--display-format <FORMAT>` - Display format: `pretty`, `json` [default: `pretty`]
- `--lib-path <DIR>` - Additional directory to search for included CQL libraries

**Examples:**

```bash
# Summarize compiled ELM structure
rh cql elm inspect measure.cql

# Show expression, parameter, value set, code, and function dependencies
rh cql elm deps measure.cql
```

---

### `rh cql data-requirements`

Extract resource, retrieve, terminology, and parameter requirements from
compiled ELM.

**Usage:**
```bash
rh cql data-requirements [OPTIONS] <FILE>
```

**Options:**
- `--display-format <FORMAT>` - Display format: `pretty`, `json` [default: `pretty`]
- `--lib-path <DIR>` - Additional directory to search for included CQL libraries

**Example:**

```bash
rh cql data-requirements measure.cql
```

---

### `rh cql plan`

Build a first-pass relational plan from compiled ELM.

**Usage:**
```bash
rh cql plan [OPTIONS] <FILE>
```

**Options:**
- `--target <TARGET>` - Planning target label [default: `relational`]
- `--display-format <FORMAT>` - Display format: `pretty`, `json` [default: `pretty`]
- `--lib-path <DIR>` - Additional directory to search for included CQL libraries

**Example:**

```bash
rh cql plan measure.cql --target relational
```

---

### `rh cql lower-check`

Report whether compiled ELM can lower to a target backend with the current
first-pass relational lowerer.

**Usage:**
```bash
rh cql lower-check [OPTIONS] <FILE>
```

**Options:**
- `--target <TARGET>` - Lowering target label [default: `sql-on-fhir`]
- `--display-format <FORMAT>` - Display format: `pretty`, `json` [default: `pretty`]
- `--lib-path <DIR>` - Additional directory to search for included CQL libraries

**Example:**

```bash
rh cql lower-check measure.cql --target sql-on-fhir
```

---

### `rh cql eval`

Evaluate a named expression definition in a compiled CQL library.

**Usage:**
```bash
rh cql eval [OPTIONS] --expression <EXPRESSION> <FILE>
```

**Arguments:**
- `<FILE>` - Path to a CQL file, or `-` to read from stdin

**Options:**
- `-e, --expression <EXPRESSION>` - Name of the expression definition to evaluate (required)
- `--trace` - Output a step-by-step evaluation trace
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Evaluate a named expression
rh cql eval --expression "InDemographic" library.cql

# Evaluate with a trace
rh cql eval --expression "InDemographic" --trace library.cql
```

---

### `rh cql explain`

Explain a CQL parse tree or compilation details.

**Usage:**
```bash
rh cql explain [OPTIONS] <FILE>
```

**Arguments:**
- `<FILE>` - Path to a CQL file, or `-` to read from stdin

**Options:**
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Explain CQL parse tree
rh cql explain library.cql
```

---

### `rh cql repl`

Start an interactive REPL for CQL compilation and exploration.

**Usage:**
```bash
rh cql repl [OPTIONS]
```

**Options:**
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Start REPL
rh cql repl
```

## CQL Resources

- [CQL Specification (HL7)](https://cql.hl7.org/)
- [ELM Specification](https://cql.hl7.org/elm.html)
