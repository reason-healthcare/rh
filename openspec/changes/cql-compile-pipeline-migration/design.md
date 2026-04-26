## Context

The `cql-multi-stage-pipeline-refactor` change introduced a three-stage CQL compilation architecture:

1. **Syntax** — `syntax/` module: tokenizer, parser, typed AST with source spans
2. **Semantics** — `semantics/` module: `SemanticAnalyzer`, symbol tables, type inference, overload resolution, implicit conversion rules
3. **Emit** — `emit/` module: `ElmEmitter`, CQL semantic IR → ELM JSON

This pipeline is fully implemented and exercised. However, `src/compiler.rs` `compile()` still delegates to `LibraryBuilder` (in `builder.rs`) which in turn uses `translator.rs`. The new pipeline is invoked alongside the old path for comparison purposes but is not yet the primary code path.

Design Decision 9 (from `cql-multi-stage-pipeline-refactor`) deferred this migration to a follow-up change, requiring conformance equivalence validation before switching.

## Goals / Non-Goals

**Goals:**
- Switch `compile()` to route through `SemanticAnalyzer` + `ElmEmitter` exclusively.
- Validate equivalence between old and new pipelines against the full test baseline before cutting over.
- Remove `builder.rs` and `translator.rs` after equivalence is confirmed.
- Preserve all existing API contracts (public `compile()`, `compile_to_elm()`, `compile_to_elm_with_sourcemap()` signatures unchanged).

**Non-Goals:**
- Expanding operator coverage in this change (that is the scope of conformance gap waves).
- Changes to the `syntax/` or `eval/` modules.
- Any modifications to CLI plumbing or external behavior.

## Decisions

1. **Equivalence gate before deletion**
   - Decision: Run the full existing test suite and conformance baseline against both the legacy and new pipeline paths before removing legacy code. Treat any behavioral delta as a blocking defect to fix in the new pipeline, not as an acceptable divergence.
   - Rationale: The new pipeline must produce identical or strictly improved output before the old path is removed. Divergences indicate either gaps in the new pipeline or bugs to fix.
   - Alternatives considered:
     - Delete first, fix after: rejected — risks breaking CI and obscures which path is authoritative.
     - Keep both paths forever: rejected — dual maintenance is the problem this change solves.

2. **Single compile() entry point after migration**
   - Decision: After migration, `compile()` has one code path through the three-stage pipeline. Remove all feature-flag or conditional routing between old and new paths.
   - Rationale: Conditional routing is the source of the current maintenance complexity. A clean cut is more maintainable.
   - Alternatives considered:
     - Keep legacy path behind a feature flag: rejected because this preserves maintenance burden and creates confusion about which path is authoritative.

3. **Deletion scope: builder.rs and translator.rs only**
   - Decision: Delete `builder.rs` and `translator.rs`. Keep `syntax/`, `semantics/`, `emit/`, `eval/`, `explain/`, `diagnostics/`, `sourcemap/` unchanged.
   - Rationale: The removal is scoped to the exact legacy components being replaced. Other modules are orthogonal.

4. **Source maps are preserved through migration**
   - Decision: The new pipeline's `ElmEmitter` already emits source map metadata. This capability must not regress; post-migration source map output must be at least equivalent to what `translator.rs` produced.
   - Rationale: Source maps are used by explain mode and CLI trace output; regression here would be user-visible.

## Migration Plan

1. Audit all callsites of `LibraryBuilder` and `translator.rs` within `compiler.rs` and any downstream callers.
2. Wire `compile()` to invoke `SemanticAnalyzer::analyze()` → `ElmEmitter::emit()` in place of `LibraryBuilder`.
3. Run the full test suite. Document any behavioral deltas and resolve them in the new pipeline.
4. Run the HL7 conformance suite and compare error counts against the pre-migration baseline. Treat regressions as blockers.
5. Remove `builder.rs` and `translator.rs` once all tests pass and the conformance baseline is met or improved.
6. Update module documentation and `src/lib.rs` to remove references to removed modules.

## Risks / Trade-offs

- **[Risk] New pipeline produces different ELM node structure for edge cases** → **Mitigation:** Golden test snapshots for ELM output catch structural regressions; add missing golden tests during equivalence validation if needed.
- **[Risk] Source map completeness regressions** → **Mitigation:** Existing explain-mode tests cover source map output; run explain-mode integration tests as part of the equivalence gate.
- **[Risk] Legacy path contains undocumented workarounds** → **Mitigation:** Diff `builder.rs` / `translator.rs` logic against semantic/emit equivalents during audit step; document any discovered differences.

## Resolved Questions

**Q: Are there any external callers of `LibraryBuilder` outside `compiler.rs`?**

Yes. `LibraryBuilder` is publicly re-exported from `src/lib.rs` via `pub use builder::LibraryBuilder`, with a doc comment already noting it is "superseded by the `compiler` API; retained as a legacy path." `library/manager.rs` contains a comment referencing it but no active usage. The migration should remove the `pub use` re-export; since `rh-cql` is pre-1.0, this is an acceptable semver-compatible breaking change within the beta cycle. No external callsites within the workspace use `LibraryBuilder` directly.

**Q: Does the new pipeline's ELM annotation output cover `localId` / `locator` metadata?**

Yes. The new emitter already supports locator emission controlled by `CompileOptions::locators_enabled()` — when enabled, `fields.locator` is populated with source span info. Tests in `src/emit/tests.rs` validate both the enabled and disabled paths. The `localId` field is not populated by either the legacy or new pipeline (no references in `builder.rs`), so there is no regression risk on that field. Source map correlation uses the `sourcemap/` module rather than ELM `localId` annotations.
