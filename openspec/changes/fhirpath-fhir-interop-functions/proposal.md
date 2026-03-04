## Why

`rh-fhirpath` documents strong core coverage but still lists key FHIR interoperability functions as not implemented (`resolve`, `memberOf`, `subsumes`, `subsumedBy`, `htmlChecks`, and related profile-aware behavior). These gaps directly affect advanced validator conformance scenarios.

## What Changes

- Implement a first wave of FHIR interoperability functions with explicit integration points for terminology and reference resolution.
- Define extension interfaces so validator and FHIRPath can share terminology/reference services without tight coupling.
- Add compliance-focused tests and examples for supported and unsupported paths.

## Capabilities

### New Capabilities
- `reference-resolution-function`: Implement `resolve()` semantics for supported reference forms and evaluation contexts.
- `terminology-aware-membership`: Implement `memberOf`, `subsumes`, and `subsumedBy` with pluggable terminology backends.
- `security-conformance-functions`: Implement `htmlChecks` and related conformance helpers used by validation workflows.

### Modified Capabilities
- `fhirpath-extension-integration`: Extend function-extension architecture to expose validated, composable service hooks to downstream crates.

## Impact

- Affected crates: `crates/rh-fhirpath`, `crates/rh-validator`, and CLI expression/validation flows in `apps/rh-cli`.
- Expected outcome: improved FHIRPath spec alignment and better validator support for profile/terminology-dependent constraints.
