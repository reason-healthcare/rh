## 1. Package Validate Diagnostics

- [x] 1.1 Collect ERROR-severity validation issues with package resource labels in the validate processor.
- [x] 1.2 Return the first concrete ERROR issue in the processor failure message.
- [x] 1.3 Include a count of additional ERROR issues when more than one error is present.
- [x] 1.4 Add focused tests for validate processor failure messages and WARNING-only behavior.

## 2. Package Validation Context

- [x] 2.1 Register package-local StructureDefinitions and ValueSets with the validator context.
- [x] 2.2 Ensure installed dependency packages remain available for profile validation.
- [x] 2.3 Add package-level tests for package validation context resolution.

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

## 5. Local ValueSet And Binding Validation

- [x] 5.1 Resolve local and dependency ValueSets by canonical URL with version-tolerant matching.
- [x] 5.2 Validate required bindings against local ValueSet expansion entries.
- [x] 5.3 Validate required bindings against local ValueSet compose include concepts.
- [x] 5.4 Preserve optional terminology-server behavior for checks that cannot be resolved locally.
- [x] 5.5 Add ValueSet and binding tests covering expansion, compose, missing code, and versioned canonical cases.

## 6. Scope And Compatibility Guardrails

- [x] 6.1 Avoid broad Measure, MeasureReport, Bundle, narrative, and QuestionnaireResponse validation expansion.
- [x] 6.2 Keep cached validator internals private where possible and avoid new public API breaks.
- [x] 6.3 Add constructors/helpers if new public rule structs need safer construction.
- [x] 6.4 Update validator/package docs only for behavior that is actually implemented.

## 7. Verification

- [x] 7.1 Run focused `rh-validator` tests for new validation categories.
- [x] 7.2 Run focused `rh-packager` validate processor tests.
- [x] 7.3 Run relevant CLI validation integration tests if shared validator behavior changes CLI output.
- [x] 7.4 Run `cargo check -p rh-validator` and `cargo check -p rh-packager`.
