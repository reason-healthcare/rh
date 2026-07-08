# fhir-package-build

## Purpose

Defines the behaviour of `rh package build` and `rh package pack` — the commands that assemble a FHIR Package from a source directory and compress it into a distributable tarball.
## Requirements
### Requirement: rh package build assembles a FHIR Package from a source directory

`rh package build <dir>` SHALL read a flat source directory containing `package.json`,
`ImplementationGuide.json`, and `*.json` FHIR resource files, execute configured lifecycle
hooks, and write an expanded output package directory conforming to the FHIR Package Spec
layout under `<dir>/output/package/`.

#### Scenario: Successful build produces output directory
- **WHEN** `rh package build <dir>` is run against a valid source directory
- **THEN** an expanded package directory is written to `<dir>/output/package/` containing all processed FHIR resources and a `.index.json`

#### Scenario: Missing package.json fails with clear error
- **WHEN** `rh package build <dir>` is run and `<dir>/package.json` does not exist
- **THEN** the command exits with a non-zero status and reports that `package.json` is required

#### Scenario: Missing ImplementationGuide.json fails with clear error
- **WHEN** `rh package build <dir>` is run and no `ImplementationGuide.json` exists in the source directory
- **THEN** the command exits with a non-zero status and reports that an `ImplementationGuide` resource is required

### Requirement: rh package pack produces a FHIR Package tarball

`rh package pack <dir>` SHALL compress the expanded output directory into a `.tgz` tarball
named `<package-name>-<version>.tgz` written alongside the output directory, following the
FHIR Package Spec tarball layout (all entries under a `package/` path prefix).

#### Scenario: Pack produces correctly named tarball
- **WHEN** `rh package pack <dir>` is run after a successful build
- **THEN** a file named `<name>-<version>.tgz` is written to `<dir>/output/` where `name` and `version` are read from `package.json`

#### Scenario: Pack tarball contains package/ prefix
- **WHEN** the produced `.tgz` is inspected
- **THEN** all entries are prefixed with `package/` as required by the FHIR Package Spec

### Requirement: rh package build generates .index.json

The build SHALL generate a `.index.json` file in the output package directory listing every
FHIR resource with its `filename`, `resourceType`, `id`, `url`, and `version`.

#### Scenario: .index.json lists all resources
- **WHEN** a build completes with three FHIR resources
- **THEN** `.index.json` contains an entry for each resource with `filename`, `resourceType`, `id`, `url` (when present), and `version` (when present)

#### Scenario: .index.json index-version field
- **WHEN** `.index.json` is written
- **THEN** it contains `"index-version": 2` per the current FHIR Package Spec

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

### Requirement: Output directory is configurable

The default output directory SHALL be `<source-dir>/output/` but MAY be overridden via
`--output <path>` CLI flag.

#### Scenario: Custom output path
- **WHEN** `rh package build <dir> --output /tmp/my-pkg` is run
- **THEN** the expanded package is written to `/tmp/my-pkg/package/`

### Requirement: rh package check validates source directory without building

`rh package check <dir>` SHALL run all pre-build validation (sync checks, hook dry-run checks)
and report results without writing any output files.

#### Scenario: Check exits non-zero on sync errors
- **WHEN** `rh package check <dir>` detects a package.json / ImplementationGuide mismatch
- **THEN** the command exits with a non-zero status and prints the error without writing output

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

