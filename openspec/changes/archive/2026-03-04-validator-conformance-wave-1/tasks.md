## 1. Security checks runtime configuration

- [x] 1.1 Add validator-level runtime configuration for security-checks mode with a default of disabled
- [x] 1.2 Update string security validation flow to emit informational issues when disabled and error issues when enabled
- [x] 1.3 Add unit/integration tests covering both security-checks modes and default behavior
- [x] 1.4 Ensure validation output reports effective security-checks mode at run start or summary

## 2. CLI and runner runtime option plumbing

- [x] 2.1 Add CLI options for security-checks mode while preserving backward-compatible defaults
- [x] 2.2 Add CLI option for optional terminology server endpoint and map it to validator terminology configuration
- [x] 2.3 Wire runtime options through command handlers into validator construction (`new` vs `with_terminology` path)
- [x] 2.4 Add CLI tests for default invocation compatibility and explicit conformance option behavior

## 3. Conformance quick-win validations

- [x] 3.1 Implement deterministic bundle uniqueness checks for targeted duplicate fullUrl and identity cases
- [x] 3.2 Implement UCUM/unit quick-win checks gated on terminology service availability
- [x] 3.3 Add informational reporting when UCUM/unit checks are skipped due to missing terminology service
- [x] 3.4 Add focused validator tests for quick-win pass/fail scenarios without introducing suite-specific exceptions

## 4. CI conformance execution and staged gating

- [x] 4.1 Add CI workflow/job to run selected stable conformance subsets with deterministic command parameters
- [x] 4.2 Publish per-run pass/fail conformance summaries and artifacts without historical trend tooling
- [x] 4.3 Configure staged gating modes (non-blocking first, optional blocking threshold configuration)
- [x] 4.4 Add CI documentation describing subset selection, outputs, and gating mode transitions

## 5. Documentation and rollout safeguards

- [x] 5.1 Update validator and CLI docs for new runtime options and effective-configuration reporting
- [x] 5.2 Document terminology-service-dependent UCUM/unit behavior and unsupported out-of-scope terminology parity
- [x] 5.3 Update roadmap/phase docs to reflect implemented quick wins and current conformance scope
- [x] 5.4 Run targeted test suite and conformance subset dry run, then record expected rollback toggles
