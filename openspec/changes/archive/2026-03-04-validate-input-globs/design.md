## Context

The `rh validate` CLI currently expects explicit file paths for `-i/--input`, which makes batch validation cumbersome when users need to target many JSON files. The proposed capability adds glob support while preserving existing explicit-path behavior and deterministic validation execution.

Constraints:
- Keep CLI UX backward compatible.
- Avoid surprising shell-dependent behavior.
- Ensure repeatable ordering and duplicate suppression for stable runs and tests.

## Goals / Non-Goals

**Goals:**
- Accept glob patterns in `-i/--input` alongside direct file paths.
- Resolve inputs into a deterministic list of concrete files before validation.
- Fail with actionable messaging when a provided pattern resolves to no files.
- Keep validation semantics unchanged after input resolution.

**Non-Goals:**
- Changing validator business logic or result formatting.
- Adding implicit recursive directory scanning without explicit globs.
- Introducing shell-specific expansion requirements.

## Decisions

### 1) Add a dedicated input-resolution phase in the validate command

Implement a small resolver layer in `apps/rh-cli` that transforms raw input tokens into concrete file paths before existing validation begins.

Rationale:
- Isolates glob/path concerns from validator logic.
- Makes behavior testable via focused unit tests.
- Preserves current validation pipeline contracts.

Alternative considered:
- Resolve globs inline where files are iterated. Rejected because it interleaves parsing, expansion, and validation flow, reducing clarity and testability.

### 2) Resolution rule: existing path wins; otherwise treat token as glob pattern

For each input token:
1. If path exists as provided, include it.
2. Otherwise, evaluate as a glob pattern.
3. If glob yields no matches, collect a no-match error.

Rationale:
- Maintains backward compatibility for explicit paths.
- Enables intuitive mixed usage (`-i file.json -i '*.json'`).

Alternative considered:
- Always treat token as glob. Rejected because exact file references could behave unexpectedly with special characters.

### 3) Deterministic output list with deduplication

After resolving all tokens:
- Normalize and deduplicate file paths.
- Sort resolved paths deterministically before validation.

Rationale:
- Stable behavior across runs.
- Prevents duplicate validations from overlapping patterns.
- Reduces test flakiness.

Alternative considered:
- Preserve discovery order. Rejected because filesystem traversal ordering may vary by platform/environment.

### 4) Error model for unmatched patterns

If any token interpreted as a glob matches nothing, return an error that identifies each offending pattern and stop before validation.

Rationale:
- Surfaces operator mistakes early.
- Avoids silent partial validations.

Alternative considered:
- Warn and continue. Rejected for default behavior because it can mask input mistakes.

### 5) Use lightweight glob support in CLI layer

Use the existing dependency if already available in the workspace; otherwise add a minimal glob crate to `apps/rh-cli` (or workspace) for filesystem pattern expansion.

Rationale:
- Keeps implementation simple and maintainable.
- Avoids custom parser complexity.

Alternative considered:
- Implement custom wildcard matching. Rejected due to edge-case risk and maintenance burden.

## Risks / Trade-offs

- [Cross-platform path/glob differences] → Normalize paths and rely on crate behavior; cover with CLI/integration tests for common patterns.
- [Confusion between shell expansion and CLI expansion] → Document examples with quoted globs and clarify that CLI handles expansion.
- [Potentially large match sets] → Keep behavior explicit and user-driven; future optimization can stream or cap if needed.
- [Deterministic sort may differ from user expectation of discovery order] → Prefer reproducibility; document deterministic behavior.

## Migration Plan

1. Add/confirm glob dependency in CLI crate configuration.
2. Implement input resolver module/function in validate command path.
3. Wire resolver output into existing validation entrypoint.
4. Add unit tests for resolver (direct path, glob match, overlap dedupe, no-match).
5. Add/update CLI integration test for `-i` glob usage and mixed inputs.
6. Update CLI docs/help examples for quoted globs.

Rollback strategy:
- Revert resolver integration and dependency change; `-i` falls back to explicit-file-only behavior.

## Open Questions

- Should no-match behavior be strict error only, or should we add a future opt-in `--allow-empty-glob` mode?
- Should we canonicalize paths before dedupe (stronger uniqueness) or dedupe by normalized display path only?
- Do we want to surface a summary count of resolved files before validation starts for operator transparency?