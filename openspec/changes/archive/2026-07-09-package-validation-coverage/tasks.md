## 1. Package Validate Diagnostics

- [x] 1.1 Collect ERROR-severity validation issues with package resource labels in the validate processor.
- [x] 1.2 Return the first concrete ERROR issue in the processor failure message.
- [x] 1.3 Include a count of additional ERROR issues when more than one error is present.
- [x] 1.4 Add focused tests for validate processor failure messages and WARNING-only behavior.

## 2. Package Validation Context

- [x] 2.1 Register package-local StructureDefinitions, Questionnaires, and ValueSets with the validator context.
- [x] 2.2 Ensure installed dependency packages remain available for profile validation.
- [x] 2.3 Add package-level tests for local Questionnaire and ValueSet resolution.

## 3. Resource Shape Validation

- [x] 3.1 Report unknown R4 resource types as deterministic ERROR issues.
- [x] 3.2 Report unknown properties on known R4 resources with accurate paths.
- [x] 3.3 Validate choice-element names against R4 metadata.
- [x] 3.4 Add accepted/rejected tests for resource shape and choice-element cases.

## 4. Profile Rule Validation

- [x] 4.1 Enforce fixed and pattern constraints from applicable StructureDefinition snapshots.
- [x] 4.2 Validate Reference target type constraints from base/dependency/local profiles.
- [x] 4.3 Apply slice-aware cardinality, binding, fixed, and pattern rules.
- [x] 4.4 Add focused profile tests for valid and invalid fixed, pattern, reference, and slicing cases.

## 5. QuestionnaireResponse Validation

- [x] 5.0 Validate selected Questionnaire item authoring invariants, including `que-5` answer option applicability and incomplete answer option Coding warnings.
- [x] 5.1 Resolve Questionnaire resources from package-local and dependency context.
- [x] 5.2 Validate required answers and answer type compatibility.
- [x] 5.3 Validate answer options, local answerValueSet references, and reference answer constraints.
- [x] 5.4 Keep any lenient Questionnaire parsing internal without changing the public required-field model.
- [x] 5.5 Add QuestionnaireResponse validation tests for passing and failing resources.

## 6. Local ValueSet And Binding Validation

- [x] 6.1 Resolve local and dependency ValueSets by canonical URL with version-tolerant matching.
- [x] 6.2 Validate required bindings against local ValueSet expansion entries.
- [x] 6.3 Validate required bindings against local ValueSet compose include concepts.
- [x] 6.4 Preserve optional terminology-server behavior for checks that cannot be resolved locally.
- [x] 6.5 Add ValueSet and binding tests covering expansion, compose, missing code, and versioned canonical cases.

## 7. Scope And Compatibility Guardrails

- [x] 7.1 Avoid broad Measure, MeasureReport, Bundle, and narrative validation expansion.
- [x] 7.2 Keep cached validator internals private where possible and avoid new public API breaks.
- [x] 7.3 Add constructors/helpers if new public rule structs need safer construction.
- [x] 7.4 Update validator/package docs only for behavior that is actually implemented.

## 8. Verification

- [x] 8.1 Run focused `rh-validator` tests for new validation categories.
- [x] 8.2 Run focused `rh-packager` validate processor tests.
- [x] 8.3 Run relevant CLI validation integration tests if shared validator behavior changes CLI output.
- [x] 8.4 Run `cargo check -p rh-validator` and `cargo check -p rh-packager`.
