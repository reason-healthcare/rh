# Returning Reference Types in Generated ELM

Date: 2026-06-16

## Summary

`rh-cql` already has the option and part of the machinery for ELM result type output, but it currently emits only `resultTypeName` from a flattened `DataType -> QName` conversion. That is enough for simple named types such as `Integer`, `Boolean`, `FHIR.Patient`, or `FHIR.Reference`, but it loses important structure for list, interval, tuple, and choice/reference-shaped results.

The reference CQL-to-ELM implementation treats result types as optional output metadata. It builds ELM with type information first, then removes `resultTypeName` and `resultTypeSpecifier` unless `EnableResultTypes` is present. Its rule is:

- named result type: emit `resultTypeName`
- non-named result type: emit `resultTypeSpecifier`
- option disabled: remove both fields

The recommended implementation for `rh-cql` is to keep the current option surface, replace the flat `datatype_to_qname` use with a `DataType -> ResultTypeMetadata` conversion, and populate both expression-level and definition-level result type fields consistently.

## Reference Behavior

The CQL Translation Service exposes the option as `result-types=true`; the documented default is `false`. The service maps that option to the Java translator's `EnableResultTypes`.

In the Java reference implementation:

- CQL is parsed and emitted with complete type information.
- The compiler then maps `EnableResultTypes` to an edit named `REMOVE_RESULT_TYPE`.
- That edit is applied only when `EnableResultTypes` is absent.
- `REMOVE_RESULT_TYPE` clears both `resultTypeName` and `resultTypeSpecifier`.
- During decoration, if the ELM element has a named result type, it sets `resultTypeName`; otherwise, it sets `resultTypeSpecifier`.

Useful upstream references:

- [CQL Translation Service README, translator options](https://github.com/cqframework/cql-translation-service#cql-to-elm-translator-options)
- [HL7 CQL Appendix C, reference implementation](https://cql.hl7.org/10-c-referenceimplementations.html)
- [Reference implementation repository](https://github.com/cqframework/clinical_quality_language)

## Current `rh-cql` State

The public option already exists:

- `CompilerOption::EnableResultTypes` is defined in `crates/rh-cql/src/options.rs`.
- `CompilerOptions::debug()` enables annotations, locators, and result types.
- `CompilerOptions::result_types_enabled()` is available.
- The CLI `rh cql compile --debug` uses `CompilerOptions::debug()`, so result types are currently tied to debug output.

The ELM model also already supports the relevant fields:

- `ElementFields` has `result_type_name` and `result_type_specifier`.
- `ExpressionDef` and `FunctionDef` have `result_type_name` and `result_type_specifier`.
- `TypeSpecifier` supports named, list, interval, tuple, choice, and parameter forms.

The main gap is in emission. `ElmEmitter::element_fields()` currently does this:

```rust
if self._options.result_types_enabled() {
    fields.result_type_name = Some(datatype_to_qname(&node.data_type));
}
```

That means all result types are forced through a QName, including types that should be represented structurally. For example, `List<FHIR.Observation>` should be a `ListTypeSpecifier` with an `elementType` named `FHIR.Observation`, not a synthetic system QName such as `List<Observation>`.

Another gap is definition-level metadata: `ExpressionDef` and `FunctionDef` currently set result type fields to `None` even though the typed statement body has a resolved type.

## Why Reference Types Are Affected

FHIR `Reference` itself can be represented as a named model type, but the useful cases around references often become structural:

- retrieves return `List<FHIR.<Resource>>`
- query expressions often return lists of resources or tuples containing resources/references
- member access can produce `FHIR.Reference`, `List<FHIR.Reference>`, or choice-like model elements
- function parameters and returns may use list/interval/tuple/choice wrappers

Flattening these to `resultTypeName` loses the shape needed by downstream tools that inspect ELM statically. The reference implementation avoids this by using `resultTypeSpecifier` for any non-named type.

## Proposed Design

Add a small conversion layer in the emitter:

```rust
struct ResultTypeMetadata {
    result_type_name: Option<elm::QName>,
    result_type_specifier: Option<elm::TypeSpecifier>,
}
```

Then implement:

```rust
fn datatype_to_result_type(dt: &DataType) -> ResultTypeMetadata
fn datatype_to_type_specifier(dt: &DataType) -> elm::TypeSpecifier
fn datatype_to_named_qname(dt: &DataType) -> Option<elm::QName>
```

Recommended mapping:

| `DataType` | ELM result field |
| --- | --- |
| `DataType::System(_)` | `resultTypeName` |
| `DataType::Model { namespace, name }` | `resultTypeName` |
| `DataType::List(elem)` | `resultTypeSpecifier: ListTypeSpecifier { elementType: ... }` |
| `DataType::Interval(point)` | `resultTypeSpecifier: IntervalTypeSpecifier { pointType: ... }` |
| `DataType::Tuple(elements)` | `resultTypeSpecifier: TupleTypeSpecifier { element: ... }` |
| `DataType::Choice(types)` | `resultTypeSpecifier: ChoiceTypeSpecifier { choice: ... }` |
| `DataType::TypeParameter(name)` | `resultTypeSpecifier: ParameterTypeSpecifier { parameterName: ... }` |
| `DataType::Unknown` | omit both, or emit `Any` only if compatibility tests show reference parity |

`datatype_to_qname()` can remain for `valueType`, `asType`, `isType`, and other places that genuinely need a QName, but result type metadata should stop using it blindly.

## Emission Changes

1. Update `ElmEmitter::element_fields()`

Replace the direct `result_type_name = Some(datatype_to_qname(...))` assignment with:

```rust
if self._options.result_types_enabled() {
    let result_type = datatype_to_result_type(&node.data_type);
    fields.result_type_name = result_type.result_type_name;
    fields.result_type_specifier = result_type.result_type_specifier;
}
```

2. Populate `ExpressionDef`

When emitting `TypedStatement::ExpressionDef`, derive result type metadata from `body.data_type` when `EnableResultTypes` is set. Assign the fields on `elm::ExpressionDef`.

3. Populate `FunctionDef`

For functions with a body, derive metadata from `body.data_type`. If a function has an explicit return annotation and no body, use the declared return type as a fallback once there is a conversion path from parser `TypeSpecifier` to result metadata.

4. Populate function operands

Function parameters already carry parser-level type specifiers. `OperandDef` should emit `operandTypeName` for named parameter types and `operandTypeSpecifier` for structural parameter types. This is adjacent to result types, but downstream ELM inspection usually expects both return and operand typing.

5. Keep the explicit CLI flag documented

`--debug` currently enables result types. For reference parity, `rh cql compile --result-types` is also available as an independent flag, while `--debug` behavior remains unchanged.

## Test Plan

Add focused tests before touching broad golden files:

- `CompilerOptions::new()` emits no result type fields.
- `EnableResultTypes` emits `resultTypeName` for `define X: 1`.
- `EnableResultTypes` emits `resultTypeSpecifier` for `define X: { 1, 2 }`.
- FHIR retrieve emits a list type specifier, e.g. `[Observation]` -> `ListTypeSpecifier` with `NamedTypeSpecifier` element for `FHIR.Observation`.
- Property/member access returning `FHIR.Reference` emits `resultTypeName` for the named `FHIR.Reference` result.
- Tuple expression emits `TupleTypeSpecifier` with per-element type specifiers.
- Function return and operand definitions emit the same named-vs-structural split.
- `rh cql compile --result-types` works independently of `--debug`.

Then add a Java reference comparison fixture that compiles with `EnableResultTypes` and checks metadata shape rather than exact local IDs or locator placement.

## Risks and Open Questions

- Namespace formatting must match the existing ELM JSON shape. Current code uses expanded QName strings like `{urn:hl7-org:elm-types:r1}Integer`; reference XML examples show prefixes such as `t:Boolean`, but JSON commonly uses expanded names. Keep the current JSON convention unless comparison fixtures prove otherwise.
- `DataType::Unknown` should probably omit result type metadata. Emitting `Any` can hide type resolution gaps and may diverge from the reference implementation.
- Existing tests document known differences around signatures. Result type work should not be mixed with signature emission unless a reference fixture requires it.
- The semantic analyzer must be trusted as the source of truth. The emitter should only serialize `TypedNode.data_type`, not re-resolve types.
- Golden files compiled with default options may not change because default options do not enable result types. Debug/reference comparison snapshots will change.

## Suggested Implementation Order

1. Add `datatype_to_type_specifier()` and `datatype_to_result_type()` in `emit/mod.rs` or a new `emit/result_types.rs`.
2. Unit-test the conversion layer directly for system, model, list, interval, tuple, choice, and type parameter cases.
3. Wire `ElmEmitter::element_fields()` to use the conversion layer.
4. Wire `ExpressionDef`, `FunctionDef`, and `OperandDef`.
5. Keep the CLI `--result-types` documentation and tests in sync with emitter behavior.
6. Add one or two Java reference fixtures for structural result type parity.

## Recommendation

Implement this as a narrow emitter enhancement. The compiler already has the option, typed AST, and ELM structs needed to support reference-style result metadata. The highest-value change is not adding more semantic analysis; it is serializing the resolved `DataType` using the same named-vs-structural split as the reference translator.
