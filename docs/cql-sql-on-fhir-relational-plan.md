# CQL to SQL-on-FHIR Relational Analytics Plan

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
an RH-owned relational algebra intermediate representation. From that IR, RH can
emit multiple targets:

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

SQL-on-FHIR should be treated as a portable artifact target, not the internal
compiler IR. Relational algebra should be the stable internal contract between
`rh` and downstream runtimes such as ReasonHealth Analytics.

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

Defer initially:

- full UCUM quantity normalization;
- complex list semantics;
- advanced CQL functions;
- complete interval precision semantics;
- cross-library optimization;
- distributed execution;
- server-managed materialization.
- local execution and DataFusion-backed runtime behavior, which belong in
  ReasonHealth Analytics.

## Incremental Tooling

Build inspectable compiler tooling before execution.

### 1. ELM Inspection

```bash
rh cql compile measure.cql --output measure.elm.json
rh cql elm inspect measure.elm.json
rh cql elm deps measure.elm.json
```

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

This extracts resource, retrieve, value set, and date-path requirements before
attempting relational lowering.

### 3. Relational Plan Explain

```bash
rh cql plan measure.cql --target relational --format pretty
rh cql plan measure.cql --target relational --format json
```

Example:

```text
PatientContext
  SemiJoin patient_id
    Scan Patient
    Filter InValueSet(code, "Diabetes")
      Scan Condition
```

This should be the primary debugging surface.

### 4. Lowering Support Report

```bash
rh cql lower-check measure.cql --target sql-on-fhir
```

The report should separate supported and unsupported constructs so users can see
why a CQL library cannot lower.

### 5. ViewDefinition Generation

```bash
rh cql emit-views measure.cql --out views/
```

This should emit deterministic ViewDefinitions for the resource projections
required by CQL retrieves.

### 6. SQLQuery Generation

```bash
rh cql emit-sql measure.cql --views views/ --out query-library.json
rh cql emit-sql measure.cql --sql-only
```

The first command emits a SQL-on-FHIR SQLQuery Library. The second emits raw SQL
for review and backend experimentation.

### 7. Measure Runtime Manifest

```bash
rh cql emit-runtime measure.cql --views views/ --query query-library.json --out measure-runtime.json
```

This emits the path-oriented runtime manifest consumed by ReasonHealth
Analytics. The manifest binds generated ViewDefinition and SQLQuery artifacts to
stable measure result names without linking runtime execution dependencies into
open-source `rh`.

### 8. Local Execution

```bash
rh-analytics sql view run --view views/condition.json --input data.ndjson
rh-analytics sql query run --query query-library.json --view views/*.json --input data/
```

This belongs in ReasonHealth Analytics. It can use generated Arrow tables and
DataFusion internally, but execution should come after the `rh` lowering stages
are inspectable and testable.

### 9. Measure Harness

```bash
rh-analytics measure run measure.cql --input synthea/ --engine datafusion
rh-analytics measure compare measure.cql --engine evaluator --engine datafusion
```

The compare command keeps relational lowering honest by comparing against the
existing CQL evaluator where possible.

## Recommended Build Order

Open-source `rh`:

1. Add ELM inspection tooling.
2. Add data-requirements extraction.
3. Add relational algebra IR and `rh cql plan`.
4. Add lowering support reports.
5. Add ViewDefinition generation.
6. Add SQLQuery generation.
7. Add measure runtime manifest generation.

ReasonHealth Analytics:

8. Add `rh-analytics sql view run`.
9. Add `rh-analytics sql query run` with a DataFusion backend.
10. Add measure execution and comparison harnesses.

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
- Some CQL will always require fallback evaluation rather than relational
  lowering.

## Design Principle

Keep RH's stable contract at the compiler IR boundary:

```text
ELM -> clinical relational algebra
```

Everything after that is a backend:

- SQL-on-FHIR artifacts for portability;
- DataFusion for embedded execution in ReasonHealth Analytics;
- SQL text for external engines such as DuckDB, Trino, Postgres, or Spark.
