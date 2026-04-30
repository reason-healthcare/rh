# fhir-package-lock

## Purpose

Defines the behaviour of `rh package lock` and `rh package lock-check` — commands that resolve
canonical references in FHIR source resources against installed dependency packages and record
the resolved mappings in `fhir-lock.json`. Also specifies how `rh package build` applies
canonical pinning from the lock file.

## Canonical Field Scoping

Pinning is applied only to JSON fields of FHIR type `canonical`. The following field names
are recognised as canonical-typed and are the only fields inspected or modified:

- `baseDefinition` — StructureDefinition base
- `valueSet` — ElementDefinition binding, ValueSet compose include
- `profile` — ElementDefinition type profile, `meta.profile`, CapabilityStatement profile
- `targetProfile` — ElementDefinition type target profile
- `supportedProfile` — CapabilityStatement supported profile
- `imports` — CapabilityStatement imports
- `instantiatesCanonical` — Task, CarePlan, RequestGroup, etc.
- `library` — Measure, PlanDefinition, ActivityDefinition
- `derivedFrom` — SearchParameter, Questionnaire, StructureDefinition

Non-canonical URL-bearing fields (e.g. `url`, `system`, `reference`, `endpoint`) are never
pinned.

## Requirements

### Requirement: rh package lock resolves and records canonical versions

`rh package lock <dir>` SHALL scan all `*.json` FHIR resources in the source directory for
canonical references in canonical-typed fields (see above), resolve each against the current
package's own resources and against dependency packages loaded from `~/.fhir/packages/`, and
write the resolved mappings to `<dir>/fhir-lock.json`.

Own-package canonicals (URLs whose base matches the `canonical` field in `packager.toml`) are
resolved against the source resources; external canonicals are resolved against dependency
packages.

#### Scenario: Unversioned canonical is resolved and recorded
- **WHEN** a source resource contains `"valueSet": "http://example.org/ValueSet/foo"` (no `|version`) and `http://example.org/ValueSet/foo` exists in a loaded dependency package at version `1.2.0`
- **THEN** `fhir-lock.json` contains an entry with `"url": "http://example.org/ValueSet/foo"`, `"resolvedVersion": "1.2.0"`, and `"pinned": true`

#### Scenario: Own-package canonical is resolved from source resources
- **WHEN** a source resource references a canonical URL whose base matches the package's own canonical base and a matching source resource exists at version `1.0.0`
- **THEN** `fhir-lock.json` records the entry with `"resolvedVersion": "1.0.0"` resolved from the source resource

#### Scenario: Unresolvable canonical is recorded with warning
- **WHEN** a canonical URL cannot be resolved in any loaded dependency package or source resource
- **THEN** `fhir-lock.json` records the entry with `"resolvedVersion": null` and the command emits a warning (non-fatal)

### Requirement: rh package lock requires dependency packages to be pre-downloaded

If a package listed in `package.json` `dependencies` is not present in the configured packages
directory, `rh package lock` SHALL fail with a clear error identifying the missing package.

#### Scenario: Missing dependency package fails with named error
- **WHEN** `package.json` declares `"hl7.fhir.us.core": "6.1.0"` as a dependency but that package is not present in `~/.fhir/packages/`
- **THEN** the command exits with a non-zero status and reports `hl7.fhir.us.core#6.1.0` as missing

### Requirement: fhir-lock.json format records pinning metadata

`fhir-lock.json` SHALL be a JSON object containing a `generated` timestamp, a `pinMode`
field, and a `canonicals` array.

#### Scenario: fhir-lock.json structure
- **WHEN** `rh package lock` completes successfully
- **THEN** `fhir-lock.json` contains `"generated"` (ISO 8601 timestamp), `"pinMode"` (`"pin-all"`), and `"canonicals"` (array of entries each with `url`, `resolvedVersion`, `resolvedPackage`, `pinned`)

### Requirement: Build applies canonical pinning from fhir-lock.json

When `fhir-lock.json` is present and `pinMode` is `"pin-all"`, `rh package build` SHALL
append `|<resolvedVersion>` to all unversioned canonical references in output resources that
have a matching pinned entry in the lock file.

#### Scenario: Pinned canonical is versioned in output
- **WHEN** a source resource contains `"valueSet": "http://example.org/ValueSet/foo"` and `fhir-lock.json` records `resolvedVersion: "1.2.0"` with `pinned: true`
- **THEN** the output resource contains `"valueSet": "http://example.org/ValueSet/foo|1.2.0"`

#### Scenario: Build without fhir-lock.json skips pinning
- **WHEN** `rh package build` is run and no `fhir-lock.json` exists in the source directory
- **THEN** canonical references in output resources are unchanged and a warning is emitted recommending `rh package lock`

### Requirement: rh package lock-check reports pinning status

`rh package lock-check <dir>` SHALL scan all canonical-typed fields in source resources and
report which references have `|version` appended (pinned) and which do not (unpinned), without
modifying any files.

#### Scenario: lock-check shows unpinned canonicals
- **WHEN** a source resource contains `"valueSet": "http://example.org/ValueSet/foo"` (no version)
- **THEN** `rh package lock-check` lists the reference under "unpinned" with the resource key and field path

#### Scenario: lock-check shows pinned canonicals
- **WHEN** a source resource contains `"valueSet": "http://example.org/ValueSet/foo|1.2.0"`
- **THEN** `rh package lock-check` lists the reference under "pinned" with the version

#### Scenario: lock-check exits non-zero when unpinned references exist
- **WHEN** any canonical-typed field contains an unversioned URL
- **THEN** `rh package lock-check` exits with a non-zero status

### Requirement: Known external code systems are not pinned

The publisher SHALL not attempt to pin canonical references to well-known external code systems
(SNOMED CT, LOINC, RxNorm, ICD-10, ICD-11) as these are not distributed as FHIR packages.

#### Scenario: SNOMED CT system URL is not pinned
- **WHEN** a resource references `http://snomed.info/sct` as a system
- **THEN** no entry for that URL appears in `fhir-lock.json` and no `|version` is appended in output
