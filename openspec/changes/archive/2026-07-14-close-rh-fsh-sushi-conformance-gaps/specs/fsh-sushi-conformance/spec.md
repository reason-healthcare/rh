## ADDED Requirements

### Requirement: Compare rh-fsh output against project-level SUSHI output
The system SHALL provide a project-level conformance runner that compiles real IG FSH source with both reference `fsh-sushi` and `rh-fsh`, then compares generated resources by `(resourceType, id)`.

#### Scenario: Run default project comparison
- **WHEN** the conformance runner is invoked with no project filter
- **THEN** it compares all configured default projects and writes JSON and Markdown summary reports

#### Scenario: Compare resource identity and content
- **WHEN** both tools successfully compile a project
- **THEN** the report includes missing resource count, extra resource count, mismatch count, and first-difference paths for mismatched resources

#### Scenario: Retain pairwise comparison artifacts
- **WHEN** the conformance runner is invoked with an artifacts directory
- **THEN** it writes raw and harness-normalized SUSHI and rh-fsh resources under identical deterministic filenames, per-resource unified diffs, and a manifest describing matches and differences

#### Scenario: Skip project with no FSH source
- **WHEN** a configured project has no `input/fsh/*.fsh` files
- **THEN** the runner marks that project as skipped rather than reporting a compiler failure

### Requirement: Categorize project-level SUSHI gaps
The conformance report SHALL categorize differences into actionable root-cause groups.

#### Scenario: Categorize resource identity differences
- **WHEN** a SUSHI resource is missing and a rh-fsh resource has the same id with a non-FHIR profile resourceType
- **THEN** the report categorizes the pair as a resource identity or base profile resolution gap

#### Scenario: Categorize JSON shape differences
- **WHEN** a mismatch is caused by an array/scalar difference, Coding/CodeableConcept shape, Quantity field shape, primitive shadow field, nested extension, or contained resource shape
- **THEN** the report categorizes the mismatch as JSON shape fidelity

#### Scenario: Categorize metadata differences
- **WHEN** a mismatch is isolated to canonical URL, title, description, status, version, count, or generated ImplementationGuide metadata
- **THEN** the report categorizes the mismatch as metadata fidelity

### Requirement: Track SUSHI conformance thresholds
The conformance capability SHALL define explicit thresholds for non-blocking baseline runs and future blocking gates.

#### Scenario: Baseline threshold report
- **WHEN** the runner completes
- **THEN** it reports whether each project is within configured thresholds without hiding the full diff counts

#### Scenario: Tighten thresholds after implementation wave
- **WHEN** a conformance wave is completed
- **THEN** the documented threshold and latest result are updated together in `CONFORMANCE.md`
