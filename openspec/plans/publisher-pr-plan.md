# rh-publisher PR Plan

**Branch**: `feature/rh-publisher`  
**Change**: `openspec/changes/rh-publisher/`

This plan defines a sequenced set of pull requests for implementing the `rh-publisher` feature.
Each PR is independently reviewable and mergeable. The ordering follows strict dependency
layers — infrastructure first, pipeline second, processors third, polish last.

---

## PR 1 — Workspace Scaffold

**Tasks**: 1.1–1.5  
**Depends on**: nothing  
**Goal**: Establish the new `rh-publisher` crate and wire a stub `rh publish` subcommand into the CLI.

Creates the crate skeleton, adds `pulldown-cmark` and `toml` to workspace deps, adds the
`rh-publisher` crate dependency to `rh-cli`, and registers a `rh publish` subcommand that
exits with "not yet implemented". CI must pass (build + lint).

---

## PR 2 — Core Data Models

**Tasks**: 2.1–2.5  
**Depends on**: PR 1  
**Goal**: Deserializable `PackageJson`, `PublisherConfig`, and `PublishContext` types with full test coverage.

No I/O yet — just the structs, their serde implementations, and unit tests for round-trip
correctness and missing-field defaults. This is the shared foundation all subsequent PRs build on.

---

## PR 3 — Build Core (Load → Sync Check → Assemble)

**Tasks**: 3.1–3.5, 4.1–4.3, 5.1–5.3, 6.1–6.4  
**Depends on**: PR 2  
**Goal**: A working `rh publish build` that loads a source directory, validates IG sync, generates `.index.json`, and writes the expanded output directory. No hooks, no narrative, no locking yet.

This is the minimum viable build command. Provides a testable end-to-end path early, before
any processors are added. Integration test fixture created here.

---

## PR 4 — Tarball Pack

**Tasks**: 7.1–7.2, 8.2, 8.5  
**Depends on**: PR 3  
**Goal**: `rh publish pack` produces a `.tgz` with correct `package/` prefix from the built output directory.

Small, self-contained. Reuses `tar` + `flate2` already in the workspace (used by
`rh-foundation` for reading; this adds writing). Verifiable by inspecting tarball entries.

---

## PR 5 — Hook Runner Framework

**Tasks**: 9.1–9.5, 8.1 (updated to run hooks), 8.3, 8.5  
**Depends on**: PR 3  
**Goal**: `HookProcessor` trait, processor registry, sequential runner with abort-on-first-failure, `publisher.toml` hook config parsing, and unknown-name startup validation.

No concrete processors yet — the registry is empty except for a test-only no-op processor.
`rh publish build` is updated to call the hook runner at each stage. `rh publish check` is
implemented.

---

## PR 6 — Narrative Processor

**Tasks**: 10.1–10.5  
**Depends on**: PR 5  
**Goal**: Markdown → XHTML conversion, resource matching by filename stem, `.text` embedding, standalone file routing to `other/`.

Self-contained. Registered as a named processor `"narrative"` (always-on, not hook-configured
— runs as part of core build assembly). Requires `pulldown-cmark` added in PR 1.

---

## PR 7 — Canonical Lock

**Tasks**: 11.1–11.9, 8.4  
**Depends on**: PR 3  
**Goal**: `rh publish lock` scans source resources, resolves canonical URLs against dependency packages, writes `fhir-lock.json`; build applies pinning from lock file.

Can be developed in parallel with PRs 5 and 6 since it depends only on PR 3. The lock command
and build-time pinning are useful independently of any hook processor.

---

## PR 8 — `snapshot` Processor

**Tasks**: 12.1–12.6  
**Depends on**: PR 5  
**Goal**: `snapshot` built-in processor — finds StructureDefinitions lacking snapshots, runs `SnapshotGenerator` from `rh-foundation`, updates resource map.

Reuses the existing `SnapshotGenerator` API from `rh-foundation` with minimal glue. Small PR.
Recommended default position in `publisher.toml`: first in `before_build`.

---

## PR 9 — `validate` Processor

**Tasks**: 13.1–13.6  
**Depends on**: PR 5 (and ideally PR 8, so validator sees fully-snapshotted SDs)  
**Goal**: `validate` built-in processor — runs `rh-validator` over all resources in the map, ERROR-aborts, WARNING-passes.

Reuses `rh-validator` `FhirValidator` API. Wires `[validate]` config block for
`terminology_server`, `skip_invariants`, `skip_bindings`.

---

## PR 10 — `cql` Processor

**Tasks**: 14.1–14.10  
**Depends on**: PR 5  
**Goal**: `cql` built-in processor — discovers `.cql` files, compiles to ELM, embeds source + ELM into Library resources per CQL IG guidance, auto-creates Library when absent.

Largest processor PR. Reuses `rh-cql` compile pipeline and library source providers.
Recommended default position in `publisher.toml`: after `snapshot`, before `validate`.

---

## PR 11 — Integration Tests + Documentation

**Tasks**: 15.1–15.6, 16.1–16.4  
**Depends on**: PRs 8, 9, 10  
**Goal**: Full end-to-end integration tests across all subcommands and processors; `README.md` for `rh-publisher` and CLI update.

Fixture source directory covers the complete happy path (StructureDefinition, ValueSet, CQL
library, markdown narrative, `publisher.toml` with all three processors). Each integration test
is a CLI-level `assert_cmd` test, consistent with existing `rh-cli` test patterns.

---

## Suggested Parallelism

PRs 5, 6, and 7 can all start from PR 3 in parallel:

```
PR 1 → PR 2 → PR 3 ──┬── PR 4
                      ├── PR 5 ──┬── PR 6
                      │          ├── PR 8
                      │          ├── PR 9 (after PR 8)
                      │          └── PR 10
                      └── PR 7
                                └── PR 11 (after PR 8, 9, 10)
```
