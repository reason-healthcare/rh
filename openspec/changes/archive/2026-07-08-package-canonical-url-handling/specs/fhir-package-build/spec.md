## MODIFIED Requirements

### Requirement: rh package build validates ImplementationGuide sync with package.json

The build SHALL check that key identity fields in `ImplementationGuide.json` are
consistent with `package.json` and fail with a descriptive error for blocking
identity mismatches. The build SHALL derive the expected `ImplementationGuide.url`
as `{package canonical}/ImplementationGuide/{ImplementationGuide.id}` and SHALL
surface URL mismatches as warnings, not build failures.

#### Scenario: Mismatched version fails build
- **WHEN** `package.json` has `"version": "1.0.0"` and `ImplementationGuide.json` has `"version": "1.1.0"`
- **THEN** the build fails with an error identifying the version mismatch

#### Scenario: Mismatched packageId fails build
- **WHEN** `package.json` has `"name": "example.fhir.pkg"` and `ImplementationGuide.json` has `"packageId": "other.pkg"`
- **THEN** the build fails with an error identifying the packageId/name mismatch

#### Scenario: Matching derived ImplementationGuide URL passes sync check
- **WHEN** `package.json` has `"url": "http://example.org/fhir"` and `ImplementationGuide.json` has `"id": "example.fhir.pkg"` and `"url": "http://example.org/fhir/ImplementationGuide/example.fhir.pkg"`
- **THEN** the sync check passes and the build continues

#### Scenario: Mismatched ImplementationGuide URL warns without failing
- **WHEN** `package.json` has `"url": "http://example.org/fhir"` and `ImplementationGuide.json` has `"id": "example.fhir.pkg"` and `"url": "http://wrong.example/ImplementationGuide/example.fhir.pkg"`
- **THEN** the sync check emits a warning with the derived expected URL and the build continues if no blocking identity mismatches exist

## ADDED Requirements

### Requirement: rh package init derives ImplementationGuide URL from generated id

When `rh package init` creates an `ImplementationGuide` resource, it SHALL set
`ImplementationGuide.url` to `{canonical}/ImplementationGuide/{ImplementationGuide.id}`.

#### Scenario: Generated ImplementationGuide URL matches generated id
- **WHEN** `rh package init` is run with package name `com.example.fhir` and canonical `https://example.org/fhir`
- **THEN** the generated `ImplementationGuide.id` is the CLI-derived IG id
- **AND** the generated `ImplementationGuide.url` is `https://example.org/fhir/ImplementationGuide/{ImplementationGuide.id}`

### Requirement: Package canonical is preserved literally

The packager SHALL use the `packager.toml` `canonical` value literally as the
package canonical base. It SHALL NOT reject or normalize a canonical solely
because it contains an `ImplementationGuide` path segment.

#### Scenario: Canonical that looks like an ImplementationGuide resource URL warns
- **WHEN** `packager.toml` contains `canonical = "http://example.org/fhir/ImplementationGuide/example.fhir"`
- **THEN** package commands emit a warning suggesting `http://example.org/fhir` but continue using the configured canonical literally

#### Scenario: Canonical with deeper ImplementationGuide path is accepted literally
- **WHEN** `packager.toml` contains `canonical = "http://example.org/fhir/ImplementationGuide/root/ns"`
- **THEN** package commands use that value as the canonical base without attempting to infer a different base

### Requirement: package.json includes canonical alias

Generated `package.json` SHALL include both `url` and `canonical` when
`packager.toml` provides a canonical value.

#### Scenario: Generated package manifest contains url and canonical
- **WHEN** `packager.toml` contains `canonical = "http://example.org/fhir"`
- **THEN** generated `package.json` contains `"url": "http://example.org/fhir"` and `"canonical": "http://example.org/fhir"`

### Requirement: Canonical resource URL mismatches SHALL warn without mutation

Package commands SHALL derive the expected resource URL as
`{canonical}/{resourceType}/{id}` for diagnostics when a resource type's R4
metadata defines a root `url` field. The packager SHALL warn when an existing
URL differs, SHALL ignore missing URL fields, and SHALL NOT rewrite resource
URLs.

#### Scenario: Existing canonical resource URL mismatch warns
- **WHEN** a `ValueSet` with `id = "example"` has `"url": "http://other.example/ValueSet/example"` and the package canonical is `http://example.org/fhir`
- **THEN** package commands warn that the expected URL is `http://example.org/fhir/ValueSet/example` and continue

#### Scenario: Missing canonical resource URL is ignored
- **WHEN** a `Library` has an `id` but no `url`
- **THEN** package commands do not fail or write a generated URL for that resource

#### Scenario: Non-canonical resource type is ignored
- **WHEN** a resource type without a root R4 `url` field is present
- **THEN** package commands do not apply canonical URL mismatch diagnostics to that resource
