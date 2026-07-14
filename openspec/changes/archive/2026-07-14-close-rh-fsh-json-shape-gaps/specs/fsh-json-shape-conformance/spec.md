## ADDED Requirements

### Requirement: Maintain reviewed recursive JSON-shape fixtures
The conformance suite SHALL include reviewed SUSHI golden fixtures for each
corrected recursive JSON-shape behavior. The fixtures SHALL cover top-level and
embedded-resource shape decisions where applicable.

#### Scenario: Fixture covers recursive FHIR shapes
- **WHEN** a JSON-shape behavior involving an extension, primitive shadow,
  Bundle entry, Parameters part, repeating BackboneElement, or profile default
  is corrected
- **THEN** the suite contains a focused FSH fixture and reviewed SUSHI output
  that exercises the behavior

#### Scenario: Fixture suite compares generated JSON
- **WHEN** the SUSHI compatibility test suite is run with golden tests enabled
- **THEN** each recursive JSON-shape fixture compares `rh-fsh` output against
  its reviewed SUSHI JSON output

### Requirement: Diagnose JSON-shape regressions from project comparisons
The project comparison tooling SHALL retain enough normalized comparison detail
to identify the resource and JSON path of a representative JSON-shape mismatch
and associate it with a focused regression fixture or implementation slice.

#### Scenario: Project run reports representative shape path
- **WHEN** a project comparison identifies a JSON-shape first difference
- **THEN** the result identifies the affected resource and first-difference JSON
  path in its retained summary or pairwise artifacts

#### Scenario: Threshold reduction requires deterministic evidence
- **WHEN** a JSON-shape implementation wave lowers a configured project mismatch
  threshold
- **THEN** two consecutive full project comparison runs produce identical counts
  and category summaries before the new threshold is recorded
