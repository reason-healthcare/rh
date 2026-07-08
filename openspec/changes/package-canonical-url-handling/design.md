## Context

`rh-packager` derives package metadata from `packager.toml` and validates that
the package manifest and `ImplementationGuide` resource stay in sync. The
current sync behavior treats the package canonical as if it must equal
`ImplementationGuide.url`, which is not the FHIR resource URL pattern for an
ImplementationGuide.

This caused two related problems in the exploratory branch:

- rejecting or rewriting canonicals that contain `/ImplementationGuide/` would
  break projects that intentionally use that path segment in a package canonical
  root;
- accepting `ImplementationGuide.url == canonical` hides the real expected IG
  URL, which should be derived from the canonical root and IG id.

## Goals / Non-Goals

**Goals:**

- Preserve the author-provided package canonical literally.
- Validate/surface canonical URL inconsistencies as warnings instead of build
  failures.
- Keep generated `package.json` compatible by emitting both `url` and
  `canonical`.
- Use generated R4 metadata as the source of truth for whether a resource type
  has a root canonical `url` field.
- Improve validate processor errors enough that CI and users see the first
  concrete validation issue.
- Include the `crossbeam-epoch` audit fix required for this PR to pass CI.

**Non-Goals:**

- Do not normalize or rewrite resource `url` fields.
- Do not reject canonical roots that contain `/ImplementationGuide/`.
- Do not remove `canonical` from generated `package.json`.
- Do not expand validator conformance behavior beyond the package validate
  processor error message.
- Do not include CQL/FHIR JSON changes.

## Decisions

1. Use `packager.toml` `canonical` literally.

   Rationale: authors own their package canonical root. The CLI should not
   reinterpret a configured canonical as invalid solely because it contains an
   `ImplementationGuide` path segment.

   Alternative considered: reject any canonical containing
   `/ImplementationGuide/`. This catches common mistakes but is a migration
   break for valid projects with that path in the canonical root.

2. Derive, but do not write, expected resource URLs.

   Rationale: URL writing remains the author's/resource-generation
   responsibility. The packager should surface mismatches without mutating
   existing resources in surprising ways.

   For resources created by `rh package init`, the CLI is the author. New
   `ImplementationGuide.url` values should therefore use the same derivation
   rule and be based on the generated `ImplementationGuide.id`.

   Alternative considered: automatically repair missing or mismatched resource
   URLs. That would make output more consistent but changes author-authored
   resources and can hide source errors.

3. Treat canonical URL mismatches as warnings.

   Rationale: this PR is a compatibility-safe correction. Existing projects
   with mismatched URLs should still build while receiving actionable feedback.

   Alternative considered: fail `check`/`build` for mismatches. That would be
   stricter but is too disruptive for a packaging behavior correction.

4. Use R4 metadata to identify canonical resources.

   Rationale: generated FHIR metadata already knows which resource roots have a
   `url` field. Using it avoids a second hardcoded resource-type list that can
   drift.

   Alternative considered: maintain a local `CANONICAL_RESOURCE_TYPES` array.
   This is simpler locally but duplicates generated source-of-truth metadata.

5. Keep audit dependency update in this PR.

   Rationale: the packaging PR needs to pass CI independently, and the branch
   already identified a RustSec advisory in `crossbeam-epoch`. The
   implementation should prefer the smallest lockfile update that resolves the
   advisory rather than carrying the broad dependency churn from the exploratory
   branch.

## Risks / Trade-offs

- Warning-only behavior may let bad URLs continue into packages.
  Mitigation: warning messages include the expected derived URL so authors can
  fix source resources without blocking this compatibility-focused change.
- Adding `rh-hl7-fhir-r4-core` as a packager dependency expands the dependency
  graph.
  Mitigation: the dependency is already generated and local to the workspace,
  and docs-sync will record the graph change.
- Existing callers constructing `PackageJson` with struct literals must include
  the `canonical` field.
  Mitigation: this preserves generated package manifest compatibility and is
  narrower than removing the field.
