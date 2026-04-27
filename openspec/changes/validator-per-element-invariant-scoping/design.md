## Context

`rh-validator` evaluates FHIR invariants (e.g. `dom-*`, `ext-1`, element-specific constraints) by running FHIRPath expressions via `rh-fhirpath`. The current implementation passes the FHIR resource root as the FHIRPath evaluation context for all invariants, regardless of where in the resource the invariant is declared.

FHIR defines two classes of invariants:
- **Resource-level invariants** (e.g. `dom-2`, `dom-3`): correctly use the resource root as context.
- **Element-level invariants**: declared on a specific `ElementDefinition` and should be evaluated with `%context` bound to that element node, not the resource root. When evaluated at the wrong scope, expressions using `%context`, relative paths, or element-specific navigation produce incorrect results.

Three tests in `crates/rh-validator/tests/invariant_scoping_test.rs` are marked `#[ignore]` with a comment pointing to this limitation. These represent known correctness failures, not missing features.

## Goals / Non-Goals

**Goals:**
- Pass the current element value as the FHIRPath `%context` and evaluation root when executing element-level invariant expressions.
- Un-ignore and pass all 3 tests in `invariant_scoping_test.rs`.
- Add regression coverage for nested element constraint scenarios.

**Non-Goals:**
- Broad rework of the FHIRPath evaluator unrelated to context passing.
- Changes to resource-level invariant behavior (must remain unchanged).
- Expanding the set of invariants evaluated beyond what is already wired.

## Decisions

1. **Distinguish element-level vs resource-level invariant context at the call site**
   - Decision: At the point where `validate_invariant()` is called for a given `ElementDefinition`, determine whether the invariant is element-scoped (declared on a non-root element) and, if so, resolve the element node from the resource value and pass it as the FHIRPath evaluation context.
   - Rationale: This is the minimal change that corrects the three known failures. The validator already walks element definitions during rule evaluation; the element path is available at the call site.
   - Alternatives considered:
     - Global context rework across all FHIRPath calls: rejected as high scope/risk and out of proportion to the targeted fix.
     - Post-processing correction pass: rejected because it would require re-evaluating expressions under a different context after the fact.

2. **FHIRPath evaluator context API extension**
   - Decision: No API extension needed. `EvaluationContext::with_current(element_value)` already exists in `rh-fhirpath` and provides exactly the semantics required: root = full resource, current = element node.
   - Rationale: `EvaluationContext::new(resource).with_current(element)` sets root to the resource and current to the element; FHIRPath evaluation starts from `current`, so invariant expressions using relative paths are scoped to the element while `%resource` / `%context` resolution continues to work correctly.
   - Alternatives considered:
     - Thread context through environment variables: rejected for poor discoverability and test isolation.

3. **Element node resolution strategy**
   - Decision: Use the existing FHIRPath navigation utilities to resolve the element's path within the resource value at validation time, producing the element node to use as context. If the path does not resolve (e.g. optional element not present), skip invariant evaluation for that element (consistent with current resource-level behavior for missing nodes).
   - Rationale: Consistent with how the validator already handles missing optional elements.

## Architecture Notes

The fix involves two coordinated steps:

1. **`rh-validator` invariant caller** (`validate_invariant` in `validator.rs`): When iterating elements at a path for element-level invariants:
   - Changed `EvaluationContext::new(element.clone())` to `EvaluationContext::new(resource.clone()).with_current(element.clone())` so root = full resource, current = element.
   - Added index tracking per element and a new `create_element_invariant_issue(rule, element_name, idx)` helper that includes `[at element_name[idx]]` in the message (e.g. `ext-1: ... [at extension[1]]`).

2. **Test updates** (`invariant_scoping_test.rs`):
   - Removed `#[ignore]` from the 3 tests.
   - Updated test data: the original test cases used extensions with only `valueString` (no url), which **pass** ext-1 (`extension.exists() != value.exists()` → `false != true` → true). The correct scenario that **fails** ext-1 is an extension with neither value[x] nor sub-extensions (e.g. `{"url": "..."}` — `false != false` → false → violation).
   - The nested-path test was updated to use `Patient.contact.extension` (present in the Patient snapshot) rather than `Patient.name.extension` (which is not in the Patient snapshot and would silently skip).

## Risks / Trade-offs

- **[Risk] Behavior change for invariants that previously evaluated (perhaps incorrectly) at resource scope** → **Mitigation:** Run full integration test suite before and after; any behavioral delta outside the 3 target tests is a regression to investigate.
- **[Risk] FHIRPath `%context` binding semantics differ across spec versions** → **Mitigation:** Cross-check against FHIR R4 spec definition of `%context` in invariant evaluation; document the binding contract in the updated code.

## Resolved Questions

**Q: Does `rh-fhirpath` already expose a mechanism to set `%context` independently of the evaluation root?**

Yes — no API extension is needed. `EvaluationContext::with_current(element_value)` creates a context where `current` = the element node and `root` = the original resource root (preserved unchanged). `%context` in `extensions/fhir/variables.rs` resolves directly from `context.current`, so calling `with_current(element_node)` before invoking the evaluator is sufficient. Root-anchored navigation still works correctly because `root` is unchanged.

**Q: Do any currently-passing invariant tests rely on root-level context for what are technically element-scoped invariants?**

This must be confirmed by a pre-flight run of the full validator integration suite with element-scoped context applied to all non-root `ElementDefinition` constraints. The `invariant_scoping_test.rs` tests that are currently `#[ignore]`-ed are the only known cases where the wrong behavior is intentionally preserved. Any other tests that begin failing after the change was unintentionally relying on incorrect root-scoped context and should be treated as newly-discovered correctness issues to resolve, not regressions to revert.
