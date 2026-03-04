# Conformance CI Subset

This document describes the initial conformance CI setup for `validator-conformance-wave-1`.

## Selected Subset

The CI workflow runs a stable conformance subset using:

```bash
cargo test -p rh-validator --test conformance_quick_wins_test
```

This subset validates:
- security-checks severity behavior (`information` when disabled, `error` when enabled)
- deterministic duplicate Bundle fullUrl detection
- deterministic duplicate Bundle resource identity detection
- UCUM/unit skip-note behavior when terminology is unavailable
- UCUM/unit terminology path activation when terminology is configured

## Scenario Traceability Matrix

| Scenario | Evidence |
|----------|----------|
| Security checks disabled uses informational severity | `security_checks_disabled_reports_information` in `crates/rh-validator/tests/conformance_quick_wins_test.rs` |
| Security checks enabled uses error severity | `security_checks_enabled_reports_error` in `crates/rh-validator/tests/conformance_quick_wins_test.rs` |
| Duplicate fullUrl in Bundle is reported | `document_bundle_duplicate_fullurl_is_reported` in `crates/rh-validator/tests/conformance_quick_wins_test.rs` |
| Duplicate logical identity in Bundle is reported | `bundle_duplicate_resource_identity_is_reported` in `crates/rh-validator/tests/conformance_quick_wins_test.rs` |
| Terminology service not configured skips UCUM/unit checks | `ucum_skip_note_present_without_terminology_service` in `crates/rh-validator/tests/conformance_quick_wins_test.rs` |
| Terminology service configured enables UCUM/unit checks | `ucum_skip_note_not_present_with_terminology_service` in `crates/rh-validator/tests/conformance_quick_wins_test.rs` |
| CLI enables security-checks mode via option | `test_validate_resource_with_security_checks_flag` in `apps/rh-cli/tests/validator_integration_test.rs` |
| CLI accepts terminology server option | `test_validate_resource_with_terminology_server_flag` in `apps/rh-cli/tests/validator_integration_test.rs` |
| Effective runtime configuration is printed | stderr assertions in `apps/rh-cli/tests/validator_integration_test.rs` |
| CI run executes configured subset | `.github/workflows/ci.yml` conformance job command `cargo test -p rh-validator --test conformance_quick_wins_test` |
| Per-run summary is available without historical dashboards | `.github/workflows/ci.yml` step summary + artifact `conformance-subset-summary` |
| Non-blocking mode preserves merge flow | `.github/workflows/ci.yml` branch `RH_CONFORMANCE_BLOCKING=false` |
| Blocking mode enforces configured threshold | `.github/workflows/ci.yml` branch `RH_CONFORMANCE_BLOCKING=true` |

## Outputs

Each CI run publishes:
- GitHub Step Summary with command and `test result` line
- `conformance-summary.txt` artifact (`conformance-subset-summary`)

No trend tooling or historical aggregation is included in this wave.

## Gating Modes

The conformance job supports staged gating via repository variable:

- `RH_CONFORMANCE_BLOCKING=false` (default): non-blocking visibility mode
- `RH_CONFORMANCE_BLOCKING=true`: blocking mode (job fails on subset failure)

This enables a safe rollout from observation to enforcement.

The CI workflow includes a deterministic policy validation step that exercises both blocking and non-blocking decision paths with a simulated failing exit code.

## Rollback Toggles

If instability occurs:
1. Set `RH_CONFORMANCE_BLOCKING=false` to return to non-blocking mode.
2. Narrow the subset command to the most stable tests until failures are addressed.
3. Keep summary artifact publication enabled for diagnostics.
