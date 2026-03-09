## Context

The rh-cql crate implements CQL 1.5.3 through a four-stage pipeline: Parse → Semantic Analysis → ELM Emission → Evaluation. Coverage information is currently scattered across CONFORMANCE.md (high-level feature tables, HL7 test results) and tribal knowledge of the codebase. No single document maps every CQL spec operator/type/keyword to its implementation status in each stage.

The codebase has concrete inventoriable structures for each stage:
- **Parse**: `KEYWORDS` array (130 entries), `UnaryOperator` (30 variants), `BinaryOperator` (40 variants), `TernaryOperator` (2 variants), `TypeOperator` (4 variants)
- **Semantic**: `OperatorResolver` with 9 registration methods covering all operator categories
- **Emit**: `emit_system_function` (11 functions), operator emit mappings for all AST operators
- **Eval**: `engine.rs` dispatch (~95+ expression types), 7 operator submodules (arithmetic, comparison, string_ops, conversion, temporal, lists, intervals)

## Goals / Non-Goals

**Goals:**
- Create a single exhaustive reference that maps every CQL 1.5.3 operator (Appendix B), type, and keyword to implementation status across all 4 pipeline stages
- Make it trivially easy to identify specific gaps and plan implementation work
- Provide both at-a-glance summaries (per-category dashboards) and detailed per-operator tables
- Ground every status cell in verifiable source code evidence (enum variant, function name, dispatch arm)
- Keep the document maintainable — structure it so status updates are localized edits

**Non-Goals:**
- Automated generation or CI enforcement of coverage tracking (future work)
- Testing coverage or code coverage metrics — this tracks feature presence, not test depth
- Replacing CONFORMANCE.md — that document covers HL7 test results and ELM fidelity, which are complementary
- Tracking implementation quality or correctness — only whether a stage handles the feature at all

## Decisions

### 1. Organize by CQL Spec Appendix B Sections

**Decision**: Structure the document to mirror the 13 categories in CQL Appendix B (§9.1 through §9.13), plus two additional sections for Keywords and Grammar Productions.

**Rationale**: Direct traceability to the spec. Anyone reading the CQL spec can find the corresponding coverage status by section number. Alternatives considered:
- *Organize by pipeline stage* — rejected because it fragments operator information across 4 separate lists, making it hard to see total coverage for a given operator
- *Organize alphabetically* — rejected because it loses the logical grouping that the spec provides

### 2. Four-Column Status Matrix per Operator

**Decision**: Each operator row has 4 status columns: Parse | Semantic | Emit | Eval, using ✅/❌/➖ indicators.

**Rationale**: Maps directly to the pipeline stages. ➖ (not applicable) handles cases where a stage genuinely doesn't apply (e.g., some operators have no distinct parse step because they're expressed through function call syntax). Alternatives considered:
- *Single "implemented" column* — rejected because a feature can be parsed but not evaluated, and tracking partial progress is the whole point
- *Free-text status* — rejected because it's harder to scan and aggregate

### 3. Category Dashboard at Document Top

**Decision**: Include a summary table at the top showing `Category | Operators | ✅ | ❌ | Coverage %` for each of the 13 Appendix B categories.

**Rationale**: Provides at-a-glance progress without scrolling through hundreds of operator rows. The per-category percentages make it easy to identify which areas need the most work and to track progress over time.

### 4. Source Code Evidence in Notes Column

**Decision**: Each per-operator table includes an optional Notes column citing the specific source location (e.g., `ast.rs:UnaryOperator::Negate`, `engine.rs:L450`).

**Rationale**: Makes the document verifiable. Anyone can check whether a status is accurate by looking at the cited code. Also helps new contributors understand where each feature lives in the codebase.

### 5. Populate from Existing Codebase Inventory

**Decision**: Initial population uses the codebase inventory already gathered (lexer keywords, AST enums, operator resolver registrations, emitter mappings, evaluator dispatch arms) rather than running automated tooling.

**Rationale**: The inventory is already complete and verified. Automated extraction would require building tooling that doesn't exist yet and isn't justified for a one-time document creation. Future changes could add automation.

### 6. CONFORMANCE.md §4 Becomes a Link

**Decision**: Replace the feature status tables in CONFORMANCE.md §4 with a brief summary and a link to SPEC_COVERAGE.md for detailed per-operator tracking.

**Rationale**: Avoids maintaining duplicate information. CONFORMANCE.md retains its role for HL7 test results and ELM fidelity tracking, while SPEC_COVERAGE.md owns the exhaustive operator matrix.

## Risks / Trade-offs

- **Staleness risk** → Mitigate by keeping the document structure simple (status emoji edits) and including it in the development workflow for CQL feature changes. Future CI automation could enforce updates.
- **Initial accuracy** → Mitigate by cross-referencing multiple code locations per feature (AST enum + evaluator dispatch + tests). Some borderline cases (partial implementation, known bugs) may need judgment calls on ✅ vs ❌.
- **Document size** → ~170 operators × 4 columns plus keywords will be a large document. Mitigate with the category dashboard at top and clear section headers for navigation.
- **Maintenance burden** → Each new operator implementation requires a manual edit. Acceptable for now given the documentation-only scope. Automated tracking is a future capability.
