## Context

The exporter already compiles FHIR and resolved-profile metadata into a
`CompiledSchema` and writes instances through `TypedInstanceTree`. It has
closed resource identity gaps, but 179 shared resources still first differ from
SUSHI in JSON shape. The recurring failures occur when recursive paths cross
an extension, an embedded resource, a Bundle entry, or a Parameters part, or
when a profile-derived default and an explicit assignment interact.

The implementation must preserve the current deterministic resource order,
resource identities, and library callers that supply `FshConfig` directly.
SUSHI golden fixtures and the project comparison runner are the reference
oracles; the runner's first-difference category is a prioritization signal, not
a complete field-level diagnosis.

## Goals / Non-Goals

**Goals:**

- Use one recursive path-shaping mechanism for top-level, contained, inline,
  Bundle, and Parameters assignments.
- Select array/scalar, primitive-shadow, choice, and datatype JSON shapes from
  FHIR and resolved-profile metadata at every path segment.
- Apply defaults deterministically without allowing them to overwrite explicit
  FSH assignments or materialize unused optional extension placeholders.
- Add focused SUSHI goldens and diagnostics that isolate each corrected shape.

**Non-Goals:**

- Achieving complete StructureDefinition, metadata, or IG-generation parity in
  this change.
- Adding FSH syntax or changing the public CLI JSON envelope.
- Materializing every dependency-derived optional default merely because its
  package metadata is available.

## Decisions

### Extend `TypedInstanceTree` as the sole recursive writer

Every assignment, including assignments made while embedding an inline or
contained instance, will pass through `TypedInstanceTree` with a `SchemaView`
for the concrete resource or datatype at that location. Direct JSON mutation
paths will be removed or reduced to final serialization only.

This retains the existing deterministic tree representation and gives every
embedding context the same cardinality and datatype rules. Keeping separate
writers for Bundle and Parameters would be smaller initially, but would repeat
the extension and primitive-shadow bugs at each new recursive boundary.

### Carry resolved shape information through each path traversal

Path traversal will resolve a shape containing cardinality, primitive status,
choice type, effective datatype, and applicable local/dependency profile
metadata. The shape will be derived from the compiled core FHIR metadata first,
then refined by the resolved profile element. The resulting node determines
whether to append, index, replace, wrap as a `CodeableConcept`, emit a primitive
companion, or recursively enter an embedded resource.

Inferring shape from the FSH value or path spelling alone is rejected because
the same syntax has different output shapes for, for example, a `Coding`, a
`CodeableConcept`, and a repeating BackboneElement.

### Define one default and assignment precedence order

Values will be applied in this order: core FHIR structural defaults, applicable
dependency profile defaults, local profile fixed/default values, then explicit
instance assignments. A later, more specific value replaces an earlier value
at the same effective path; indexed and sliced repetitions retain their stable
position. Required extension children may be materialized when their parent is
used, but unused optional placeholders remain internal.

This makes the existing profile-aware behavior predictable while preventing a
dependency default from replacing local source or explicit instance content.

### Treat extension and primitive companions as node-level shapes

Extension paths will create the FHIR `extension[]` and `url` structure from
their resolved parent-scoped slice metadata. Primitive extensions will create
the matching `_field` scalar or array companion on the same node, preserving
the corresponding value array indexes. Nested extension serialization will use
the same tree and precedence rules as ordinary paths.

Serializing extensions as an after-the-fact JSON transform is rejected because
it loses the parent profile and array index context needed for nested slices.

### Make regressions diagnostic and corpus-backed

Each fixed shape will receive a minimal FSH fixture with reviewed SUSHI output.
The comparison harness will retain and summarize representative path-level
shape differences so a project regression can be mapped to a focused fixture.
Focused mCODE and CARIN runs will precede full runs, since they contain most of
the current JSON-shape first differences.

## Risks / Trade-offs

- [Profile metadata is incomplete for a path] → Fall back to core FHIR metadata
  without inventing a dependency default; add the observed shape to a fixture.
- [Changing shared traversal alters previously correct output] → Preserve and
  expand golden coverage, run the full compatibility suite, and lower no
  threshold until two full runs are identical.
- [Default precedence hides explicit source] → Record explicit assigned paths
  before defaults are applied and test local, dependency, and explicit conflicts.
- [First-difference counts move rather than fall] → Use retained pairwise
  artifacts and path-level summaries to distinguish newly exposed failures from
  regressions.

## Migration Plan

1. Add failing, reviewed SUSHI fixtures for extension, primitive-shadow, Bundle,
   and Parameters paths plus precedence conflicts.
2. Refactor the recursive writer behind the existing exporter API; no persisted
   data or public CLI migration is required.
3. Run focused project comparisons, then the complete corpus, and update the
   documented category counts only from the full result.
4. Lower a threshold only after two identical full comparison runs.

Rollback is a source revert of the exporter and fixtures; no runtime state or
external schema migration is involved.

## Open Questions

- Which remaining mCODE JSON-shape paths share the same missing schema metadata
  rather than requiring a value-conversion correction?
- Which dependency-required extension defaults does SUSHI materialize without an
  explicit local assignment, and how should that be represented in the compiled
  schema?
