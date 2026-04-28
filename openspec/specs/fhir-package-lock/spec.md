# fhir-package-lock

## Purpose

Defines the behaviour of `rh publish lock` — the command that resolves unversioned canonical references in FHIR source resources against installed dependency packages and records the resolved mappings in `fhir-lock.json`. Also specifies how `rh publish build` applies canonical pinning from the lock file.

## Requirements

### Requirement: rh publish lock resolves and records canonical versions

`rh publish lock <dir>` SHALL scan all `*.json` FHIR resources in the source directory for
unversioned canonical references, resolve each against the dependency packages loaded from
`~/.fhir/packages/`, and write the resolved mappings to `<dir>/fhir-lock.json`.

#### Scenario: Unversioned canonical is resolved and recorded
- **WHEN** a source resource contains `"valueSet": "http://example.org/ValueSet/foo"` (no `|version`) and `http://example.org/ValueSet/foo` exists in a loaded dependency package at version `1.2.0`
- **THEN** `fhir-lock.json` contains an entry with `"url": "http://example.org/ValueSet/foo"`, `"resolvedVersion": "1.2.0"`, and `"pinned": true`

#### Scenario: Already-versioned canonical is recorded but not re-pinned
- **WHEN** a source resource contains `"valueSet": "http://example.org/ValueSet/foo|1.0.0"` (already pinned)
- **THEN** `fhir-lock.json` records the entry with `"pinned": false` indicating no action was needed

#### Scenario: Unresolvable canonical is recorded with warning
- **WHEN** a canonical URL cannot be resolved in any loaded dependency package
- **THEN** `fhir-lock.json` records the entry with `"resolvedVersion": null` and the command emits a warning (non-fatal)

### Requirement: rh publish lock requires dependency packages to be pre-downloaded

If a package listed in `package.json` `dependencies` is not present in the configured packages
directory, `rh publish lock` SHALL fail with a clear error identifying the missing package.

#### Scenario: Missing dependency package fails with named error
- **WHEN** `package.json` declares `"hl7.fhir.us.core": "6.1.0"` as a dependency but that package is not present in `~/.fhir/packages/`
- **THEN** the command exits with a non-zero status and reports `hl7.fhir.us.core#6.1.0` as missing

### Requirement: fhir-lock.json format records pinning metadata

`fhir-lock.json` SHALL be a JSON object containing a `generated` timestamp, a `pinMode`
field, and a `canonicals` array.

#### Scenario: fhir-lock.json structure
- **WHEN** `rh publish lock` completes successfully
- **THEN** `fhir-lock.json` contains `"generated"` (ISO 8601 timestamp), `"pinMode"` (`"pin-all"`), and `"canonicals"` (array of entries each with `url`, `resolvedVersion`, `resolvedPackage`, `pinned`)

### Requirement: Build applies canonical pinning from fhir-lock.json

When `fhir-lock.json` is present and `pinMode` is `"pin-all"`, `rh publish build` SHALL
append `|<resolvedVersion>` to all unversioned canonical references in output resources that
have a matching pinned entry in the lock file.

#### Scenario: Pinned canonical is versioned in output
- **WHEN** a source resource contains `"valueSet": "http://example.org/ValueSet/foo"` and `fhir-lock.json` records `resolvedVersion: "1.2.0"` with `pinned: true`
- **THEN** the output resource contains `"valueSet": "http://example.org/ValueSet/foo|1.2.0"`

#### Scenario: Build without fhir-lock.json skips pinning
- **WHEN** `rh publish build` is run and no `fhir-lock.json` exists in the source directory
- **THEN** canonical references in output resources are unchanged and a warning is emitted recommending `rh publish lock`

### Requirement: Known external code systems are not pinned

The publisher SHALL not attempt to pin canonical references to well-known external code systems
(SNOMED CT, LOINC, RxNorm, ICD-10, ICD-11) as these are not distributed as FHIR packages.

#### Scenario: SNOMED CT system URL is not pinned
- **WHEN** a resource references `http://snomed.info/sct` as a system
- **THEN** no entry for that URL appears in `fhir-lock.json` and no `|version` is appended in output
