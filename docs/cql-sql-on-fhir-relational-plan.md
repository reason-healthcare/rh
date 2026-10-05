# CQL to SQL-on-FHIR Relational Analytics Plan

This document describes the target architecture. Reviewed 2026-10-02 against
`10313d4e`; the [current RA implementation](../crates/rh-cql/ARCHITECTURE.md#experimental-relational-algebra)
is a diagnostic sketch, and SQL emission still bypasses it. The coordinated MVP
backlog and local demonstration guide live in the separate
`reasonhealth-analytics` repository at `docs/cms122-measure-validation-plan.md`
and `docs/local-demo.md`.

## Purpose

This plan explores a compiler-style path for population health analytics in RH:

```text
CQL source
  -> ELM
  -> clinical relational algebra
  -> SQL-on-FHIR artifacts and/or SQL text
```

The goal is not to make `rh` a database. The goal is to make RH the healthcare
analytics compiler/toolchain that can parse, inspect, lower, validate, and emit
portable FHIR-based analytics artifacts. Runtime execution belongs in the
separate ReasonHealth Analytics product.

## Core Idea

Use CQL-to-ELM as the front end, then translate supported ELM expressions into
an RH-owned relational algebra intermediate representation. Once that IR
preserves the required semantics, RH should emit multiple targets from it:

```text
                 +-----------------------------+
                 | CQL / ELM clinical semantics |
                 +--------------+--------------+
                                |
                                v
                 +-----------------------------+
                 | Clinical relational algebra  |
                 +--------------+--------------+
                                |
                 +--------------+--------------+
                 |                             |
                 v                             v
          SQL-on-FHIR artifacts             SQL text
          ViewDefinition+SQLQuery           DuckDB/Trino/etc.
```

SQL-on-FHIR should be treated as a portable artifact target. Relational algebra
is the intended semantic boundary inside the compiler; the current serialized
plan is not yet a stable executable contract. Runtime integration uses generated
JSON artifacts (ViewDefinition, SQLQuery Library, and runtime manifest), without
linking compiler internals. Arrow, DataFusion, local materialization, and measure
execution belong in ReasonHealth Analytics.

## Why Relational Algebra

CQL and SQL-on-FHIR are both declarative, but they operate at different levels.
CQL expresses clinical concepts such as retrieves, value sets, intervals,
patient context, and population membership. SQL-on-FHIR expresses tabular FHIR
projections and SQL queries over those projections.

A relational algebra layer gives RH a place to normalize CQL before choosing an
execution or artifact target.

## Conceptual Mappings

| CQL / ELM concept | Relational algebra | SQL-on-FHIR target |
|---|---|---|
| Retrieve `[Condition: "Diabetes"]` | `Scan + Filter(InValueSet)` | `Condition` ViewDefinition plus SQL filter/value set join |
| `where` | `Filter` | SQL `WHERE`, or ViewDefinition `where` if resource-local |
| `return` / projection | `Project` | ViewDefinition columns or SQL `SELECT` |
| `exists(...)` | `SemiJoin` / `Exists` | SQL `EXISTS` |
| `not exists(...)` / `without` | `AntiJoin` | SQL `NOT EXISTS` / anti-join |
| `with` | `Join` / `SemiJoin` | SQL `JOIN` / `EXISTS` |
| `union` | `Union` | SQL `UNION` / `UNION ALL` |
| `Count`, `Sum`, etc. | `Aggregate` | SQL `GROUP BY` |
| population membership | patient-key projection and set operations | SQLQuery result table |
| CQL parameters | relational parameters | SQLQuery Library parameters |

## Intermediate Representation Shape

The relational layer should be extended relational algebra, not only textbook
relational algebra. It needs clinical expression nodes for terminology,
intervals, quantities, and CQL null semantics.

Illustrative shape:

```rust
enum RelNode {
    Scan(ResourceScan),
    ViewScan(String),
    Filter { input: Box<RelNode>, predicate: Expr },
    Project { input: Box<RelNode>, expressions: Vec<Projection> },
    Join { left: Box<RelNode>, right: Box<RelNode>, kind: JoinKind, on: Expr },
    SemiJoin { left: Box<RelNode>, right: Box<RelNode>, on: Expr },
    AntiJoin { left: Box<RelNode>, right: Box<RelNode>, on: Expr },
    Aggregate { input: Box<RelNode>, group_by: Vec<Expr>, aggs: Vec<Aggregate> },
    Union { inputs: Vec<RelNode>, distinct: bool },
    Distinct { input: Box<RelNode> },
    Unnest { input: Box<RelNode>, expr: Expr, alias: String },
}

enum Expr {
    Column(String),
    Literal(Value),
    Call(Function, Vec<Expr>),
    InValueSet { code: Box<Expr>, value_set: Canonical },
    IntervalOp { op: IntervalOperator, left: Box<Expr>, right: Box<Expr> },
    IsNull(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}
```

## Initial Scope

Start with a deliberately narrow CQL subset:

1. FHIR R4 retrieves.
2. Patient-context expressions.
3. Value set filters.
4. Basic `where`, `exists`, `with`, and `without`.
5. Basic date/dateTime interval comparisons.
6. SQL-on-FHIR ViewDefinition generation for required resource projections.
7. SQLQuery generation for joins, filters, set operations, and aggregates.

These are implementation goals, not a list of supported semantics. The first
real-measure acceptance target is the pinned historical CMS122 HbA1c example;
its required age, encounter, diabetes, latest-observation, and exclusion logic
must survive the complete pipeline before the MVP gate can pass.

Defer initially:

- full UCUM quantity normalization;
- complex list semantics;
- advanced CQL functions;
- complete interval precision semantics;
- cross-library optimization;
- distributed execution;
- server-managed materialization;
- local execution and DataFusion-backed runtime behavior, which belong in
  ReasonHealth Analytics.

## Incremental Tooling

Build inspectable compiler tooling before execution.

### 1. ELM Inspection

```bash
rh cql compile measure.cql --output measure.elm.json
rh cql elm inspect measure.cql
rh cql elm deps measure.cql
```

`elm inspect` and `elm deps` compile CQL internally. The separate `compile`
command saves ELM JSON for review; that file is not their input.

Outputs:

- libraries and includes;
- parameters;
- retrieves;
- value sets and code systems;
- expression dependency graph;
- unsupported ELM node inventory.

### 2. Data Requirements

```bash
rh cql data-requirements measure.cql --format json
```

This currently inventories resources, retrieves, terminology declarations, and
parameters in the main compiled library. Full included-library closure and all
predicate-specific projection paths remain work for semantic lowering.

### 3. Relational Plan Explain

```bash
rh cql plan measure.cql --target relational --display-format pretty
rh cql plan measure.cql --target relational --display-format json
```

Target explain shape (not current CLI output):

```text
PatientContext
  SemiJoin patient_id
    Scan Patient
    Filter InValueSet(code, "Diabetes")
      Scan Condition
```

This should become the primary debugging surface. Today's generic `Expr` nodes
retain only an operator kind; query planning also omits sources, relationship
predicates, and complete sorting/aggregation semantics.

### 4. Lowering Support Report

```bash
rh cql lower-check measure.cql --target sql-on-fhir
```

The current report classifies node kinds as `supportedNodes`, `fallbackNodes`,
or `unsupportedNodes`. `supported: true` means only that `unsupportedNodes` is
empty, including when fallback nodes remain. The `target` is a report label,
not a backend capability check. A future executable gate must validate required
semantics and dependencies; the classifier alone does not do that.

### 5. ViewDefinition Generation

```bash
rh cql emit-views measure.cql --out views/
```

This emits deterministic ViewDefinitions for main-library retrieve requirements.
The target is to include all projections needed by the complete clinical plan.

### 6. SQLQuery Generation

```bash
rh cql emit-sql measure.cql --views views/ --out query-library.json
rh cql emit-sql measure.cql --sql-only
```

The first command emits a SQL-on-FHIR SQLQuery Library. The second emits raw SQL
for review and backend experimentation. Today both use a retrieve skeleton:
CTEs, `code IS NOT NULL` terminology placeholders, and a final select from the
first CTE. Neither consumes the relational plan or implements the full measure.

### 7. Measure Runtime Manifest

```bash
rh cql emit-runtime measure.cql --views views/ --query query-library.json --out measure-runtime.json
```

This emits the path-oriented runtime manifest consumed by ReasonHealth
Analytics. The manifest binds generated ViewDefinition and SQLQuery artifacts to
caller-supplied result names without linking runtime execution dependencies into
open-source `rh`. It does not infer population predicates, and parameter metadata
does not ensure the SQL uses those parameters.

### 8. Local Execution

```bash
rh-analytics sql view run --view views/condition.json --input data.ndjson
rh-analytics sql query run --query query-library.json --view views/ --input data/
```

This is implemented for a bounded projection/query subset in ReasonHealth
Analytics, using Arrow tables and DataFusion. A passing curated artifact demo
verifies that runtime path; it does not establish CQL-to-SQL equivalence.

### 9. Measure Harness

```bash
rh-analytics measure run measure-runtime.json --input data/ --engine datafusion
rh-analytics measure compare measure-runtime.json --input data/ --expected expected-results.json
```

These commands consume a runtime manifest, not CQL source. Comparison uses an
exported expected-results JSON file. Native evaluation is a separate audit;
there is no evaluator engine or automatic CQL fallback in the analytics runtime.

## Implementation Status and Next Gates

The inspection, plan, classification, ViewDefinition, SQLQuery, and manifest
commands exist, as do local analytics execution/comparison and demo scripts.
Their existence does not complete semantic lowering. The native CMS122 suite
has six assertions; the broader 28-call audit still records 10 errors. See the
[evaluator status](../crates/rh-cql/docs/user-defined-function-eval-plan.md#verification).

The next compiler gates, aligned with the analytics MVP plan, are:

1. Establish a complete native evaluator baseline with reviewed terminology,
   all patient/population results, temporal boundary fixtures, and independent
   reference evidence (M1).
2. Preserve included libraries, typed operands/reference bindings, terminology,
   parameters/defaults, and required FHIR paths in the semantic plan (M2).
3. Make executable emission reject unresolved required semantics. Generate
   initial population predicates first, then numerator/exclusions and every
   result mapping. Validate generated SQL against independent expected
   membership; handwritten measure SQL is not a substitute (M2).
4. Keep strict projection validation, typed Arrow execution, provenance, and
   population/scoring contracts in analytics (M3). Complete the guided local
   CMS122 demonstration only when both evaluation paths and comparisons pass
   from a clean setup (M4).

## Testing Strategy

Every stage should be serializable, diffable, and testable.

Golden fixtures should cover:

```text
CQL -> ELM
ELM -> relational algebra
relational algebra -> SQL-on-FHIR ViewDefinitions
relational algebra -> SQLQuery
SQLQuery + ViewDefinitions -> execution result
```

The test suite should include both positive lowering cases and unsupported
construct cases with stable diagnostic output.

## Key Risks

- CQL semantics are richer than SQL semantics, especially for nulls, intervals,
  quantities, terminology, and list handling.
- SQL-on-FHIR ViewDefinitions are intentionally per-resource and do not support
  joins or aggregates; those must stay in SQLQuery.
- DataFusion is a strong embedded execution option, but it should live behind
  the ReasonHealth Analytics runtime boundary because its APIs evolve.
- Terminology expansion and versioning are outside relational algebra and need a
  first-class service boundary.
- CQL outside the implemented lowering subset needs an explicit failure or a
  separately designed and validated fallback contract. The current analytics
  runtime has no such fallback executor.

## Design Principle

Build RH's semantic compiler boundary around the completed IR:

```text
ELM -> clinical relational algebra
```

Emit portable artifacts from that boundary and keep runtime dependencies out of
the compiler:

- SQL-on-FHIR artifacts for portability;
- generated artifacts executed with DataFusion in ReasonHealth Analytics;
- SQL text for external engines such as DuckDB, Trino, Postgres, or Spark.
