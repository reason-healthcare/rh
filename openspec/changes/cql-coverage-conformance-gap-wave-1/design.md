## Context

`rh-cql` has strong safety on implemented behavior (0 wrong-answer failures) but still has significant unimplemented surface area documented in:
- `crates/rh-cql/SPEC_COVERAGE.md` (143/175 operators fully implemented, 82%)
- `crates/rh-cql/CONFORMANCE.md` (2026-03-09 baseline: 149 compile errors, 628 eval errors in HL7 suite)

Most remaining gaps fall into two groups:
1. Existing implementations are present but not connected through semantic registration, emitter mapping, or eval dispatch.
2. Small but high-value operator families are absent end-to-end.

The change targets a bounded conformance wave that closes high-yield gaps without destabilizing already-correct operator behavior.

## Goals / Non-Goals

**Goals:**
- Reduce conformance errors by closing a scoped set of high-impact operator gaps from the coverage/conformance documents.
- Preserve the existing invariant of zero wrong-answer regressions.
- Make progress auditable through updated coverage tables, conformance metrics, and targeted regression tests.
- Improve wiring consistency between semantic signatures, emitter mapping, and eval dispatch.

**Non-Goals:**
- Full CQL 1.5.3 parity in one wave.
- FHIR patient-context clinical operators (`AgeIn*`, `Children`, `Descendents`) requiring broader model/runtime work.
- Retrieval architecture redesign, multi-source query redesign, or new external dependencies.

## Decisions

1. **Wave-1 scope is wiring-first and metric-driven**
   - Decision: Prioritize operators with existing partial implementation and high error contribution before larger new subsystems.
   - Rationale: Provides fast conformance gains with lower regression risk.
   - Alternatives considered:
     - Full feature parity push: rejected for high scope/risk in one increment.
     - Parser-only pass: rejected because current error volume is predominantly eval/dispatch related.

2. **Use explicit operator coverage contracts in code**
   - Decision: Add or tighten centralized mappings for:
     - system function name -> typed ELM node emission
     - semantic operator/function signatures -> resolver registration
     - ELM/operator variant -> eval dispatch branch
   - Rationale: Most observed gaps are contract mismatches between stages, not missing primitives.
   - Alternatives considered:
     - Ad hoc fixes per failing test: rejected due to repeated drift risk and poor maintainability.

3. **Preserve zero-wrong-answer quality gate**
   - Decision: Keep `Fail = 0` as a mandatory invariant while reducing compile/eval error counts.
   - Rationale: Existing correctness is a core quality property and must not regress.
   - Alternatives considered:
     - Temporary tolerance of wrong answers for velocity: rejected as unsafe.

4. **Document conformance progress as a versioned baseline delta**
   - Decision: Treat 2026-03-09 full-suite metrics as the baseline and require post-change deltas in docs.
   - Rationale: Prevents ambiguous claims and enables clear measurement.
   - Alternatives considered:
     - Narrative-only updates: rejected because they are not auditable.

## Risks / Trade-offs

- **[Risk] Dispatch/registration additions may alter null semantics in existing operators** -> **Mitigation:** expand targeted null-semantics tests alongside each operator family touched.
- **[Risk] Conformance metric changes become stale as fixtures evolve** -> **Mitigation:** include run date, command, and baseline reference in doc updates.
- **[Risk] Scope creep into broader clinical/query features** -> **Mitigation:** enforce wave scope boundaries in tasks and defer out-of-scope items.
- **[Trade-off] Focused wave leaves known low-priority gaps unresolved** -> **Mitigation:** record deferred items explicitly in updated gap summaries.

## Migration Plan

1. Implement emitter/semantic/eval changes for the selected operator set behind existing code paths (no API break).
2. Add or update focused tests for each changed operator family.
3. Run targeted `rh-cql` test suites and HL7 conformance command to verify:
   - zero wrong-answer failures remain intact
   - compile/eval error counts improve from baseline
4. Update `SPEC_COVERAGE.md` and `CONFORMANCE.md` with dated post-change metrics and evidence links.
5. If regression appears, revert the affected operator-family patch set while preserving unrelated wave improvements.

## Open Questions

- What minimum error-reduction threshold should be required for this wave to be considered complete?
- Should the first wave include any parser additions (if uncovered during implementation), or strictly stay eval/emitter/semantic?
- Should deferred clinical operators be split into a dedicated follow-up capability immediately after this wave?
