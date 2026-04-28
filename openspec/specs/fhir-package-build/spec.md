# fhir-package-build

## Purpose

Defines the behaviour of `rh publish build` and `rh publish pack` — the commands that assemble a FHIR Package from a source directory and compress it into a distributable tarball.

## Requirements

### Requirement: rh publish build assembles a FHIR Package from a source directory

`rh publish build <dir>` SHALL read a flat source directory containing `package.json`,
`ImplementationGuide.json`, and `*.json` FHIR resource files, execute configured lifecycle
hooks, and write an expanded output package directory conforming to the FHIR Package Spec
layout under `<dir>/output/package/`.

#### Scenario: Successful build produces output directory
- **WHEN** `rh publish build <dir>` is run against a valid source directory
- **THEN** an expanded package directory is written to `<dir>/output/package/` containing all processed FHIR resources and a `.index.json`

#### Scenario: Missing package.json fails with clear error
- **WHEN** `rh publish build <dir>` is run and `<dir>/package.json` does not exist
- **THEN** the command exits with a non-zero status and reports that `package.json` is required

#### Scenario: Missing ImplementationGuide.json fails with clear error
- **WHEN** `rh publish build <dir>` is run and no `ImplementationGuide.json` exists in the source directory
- **THEN** the command exits with a non-zero status and reports that an `ImplementationGuide` resource is required

### Requirement: rh publish pack produces a FHIR Package tarball

`rh publish pack <dir>` SHALL compress the expanded output directory into a `.tgz` tarball
named `<package-name>-<version>.tgz` written alongside the output directory, following the
FHIR Package Spec tarball layout (all entries under a `package/` path prefix).

#### Scenario: Pack produces correctly named tarball
- **WHEN** `rh publish pack <dir>` is run after a successful build
- **THEN** a file named `<name>-<version>.tgz` is written to `<dir>/output/` where `name` and `version` are read from `package.json`

#### Scenario: Pack tarball contains package/ prefix
- **WHEN** the produced `.tgz` is inspected
- **THEN** all entries are prefixed with `package/` as required by the FHIR Package Spec

### Requirement: rh publish build generates .index.json

The build SHALL generate a `.index.json` file in the output package directory listing every
FHIR resource with its `filename`, `resourceType`, `id`, `url`, and `version`.

#### Scenario: .index.json lists all resources
- **WHEN** a build completes with three FHIR resources
- **THEN** `.index.json` contains an entry for each resource with `filename`, `resourceType`, `id`, `url` (when present), and `version` (when present)

#### Scenario: .index.json index-version field
- **WHEN** `.index.json` is written
- **THEN** it contains `"index-version": 2` per the current FHIR Package Spec

### Requirement: rh publish build validates ImplementationGuide sync with package.json

The build SHALL check that key fields in `ImplementationGuide.json` are consistent with
`package.json` and fail with a descriptive error if they are not.

#### Scenario: Mismatched version fails build
- **WHEN** `package.json` has `"version": "1.0.0"` and `ImplementationGuide.json` has `"version": "1.1.0"`
- **THEN** the build fails with an error identifying the version mismatch

#### Scenario: Mismatched packageId fails build
- **WHEN** `package.json` has `"name": "example.fhir.pkg"` and `ImplementationGuide.json` has `"packageId": "other.pkg"`
- **THEN** the build fails with an error identifying the packageId/name mismatch

#### Scenario: Consistent fields pass sync check
- **WHEN** `name`, `version`, `url`, and `fhirVersions` are consistent between `package.json` and `ImplementationGuide.json`
- **THEN** the sync check passes and the build continues

### Requirement: Output directory is configurable

The default output directory SHALL be `<source-dir>/output/` but MAY be overridden via
`--output <path>` CLI flag.

#### Scenario: Custom output path
- **WHEN** `rh publish build <dir> --output /tmp/my-pkg` is run
- **THEN** the expanded package is written to `/tmp/my-pkg/package/`

### Requirement: rh publish check validates source directory without building

`rh publish check <dir>` SHALL run all pre-build validation (sync checks, hook dry-run checks)
and report results without writing any output files.

#### Scenario: Check exits non-zero on sync errors
- **WHEN** `rh publish check <dir>` detects a package.json / ImplementationGuide mismatch
- **THEN** the command exits with a non-zero status and prints the error without writing output
