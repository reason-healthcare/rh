## Why

Package canonical handling currently conflates the package canonical base with
resource canonical URLs, especially for `ImplementationGuide.url`. This causes
valid projects to fail sync checks or generate nested resource URLs when authors
use canonical roots that contain `ImplementationGuide` path segments.

## What Changes

- Treat `packager.toml` `canonical` as the literal package canonical base.
- Keep generated `package.json` emitting both `url` and `canonical`.
- Derive the expected `ImplementationGuide.url` as
  `{canonical}/ImplementationGuide/{ImplementationGuide.id}`.
- Generate new `ImplementationGuide.url` values in `rh package init` from the
  generated `ImplementationGuide.id`.
- Warn, but do not fail, when:
  - the configured canonical looks like an `ImplementationGuide` resource URL;
  - `ImplementationGuide.url` differs from the derived URL;
  - an existing canonical resource `url` differs from
    `{canonical}/{resourceType}/{id}`.
- Detect canonical resources from generated R4 metadata by checking for a root
  `url` field, rather than maintaining a hardcoded resource-type list.
- Improve package validate processor failures by including the first validation
  error in the returned error message.
- Update the dependency lockfile minimally to resolve the current `cargo audit`
  `crossbeam-epoch` advisory.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `fhir-package-build`: package build/check canonical sync and canonical resource
  URL warning behavior changes.
- `fhir-package-validate`: validate processor error messages include concrete
  validation details.

## Impact

- Affected crates: `rh-packager`.
- Affected metadata dependency: `rh-packager` will read generated R4 type
  metadata from `rh-hl7-fhir-r4-core`.
- Affected generated output: `package.json` continues to include both `url` and
  `canonical`.
- Affected CI/security: `Cargo.lock` updates `crossbeam-epoch` to the audited
  non-vulnerable version with minimal unrelated dependency churn.
- Compatibility: URL inconsistencies introduced by this change are surfaced as
  warnings, not build failures.
