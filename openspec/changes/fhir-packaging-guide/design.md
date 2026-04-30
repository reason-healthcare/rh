## Context

`rh-packager` has a fully functional processor pipeline (FSH, snapshot, validate, CQL, shell) and
comprehensive reference documentation (`README.md`, `PROCESSORS.md`). However, there is no
end-to-end worked example showing how all these pieces fit together in a single project. New users
must synthesize the reference docs themselves, which creates a steep onboarding curve and means the
built-in processors often go unused.

The guide will be a single Markdown file (`GUIDE.md`) that walks from an empty directory to a
publishable `.tgz` FHIR Package, exercising every built-in processor in a natural, realistic
sequence.

## Goals / Non-Goals

**Goals:**
- Produce `crates/rh-packager/GUIDE.md` — a self-contained step-by-step tutorial
- Cover all five built-in processors: `fsh`, `snapshot`, `validate`, `cql`, `shell`
- Use a realistic but minimal clinical domain (e.g., a simple BP observation profile + CQL logic)
- Show a complete, runnable `packager.toml` wiring all processors into the `[hooks]` pipeline
- Show the resulting FHIR Package directory layout and `.tgz` output
- Link the guide from `README.md`

**Non-Goals:**
- Does not introduce any new Rust code or behavioral changes to `rh-packager`
- Does not replace or duplicate the existing `README.md` reference or `PROCESSORS.md` guide
- Does not cover deployment or publication to a FHIR registry
- Does not cover custom/native Rust processor authorship (already in `PROCESSORS.md`)

## Decisions

### Decision 1 — Single file vs. multi-file guide
**Choice**: Single `GUIDE.md` at the crate root.  
**Rationale**: Keeps discovery simple (one link from README), works well as a GitHub-rendered page,
and the content fits comfortably in one file (~300–500 lines). A `docs/` subdirectory would be
over-engineering for this scope.  
**Alternative considered**: `docs/guide/` with per-processor pages — rejected as too fragmented for
a linear tutorial.

### Decision 2 — Example clinical domain
**Choice**: A minimal "Blood Pressure Observation" profile with a single CQL library that flags
out-of-range readings.  
**Rationale**: BP is universally understood, maps naturally to a `StructureDefinition` + `ValueSet`
+ `CQL Library`, and keeps the example small enough that all code fits inline in the guide without
overwhelming the reader.

### Decision 3 — Shell processor example
**Choice**: A shell step that runs `echo` to append a build timestamp to `other/build-info.txt`.  
**Rationale**: A trivial shell command keeps the guide self-contained (no external tooling
required) while demonstrating the mechanism (env vars, working directory, resource exchange
protocol).

### Decision 4 — Processor ordering in the guide
**Choice**: `fsh` → `snapshot` → `validate` → `cql` → `shell`.  
**Rationale**: This is the natural dependency order: FSH compiles source FHIR; snapshot expands
profiles so validate can check fully-expanded resources; CQL can then reference the validated
profiles; shell runs last for any post-processing.

## Risks / Trade-offs

- [Risk] Example code in the guide drifts out of sync with actual processor behavior → Mitigation:
  keep the example minimal and clearly tied to concepts described in `README.md`; note in the guide
  that exact CLI flags / field names are authoritative in `README.md`
- [Risk] Guide becomes too long and loses readers → Mitigation: each step is bounded to a small
  code block + 2–3 sentences of explanation; use headings to allow skipping steps

## Open Questions

(none — scope is fully bounded)
