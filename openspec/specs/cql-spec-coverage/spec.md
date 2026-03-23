# Spec: CQL Specification Coverage Tracking

### Requirement: Category summary dashboard
The SPEC_COVERAGE.md document SHALL include a summary table at the top listing every CQL 1.5.3 Appendix B category (§9.1 through §9.13) with columns for category name, total operator count, implemented count, not-implemented count, and coverage percentage.

#### Scenario: Dashboard reflects accurate category totals
- **WHEN** a reader views the summary dashboard
- **THEN** each row SHALL show the category name, total operators in that category per the CQL spec, count of fully-implemented operators, count of not-implemented operators, and a coverage percentage computed as (implemented / total × 100)

#### Scenario: Dashboard covers all 13 Appendix B categories
- **WHEN** a reader scans the dashboard
- **THEN** all 13 CQL 1.5.3 Appendix B categories SHALL be present: Types, Logical Operators, Type Operators, Nullological Operators, Comparison Operators, Arithmetic Operators, String Operators, Date/Time Operators, Interval Operators, List Operators, Aggregate Operators, Clinical Operators, and Errors and Messaging

### Requirement: Per-operator four-stage status matrix
Each operator listed in CQL 1.5.3 Appendix B SHALL have a row in a per-category table with four status columns: Parse, Semantic, Emit, and Eval.

#### Scenario: Operator row shows all four pipeline stages
- **WHEN** a reader looks up a specific operator (e.g., `Add`)
- **THEN** the row SHALL display the operator name and a status indicator (✅, ❌, or ➖) for each of the four stages: Parse, Semantic, Emit, Eval

#### Scenario: Not-applicable stages use distinct indicator
- **WHEN** a pipeline stage does not apply to an operator (e.g., some operators have no distinct parse step because they use function-call syntax)
- **THEN** the cell SHALL use ➖ (not applicable) rather than ❌ (not implemented)

### Requirement: Appendix B section traceability
The document SHALL organize operator tables by CQL Appendix B section numbers so readers can cross-reference directly from the spec.

#### Scenario: Section headers match CQL spec structure
- **WHEN** a reader navigates to a section (e.g., "§6 Arithmetic Operators")
- **THEN** the section SHALL contain exactly the operators defined in the corresponding CQL 1.5.3 Appendix B section

### Requirement: Source code evidence
Each operator row SHALL include a Notes column that cites the source code location where the operator is implemented (e.g., enum variant name, function name, or file reference).

#### Scenario: Notes column provides verifiable code reference
- **WHEN** a reader sees an operator marked ✅ for a stage
- **THEN** the Notes column SHALL cite at least one source code reference (e.g., `ast.rs:UnaryOperator::Negate`, `engine.rs:eval_add`) that can be verified in the codebase

#### Scenario: Not-implemented operators have empty or gap-indicating notes
- **WHEN** an operator is marked ❌ for all stages
- **THEN** the Notes column MAY be empty or contain a brief explanation of why the feature is missing

### Requirement: Keywords and grammar productions section
The document SHALL include a section tracking all CQL 1.5.3 keywords and reserved words with their parser support status.

#### Scenario: All spec keywords are listed
- **WHEN** a reader views the keywords section
- **THEN** all keywords defined in CQL 1.5.3 Appendix L (reserved words and keyword lists) SHALL be listed with a status indicating whether the parser recognizes them

### Requirement: CONFORMANCE.md cross-reference
The existing CONFORMANCE.md §4 (Feature Implementation Status) SHALL be updated to link to SPEC_COVERAGE.md for detailed per-operator tracking, replacing any duplicated feature tables.

#### Scenario: CONFORMANCE.md links to SPEC_COVERAGE.md
- **WHEN** a reader views CONFORMANCE.md §4
- **THEN** it SHALL contain a link to SPEC_COVERAGE.md and a brief summary, rather than duplicating the per-operator coverage tables

### Requirement: Exhaustive operator coverage
The document SHALL cover every operator defined in CQL 1.5.3 Appendix B without omission. Partial-coverage operators (implemented in some stages but not others) SHALL be clearly distinguishable from fully-implemented or fully-missing operators.

#### Scenario: All ~170 Appendix B operators are present
- **WHEN** a reader counts operator rows across all category sections
- **THEN** the total SHALL match the number of operators defined in CQL 1.5.3 Appendix B (approximately 170)

#### Scenario: Partial implementation is visible
- **WHEN** an operator is parsed but not yet evaluated
- **THEN** the row SHALL show ✅ for Parse and ❌ for Eval (or whichever stages apply), making partial progress immediately visible

### Requirement: Wave-2 documents baseline and post-change metric deltas

After wave-2 implementation, `SPEC_COVERAGE.md` and `CONFORMANCE.md` SHALL include a dated before/after comparison against the wave-2 baseline (2026-03-09 post-wave-1 metrics).

#### Scenario: Baseline and post-wave-2 values are both present
- **WHEN** a reader reviews updated documents
- **THEN** baseline values, post-wave-2 values, and deltas are all visible for key conformance metrics

#### Scenario: Command evidence and run date are captured
- **WHEN** conformance metrics are updated for wave-2
- **THEN** the documents include the command(s) and run date used to generate the metrics

### Requirement: Wave-2 coverage updates include gap-to-evidence traceability

Each operator/function closed in wave-2 SHALL include traceable references from coverage rows/notes to implementation locations and validating tests.

#### Scenario: Closed wave-2 row references implementation and tests
- **WHEN** a previously open wave-2 gap is marked implemented
- **THEN** coverage notes include implementation and test evidence references

#### Scenario: Deferred items remain explicit after wave-2
- **WHEN** wave-2 completes with remaining out-of-scope gaps
- **THEN** deferred gaps are still explicitly listed and not implied as complete
