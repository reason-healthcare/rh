# rh-fsh Conformance Gap Report

**Run date**: 2026-07-10  
**Reference**: SUSHI 3.19.0, implementing FSH 3.0.0  
**Scope**: 61 local fixtures and six pinned implementation guide projects

## Executive Summary

Resource generation and identity are now complete for the measured corpus. All
61 fixture comparisons pass, all 114 library tests pass, and rh-fsh emits the
same 920 `(resourceType, id)` pairs as SUSHI across 379 real-project FSH files.
There are no missing or extra resources.

Content parity remains incomplete. Of the 920 shared resources, 366 (39.8%)
match exactly after normalization and 554 (60.2%) have at least one JSON
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
| Library unit tests | 114 | 0 | 0 | 114 |
| SUSHI golden fixtures | 61 | 0 | 0 | 61 |

All fixture directories contain either reviewed SUSHI JSON or a
`.sushi-empty` marker proving that SUSHI intentionally emitted no resources.
The compatibility test treats a directory with neither as a failure. The CI
workflow explicitly runs the ignored compatibility suite.

## Project Results

| Project | FSH files | SUSHI | rh-fsh | Missing | Extra | Mismatch | Exact match |
|---|---:|---:|---:|---:|---:|---:|---:|
| CARIN Blue Button | 71 | 134 | 134 | 0 | 0 | 77 | 57 |
| mCODE | 57 | 350 | 350 | 0 | 0 | 167 | 183 |
| Da Vinci CRD | 69 | 85 | 85 | 0 | 0 | 62 | 23 |
| Da Vinci DTR | 39 | 75 | 75 | 0 | 0 | 52 | 23 |
| Da Vinci PAS | 20 | 158 | 158 | 0 | 0 | 122 | 36 |
| IPS | 123 | 118 | 118 | 0 | 0 | 74 | 44 |
| **Total** | **379** | **920** | **920** | **0** | **0** | **554** | **366** |

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
- The harness pins SUSHI and project revisions, detects duplicate identities,
  records complete counts, and no longer accepts missing goldens as passes.

These fixes reduced the original 43 missing, 149 extra, and 838 mismatched
resources to 0 missing, 0 extra, and 554 mismatched resources.

## Remaining Gap Categories

The runner assigns each mismatched resource to the category of its first JSON
difference. Counts therefore measure affected resources, not all bad fields.

| Category | Count | Share | Primary remaining work |
|---|---:|---:|---|
| JSON shape | 209 | 37.7% | Extension ordering, Bundle/Parameters recursion, optional backbone defaults, and remaining primitive shadow alignment |
| StructureDefinition | 234 | 42.2% | Root constraints, context, differential merging/order, slices, bindings, fixed/pattern values, cardinalities, and type profiles |
| Other | 68 | 12.3% | Reference details, uncategorized project-specific fields, and metadata paths that need narrower classifiers |
| Metadata | 31 | 5.6% | Resource-specific defaults, names, descriptions, and derived metadata |
| IG generation | 6 | 1.1% | Page trees, grouping, globals, parameters, and definition resource metadata |
| Terminology | 6 | 1.1% | Concept properties and remaining CodeSystem/ValueSet canonical/compose differences |
| Resource identity | 0 | 0.0% | Closed for the pinned corpus |

### JSON shape: 209 resources

This group has fallen by 225 resources and is now concentrated in mCODE (106),
IPS (34), CARIN (32), and PAS (18). Repeated first differences include:

- extensions emitted at the wrong nesting level or without aligned primitive
  `_field` companions;
- Bundle `entry.resource` and Parameters `parameter.part` shapes;
- repeating BackboneElements treated as scalars or later assignments replacing
  earlier repetitions;
- flat Coding-shaped objects where the target is a CodeableConcept;
- arrays such as `supportedProfile`, `targetProfile`, adjudication, category,
  and dose/rate fields.

The next implementation slice should make one recursive path-shape service the
only route for instance, contained, inline, Bundle, and Parameters assignments.

### StructureDefinition: 234 resources

The count is unchanged as a leading category, although parent canonical URLs,
logical paths, choice slices, and indexed caret paths have improved within many
of these resources. Remaining first differences are dominated by:

- constraints attached to the root differential element;
- extension context and complex slicing/reslicing;
- fixed versus pattern values and CodeableConcept wrapping;
- differential element ordering, merging, ids, and slice names;
- bindings and type/profile/targetProfile arrays.

The comparison currently reports only the first difference per resource. A
differential-aware diagnostic should report all bad element ids in one run.

### IG, terminology, metadata, and other: 85 resources

Every ImplementationGuide still differs, normally first at `definition.page`
or `definition.grouping`. Publisher/contact parity is implemented, but page
trees, grouping, globals, parameters, and resource metadata are not.

Terminology gaps are now small and mainly involve concept properties and local
canonical resolution. The broad `other` group should be split as fixes proceed;
known examples include nested indentation context in instance rules and
resource-specific fields that the current classifier does not recognize.

## Generated Resource and Schema Impact

The schema-informed exporter exposed an error in committed generated metadata:
nested StructureDefinition elements could overwrite their BackboneElement
parent's metadata. The code generator now emits direct-child maps and dedicated
BackboneElement metadata types. Both committed R4 and R5 crates were regenerated
and tested. This is a large generated diff, but it is required for reproducible
array/datatype decisions in rh-fsh and keeps `regen-check` from reporting drift.

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
