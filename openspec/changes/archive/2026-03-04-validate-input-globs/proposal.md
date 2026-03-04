## Why

Validating many files currently requires listing each input path manually, which is slow and error-prone for common batch workflows. Adding glob support to `rh validate -i/--input` lets users validate multiple files with a single, predictable pattern.

## What Changes

- Extend `rh validate -i/--input` to accept glob patterns such as `*.json` and `**/*.json`.
- Support mixed inputs in one command invocation (direct file paths plus glob patterns).
- Resolve globs to concrete file paths before validation, with deterministic ordering.
- Deduplicate overlapping matches so the same file is validated once.
- Return a clear error when an input pattern matches no files.
- Preserve current behavior for direct file path inputs.

## Capabilities

### New Capabilities
- `validate-input-globs`: Allow `validate` CLI inputs to include glob patterns and process resolved files deterministically.

### Modified Capabilities
- None.

## Impact

- Affected CLI area: `apps/rh-cli` validate command argument ingestion and input resolution path.
- Likely dependency impact: add or use an existing Rust glob-matching crate in CLI/workspace dependencies.
- Test impact: add resolver-focused tests and CLI integration coverage for glob, mixed inputs, dedupe, and no-match behavior.
- User-facing impact: improved ergonomics for batch validation without changing validation semantics.
