# Wave 1 Scope Notes

## Baseline (2026-03-09)

Source: `crates/rh-cql/CONFORMANCE.md` full 15-suite run dated 2026-03-09.

- Wrong-answer failures (`Fail`): `0`
- Compile errors: `149`
- Eval errors: `628`

These are the comparison baselines for wave-1 conformance validation.

## Wave-1 Closure Set

Selected from documented gaps in `SPEC_COVERAGE.md` and `CONFORMANCE.md`:

- Emitter parity:
  - system function native ELM emission regression coverage for `Abs`, `Ceiling`, `Floor`, `Truncate`, `Round`, `Ln`, `Exp`, `Log`, `Power`, `Predecessor`, `Successor`
  - canonical negative numeric literal emission (`Negate(Literal(...))`)
- Semantic registration (function-syntax gap closure):
  - `Substring`, `PositionOf`, `LastPositionOf`, `SplitOnMatches`
  - list slice family: `Tail`, `Skip`, `Take`, `Slice`
  - unary extraction signatures: `DateFrom`, `TimeFrom`, `TimezoneOffsetFrom`
- Eval dispatch/implementation:
  - `Substring`, `PositionOf`, `LastPositionOf`, `SplitOnMatches`, `ReplaceMatches`, string `IndexOf`
  - `DateFrom`, `TimeFrom`, `TimezoneOffsetFrom`
  - `Tail`, `Skip`, `Take`, `Slice`

## Stage Ownership Mapping

| Item | Semantic | Emit | Eval |
|---|---|---|---|
| `Substring` | register signatures | emit dedicated `Substring` | dispatch + evaluate |
| `PositionOf` / `LastPositionOf` | register signatures | emit dedicated nodes | dispatch + evaluate |
| `SplitOnMatches` | register signatures | emit dedicated node | dispatch + evaluate |
| `ReplaceMatches` | already typed | already emitted | dispatch + evaluate |
| string `IndexOf` | keep resolver coverage | function/native mapping | builtin dispatch |
| `DateFrom` / `TimeFrom` / `TimezoneOffsetFrom` | register unary signatures | already emitted as unary nodes | dispatch + evaluate |
| `Tail` / `Skip` / `Take` / `Slice` | register signatures | map to `Slice` where applicable | dispatch + evaluate |

## Deferred (Explicitly Out of Wave-1)

- Clinical context operators (`AgeIn*`, `Children`, `Descendents`)
- `TimeOfDay`, `Precision`, `LowBoundary`, `HighBoundary`
- Broader query/multi-source and retrieve expansions
- Full CQL parity outside selected high-impact wiring gaps
