# rh-fhirpath specification coverage

This page summarizes the FHIRPath 3.0.0 STU 3 implementation. The native
suite gate and its case-level exceptions are recorded in
[CONFORMANCE.md](CONFORMANCE.md).

| Area | Implemented coverage |
| --- | --- |
| Grammar | Operator precedence, whitespace and comments, collection literals, qualified identifiers, UTF-16 escape pairs, signed Integer and Long bounds, constructor syntax, and explicit sort directions |
| Scope and collections | Starting context and scoped iteration state, three-valued equality, membership and set operations, singleton checks, lazy iif and coalesce, lexical defineVariable, repeat and repeatAll, and multi-key sort |
| Numeric types | Typed Long promotion, checked Integer and Decimal operations, conversion and math functions, sum, min, max, avg, and the aggregate accumulator |
| Date and time | Partial precision, timezone comparisons, date/time arithmetic, component extraction, formatted conversion, duration and difference, and precision and boundary functions |
| Quantity | Arithmetic, conversion, comparability, and ordering for supported calendar and UCUM units. The supported UCUM subset is in [units.md](units.md). |
| String operations | Unicode scalar indexing, full and partial regex matching, flag handling, lexical trimming, and base64, hex, ASCII, JSON, and HTML transformations |
| FHIR values | R4 and R5 choice metadata and inheritance, primitive values and extensions, logical children, authored object construction, per-item type reflection, and paths to source nodes |
| Host functions | Local contained and Bundle resolution, extension queries, primitive value extraction, modifier and XHTML checks, and host interfaces for profile validation, slice/element metadata, external references, and terminology |

## Public entry points

- Rust `EvaluationContext` selects `FhirModel::R4` or `FhirModel::R5`.
  `evaluate_nodes` preserves source nodes for subsequent evaluation with a
  typed focus.
- `rh fhirpath eval --fhir-version r5` selects the R5 model. `--context`
  selects the starting focus, and the strict and ordered-function options
  control runtime checks. [CLI usage](../../apps/rh-cli/docs/FHIRPATH.md)
- WASM `EvaluateOptions.fhir_version` selects the model. The Node.js test
  exercises typed R5 values and exact JSON Decimal output.
- The validator installs a profile and terminology host for R4 invariant
  evaluation. Library callers can use an implementation of `FhirHost` for
  services specific to their FHIR model or deployment. [Host interface](src/extensions/fhir/host.rs)

The public AST includes `Term::InstanceSelector` and `Expression::SortKey`.
Source-preserving evaluation can return `FhirPathValue::Node`. Exhaustive
Rust matches should handle these variants. Quantity JSON results use an
object with `value` and an optional `unit` in Rust, the CLI, and WASM.

The published grammar and function definitions govern the language behavior.
The R4 and R5 model-specific behavior follows the published release pages.
[Grammar](https://hl7.org/fhirpath/STU3/en/grammar.html) ·
[Language](https://hl7.org/fhirpath/STU3/en/index.html) ·
[FHIR R4](https://hl7.org/fhir/R4/fhirpath.html) · [FHIR R5](https://hl7.org/fhir/R5/fhirpath.html)
