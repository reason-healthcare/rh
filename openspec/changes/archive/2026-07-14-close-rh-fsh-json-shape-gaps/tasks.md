## 1. Diagnose and Isolate JSON-shape Families

- [x] 1.1 Review retained mCODE and CARIN comparison artifacts and group the leading JSON-shape paths by recursive field family.
- [x] 1.2 Add harness coverage that preserves a representative resource and first-difference path for each JSON-shape family.
- [x] 1.3 Add focused FSH fixtures for unresolved recursive extension, primitive-shadow, Bundle-entry, Parameters-part, and repeating BackboneElement shapes.
- [x] 1.4 Generate and review SUSHI goldens for the new fixtures before changing exporter behavior.

## 2. Unify Recursive Instance Shape Resolution

- [x] 2.1 Identify and route any remaining direct JSON mutation paths for contained, inline, Bundle, and Parameters assignments through `TypedInstanceTree`.
- [x] 2.2 Extend recursive path traversal to retain core FHIR cardinality, primitive, choice, and effective datatype metadata at each embedded path segment.
- [x] 2.3 Refine each resolved path shape with local and dependency profile element metadata without changing resource identity or order.
- [x] 2.4 Apply the resolved shape consistently when appending, indexing, or replacing repeating fields and BackboneElement values.
- [x] 2.5 Add unit tests that prove an embedded resource emits the same array/scalar and datatype shape as an equivalent top-level resource.

## 3. Apply Defaults and Extension Shapes Safely

- [x] 3.1 Encode and test the default precedence order: core structural defaults, dependency profile defaults, local profile values, then explicit Instance assignments.
- [x] 3.2 Preserve stable indexes and slice order when a more-specific value overrides an inherited default.
- [x] 3.3 Materialize required extension children only when their parent is used, while keeping unused optional extension placeholders out of output.
- [x] 3.4 Align nested extension URLs, `extension[]` wrappers, and primitive `_field` companions across top-level and recursively embedded resources.

## 4. Verify Focused SUSHI Parity

- [x] 4.1 Run `cargo test -p rh-fsh --lib` and the SUSHI compatibility suite with ignored golden tests enabled.
- [x] 4.2 Run focused mCODE and CARIN project comparisons, inspect retained artifacts, and add a minimal regression fixture for each exporter behavior corrected.
- [x] 4.3 Confirm the comparison results retain actionable representative JSON paths for remaining shape mismatches.

## 5. Measure and Record the Conformance Wave

- [x] 5.1 Run the full project comparison for all configured projects and update the conformance reports with the resulting category counts.
- [x] 5.2 Repeat the full comparison and confirm counts and category summaries are deterministic.
- [x] 5.3 Tighten only improved project thresholds after the two full runs match, and document the evidence in the improvement plan.
- [x] 5.4 Run `cargo fmt --all -- --check`, `cargo clippy -p rh-fsh --all-targets --all-features -- -D warnings`, and targeted `rh-cli` FSH integration tests.
