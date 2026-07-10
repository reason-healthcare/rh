## 1. Structural Model

- [ ] 1.1 Add a validator-internal snapshot structural model that derives allowed child fields from StructureDefinition snapshot element paths.
- [ ] 1.2 Include base resource, backbone element, datatype, choice element, and primitive extension sibling paths in the structural model.
- [ ] 1.3 Overlay resolvable `meta.profile` snapshots on top of the base resource model without removing base-valid fields unless the profile explicitly constrains them.

## 2. Recursive Validation

- [ ] 2.1 Replace root-only R4 shape validation with recursive validation over the active structural model.
- [ ] 2.2 Report unknown nested properties with full paths, including array-containing paths such as `PlanDefinition.action[0].relatedAction[0].description`.
- [ ] 2.3 Preserve existing unknown resource type, unknown root property, and invalid choice-field diagnostics.
- [ ] 2.4 Accept valid nested backbone fields and valid concrete choice fields.

## 3. Expression Language Codes

- [ ] 3.1 Add local known-code validation for `Expression.language`.
- [ ] 3.2 Reject unsupported expression-language values such as `text/cql-identifier`.
- [ ] 3.3 Accept R4-known values such as `text/cql` and `text/fhirpath` without requiring a terminology server.

## 4. Package Validation Parity

- [ ] 4.1 Ensure the package validate processor continues to use the same `validate_auto()` behavior as `rh validate resource` for normal resources.
- [ ] 4.2 Add package-build validation tests proving nested unknown-property errors fail when `before_build = ["validate"]`.
- [ ] 4.3 Add package-build validation tests proving invalid `Expression.language` errors fail when `before_build = ["validate"]`.
- [ ] 4.4 Add a regression test confirming package build behavior is unchanged when the validate hook is not configured.

## 5. Scope Guardrails

- [ ] 5.1 Do not add new QuestionnaireResponse-vs-Questionnaire semantic validation in this change.
- [ ] 5.2 Do not make package build run validation implicitly without the validate hook.
- [ ] 5.3 Run focused `rh-validator` and `rh-packager` tests plus `cargo clippy -p rh-validator -p rh-packager --all-targets -- -D warnings`.
