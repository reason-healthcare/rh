# Baseline Structural Differences: Rust vs Java CQL-to-ELM

Based on comparison of OperatorTests with matching compiler options.

## Test Results Summary

- **Total operator test files**: 32
- **Compilation failures in Rust**: 22 (parser limitations)
- **Successfully compared**: 10 (all have differences)
- **Zero-difference files**: 0

## Core Structural Differences

### 1. LocalId Generation
- **Java**: Only includes `localId` when `--debug` is enabled OR when needed for annotation references
- **Rust**: Always includes `localId` when annotations are enabled
- **Impact**: 16+ extra fields per file

**Recommendation**: Make `localId` emission conditional on debug mode or when referenced by annotations.

### 2. Locator Format
- **Java**: `"line:start-line:end"` (e.g., `"1:23-1:36"`)
- **Rust**: `"line:col"` (e.g., `"1:1"`) - only start position
- **Impact**: Missing end positions, inconsistent format

**Recommendation**: Track and emit both start and end positions in locator strings.

### 3. Default Context
- **Java**: Includes `"context": "Unfiltered"` on expression definitions without explicit context
- **Rust**: Omits context when unfiltered
- **Impact**: Every statement definition missing context field

**Recommendation**: Always emit `context` field on ExpressionDef, defaulting to `"Unfiltered"`.

### 4. Library Identifier
- **Java**: Always includes `"identifier": {}` even when empty
- **Rust**: Omits identifier when no id/version
- **Impact**: Missing field in library

**Recommendation**: Always emit `identifier` object (can be empty).

### 5. Annotation Type Field
- **Java**: `{"type": "CqlToElmInfo", ...}`
- **Rust**: `{...}` (no type field)
- **Impact**: Missing discriminator for annotation type

**Recommendation**: Add `"type": "CqlToElmInfo"` to translator info annotation.

### 6. Empty Arrays
- **Java**: Includes empty `annotation: []` and `signature: []` arrays on elements
- **Rust**: Omits empty arrays
- **Impact**: Many extra/missing fields

**Recommendation**: Consider matching Java's behavior of including empty arrays (or update comparison script to normalize).

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

## Recommended Fix Order

1. **Quick wins** (structural): identifier, context, annotation type
2. **Medium effort**: locator format (start-end)
3. **Larger changes**: conditional localId emission

