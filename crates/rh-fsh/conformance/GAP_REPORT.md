# rh-fsh Conformance Gap Report

**Run date**: 2026-07-14
**Reference**: SUSHI 3.19.0, implementing FSH 3.0.0  
**Scope**: 63 local fixtures and six pinned implementation guide projects

## Executive Summary

Resource generation and identity are now complete for the measured corpus. All
63 fixture comparisons pass, all 150 library tests pass, and rh-fsh emits the
same 920 `(resourceType, id)` pairs as SUSHI across 379 real-project FSH files.
There are no missing or extra resources.

Content parity remains incomplete. Of the 920 shared resources, 461 (50.1%)
match exactly after normalization and 459 (49.9%) have at least one JSON
difference.

A project passing its threshold means it did not regress past this checked-in
baseline. It does not mean that the project is fully conformant.

## Reproduction Context

The runner verifies the exact revisions in `conformance/projects.json`:

| Project | Revision |
|---|---|
| CARIN Blue Button | `d23e3b70d641633d6944024a68b0fb535be8a168` |
| mCODE | `543477b5585f2dcc717820a2bea141ea85f03012` |
| Da Vinci CRD | `138d34a517753f2e4aa286a7f17ceb53f4354310` |
| Da Vinci DTR | `ca6c657644e60405010f11d5b98ad5ebd9ff3b45` |
| Da Vinci PAS | `e5329c6a216e1bf8ff32894e3827f9223e77628a` |
| IPS | `df270978cf5bf4ee7c907f005d722bed7235f384` |

SUSHI is invoked as `npx --yes fsh-sushi@3.19.0`. The CLI is built in an
isolated target directory to avoid stale workspace/package artifacts:

```bash
CARGO_TARGET_DIR=target/conformance cargo build -p rh-cli
cargo test -p rh-fsh --lib
cargo test -p rh-fsh --test sushi_compat -- --include-ignored
python3 crates/rh-fsh/conformance/scripts/run_sushi_comparison.py
```

Project checkouts with the wrong revision are rejected. `--update-projects`
restores the pinned revision but refuses to replace a dirty checkout.

## Fixture and Unit Results

| Layer | Passed | Failed | Unverified | Total |
|---|---:|---:|---:|---:|
| Library unit tests | 150 | 0 | 0 | 150 |
| SUSHI golden fixtures | 63 | 0 | 0 | 63 |

All fixture directories contain either reviewed SUSHI JSON or a
`.sushi-empty` marker proving that SUSHI intentionally emitted no resources.
The compatibility test treats a directory with neither as a failure. The CI
workflow explicitly runs the ignored compatibility suite.

## Project Results

| Project | FSH files | SUSHI | rh-fsh | Missing | Extra | Mismatch | Exact match |
|---|---:|---:|---:|---:|---:|---:|---:|
| CARIN Blue Button | 71 | 134 | 134 | 0 | 0 | 53 | 81 |
| mCODE | 57 | 350 | 350 | 0 | 0 | 155 | 195 |
| Da Vinci CRD | 69 | 85 | 85 | 0 | 0 | 51 | 34 |
| Da Vinci DTR | 39 | 75 | 75 | 0 | 0 | 33 | 42 |
| Da Vinci PAS | 20 | 158 | 158 | 0 | 0 | 96 | 62 |
| IPS | 123 | 118 | 118 | 0 | 0 | 71 | 47 |
| **Total** | **379** | **920** | **920** | **0** | **0** | **459** | **461** |

## Closed Gaps

The rerun and implementation work closed the following high-impact gaps:

- Local and dependency profile instances resolve to their base FHIR resource
  type while retaining the profile canonical in `meta.profile`.
- `Usage: #inline` instances are embedded at use sites and suppressed as
  top-level package resources.
- R4/R5 generated metadata includes direct-child cardinality/type information
  for BackboneElements instead of allowing nested fields to overwrite parents.
- Repeating elements, CodeableConcept wrappers, choice slices, indexed caret
  paths, primitive shadows, and recursive inline resources use schema metadata.
- Local and dependency StructureDefinition parents use their indexed canonical
  URL, including explicit version suffixes.
- Reference displays, hierarchical CodeSystem concepts, multiline ValueSet
  filters, and deterministic ValueSet group ordering are preserved.
- SUSHI config dependencies, publisher contacts, and explicit contacts are
  normalized into generated ImplementationGuide resources.
- Local extension slice definitions are compiled once into the shared schema;
  required nested slices are recursively materialized before optional assignments,
  and instance export no longer rebuilds a global extension URL map per resource.
- Dependency extension differentials contribute parent-scoped child names,
  canonical URLs, and cardinalities to the same schema. Explicit nested
  assignments use that metadata, while dependency cardinalities do not create
  defaults that may conflict with local project source.
- Required named extension placeholders remain internal until used. Their
  schema-derived canonical URL is attached on the first nested assignment and
  survives recursive Bundle and Parameters embedding without emitting unused
  placeholder extensions.
- Compiled profile views canonicalize local parent references expressed as an
  FSH `Id` back to the local entity name. Inherited local defaults and required
  slice ordering therefore apply through the same lineage path as name-based
  parents, including recursively embedded resources.
- Root caret paths preserve soft-index state across related rules. Extension
  `context[+]` and `context[=]` assignments now produce ordered arrays with
  primitive context-type codes and paired expressions instead of a malformed
  singleton object.
- Root caret paths also consult generated FHIR metadata for omitted indices.
  Repeating fields such as `StructureDefinition.contact` and nested `telecom`
  are array-wrapped even when the FSH uses `^contact.telecom` shorthand. This
  corrected the leading contact shape on 31 IPS StructureDefinitions; later
  differences keep those resources in the mismatch total.
- ValueSet compose systems and imported ValueSets resolve local names and ids
  to their canonical URLs. This removed 40 leading terminology differences
  across five projects.
- The harness pins SUSHI and project revisions, detects duplicate identities,
  records complete counts, and no longer accepts missing goldens as passes.
- Recursive instance export now applies dependency, local-profile, and explicit
  assignments through one typed tree; preserves primitive shadows, Bundle and
  Parameters recursion, integral decimal shape, and core extension canonical
  URLs without allowing core extension names to shadow resources. Two retained
  full-corpus manifests matched exactly before the CARIN, mCODE, and CRD
  thresholds were reduced.

These fixes reduced the original 43 missing, 149 extra, and 838 mismatched
resources to 0 missing, 0 extra, and 459 mismatched resources.

## Remaining Gap Categories

The runner assigns each mismatched resource to the category of its first JSON
difference. Counts therefore measure affected resources, not all bad fields.

| Category | Count | Share | Primary remaining work |
|---|---:|---:|---|
| JSON shape | 176 | 38.3% | Dependency-backed extension ordering, Bundle/Parameters recursion, optional backbone defaults, and remaining primitive shadow alignment |
| StructureDefinition | 217 | 47.3% | Root constraints, differential merging/order, slices, bindings, fixed/pattern values, cardinalities, and type profiles |
| Other | 14 | 3.0% | Reference details and uncategorized project-specific fields that need narrower classifiers |
| Metadata | 41 | 8.9% | Resource-specific defaults, names, descriptions, and derived metadata |
| IG generation | 6 | 1.3% | Dependency-only grouping resources, inferred pages, globals, and definition resource metadata |
| Terminology | 5 | 1.1% | Concept properties and remaining CodeSystem/ValueSet compose differences |
| Resource identity | 0 | 0.0% | Closed for the pinned corpus |

### JSON shape: 176 resources

This group is now concentrated in mCODE (97), CARIN (30), PAS (18), and IPS
(11). Repeated examples include:

- extensions emitted at the wrong nesting level or without aligned primitive
  `_field` companions;
- Bundle `entry.resource` and Parameters `parameter.part` shapes;
- repeating BackboneElements treated as scalars or later assignments replacing
  earlier repetitions;
- flat Coding-shaped objects where the target is a CodeableConcept;
- arrays such as `supportedProfile`, `targetProfile`, adjudication, category,
  and dose/rate fields.

The next implementation slice should make the recursive path-shape service the
only route for instance, contained, inline, Bundle, and Parameters assignments,
then define safe precedence rules before enabling dependency-derived defaults.

### StructureDefinition: 217 resources

Full invariant constraints, differential element merging, slicing metadata,
local profile type references, and fixed/pattern scalar values reduced this
leading category by six resources. Parent canonical URLs, logical paths, choice
slices, and indexed caret paths have also improved within many resources.
Remaining first differences are dominated by:

- constraints attached to the root differential element;
- complex slicing and reslicing;
- fixed versus pattern values and CodeableConcept wrapping;
- differential element ordering, merging, ids, and slice names;
- bindings and type/profile/targetProfile arrays.

The comparison currently reports only the first difference per resource. A
differential-aware diagnostic should report all bad element ids in one run.

### IG, terminology, metadata, and other: 66 resources

Every ImplementationGuide still differs. Configured page trees, grouping,
parameters, root metadata, and publisher plus explicit contacts are implemented;
Local definition resources now use SUSHI display-name ordering, FSH instance
titles/descriptions, and profile-aware example metadata. CARIN, mCODE, CRD, and
DTR move deeper into their resource arrays but still diverge where configured
groups introduce dependency-only entries. Remaining work includes those group
entries, inferred pages when `pages` is omitted, globals, and long-tail example
defaults.

Terminology gaps are now small and mainly involve concept properties and local
canonical resolution. The broad `other` group should be split as fixes proceed;
known examples include nested indentation context in instance rules and
resource-specific fields that the current classifier does not recognize.

## Generated Resource and Schema Impact

The schema-informed exporter exposed two errors in committed generated metadata:
nested StructureDefinition elements could overwrite their BackboneElement
parent's metadata, and inline structures or `contentReference` fields were not
navigable. The code generator now emits direct-child maps, dedicated nested
metadata types, and reference-target types while excluding profile snapshot
roots from nested inference. Both committed R4 and R5 crates were regenerated
and tested. This keeps shape decisions static and reproducible without adding
runtime package traversal.

## Evidence Limitations

- The runner normalizes volatile or publisher-generated fields before comparing;
  exact-match counts apply to the normalized representation.
- Only the first difference per shared resource determines its category.
- Detailed mismatch arrays in JSON are capped at 100 entries per project, while
  aggregate counts remain complete.
- The six projects are a broad regression corpus, not an exhaustive proof of
  every FSH 3.0 language behavior.

Machine-readable evidence is in `results/latest-summary.json`; the generated
human summary is in `results/latest-summary.md`. The remaining sequence and exit
criteria are maintained in `IMPROVEMENT_PLAN.md`.
