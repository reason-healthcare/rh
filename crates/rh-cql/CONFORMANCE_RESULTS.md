# CQL Parser Conformance Results

**Baseline Date**: 2024-12-10 (75.6%)
**Updated**: 2026-03-07 (multi-stage pipeline refactor)
**Test Infrastructure**: `cql-test-parse` binary; full jvmTest rerun requires `conformance/scripts/setup.sh` (Java 17+)

## Test Results

### jvmTest Suite (Main Conformance Tests)
- **Total**: 119 files
- **Passed**: 90 files (75.6%)
- **Failed**: 29 files (24.4%)

### Examples (Real-world CQL Files)
- **Total**: 9 files
- **Passed**: 3 files (33.3%)
- **Failed**: 6 files (66.7%)

**Passing files:**
- CMS9v4_FHIRv16_Common.cql
- FHIRHelpers-1.0.2.cql
- FHIRHelpers-1.8.cql

**Failing files:**
- CMS9v4_CDS.cql
- CMS9v4_CQM.cql
- CMS9v4_FHIRv16_CDS.cql
- CMS9v4_FHIRv16_CQM.cql
- CMS9v4_QDM.cql
- CMS9v4_Quick.cql

## Recent Fixes (2024-12-10)

### 1. Single-source Query Alias Parsing
**Commit**: 4d3aec2
- Fixed parser to allow query aliases without immediate clauses
- Added keyword rejection for aliases (prevent `define`, `context`, etc.)
- Enables patterns like:
  ```cql
  ["Encounter": "ED"] LastED
      where LastED.period ends before start of Visit.period
  ```

### 2. Member Access in Timing Operators  
**Commit**: 139d686
- Changed `start of`/`end of` to parse invocation expressions (not just unary)
- Enables member access: `start of Visit.period`
- Fixes timing phrases like:
  ```cql
  ends 1 hour or less on or before start of Visit.relevantPeriod
  ```

## Updates from Multi-Stage Pipeline Refactor (2026-03-07)

The following parser improvements were added as part of the `cql-multi-stage-pipeline-refactor` change.
These are expected to raise the jvmTest pass rate above the 75.6% baseline, but a full rerun
requires the Java tooling from `conformance/scripts/setup.sh`.

### New Syntax Supported

1. **Predecessor / Successor keywords** — `predecessor of` and `successor of` now
   lex and parse as `Predecessor`/`Successor` ELM types.

2. **Power operator (`^`)** — `2 ^ 10` parses with correct precedence; emitted as
   the native `Power` ELM type.

3. **Minimum / Maximum type expressions** — `minimum Integer`, `maximum DateTime`, etc.

4. **Aggregate clause in queries** — `aggregate A with merge starting: ...` syntax.

5. **Extended timing phrase keywords** — `starts before`, `properly includes`,
   `occurs during`, and similar phrases now parse correctly.

### Current State (2026-03-07)

- `cargo test -p rh-cql`: **47/47 tests pass** ✅
- Local conformance suite (`crates/rh-cql/conformance/test-cases/`):
  - `arithmetic/ArithmeticTests.cql` — ✅ passes
  - `simple/SimpleTest.cql` — ✅ passes
- `comparison/` CQL files (FHIR-dependent, require model provider) — expected failures in `cql-test-parse` (standalone binary has no FHIR model)

### Full jvmTest Rerun

To reproduce the 119-file jvmTest pass rate:

```bash
cd crates/rh-cql/conformance
./scripts/setup.sh          # one-time Java tooling setup
python3 scripts/compare_translators.py --suite test-cases/jvmTest/
```

Update this file with results when a full rerun is completed.

## Next Steps

Based on the 29 failing jvmTest files and 6 failing Examples, priority areas:

1. **Complex timing expressions** - CMS measure files use sophisticated timing logic
2. **QDM-specific syntax** - CMS9v4_QDM.cql suggests QDM data model challenges
3. **CDS vs CQM patterns** - Different use cases in clinical decision support vs quality measures

The 75.6% baseline was measured before the parser improvements listed above.
The updated pass rate is expected to be higher; rerun jvmTest to confirm.
