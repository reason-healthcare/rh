## Context

`rh-validator` currently applies `validate_auto()` from both `rh validate
resource` and the package validate hook, but its R4 structural checks are
root-oriented. The generated R4 resource structs and snapshots know nested
backbone/datatype paths such as `PlanDefinition.action.relatedAction`, but that
path information is not consistently used to reject unknown nested JSON fields.

## Goals / Non-Goals

**Goals:**

- Make structural validation derive allowed fields from base/profile
  StructureDefinition snapshot paths.
- Apply the same validation behavior through standalone resource validation and
  package validation when the validate hook is configured.
- Catch nested unknown properties, invalid nested choice fields, and selected
  known-code datatype values such as `Expression.language`.

**Non-Goals:**

- Do not make package builds run validation unless the validate hook is
  configured.
- Do not add new QuestionnaireResponse-vs-Questionnaire semantic validation.
- Do not require a remote terminology server for the expression-language check.

## Decisions

1. Build an active structural model from StructureDefinition snapshots.

   Rationale: snapshot paths are the source of truth for base resources,
   backbone elements, datatypes, and profile overlays. Using them avoids
   resource-specific checks such as hard-coding `PlanDefinition.relatedAction`.

   Alternative considered: extend `FHIR_TYPE_REGISTRY` root metadata manually.
   That would still miss profile-specific paths unless codegen also emitted a
   complete nested schema.

2. Use `validate_auto()` as the shared entrypoint.

   Rationale: CLI resource validation and package validation already call
   `validate_auto()` for normal resources. Keeping the shared entrypoint makes
   parity testable and avoids a second package-specific validator path.

   Alternative considered: add package-only checks in `rh-packager`. That would
   recreate the drift this change is intended to remove.

3. Treat `Expression.language` as a focused local known-code check.

   Rationale: R4 binds `Expression.language` to an extensible value set, but
   values from the HL7 expression-language code system can still be checked
   locally for known unsupported codes. This covers the observed
   `text/cql-identifier` failure without broad terminology-server parity.

   Alternative considered: enforce all extensible bindings as errors. That is
   too broad and would over-fail valid resources using external codes.

## Risks / Trade-offs

- More resources may fail validation after nested fields are checked.
  Mitigation: add focused regression tests and preserve existing severity
  behavior.
- Snapshot traversal may mishandle slices or choice paths.
  Mitigation: start with base/profile allowed-field derivation and add accepted
  tests for valid nested backbone and choice fields.
- Generated metadata may not expose all nested paths directly.
  Mitigation: derive from available snapshot/cardinality paths first; only
  change codegen if implementation shows the runtime snapshots cannot provide
  enough information.
