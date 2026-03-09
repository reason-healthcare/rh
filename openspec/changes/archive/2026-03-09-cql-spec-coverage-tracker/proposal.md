## Why

The rh-cql CONFORMANCE.md tracks HL7 test results and high-level feature status, but has no exhaustive mapping of every CQL 1.5.3 language feature against our four-stage pipeline (Parse → Semantic → Emit → Eval). The CQL spec defines ~170 operators across 13 categories, 13 types, and ~130 keywords. Without a per-operator tracking document, it's difficult to identify specific gaps, plan work, or measure progress toward full spec compliance.

## What Changes

- Create a new `crates/rh-cql/SPEC_COVERAGE.md` document that exhaustively maps every CQL 1.5.3 operator, type, and keyword to implementation status across all four pipeline stages
- Organize by CQL spec Appendix B sections (§9.1 Types through §9.13 Errors and Messaging) for direct traceability to the spec
- Include a category-level summary dashboard at the top for at-a-glance progress, with detailed per-operator tables in each section
- Add a Language Constructs & Keywords section tracking all ~130 parser keywords and grammar productions
- Update CONFORMANCE.md §4 to link to SPEC_COVERAGE.md rather than duplicating feature tables
- Populate every cell by cross-referencing actual source code (AST enums, operator resolver, emitter mappings, evaluator dispatch)

## Capabilities

### New Capabilities
- `cql-spec-coverage`: Exhaustive per-operator specification coverage matrix tracking CQL 1.5.3 implementation status across Parse, Semantic Analysis, ELM Emission, and Evaluation stages

### Modified Capabilities
<!-- No existing spec-level behavior changes — this is a documentation-only change -->

## Impact

- **Files created**: `crates/rh-cql/SPEC_COVERAGE.md`
- **Files modified**: `crates/rh-cql/CONFORMANCE.md` (§4 updated to link to SPEC_COVERAGE.md)
- **Code changes**: None — documentation only
- **Dependencies**: None
