## ADDED Requirements

### Requirement: evaluate_elm_with_libraries API function

The eval engine SHALL expose a public `evaluate_elm_with_libraries(library, included: &HashMap<String, &elm::Library>, expression_name, context) -> Result<Value>` function that evaluates a named expression from `library` with cross-library reference resolution. The `included` map keys are include aliases as declared in the main library.

#### Scenario: Named expression with cross-library ref evaluates correctly
- **WHEN** `evaluate_elm_with_libraries(main_lib, included, "ReportCase", ctx)` is called and `ReportCase` references `CaseLogic."Some Expression"`
- **THEN** the engine resolves `CaseLogic` from `included`, evaluates `Some Expression` in that library, and returns the correct composite result

#### Scenario: Empty included map falls back gracefully
- **WHEN** `evaluate_elm_with_libraries(lib, &HashMap::new(), "SimpleExpr", ctx)` is called and `SimpleExpr` has no cross-library refs
- **THEN** evaluation completes normally, identical to `evaluate_elm(lib, "SimpleExpr", ctx)`

#### Scenario: Cross-library function call with arguments resolves
- **WHEN** a main library calls `FHIRHelpers.ToInterval(period)` and `FHIRHelpers` is in `included`
- **THEN** the `ToInterval` function body from `FHIRHelpers` is evaluated with `period` bound and the result returned
