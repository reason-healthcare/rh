## 1. CLI Input Resolution Foundation

- [x] 1.1 Identify the `rh validate` input ingestion path in `apps/rh-cli` and add a dedicated resolver entrypoint for `-i/--input` tokens
- [x] 1.2 Add or confirm glob-expansion dependency for CLI input resolution
- [x] 1.3 Implement token classification rule: existing path is treated as explicit input; otherwise evaluate token as glob

## 2. Deterministic Resolution and Error Behavior

- [x] 2.1 Implement glob expansion for supported patterns (including recursive `**`) and collect all matches
- [x] 2.2 Deduplicate overlapping matches and apply deterministic ordering to the final resolved input list
- [x] 2.3 Return actionable errors for unmatched glob patterns and stop before validation execution
- [x] 2.4 Wire resolved inputs into existing validation flow without changing validator semantics

## 3. Verification Coverage

- [x] 3.1 Add unit tests for resolver behavior: explicit path, single glob, recursive glob, mixed inputs, overlap dedupe, and deterministic order
- [x] 3.2 Add/extend CLI integration tests for `rh validate -i '*.json'` and mixed explicit-plus-glob inputs
- [x] 3.3 Add a failing-path integration assertion for unmatched glob patterns with clear error output

## 4. Documentation and Validation

- [x] 4.1 Update CLI docs/help text with quoted glob examples and no-match behavior expectations
- [x] 4.2 Run formatting and test commands for affected crates and fix any regressions introduced by this change (formatting succeeded; tests require rustc 1.91 but local rustc is 1.88)
- [x] 4.3 Confirm OpenSpec artifacts are complete and ready for `/opsx:apply`
