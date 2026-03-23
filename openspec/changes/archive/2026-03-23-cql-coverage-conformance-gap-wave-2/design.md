## Context

Wave-1 closed a targeted wiring set and improved HL7 conformance results, but the documented post-wave baseline still shows large unimplemented-behavior totals (`Compile err=149`, `Eval err=572`, `Fail=0`, dated 2026-03-09 in `crates/rh-cql/CONFORMANCE.md`).

`crates/rh-cql/SPEC_COVERAGE.md` still lists unresolved non-clinical high/medium gaps (`TimeOfDay`, `Precision`/`LowBoundary`/`HighBoundary`, `Repeat` full semantics, `Size`, `Product`, `GeometricMean`) and `CONFORMANCE.md` still attributes substantial eval-error volume to nullological and aggregate/function families.

Wave-2 is a cross-stage closure effort across parser/semantic/emitter/eval plus docs/tests. The design prioritizes high-yield non-clinical gaps and keeps correctness invariant (`Fail=0`) as a hard gate.

## Goals / Non-Goals

**Goals:**
- Reduce compile/eval unimplemented outcomes from the wave-1 post baseline while preserving zero wrong-answer failures.
- Close the selected wave-2 operator/function set end-to-end (parse where applicable, semantic registration, emitter mapping, eval dispatch/implementation, tests).
- Make closure auditable via explicit before/after metrics and gap-to-evidence references in coverage/conformance docs.
- Keep implementation incremental by operator family to localize regressions.

**Non-Goals:**
- Full CQL 1.5.3 parity in a single wave.
- Clinical-context/FHIR-navigation operators (`AgeIn*`, `Children`, `Descendents`).
- Large runtime architecture changes (new retrieval model, broad query redesign, external service dependencies).

## Decisions

1. **Wave-2 scope is non-clinical, error-density first**
   - Decision: Prioritize operators that are both explicitly listed as open in `SPEC_COVERAGE.md` and/or represented in remaining conformance error buckets.
   - In-scope families:
     - nullological dispatch/registration (`IsNull`, `IsTrue`, `IsFalse`, `Coalesce`)
     - aggregate completion (`Product`, `GeometricMean`, aggregate dispatch parity)
     - date/time uncertainty and clock (`Precision`, `LowBoundary`, `HighBoundary`, `TimeOfDay`)
     - list/interval control (`Repeat` full fixpoint behavior, interval `Size`)
   - Alternatives considered:
     - Clinical operator wave now: rejected due to patient-context/model-navigation complexity.
     - Date/time full parity wave now: rejected as too large for bounded wave-2.

2. **Use a stage-contract checklist per operator family**
   - Decision: Every operator family must pass a fixed closure checklist: parser support (if needed), semantic signatures/overloads, canonical emit form, eval dispatch + semantics, targeted tests.
   - Rationale: Prior waves showed drift between stages as a primary source of compile/eval errors.
   - Alternatives considered:
     - Eval-only fixes: rejected because unresolved semantic/emit gaps reintroduce failures.

3. **Implement `Repeat` with deterministic fixpoint semantics and bounds safety**
   - Decision: Replace current stub behavior with spec-aligned iterative evaluation that terminates deterministically and handles nulls/duplicates consistently.
   - Rationale: `Repeat` is currently marked partial and blocks list-family completeness.
   - Alternatives considered:
     - Keep source-only stub: rejected; does not reduce documented gap.

4. **Treat conformance metrics as required acceptance artifacts**
   - Decision: Require pre/post metric table updates in `CONFORMANCE.md` and status transitions in `SPEC_COVERAGE.md` as part of completion, not optional documentation.
   - Rationale: Prevents unverifiable “implemented” claims.
   - Alternatives considered:
     - Code/test-only completion: rejected due to lack of auditability.

## Risks / Trade-offs

- **[Risk]** Date/time uncertainty semantics are subtle and may introduce edge-case regressions.  
  **Mitigation:** add precision- and boundary-focused cases plus HL7 suite rerun before closing wave.
- **[Risk]** `Repeat` fixpoint implementation could create non-termination or large runtime cost on pathological inputs.  
  **Mitigation:** enforce deterministic convergence checks and guardrail tests for termination behavior.
- **[Risk]** Parser changes for `Size` could affect existing grammar paths.  
  **Mitigation:** isolate grammar edits and add parser regression fixtures for neighboring constructs.
- **[Trade-off]** Deferring clinical/FHIR-navigation operators leaves some high-priority items open.  
  **Mitigation:** explicitly retain deferred list in docs and scope notes for wave-3 planning.

## Migration Plan

1. Record wave-2 baseline metrics from current `CONFORMANCE.md` (2026-03-09 post-wave-1 values).
2. Implement wave-2 families incrementally in this order: nullological -> aggregate -> date/time uncertainty -> repeat/size.
3. After each family, run focused tests; after full wave, run HL7 eval suite and confirm `Fail=0` plus improved compile/eval totals.
4. Update `SPEC_COVERAGE.md` and `CONFORMANCE.md` with dated deltas and evidence references.
5. If a family introduces regressions, revert only that family’s patch set and proceed with remaining validated families.

## Open Questions

- Should interval `Size` be implemented in wave-2 if parse/semantic support expansion materially increases scope, or deferred to wave-3?
- Is `Precision` support expected only for date/time operands in wave-2, or for all currently represented temporal/value forms?
- Should wave-2 include additional aggregate parity beyond `Product`/`GeometricMean` if testing shows adjacent aggregate gaps?
