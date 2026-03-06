## ADDED Requirements

### Requirement: explain_parse produces AST pretty-print

The `explain_parse()` function SHALL accept CQL source text and produce a human-readable rendering of the parser AST. The output SHALL show the AST tree structure with node types, source locations, and literal values. The output format SHALL be plain text with indentation reflecting tree depth.

#### Scenario: Explain parse of simple expression
- **WHEN** `explain_parse("define X: 2 + 3")` is called
- **THEN** the output shows the AST tree with `ExpressionDef(X)` containing a `BinaryExpression(Add)` with two `Literal(Integer)` children and their source locations

#### Scenario: Explain parse of invalid CQL
- **WHEN** `explain_parse("define X: 2 +")` is called with incomplete syntax
- **THEN** the output includes the parse error with source location and any partial AST that was produced

### Requirement: explain_compile produces semantic decision narrative

The `explain_compile()` function SHALL accept CQL source text and produce a human-readable narrative of the semantic decisions made during analysis. The output SHALL describe, for each expression node: the resolved type, any operator overload selected, any implicit conversions applied, and any list promotion/demotion applied. The narrative SHALL reference the original CQL source text by line/column.

#### Scenario: Explain compile shows overload resolution
- **WHEN** `explain_compile("define X: 2 + 3.0")` is called
- **THEN** the output includes a narrative entry like: "Line 1:14 — Resolved `+` to `Add(Decimal, Decimal)`, applied implicit conversion `ToDecimal` on left operand (Integer → Decimal)"

#### Scenario: Explain compile shows retrieve resolution
- **WHEN** `explain_compile("define Obs: [Observation]")` is called with FHIR R4 model
- **THEN** the output includes a narrative entry describing the retrieve type resolution: context type, data type resolved from model, and result type

#### Scenario: Explain compile shows symbol resolution
- **WHEN** `explain_compile("define A: 1\ndefine B: A + 2")` is called
- **THEN** the output includes a narrative entry for `B`'s reference to `A` showing it resolved to an expression definition with type `Integer`

### Requirement: explain_eval produces step trace (stub)

The `explain_eval()` function SHALL exist as a public API stub that returns an error indicating the eval engine is required. Once the eval engine is implemented, it SHALL produce a step-by-step trace of ELM evaluation with input/output values at each step.

#### Scenario: Explain eval without eval engine
- **WHEN** `explain_eval()` is called before the eval engine is implemented
- **THEN** it returns an error: "Evaluation engine not yet available"

### Requirement: Explain output is available via CLI

The CLI SHALL provide an `explain` subcommand with modes `parse` and `compile`. The subcommand SHALL accept a CQL file path and print the explanation output to stdout.

#### Scenario: CLI explain parse
- **WHEN** `rh cql explain parse library.cql` is run
- **THEN** the AST pretty-print is written to stdout

#### Scenario: CLI explain compile
- **WHEN** `rh cql explain compile library.cql` is run
- **THEN** the semantic decision narrative is written to stdout
