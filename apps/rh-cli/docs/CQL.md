# rh CLI - CQL Commands

## Overview

The `rh cql` command provides tools for working with CQL (Clinical Quality Language), compiling CQL source files to ELM (Expression Logical Model) JSON, evaluating expressions, and inspecting compilation details. Terminology requirements are emitted as versioned JSON; native evaluation consumes a prepared immutable terminology snapshot.

It also includes first-pass SQL-on-FHIR helpers for retrieve-centric measure
logic: `data-requirements`, `plan`, `lower-check`, `emit-views`, `emit-sql`,
and `emit-runtime`. These commands expose main-library retrieve requirements,
an inspectable plan, node-kind classifications, and generated ViewDefinition,
SQLQuery Library, and runtime manifest artifacts.

These helpers are experimental diagnostics and artifact scaffolding. SQL
emission currently bypasses the relational plan and selects the first retrieve
CTE. Static ValueSet retrieve membership uses a prepared `rh_valueset_members`
relation, while complete clinical predicates and population semantics remain
incomplete. A `supported: true` report does not establish executable SQL
equivalence, and the analytics runtime has no CQL fallback executor. See
[current relational-algebra status](../../../crates/rh-cql/ARCHITECTURE.md#experimental-relational-algebra).

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
- `--format <FORMAT>` - Output format: `human` (default), `json`, or `ndjson`
- `-o, --output <OUTPUT>` - Output file path (defaults to stdout)
- `--compact` - Output compact JSON (no pretty-printing)
- `--debug` - Enable debug mode (includes annotations, locators, and result types in output)
- `--result-types` - Include result type metadata in output
- `--strict` - Enable strict mode (disable implicit conversions)
- `--signatures` - Include all signatures in output
- `--lib-path <DIR>` - Additional directory to search for included CQL libraries; may be specified multiple times
- `--source-map` - Also emit a source-map sidecar file alongside the ELM output
- `--source-map-output <PATH>` - Path for source-map output (defaults to `<output>.sourcemap.json` or stderr)
- `-q, --quiet` - Suppress informational output on stderr
- `-v, --verbose...` - Increase verbosity (`-v` info, `-vv` debug, `-vvv` trace)
- `--color <WHEN>` - Color output policy: `auto` (default), `always`, or `never`
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

Compile with included libraries from another directory:

```bash
# Compile with an additional include search path
rh cql compile measure.cql --lib-path cql/includes
```

Output:

```json
{
  "library": {
    "identifier": {
      "id": "Measure",
      "version": "1.0.0"
    },
    "includes": { "...": "..." },
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
rh cql validate [OPTIONS] [FILE]...
```

**Arguments:**
- `[FILE]...` - Path(s) to CQL file(s) or glob pattern(s), or `-` to read from stdin

**Options:**
- `--format <FORMAT>` - Output format: `human` (default), `json`, or `ndjson`
- `--lib-path <DIR>` - Additional directory to search for included CQL libraries; may be specified multiple times
- `--details` - Show detailed error information, including locations and annotations
- `-q, --quiet` - Suppress informational output on stderr
- `-v, --verbose...` - Increase verbosity (`-v` info, `-vv` debug, `-vvv` trace)
- `--color <WHEN>` - Color output policy: `auto` (default), `always`, or `never`
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

Validate multiple CQL files:

```bash
# Validate every CQL file under measures/
rh cql validate 'measures/**/*.cql'
```

Output:

```text
[measures/diabetes.cql]
✓ CQL is valid

[measures/hypertension.cql]
✓ CQL is valid
```

Validate with detailed diagnostics:

```bash
# Include source locations for validation diagnostics
rh cql validate library.cql --details
```

Output:

```text
✗ CQL has errors

Errors (1):
  ✗ Undefined symbol: MissingExpression (line 8, col 12)
```

Validate with a JSON envelope for automation:

```bash
# Emit the standard rh JSON envelope
rh cql validate library.cql --format json
```

Output:

```json
{
  "ok": true,
  "result": [
    {
      "file": "library.cql",
      "valid": true,
      "errors": [],
      "warnings": []
    }
  ],
  "errors": [],
  "meta": { "...": "..." }
}
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

The current output is a shallow diagnostic tree, not an executable clinical
IR. Generic `Expr` nodes retain the ELM kind without its operands or reference
identity; query sort and aggregate nodes retain only placeholders. The
`--target` value is a label, and SQL emission does not consume this plan.

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
rh cql plan measure.cql --target relational --display-format json --format json
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

Report the current first-pass node-kind classification for compiled ELM.
`supported: true` means the inspected main library contains no node kind
classified as unsupported. It does not verify preserved operand semantics,
included-library dependencies, emitted predicates, or backend execution.

`fallbackNodes` assigns evaluator-fallback labels to node kinds; it does not
verify individual function bodies. The diagnostic output's wording about
runtime fallback does not mean SQL emission or `rh-analytics` executes these functions.
Both fallback and unsupported dependencies need faithful lowering, or a
separately implemented and validated fallback contract, before a full measure
can execute. The `--target` option is a report label.

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

### `rh cql emit-views`

Emit SQL-on-FHIR ViewDefinition JSON artifacts from CQL retrieve
requirements.

Current projections are derived from main-library retrieve metadata, not the
complete relational plan or included-library dependency closure. They may omit
fields needed by clinical predicates elsewhere in the measure.

**Usage:**
```bash
rh cql emit-views [OPTIONS] <FILE> --out <DIR>
```

**Options:**
- `--out <DIR>` - Output directory for generated ViewDefinition JSON files
- `--canonical-base <URL>` - Canonical base URL for generated ViewDefinitions
- `--lib-path <DIR>` - Additional directory to search for included CQL libraries

**Example:**

```bash
rh cql emit-views measure.cql --out views/
```

Output:

```text
Wrote 1 ViewDefinition file(s) to views/
  - views/condition_view.json
```

Generated `views/condition_view.json`:

```json
{
  "resourceType": "https://sql-on-fhir.org/ig/StructureDefinition/ViewDefinition",
  "id": "ConditionView",
  "url": "https://reason.health/rh/generated/sql-on-fhir/ViewDefinition/condition_view",
  "name": "condition_view",
  "status": "draft",
  "resource": "Condition",
  "select": [
    {
      "column": [
        {
          "path": "getResourceKey()",
          "name": "id",
          "type": "string"
        },
        {
          "path": "subject.getReferenceKey(Patient)",
          "name": "patient_id",
          "type": "string"
        }
      ]
    },
    {
      "column": [
        {
          "path": "system",
          "name": "system",
          "type": "uri"
        },
        {
          "path": "code",
          "name": "code",
          "type": "code"
        }
      ],
      "forEachOrNull": "code.coding"
    }
  ]
}
```

---

### `rh cql emit-sql`

Emit a SQL-on-FHIR SQLQuery Library artifact, or raw SQL text, from CQL and
ViewDefinition metadata.

Current SQL creates retrieve CTEs and selects the first one. Static ValueSet
retrieves use `EXISTS` against the prepared `rh_valueset_members` relation,
qualified by the declaring requirement ID, system, and code. Patient
`Coding.version` is not a membership predicate; ValueSet and CodeSystem versions
select the expansion. `DISTINCT` preserves resource cardinality when several
codings match. Unresolved terminology filters fail emission. Clinical filters,
patient-level population logic, and complete measure semantics remain
incomplete, so successful artifact generation is not a claim of general
CQL-to-SQL equivalence.

**Usage:**
```bash
rh cql emit-sql [OPTIONS] <FILE>
```

**Options:**
- `--views <PATH>` - ViewDefinition JSON file or directory. May be specified multiple times.
- `--out <PATH>` - Output file path. Defaults to stdout.
- `--sql-only` - Emit only SQL text instead of a SQLQuery Library JSON artifact
- `--canonical-base <URL>` - Canonical base URL for generated in-memory ViewDefinitions
- `--lib-path <DIR>` - Additional directory to search for included CQL libraries

**Examples:**

Emit raw SQL:

```bash
rh cql emit-sql measure.cql --views views/ --sql-only
```

Illustrative SQL (projection shortened and formatting expanded for readability):

```sql
WITH
  diabetes_conditions AS (
    SELECT DISTINCT source.id, source.patient_id
    FROM condition_view AS source
    WHERE EXISTS (
      SELECT 1 FROM rh_valueset_members AS member
      WHERE member.requirement_id = 'DiabetesMeasure|1.0.0|Diabetes'
      AND member.system = source.system
      AND member.code = source.code
    )
  )
SELECT *
FROM diabetes_conditions;
```

Write a SQLQuery Library artifact:

```bash
rh cql emit-sql measure.cql --views views/ --out query-library.json
```

Output:

```text
Wrote SQLQuery Library to query-library.json
```

Generated `query-library.json` excerpt (projection and SQL formatting are simplified;
the separate terminology-requirements extension is omitted):

```json
{
  "resourceType": "Library",
  "id": "diabetesmeasure_sql_query",
  "name": "DiabetesmeasureSqlQuery",
  "title": "DiabetesMeasure SQL Query",
  "status": "draft",
  "type": {
    "coding": [
      {
        "system": "https://sql-on-fhir.org/ig/CodeSystem/LibraryTypesCodes",
        "code": "sql-query"
      }
    ]
  },
  "relatedArtifact": [
    {
      "type": "depends-on",
      "label": "condition_view",
      "resource": "https://reason.health/rh/generated/sql-on-fhir/ViewDefinition/condition_view"
    }
  ],
  "parameter": [
    {
      "name": "measurementperiod",
      "use": "in",
      "type": "string"
    }
  ],
  "content": [
    {
      "contentType": "application/sql",
      "extension": [
        {
          "url": "https://sql-on-fhir.org/ig/StructureDefinition/sql-text",
	  "valueString": "(SQL text shown above)"
        }
      ],
      "data": "V0lUSAo..."
    }
  ]
}
```

The generated FHIR `Library` includes a
`https://reason.health/fhir/StructureDefinition/terminology-requirements`
extension containing the versioned requirements JSON. Analytics validates the
declared requirements before query execution, including for an empty input. A
raw SQL consumer must provide those requirements separately.

---

### `rh cql emit-runtime`

Emit a `ReasonHealthMeasureRuntime` manifest that points at generated
ViewDefinition and SQLQuery artifacts.

**Usage:**
```bash
rh cql emit-runtime [OPTIONS] <FILE> --query <PATH> --views <PATH>
```

**Options:**
- `--query <PATH>` - SQLQuery Library JSON artifact path to reference
- `--views <PATH>` - ViewDefinition JSON file or directory. May be specified multiple times.
- `--out <PATH>` - Output file path. Defaults to stdout.
- `--result <NAME=COLUMN>` - Result mapping. Defaults to `initialPopulation=patient_id`
- `--lib-path <DIR>` - Additional directory to search for included CQL libraries

**Example:**

```bash
rh cql emit-runtime measure.cql \
  --query query-library.json \
  --views views/ \
  --out measure-runtime.json
```

Output:

```text
Wrote measure runtime manifest to measure-runtime.json
```

Generated `measure-runtime.json`:

```json
{
  "resourceType": "ReasonHealthMeasureRuntime",
  "id": "diabetesmeasure",
  "measure": "DiabetesMeasure",
  "query": "query-library.json",
  "views": [
    "views/condition_view.json"
  ],
  "parameters": [
    {
      "name": "measurementperiod",
      "type": "string",
      "required": false
    }
  ],
  "results": [
    {
      "name": "initialPopulation",
      "kind": "population",
      "source": "query",
      "column": "patient_id"
    }
  ]
}
```

---

### `rh cql terminology-requirements`

Emit a `ReasonHealthTerminologyRequirements` v1 JSON document for every
declared ValueSet in the resolved CQL library closure. Requirement IDs identify
the declaring library, its optional version, and the local ValueSet name. Each
requirement preserves the canonical and version, CodeSystem constraints, and
supported terminology operations. The initial extractor conservatively
requests membership, expansion, and string-membership capabilities; this is
not an exact usage or expression-reachability analysis.

```bash
rh cql terminology-requirements measure.cql \
  --lib-path cql-libs --output requirements.json
```

### Prepare and use a terminology snapshot

Snapshot preparation belongs to the separate, private
`reasonhealth-analytics` repository; access to that repository is required to
build its CLI and follow the preparation guide. Build `rh` from the RH checkout
and `rh-analytics` from the Analytics checkout. The [Analytics snapshot guide
(private repository)](https://github.com/Vermonster/reasonhealth-analytics/blob/main/docs/value-set-snapshots.md)
covers accepted local inputs, FHIR `$expand`, provenance, reuse, refresh, and
snapshot diffs.

For production expansions, prepare without fixture options and evaluate using
the resulting immutable snapshot:

```bash
rh cql terminology-requirements measure.cql \
  --lib-path cql-libs --output requirements.json
rh-analytics terminology prepare --requirements requirements.json \
  --terminology complete-expansions.json --output terminology-snapshot
rh cql eval measure.cql 'Initial Population' --lib-path cql-libs \
  --data patient.json --terminology-snapshot terminology-snapshot
```

Synthetic fixture data requires a reason at preparation and explicit opt-in at
each consumer:

```bash
rh-analytics terminology prepare --requirements requirements.json \
  --terminology fixture-expansions.json --output fixture-snapshot \
  --fixture-justification 'Synthetic test fixture only.'
rh cql eval measure.cql 'Initial Population' --lib-path cql-libs \
  --data patient.json --terminology-snapshot fixture-snapshot --allow-fixture
```

Pass the same `--lib-path` directories used for library resolution to
requirements extraction, evaluation, and artifact emission. Preparation can
consume local complete ValueSet expansions or a configured FHIR `$expand`
endpoint. Evaluation and snapshot-backed SQL execution make no terminology
network requests. A complete empty expansion is valid and differs from a
missing or partial expansion.

Requirement versions select the expansion; CQL membership compares `system`
and `code`, ignoring display and patient `Coding.version`. Snapshots are
immutable. Matching preparation inputs reuse the existing snapshot. Changed
inputs and `--refresh` require a new output directory; compare snapshots with
`rh-analytics terminology diff OLD NEW`. Add `--allow-fixture` to the diff
command when either snapshot is fixture-scoped. Fixture scope is provenance,
not clinical validation; production scope is not clinical certification either.

To emit a SQLQuery Library and run it with the same snapshot:

```bash
rh cql emit-views measure.cql --lib-path cql-libs --out views/
rh cql emit-sql measure.cql --lib-path cql-libs --views views/ \
  --out query-library.json
rh-analytics sql query run --query query-library.json --view views/ \
  --input patients.ndjson --terminology-snapshot terminology-snapshot
```

For a fixture snapshot, add `--allow-fixture` to the Analytics command too.
SQLQuery artifacts carry compiler requirements automatically; raw SQL consumers
must provide requirements separately. The
[RH-local example](../../../examples/value-set-membership/README.md) uses the
checked-in golden snapshot. The two-repository Analytics
[membership example in the private Analytics repository](https://github.com/Vermonster/reasonhealth-analytics/tree/main/examples/value-set-membership)
also exercises generated SQL and DataFusion. Both are terminology examples,
not claims of full clinical measure conformance. CQL is read as source input;
generated requirements and runtime artifacts are separate outputs.

### `rh cql eval`

Evaluate a named expression definition in a compiled CQL library.

**Usage:**
```bash
rh cql eval [OPTIONS] <FILE> <EXPRESSION>
```

**Arguments:**
- `<FILE>` - Path to a CQL file, or `-` to read from stdin
- `<EXPRESSION>` - Name of the expression definition to evaluate

**Options:**
- `--data <FILE>` - Patient FHIR data file for retrieve operations
- `--terminology-snapshot <DIR>` - Prepared immutable terminology snapshot
- `--allow-fixture` - Explicitly permit fixture-scope terminology
- `--lib-path <DIR>` - Included-library search directory (repeatable)
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
