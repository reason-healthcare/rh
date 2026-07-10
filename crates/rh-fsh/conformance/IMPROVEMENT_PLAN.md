# rh-fsh Conformance Improvement Plan

This plan prioritizes high-leverage exporter corrections before long-tail
metadata work. Each phase should lower the stored project thresholds; thresholds
must never be raised merely to make a run pass.

**Progress (2026-07-10)**: Phase 0 and Phase 1 are complete. All 61 fixtures are
verified and passing, all six projects have zero missing and extra resources,
and the library suite has 103 passing tests. Shared-resource mismatches are down
from 838 to 575. Phases 2–4 remain in progress; Phase 5 continues with lowered
per-project thresholds.

## Success Criteria

The end state is:

- Existing library tests remain green, with regression tests added for every
  corrected behavior.
- All 61 fixture directories have reviewed SUSHI goldens and all fixtures pass.
- All six project comparisons have zero missing and zero extra resources.
- Shared-resource mismatches fall from 838 to zero, or any intentionally accepted
  normalization is explicitly documented and tested.
- A clean checkout can build the CLI and run the complete comparison suite with
  pinned tool and project versions.

## Phase 0: Make the Evidence Reproducible

**Status**: Complete.

**Goal**: turn the current measurements into a reliable development gate.

1. Resolve the `rh-cli` build failure caused by duplicate packaged/workspace
   `serde_json` and `rust_decimal` types.
2. Pin SUSHI 3.19.0 in the harness instead of relying on an unversioned `npx`
   resolution.
3. Record or configure the six project revisions in a checked-in manifest.
4. Generate and review goldens for the 31 skipped fixtures.
5. Split fixture reporting into passed, failed, and unverified counts in CI.
6. Add unit tests for duplicate resource ids in identity-gap categorization and
   for count/detail truncation behavior.

**Exit gate**: a clean environment reproduces the documented counts, and no
fixture is silently treated as passing without goldens.

## Phase 1: Eliminate Missing and Extra Resources

**Status**: Complete. Missing resources are 0 and extra resources are 0 across
the pinned six-project corpus.

**Goal**: correct resource identity and instance lifecycle before field parity.

1. Resolve every instance's `InstanceOf` chain through local and dependency
   StructureDefinitions to its base FHIR resource type.
2. Preserve the profile canonical in `meta.profile` while emitting the base type
   in `resourceType`.
3. Model instance usage explicitly in the compiler output.
4. Exclude `Usage: #inline` instances from top-level package resources.
5. Materialize inline instances only at assignment/reference sites, including
   contained resources and Bundle entries.
6. Add focused fixtures for local profiles, dependency profiles, datatype inline
   instances, contained resources, and repeated ids such as CRD's `example`.

**Expected impact**: missing 43 → 0 and extra 149 → 0. This also removes 86
identity-category gaps and 106 other-category gaps.

## Phase 2: Introduce Schema-informed Instance Export

**Status**: In progress. BackboneElement metadata, cardinality-aware arrays,
CodeableConcept wrapping, dynamic choice typing, primitive shadows, recursive
inline export, indentation contexts, dependency fixed/pattern defaults, and
named/repeated slice materialization are implemented. JSON-shape leading gaps
are down from 434 to 231; the below-100 milestone remains open.

**Goal**: stop inferring JSON shape from assignment syntax alone.

1. Build a path-shape service from loaded FHIR StructureDefinitions that returns
   cardinality, datatype, choice type, and primitive-extension information.
2. Use cardinality to decide object versus array representation, including a
   one-element assignment to a repeating field.
3. Export coded values according to their target datatype: `Coding`,
   `CodeableConcept.coding`, primitive `code`, and quantity code/system/unit.
4. Make extension construction always preserve array wrapping, `url`, nested
   `extension`, and the correct `value[x]` member.
5. Emit aligned primitive shadow fields (`_field`) for primitive extensions in
   arrays and scalar values.
6. Apply the same shape engine recursively inside Bundle resources, Parameters,
   contained resources, and inline instances.

**Milestone target**: golden fixtures 61/61 passing and project JSON-shape
first-difference count 481 → below 100.

## Phase 3: StructureDefinition Differential Parity

**Status**: In progress. Logical paths, choice slices, indexed caret paths, local
parent resolution, and dependency/local parent canonical resolution are
implemented; detailed differential parity remains.

**Goal**: make profiles, extensions, logicals, and resources structurally match
SUSHI.

1. Normalize `baseDefinition` resolution for local and dependency parents.
2. Implement FHIR-correct extension `context` representation.
3. Reconcile differential element ids, paths, slice names, ordering, and merging.
4. Complete constraint, binding, fixed/pattern, min/max, and type/profile export.
5. Verify contains/slicing behavior across reslicing and named/profile slices.
6. Add differential-specific comparisons that report every bad element rather
   than only the first resource difference.

**Milestone target**: StructureDefinition first-difference count 234 → below 25.

## Phase 4: Terminology, Metadata, and IG Parity

**Status**: In progress. Hierarchical concepts, multiline ValueSet filters,
SUSHI `fhirVersion` omission, and resource-aware instance titles are complete.
IG publisher/contact metadata is implemented. Page/grouping/resource metadata
and remaining terminology aliases are pending.

**Goal**: close systematic non-instance differences.

1. Match CodeSystem nested concepts, counts, hierarchy, and SUSHI's metadata
   omission/default rules, including `fhirVersion`.
2. Match ValueSet compose include/exclude/filter/concept/valueSet shapes.
3. Expand `sushi-config.yaml` mapping for IG contact, page, global, parameter,
   dependency, and resource metadata.
4. Reconcile entity titles, descriptions, names, urls, status, and other derived
   metadata with SUSHI defaults.
5. Decide which volatile fields should remain normalized and document each
   intentional exception.

**Milestone target**: terminology 22 → 0, metadata 29 → 0, and IG generation
6 → 0.

## Phase 5: Burn Down Residual Project-specific Gaps

**Status**: In progress. Resource output and ValueSet grouping are deterministic,
thresholds have been lowered to the current stable counts, and two consecutive
full runs produced identical results.

**Goal**: reach parity without hiding differences behind broad normalization.

1. Re-run all projects after every focused exporter wave.
2. Replace the broad `other` category with specific classifiers as patterns are
   understood.
3. Add a minimal fixture for each newly diagnosed project-only failure before
   changing exporter behavior.
4. Reduce thresholds to the newly achieved counts in the same change as each
   improvement.
5. Require two consecutive clean full runs before declaring project parity.

**Current checkpoint**: 920/920 resource identities match; 345/920 match
normalized content. Remaining mismatches: CARIN 80, mCODE 172, CRD 62, DTR 52,
PAS 128, and IPS 81.

**Next delivery order**:

1. Complete recursive schema shaping for extension arrays, Bundle entries,
   Parameters, and primitive shadow fields (434 leading JSON-shape gaps).
2. Export root constraints, context, slicing, bindings, fixed/pattern values,
   and differential element merging (234 StructureDefinition gaps).
3. Preserve nested indentation context in instance rules and finish local
   CodeSystem/ValueSet canonical resolution.
4. Map IG pages, grouping, globals, parameters, and resource metadata, then
   split the remaining `other` classifier into actionable categories.

**Exit gate**: 920 of 920 reference resources match by identity and normalized
content for the pinned project corpus.

## Recommended Delivery Slices

The first implementation cycle should contain three small, reviewable changes:

1. Fix top-level suppression for `Usage: #inline` and add lifecycle fixtures.
2. Fix local-profile base resource type resolution and add CARIN/CRD/PAS-shaped
   regression fixtures.
3. Add the schema-informed array and `CodeableConcept.coding` behavior needed to
   make the two currently failing golden fixtures pass.

Together, these slices address every missing/extra resource and the clearest
fixture failures before undertaking the larger differential exporter work.
