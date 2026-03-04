# conformance-ci-gating Specification

## Purpose
TBD - created by archiving change validator-conformance-wave-1. Update Purpose after archive.

## Requirements
### Requirement: CI executes selected conformance subsets repeatably
Project CI SHALL run selected FHIR conformance test subsets for this change using a repeatable command contract.

#### Scenario: CI run executes configured subset
- **WHEN** CI is triggered for a change that includes validator conformance checks
- **THEN** the configured conformance subset executes with stable inputs and deterministic command parameters

### Requirement: CI emits pass/fail conformance summaries without trend tooling
CI MUST publish per-run conformance pass/fail summaries for visibility and gating decisions, and SHALL NOT require historical trend tooling in this wave.

#### Scenario: Per-run summary is available without historical dashboards
- **WHEN** a conformance CI run completes
- **THEN** CI outputs per-run totals and pass/fail counts without storing or computing cross-run trend metrics

### Requirement: Gating policy is staged for stability
Conformance CI SHALL support a staged policy where early adoption can be non-blocking and later promoted to blocking thresholds.

#### Scenario: Non-blocking mode preserves merge flow
- **WHEN** conformance gating is configured as non-blocking
- **THEN** failed conformance checks are visible in CI output but do not fail the merge gate

#### Scenario: Blocking mode enforces configured threshold
- **WHEN** conformance gating is configured as blocking
- **THEN** CI fails when results do not meet the configured pass criteria
