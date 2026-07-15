## Context

The relative-timing parser currently parses an optional quantity offset but then returns a
recoverable error whenever that offset is absent. Valid zero-offset phrases therefore never reach
the timing-direction parser. The AST already stores `offset: Option<TimingOffset>`, and the emitter
already has representations for inclusive `OnOrBefore` and `OnOrAfter`, so the fix should extend
the parser path rather than introduce another expression model.

## Goals / Non-Goals

**Goals:**

- Recognize every zero-offset spelling in the CQL 1.5.3 `temporalRelationship` production.
- Preserve the complete qualified right operand and optional precision.
- Keep simple `before`/`after` ELM behavior stable while enabling inclusive forms end to end.
- Convert the affected HL7 compile errors into supported evaluations with no wrong answers.

**Non-Goals:**

- Redesign the hand-written precedence parser or replace `nom`.
- Complete nonzero offset or boundary semantics beyond behavior already supported.
- Change imported HL7 fixtures, public compiler APIs, or whitespace semantics.

## Decisions

### Recognize the relationship before using the optional offset

`parse_relative_timing` will keep quantity offset parsing optional through direction parsing. A
recognized temporal relationship, rather than the presence of an offset, will distinguish this
branch from other expressions. This matches the grammar and preserves the existing AST.

The alternative—adding a separate zero-offset parser and AST variant—would duplicate relationship
ordering and emitter logic without representing a distinct CQL concept.

### Parse longer relationship spellings first

The relationship parser will try `on or before`/`on or after` before their shorter prefixes and
will support both grammar families: `on or? before|after` and `before|after (or on)?`. Table-driven
tests will cover every spelling with and without precision.

### Verify semantics at every pipeline stage

Parser assertions will establish AST shape, emitter assertions will establish canonical ELM, and
evaluator/HL7 assertions will establish supported runtime behavior. Simple `before` and `after`
will receive emission regressions because their internal parse route may change.

### Make the motivating function an integration regression

The exact multiline medication-request function and a single-line equivalent will be parsed and
compared. This change owns complete operand consumption and the outer conjunction/timing shape;
canonical `IsNull` lowering remains owned by `canonicalize-cql-boolean-null-tests`.

## Risks / Trade-offs

- [Relationship prefixes consume the wrong alternative] → Order longer phrases first and test all
  eight spellings.
- [Simple `before`/`after` emission changes with the AST route] → Add ELM regressions before the
  parser refactor.
- [Parsing success exposes unsupported evaluator behavior] → Run all 17 affected HL7 cases and
  keep documentation at partial status until end-to-end evidence exists.
- [Cross-change motivating-function assertion depends on null-test canonicalization] → Assert the
  temporal subtree here and the canonical nullological subtree in the Boolean/null change.

## Migration Plan

1. Record the current focused and HL7 conformance baseline.
2. Add regressions for existing neighboring syntax and currently failing temporal forms.
3. Refactor timing recognition, then close emitter/evaluator gaps exposed by the tests.
4. Run focused, crate-wide, and HL7 conformance checks before updating coverage documents.

Rollback is a normal source revert; no persisted data or public API migration is involved.

## Open Questions

- Confirm whether any affected HL7 case depends on partially implemented nonzero offset or
  boundary semantics before marking it supported.
