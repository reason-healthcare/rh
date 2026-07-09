## Context

`rh-packager` exposes validation through the `validate` lifecycle hook, which
runs `rh-validator` over the in-memory package resources. That makes package
validation only as useful as the validator coverage and diagnostics available at
build time. The exploratory branch mixed useful validator improvements with
broader conformance work, performance changes, and unrelated validation areas.

This proposal narrows the work to package-relevant FHIR validation: resource
shape, profile rules, references, Questionnaires, local ValueSets, and clear
package pipeline diagnostics.

## Goals / Non-Goals

**Goals:**

- Improve package validate failures so authors see the first concrete resource
  error without digging through logs.
- Add validator coverage for FHIR authoring mistakes likely to appear in IG
  packages.
- Use package resources and installed dependencies as validation context.
- Preserve severity behavior: ERROR fails, WARNING does not.
- Keep public Rust API compatibility where practical.

**Non-Goals:**

- Do not add broad Measure, MeasureReport, Bundle, or narrative validation.
- Do not change package canonical/url handling.
- Do not change CQL behavior.
- Do not require a terminology server for local package validation.
- Do not make package validation silently rewrite resources.
- Do not introduce public API breaks for cached validator internals unless they
  are explicitly reviewed.

## Decisions

1. Scope validation around package authoring errors.

   Rationale: package validation should catch errors authors can fix in source
   resources: unknown fields, invalid choice fields, profile rule violations,
   bad references, QuestionnaireResponse mismatches, and local ValueSet/binding
   issues.

   Alternative considered: carry the full exploratory validator expansion. That
   was too large for this branch and included validation areas already removed
   from scope.

2. Keep `rh-validator` reusable and package-agnostic.

   Rationale: the checks belong in `rh-validator`, but `rh-packager` should own
   package-specific orchestration, context loading, and error reporting.
   Resource-shape checks such as unknown R4 resource types, unknown properties,
   and invalid concrete choice fields are core validator behavior and therefore
   apply anywhere `rh-validator` validates a resource, including standalone
   resource validation and package validation. Package-local context loading is
   not core validator behavior; it is performed by the package validate
   processor because only that path has the package's in-memory resource set.

   Alternative considered: implement package-only checks in `rh-packager`. That
   would duplicate validator logic and leave the CLI/library validator behind.

3. Prefer local context before remote terminology.

   Rationale: package validation should work offline by using package-local
   ValueSets, Questionnaires, StructureDefinitions, and installed dependency
   packages. A terminology server remains optional for checks that require it.

   Alternative considered: require terminology server access for binding checks.
   That would make packaging less deterministic and harder to run in CI.

4. Preserve validation severity semantics.

   Rationale: the validate processor already fails on ERROR and continues on
   WARNING. New checks should classify issues carefully rather than changing the
   pipeline contract.

   Alternative considered: make all newly detected inconsistencies warnings to
   avoid disruption. That weakens validation and hides real FHIR errors from
   package builds that intentionally enable validation.

5. Keep public APIs stable where possible.

   Rationale: earlier exploration introduced public struct field and ownership
   changes. This proposal should keep caches and compiled internals private where
   possible, and add constructors/helpers instead of forcing external struct
   literal updates.

   Alternative considered: expose internal cached objects directly for
   performance. That is unnecessary for the package validation scope and creates
   avoidable compatibility risk.

## Risks / Trade-offs

- Additional ERROR findings may fail packages that previously passed.
  Mitigation: changes apply through the existing validate hook contract, include
  precise issue paths/messages, and preserve WARNING as non-blocking.
- Slice/profile validation can be complex and may over-report.
  Mitigation: target focused profile constructs first and add regression tests
  for accepted resources as well as rejected ones.
- Local ValueSet validation cannot cover every terminology case.
  Mitigation: validate package-local expansion/compose cases offline and keep
  terminology-server behavior optional.
- QuestionnaireResponse validation may need lenient loading of partial
  Questionnaire fixtures.
  Mitigation: use internal parsing helpers if needed without weakening public
  Questionnaire API requirements.
