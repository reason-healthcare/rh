## Why

`rh-cql` rejects valid zero-offset temporal relationships such as `on or after` because
relative-timing parsing currently treats an optional quantity offset as a prerequisite. This
blocks the motivating medication-request function and all 17 matching expressions in the bundled
HL7 interval and date/time fixtures despite existing AST and ELM support for zero-offset forms.

## What Changes

- Parse every CQL 1.5.3 zero-offset `before`/`after` temporal relationship spelling, with optional
  precision and point or interval operands.
- Preserve optional timing offsets in the existing AST while recognizing the relationship itself
  as the timing-phrase discriminator.
- Emit canonical `SameOrBefore` and `SameOrAfter` ELM for inclusive relationships and preserve
  existing semantics for simple `before` and `after`.
- Evaluate the affected bundled HL7 temporal cases and add focused parser, emitter, evaluator, and
  full-function regressions.
- Refresh conformance evidence only after the supported cases pass without introducing a wrong
  answer.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `cql-parser-conformance`: Require the complete zero-offset CQL 1.5.3 temporal-relationship
  grammar and regression coverage for the motivating function.
- `cql-elm-emitter`: Require canonical ELM emission and precision propagation for inclusive
  zero-offset temporal relationships.
- `cql-eval-engine`: Require supported zero-offset temporal relationships to evaluate against the
  bundled HL7 expectations.
- `cql-spec-coverage`: Require evidence-backed coverage updates for the temporal relationship
  forms closed by this change.

## Impact

- Affected crate: `crates/rh-cql`
- Primary code: `src/parser/expression/precedence.rs`, `src/emit/operators.rs`, and
  `src/eval/operators/temporal.rs`
- Tests: parser tests, emitter/evaluator integration tests, and the bundled HL7 interval and
  date/time fixtures
- Documentation: `crates/rh-cql/SPEC_COVERAGE.md` and `crates/rh-cql/CONFORMANCE.md` when metrics
  change
- Public compiler APIs and imported HL7 fixture inputs remain unchanged
