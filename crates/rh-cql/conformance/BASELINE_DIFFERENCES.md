# Baseline Structural Differences: Rust vs Java CQL-to-ELM

Based on comparison of OperatorTests with matching compiler options.

## Test Results Summary

- **Total operator test files**: 32
- **Compilation failures in Rust**: 22 (parser limitations)
- **Successfully compared**: 10 (all have differences)
- **Zero-difference matches**: 0

## Structural Differences

### Fixed ✅

1. **Library Identifier** - Now always emits `identifier: {}` even when empty
2. **Annotation Type** - Now includes `"type": "CqlToElmInfo"` discriminator
3. **Default Context** - Now emits `"context": "Unfiltered"` on expression defs

### Remaining (Expected Behavior)

4. **LocalId Generation**
   - **Java**: Only includes `localId` when `--debug` is enabled
   - **Rust**: Always includes `localId` when annotations are enabled
   - **Status**: Intentional - provides better debugging/traceability

5. **Locator Format**
   - **Java**: `"line:start-line:end"` (e.g., `"1:23-1:36"`)
   - **Rust**: `"line:col"` (e.g., `"1:1"`) - only start position
   - **Status**: Would require parser changes to track end positions

### Omitted (OK per user)

6. **Empty Arrays** - Java includes empty `annotation: []`, Rust omits
   - **Status**: Acceptable to omit empty values

## Files Successfully Compared (with differences)

1. `AgeOperators.cql`
2. `AggregateOperators.cql`
3. `ComparisonOperators.cql`
4. `CqlComparisonOperators.cql`
5. `ForwardReferences.cql`
6. `LogicalOperators.cql`
7. `MessageOperators.cql`
8. `NullologicalOperators.cql`
9. `StringOperators.cql`
10. `TerminologyReferences.cql`

## Compilation Failures (Parser Limitations)

Missing language features causing parse errors:
- `predecessor`/`successor` keywords
- `let` clauses in queries
- Interval operator phrases (`starts`, `ends`, `occurs`, `properly`)
- `aggregate` clause
- Implicit type conversions syntax
- Multi-source queries
- `sort by` clauses
- Time-related operations

## Conformance Progress

| Test | Before | After | Change |
|------|--------|-------|--------|
| LogicalOperators | 30 | 24 | -6 (20% reduction) |

The 6 eliminated differences were all `missing_in_rust` issues (identifier, type, context).
