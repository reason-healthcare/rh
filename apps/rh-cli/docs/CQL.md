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

Compile a CQL file to ELM JSON:

```bash
# Compile a CQL file to ELM JSON
rh cql compile library.cql
```

Output:

```json
{
  "library": {
    "identifier": {
      "id": "Example",
      "version": "1.0.0"
    },
    "statements": {
      "def": [
        {
          "type": "ExpressionDef",
          "name": "X",
          "expression": { "...": "..." }
        }
      ]
    }
  }
}
```

Write output to a file:

```bash
# Write output to a file
rh cql compile library.cql --output library.elm.json
```

Output:

```text
stdout is empty; ELM JSON is written to library.elm.json.
```

Compile with debug annotations:

```bash
# Compile with debug annotations
rh cql compile library.cql --debug
```

Output:

```json
{
  "library": {
    "annotation": [
      {
        "translatorVersion": "..."
      }
    ],
    "statements": { "...": "..." }
  }
}
```

Compile with ELM result type metadata:

```bash
# Compile with ELM result type metadata
rh cql compile library.cql --result-types
```

Output:

```json
{
  "library": {
    "statements": {
      "def": [
        {
          "name": "X",
          "resultTypeSpecifier": { "...": "..." },
          "expression": { "...": "..." }
        }
      ]
    }
  }
}
```

Compile in strict mode:

```bash
# Compile in strict mode
rh cql compile library.cql --strict
```

Output:

```json
{
  "library": {
    "identifier": {
      "id": "Example",
      "version": "1.0.0"
    },
    "statements": { "...": "..." }
  }
}
```

Emit compact JSON with a source map:

```bash
# Emit compact JSON with a source map
rh cql compile library.cql --compact --source-map --output library.elm.json
```

Output:

```text
stdout is empty; ELM JSON is written to library.elm.json and the source map is
written to library.elm.json.sourcemap.json.
```

Compile from stdin:

```bash
# Compile from stdin
echo 'library Test version '"'"'1.0'"'"' ...' | rh cql compile -
```

Output:

```json
{
  "library": {
    "identifier": {
      "id": "Test",
      "version": "1.0"
    },
    "statements": { "...": "..." }
  }
}
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

Validate a CQL file:

```bash
# Validate a CQL file
rh cql validate library.cql
```

Output:

```text
✓ CQL is valid
```

Validate from stdin:

```bash
# Validate from stdin
cat library.cql | rh cql validate -
```

Output:

```text
✓ CQL is valid
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

Output:

```text
Library: Example
Version: 1.0.0
Using:
  FHIR version 4.0.1
Definitions:
  - X
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

Summarize compiled ELM structure:

```bash
# Summarize compiled ELM structure
rh cql elm inspect measure.cql
```

Output:

```text
Library: DiabetesMeasure
Version: 1.0.0
Usings: 1
Includes: 1
Parameters: 1
Value sets: 1
Code systems: 0
Expressions: 2
Functions: 0
Retrieves: 1

Retrieves:
  - Diabetes Conditions: Condition

ELM node counts:
  - Exists: 1
  - Retrieve: 1
  - ValueSetRef: 1
```

Show expression, parameter, value set, code, and function dependencies:

```bash
# Show expression, parameter, value set, code, and function dependencies
rh cql elm deps measure.cql
```

Output:

```text
Diabetes Conditions
expression refs: none
parameter refs: none
value set refs:
  - Diabetes
code refs: none
function refs: none

Has Diabetes
expression refs:
  - Diabetes Conditions
parameter refs: none
value set refs: none
code refs: none
function refs: none
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

Output:

```text
Library: DiabetesMeasure
resources:
  - Condition
value sets:
  - Diabetes (http://example.org/fhir/ValueSet/diabetes)
retrieves:
  - Diabetes Conditions: resource=Condition codeProperty=code dateProperty=-
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

Output:

```text
Target: relational

Diabetes Conditions
  Scan dataType={http://hl7.org/fhir}Condition resource=Condition

Has Diabetes
  Exists
    Expr kind=ExpressionRef
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

Output:

```text
Target: sql-on-fhir
Supported: true

Supported nodes:
  - Exists: 1
  - ExpressionRef: 1
  - Retrieve: 1
  - ValueSetRef: 1

Notes:
  - This report covers the first-pass relational lowerer, not full CQL semantics.
  - Terminology expansion, complete interval precision, quantities, and complex list semantics may still require fallback evaluation.
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

Evaluate a named expression:

```bash
# Evaluate a named expression
rh cql eval library.cql "InDemographic"
```

Output:

```text
true
```

Evaluate with a trace:

```bash
# Evaluate with a trace
rh cql eval library.cql "InDemographic" --trace
```

Output:

```text
Result: true

Trace (3 events):
  [1] op=ExpressionRef node=- inputs=[] output=true
  ...
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

Explain the CQL parse tree:

```bash
rh cql explain parse library.cql
```

Output:

```text
AST
Library Example
  ExpressionDef X
    Literal 1
```

Explain compilation details:

```bash
rh cql explain compile library.cql
```

Output:

```text
Typed Library: Example
Definitions:
  X: System.Integer
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

Output:

```text
CQL REPL
> 
```

## CQL Resources

- [CQL Specification (HL7)](https://cql.hl7.org/)
- [ELM Specification](https://cql.hl7.org/elm.html)
