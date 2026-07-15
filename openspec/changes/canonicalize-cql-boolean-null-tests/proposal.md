## Why

`is null`, `is true`, and `is false` currently parse as generic type tests, causing noncanonical
ELM and incorrect Boolean evaluation. Dedicated syntax lowering is needed so valid CQL produces
the standard nullological nodes while preserving generic named-type checks.

## What Changes

- Parse positive and negated null/Boolean test syntax before generic `is`/`as` type-specifier
  parsing.
- Lower positive forms to `IsNull`, `IsTrue`, and `IsFalse`, and lower negative forms to their
  canonical negated representation.
- Emit native ELM nullological nodes and evaluate them with CQL three-valued logic.
- Preserve generic checks such as `value is FHIR.Quantity` and retain legacy ELM compatibility
  where needed.
- Add parser, golden ELM, evaluator, and motivating-function regressions, then update only the
  conformance claims supported by those tests.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `cql-parser-conformance`: Require canonical AST lowering for positive and negated null/Boolean
  tests without regressing named-type tests.
- `cql-elm-emitter`: Require source null/Boolean tests to emit native `IsNull`, `IsTrue`, and
  `IsFalse` ELM nodes.
- `cql-eval-engine`: Require correct three-valued evaluation of positive and negated
  null/Boolean tests.
- `cql-spec-coverage`: Require nullological coverage claims to cite parser, emitter, and evaluator
  evidence.

## Impact

- Affected crate: `crates/rh-cql`
- Primary code: parser precedence, AST/semantic handling if required, ELM operator emission, and
  evaluator dispatch
- Tests: parser AST tests, compile-to-ELM golden assertions, and evaluation integration tests
- Documentation: `crates/rh-cql/SPEC_COVERAGE.md` and `crates/rh-cql/CONFORMANCE.md`
- No public API changes; compatibility with previously generated generic `Is(null)` ELM may be
  retained internally
