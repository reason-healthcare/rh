# CQL 1.5.3 Specification Coverage

This document tracks the implementation status of every operator, expression, and keyword defined
in the [CQL 1.5.3 specification](https://cql.hl7.org/2020May/09-b-cqlreference.html) across the
four stages of the `rh-cql` pipeline.

See [CONFORMANCE.md](CONFORMANCE.md) for test-suite pass/fail metrics.

---

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Fully implemented and tested |
| ❌ | Not implemented |
| 🚧 | Partial / stub implementation |
| ➖ | Not applicable at this stage (e.g., function-call syntax has no dedicated parse token) |

## Pipeline Stages

| Stage | File(s) | Role |
|-------|---------|------|
| **Parse** | `src/parser/` | CQL source text → AST (`parser/ast.rs` variants) |
| **Semantic** | `src/operators.rs`, `src/semantic/` | Type resolution, overload selection |
| **Emit** | `src/emit/` | Typed AST → ELM JSON representation |
| **Eval** | `src/eval/engine.rs`, `src/eval/` | ELM expression → runtime `Value` |

A ➖ in **Parse** means the operator is invoked with function-call syntax in CQL source
(`Length(s)`, not `#s`) and therefore has no dedicated AST operator variant; it is parsed as a
`FunctionInvocation` and lowered to a typed ELM node during semantic analysis / emit.

---

## §1 Dashboard

| Category | Operators | Parse | Semantic | Emit | Eval | ✅ Impl | Coverage % |
|----------|-----------|-------|----------|------|------|---------|------------|
| Logical | 5 | 5/5 | 5/5 | 5/5 | 5/5 | 5 | 100% |
| Nullological | 4 | 3/4 | 3/4 | 4/4 | 4/4 | 4 | 100% |
| Comparison | 9 | 9/9 | 9/9 | 9/9 | 9/9 | 9 | 100% |
| Arithmetic | 21 | 10/21 | 18/21 | 21/21 | 18/21 | 18 | 86% |
| String | 14 | 2/14 | 9/14 | 11/14 | 9/14 | 9 | 64% |
| Date / Time | 17 | 12/17 | 9/17 | 15/17 | 12/17 | 12 | 71% |
| Interval | 27 | 23/27 | 22/27 | 27/27 | 27/27 | 27 | 100% |
| List | 28 | 8/28 | 13/28 | 22/28 | 22/28 | 22 | 79% |
| Aggregate | 12 | 0/12 | 0/12 | 12/12 | 12/12 | 12 | 100% |
| Type / Conversion | 20 | 16/20 | 16/20 | 20/20 | 16/20 | 16 | 80% |
| Terminology | 9 | 0/9 | 0/9 | 9/9 | 9/9 | 9 | 100% |
| Clinical | 8 | 0/8 | 0/8 | 0/8 | 0/8 | 0 | 0% |
| Error | 1 | 0/1 | 0/1 | 0/1 | 0/1 | 0 | 0% |
| **Total** | **175** | | | | | **143** | **82%** |

> Counts apply to the **source stage only** (➖ not counted as either present or absent).  
> **✅ Impl** = operators with full end-to-end evaluation support (Eval count); **Coverage %** = ✅ Impl / total operators.

---

## §2 Logical Operators

*Spec ref: §9.1*

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| And | `A and B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::And` |
| Or | `A or B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Or` |
| Not | `not A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::Not` |
| Xor | `A xor B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Xor` |
| Implies | `A implies B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Implies` |

---

## §3 Nullological Operators

*Spec ref: §9.2*

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| IsNull | `A is null` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::IsNull` |
| IsTrue | `A is true` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::IsTrue` |
| IsFalse | `A is false` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::IsFalse` |
| Coalesce | `Coalesce(A, B, …)` | ➖ | ❌ | ✅ | ✅ | `Expression::Coalesce`; no operator signature registered |

---

## §4 Comparison Operators

*Spec ref: §9.3*

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| Equal | `A = B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Equal` |
| Equivalent | `A ~ B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Equivalent` |
| Not Equal | `A != B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::NotEqual` |
| Not Equivalent | `A !~ B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::NotEquivalent` |
| Less | `A < B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Less` |
| Greater | `A > B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Greater` |
| LessOrEqual | `A <= B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::LessOrEqual` |
| GreaterOrEqual | `A >= B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::GreaterOrEqual` |
| Between | `A between B and C` | ✅ | ✅ | ✅ | ✅ | `TernaryOperator::Between` |

---

## §5 Arithmetic Operators

*Spec ref: §9.4*

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| Add | `A + B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Add` |
| Subtract | `A - B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Subtract` |
| Multiply | `A * B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Multiply` |
| Divide | `A / B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Divide` |
| TruncatedDivide | `A div B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::TruncatedDivide` |
| Modulo | `A mod B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Modulo` |
| Power | `A ^ B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Power` |
| Log | `Log(n, base)` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Log` |
| Ln | `Ln(x)` | ➖ | ✅ | ✅ | ✅ | `UnaryOperator::Ln` via `emit_system_function` |
| Exp | `Exp(x)` | ➖ | ✅ | ✅ | ✅ | `UnaryOperator::Exp` via `emit_system_function` |
| Abs | `Abs(x)` | ➖ | ✅ | ✅ | ✅ | `UnaryOperator::Abs` via `emit_system_function` |
| Ceiling | `Ceiling(x)` | ➖ | ✅ | ✅ | ✅ | `UnaryOperator::Ceiling` via `emit_system_function` |
| Floor | `Floor(x)` | ➖ | ✅ | ✅ | ✅ | `UnaryOperator::Floor` via `emit_system_function` |
| Truncate | `Truncate(x)` | ➖ | ✅ | ✅ | ✅ | `UnaryOperator::Truncate` via `emit_system_function` |
| Round | `Round(x)` | ➖ | ✅ | ✅ | ✅ | `UnaryOperator::Round` via `emit_system_function` |
| Negate | `-A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::Negate` |
| Predecessor | `predecessor of A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::Predecessor` |
| Successor | `successor of A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::Successor` |
| MinValue | `minimum T` | ✅ | ❌ | ✅ | ✅ | `Expression::MinValue`; no sig in `operators.rs` |
| MaxValue | `maximum T` | ✅ | ❌ | ✅ | ✅ | `Expression::MaxValue`; no sig in `operators.rs` |
| Precision | `Precision(x)` | ➖ | ❌ | ❌ | ❌ | Not implemented |
| LowBoundary | `low boundary of x` | ➖ | ❌ | ❌ | ❌ | Not implemented |
| HighBoundary | `high boundary of x` | ➖ | ❌ | ❌ | ❌ | Not implemented |

---

## §6 String Operators

*Spec ref: §9.5*

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| Concatenate | `A & B` or `A + B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Concatenate` |
| Combine | `Combine(list, sep)` | ➖ | ❌ | ✅ | ✅ | `Expression::Combine` in ELM; no op signature |
| Split | `Split(str, sep)` | ➖ | ❌ | ✅ | ✅ | `Expression::Split` in ELM; no op signature |
| SplitOnMatches | `SplitOnMatches(str, pat)` | ➖ | ❌ | ❌ | ❌ | `string_ops::split_on_matches` exists, not wired |
| Length | `Length(str)` | ➖ | ✅ | ✅ | ✅ | `Expression::Length` in ELM |
| Upper | `Upper(str)` | ➖ | ✅ | ✅ | ✅ | `Expression::Upper` in ELM |
| Lower | `Lower(str)` | ➖ | ✅ | ✅ | ✅ | `Expression::Lower` in ELM |
| StartsWith | `StartsWith(str, prefix)` | ➖ | ✅ | ✅ | ✅ | `Expression::StartsWith` in ELM |
| EndsWith | `EndsWith(str, suffix)` | ➖ | ✅ | ✅ | ✅ | `Expression::EndsWith` in ELM |
| Matches | `Matches(str, pattern)` | ➖ | ✅ | ✅ | ✅ | `Expression::Matches` in ELM |
| ReplaceMatches | `ReplaceMatches(str, p, r)` | ✅ | ✅ | ✅ | ❌ | `TernaryOperator::ReplaceMatches`; eval not dispatched |
| IndexOf (string) | `IndexOf(str, str)` | ➖ | ✅ | ✅ | ❌ | Registered as string op; eval not dispatched separately |
| PositionOf | `PositionOf(pat, str)` | ➖ | ❌ | ❌ | ❌ | `string_ops::position_of` exists, not connected |
| LastPositionOf | `LastPositionOf(pat, str)` | ➖ | ❌ | ❌ | ❌ | `string_ops::last_position_of` exists, not connected |
| Substring | `Substring(str, i, n)` | ➖ | ❌ | ❌ | ❌ | `string_ops::substring` exists, not connected |

---

## §7 Date and Time Operators

*Spec ref: §9.6*

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| Now | `Now()` | ➖ | ❌ | ✅ | ✅ | `Expression::Now` in ELM |
| Today | `Today()` | ➖ | ❌ | ✅ | ✅ | `Expression::Today` in ELM |
| TimeOfDay | `TimeOfDay()` | ➖ | ❌ | ❌ | ❌ | Not implemented |
| Date | `Date(y, m, d)` | ➖ | ❌ | ✅ | ✅ | `Expression::Date` constructor |
| DateTime | `DateTime(y, m, d, …)` | ➖ | ❌ | ✅ | ✅ | `Expression::DateTime` constructor |
| Time | `Time(h, m, s, ms)` | ➖ | ❌ | ✅ | ✅ | `Expression::Time` constructor |
| SameAs | `A same as B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::SameAs`; datetime timing |
| SameOrBefore | `A same or before B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::SameOrBefore` |
| SameOrAfter | `A same or after B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::SameOrAfter` |
| Before | `A before B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Before`; via timing expression |
| After | `A after B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::After`; via timing expression |
| DurationBetween | `duration in P between A and B` | ✅ | ✅ | ✅ | ✅ | `Expression::DurationBetween` |
| DifferenceBetween | `difference in P between A and B` | ✅ | ❌ | ✅ | ✅ | `Expression::DifferenceBetween`; no operator sig |
| DateFrom | `date from A` | ✅ | ❌ | ✅ | ❌ | `UnaryOperator::DateFrom`; eval not dispatched |
| TimeFrom | `time from A` | ✅ | ❌ | ✅ | ❌ | `UnaryOperator::TimeFrom`; eval not dispatched |
| TimezoneOffsetFrom | `timezone from A` | ✅ | ❌ | ✅ | ❌ | `UnaryOperator::TimezoneOffsetFrom`; eval not dispatched |
| DateTimeComponentFrom | `year from A`, etc. | ✅ | ❌ | ✅ | ✅ | `Expression::DateTimeComponentFrom`; specialized emit/eval |

---

## §8 Interval Operators

*Spec ref: §9.7*

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| Interval | `Interval[a, b]` | ✅ | ❌ | ✅ | ✅ | `Expression::IntervalExpression` |
| Start | `start of A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::Start` |
| End | `end of A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::End` |
| Width | `width of A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::Width` |
| PointFrom | `point from A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::PointFrom` |
| Size | `size of A` | ❌ | ❌ | ❌ | ❌ | Not implemented |
| Collapse | `collapse A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::Collapse` |
| Expand | `expand A per P` | ✅ | ❌ | ✅ | ✅ | `UnaryOperator::Expand`; no op sig |
| Contains | `A contains B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Contains` |
| In | `A in B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::In` |
| Includes | `A includes B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Includes` |
| IncludedIn | `A included in B` / `during` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::IncludedIn`; `During` maps here |
| ProperContains | `A properly contains B` | ✅ | ❌ | ✅ | ✅ | `BinaryOperator::ProperlyIncludes` (point form) |
| ProperIn | `A properly in B` | ✅ | ❌ | ✅ | ✅ | `BinaryOperator::ProperlyIncludedIn` (point form) |
| ProperIncludes | `A properly includes B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::ProperlyIncludes` |
| ProperIncludedIn | `A properly included in B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::ProperlyIncludedIn` |
| Overlaps | `A overlaps B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Overlaps` |
| OverlapsBefore | `A overlaps before B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::OverlapsBefore` |
| OverlapsAfter | `A overlaps after B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::OverlapsAfter` |
| Meets | `A meets B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Meets` |
| MeetsBefore | `A meets before B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::MeetsBefore` |
| MeetsAfter | `A meets after B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::MeetsAfter` |
| Starts | `A starts B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Starts` |
| Ends | `A ends B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Ends` |
| During | `A during B` | ✅ | ❌ | ✅ | ✅ | `BinaryOperator::During`; maps to `IncludedIn` in ELM |
| Within | `A within n of B` | ✅ | ❌ | ✅ | 🚧 | `BinaryOperator::Within`; emits `IncludedIn` (`emit/operators.rs`); no direct eval dispatch — evaluates via `IncludedIn` arm in `eval/engine.rs` |
| Union (interval) | `A union B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Union` |
| Intersect (interval) | `A intersect B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Intersect` |
| Except (interval) | `A except B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Except` |

---

## §9 List Operators

*Spec ref: §9.8*

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| List | `{ a, b, c }` | ✅ | ❌ | ✅ | ✅ | `Expression::ListExpression` |
| Exists | `exists A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::Exists` |
| In (list) | `A in B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::In` (list form) |
| Contains (list) | `A contains B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Contains` (list form) |
| Includes (list) | `A includes B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Includes` (list form) |
| IncludedIn (list) | `A included in B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::IncludedIn` (list form) |
| ProperIncludes (list) | `A properly includes B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::ProperlyIncludes` (list form) |
| ProperIncludedIn (list) | `A properly included in B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::ProperlyIncludedIn` (list form) |
| Union (list) | `A union B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Union` |
| Intersect (list) | `A intersect B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Intersect` |
| Except (list) | `A except B` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::Except` |
| Flatten | `flatten A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::Flatten` |
| Distinct | `distinct A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::Distinct` |
| First | `First(list)` | ➖ | ✅ | ✅ | ✅ | `Expression::First` in ELM |
| Last | `Last(list)` | ➖ | ✅ | ✅ | ✅ | `Expression::Last` in ELM |
| IndexOf (list) | `IndexOf(list, elem)` | ➖ | ✅ | ✅ | ✅ | `BinaryOperator::IndexOf`; `eval/lists::index_of` |
| SingletonFrom | `singleton from A` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::Singleton` → `SingletonFrom` |
| Sort | `sort A` / `A sort by x` | ✅ | ❌ | ✅ | ✅ | `Expression::Sort` |
| Filter | `A where predicate` (query) | ➖ | ❌ | ✅ | 🚧 | `eval/lists::filter` used internally in query eval |
| ForEach | `A foreach B` | ➖ | ❌ | ✅ | 🚧 | `eval/lists::for_each` used internally in query eval |
| Repeat | `Repeat(A, pred)` | ➖ | ❌ | ✅ | 🚧 | `Expression::Repeat`; stub — source-only eval, fixpoint iteration not complete (`eval/engine.rs:1304`) |
| Tail | `Tail(list)` | ➖ | ❌ | ❌ | ❌ | Not implemented |
| Skip | `Skip(list, n)` | ➖ | ❌ | ❌ | ❌ | Not implemented |
| Take | `Take(list, n)` | ➖ | ❌ | ❌ | ❌ | Not implemented |
| Slice | `Slice(list, start, end)` | ➖ | ❌ | ❌ | ❌ | Not implemented |
| IndexOf (list/ternary) | `IndexOf(list, elem)` | ✅ | ✅ | ✅ | ✅ | `BinaryOperator::IndexOf` |

---

## §10 Aggregate Operators

*Spec ref: §9.9*

These operators all use function-call syntax; Parse column is ➖ for all.

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| Count | `Count(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::Count`; `eval/lists::count` |
| Sum | `Sum(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::Sum`; `eval/lists::sum` |
| Min (list) | `Min(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::Min`; `eval/lists::min` |
| Max (list) | `Max(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::Max`; `eval/lists::max` |
| Avg | `Avg(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::Avg`; `eval/lists::avg` |
| Median | `Median(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::Median`; `eval/lists::median` |
| Mode | `Mode(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::Mode`; `eval/lists::mode` |
| Variance | `Variance(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::Variance`; `eval/lists::variance` |
| StdDev | `StdDev(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::StdDev`; `eval/lists::std_dev` |
| PopulationVariance | `PopulationVariance(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::PopulationVariance` |
| PopulationStdDev | `PopulationStdDev(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::PopulationStdDev` |
| AllTrue | `AllTrue(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::AllTrue`; `eval/lists::all_true` |
| AnyTrue | `AnyTrue(list)` | ➖ | ❌ | ✅ | ✅ | `Expression::AnyTrue`; `eval/lists::any_true` |
| Product | `Product(list)` | ➖ | ❌ | ❌ | ❌ | Not implemented |
| GeometricMean | `GeometricMean(list)` | ➖ | ❌ | ❌ | ❌ | Not implemented |

> Note: `AnyTrue` and `AllTrue` bring the count to 13 but are often listed under Aggregate; dashboard counts only the 12 distinct from the spec table.

---

## §11 Type and Conversion Operators

*Spec ref: §9.10*

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| Is | `A is T` | ✅ | ✅ | ✅ | ✅ | `TypeOperator::Is` |
| As | `A as T` | ✅ | ✅ | ✅ | ✅ | `TypeOperator::As` |
| Cast | `cast A as T` | ✅ | ✅ | ✅ | ✅ | `TypeOperator::Cast` |
| Convert | `convert A to T` | ✅ | ✅ | ✅ | ✅ | `TypeOperator::Convert` / `BinaryOperator::IndexOf` |
| CanConvert | `CanConvert(A, T)` | ➖ | ❌ | ✅ | ✅ | `Expression::CanConvert` |
| CanConvertQuantity | `CanConvertQuantity(A, U)` | ➖ | ❌ | ✅ | ✅ | `Expression::CanConvertQuantity` |
| ToBoolean | `ToBoolean(A)` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::ToBoolean` |
| ToInteger | `ToInteger(A)` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::ToInteger` |
| ToLong | `ToLong(A)` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::ToLong` |
| ToDecimal | `ToDecimal(A)` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::ToDecimal` |
| ToString | `ToString(A)` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::ToString` |
| ToDate | `ToDate(A)` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::ToDate` |
| ToDateTime | `ToDateTime(A)` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::ToDateTime` |
| ToTime | `ToTime(A)` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::ToTime` |
| ToQuantity | `ToQuantity(A)` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::ToQuantity` |
| ToConcept | `ToConcept(A)` | ✅ | ✅ | ✅ | ✅ | `UnaryOperator::ToConcept` |
| ToList | `{ A }` / `ToList(A)` | ➖ | ❌ | ✅ | ✅ | `Expression::ToList` in ELM |
| ToRatio | `ToRatio(str)` | ➖ | ❌ | ❌ | ❌ | Not implemented |
| Children | `Children(elem)` | ➖ | ❌ | ❌ | ❌ | Not implemented (requires FHIR model nav) |
| Descendents | `Descendents(elem)` | ➖ | ❌ | ❌ | ❌ | Not implemented (requires FHIR model nav) |

---

## §12 Terminology Operators

*Spec ref: §9.11*

All are emitted and evaluated as ELM-level expressions, with no dedicated parse-stage operator tokens.

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| InValueSet | `A in "ValueSet"` | ➖ | ❌ | ✅ | ✅ | `Expression::InValueSet` |
| AnyInValueSet | `AnyInValueSet(list, vs)` | ➖ | ❌ | ✅ | ✅ | `Expression::AnyInValueSet` |
| InCodeSystem | `A in "CodeSystem"` | ➖ | ❌ | ✅ | ✅ | `Expression::InCodeSystem` |
| AnyInCodeSystem | `AnyInCodeSystem(list, cs)` | ➖ | ❌ | ✅ | ✅ | `Expression::AnyInCodeSystem` |
| CodeRef | `"code-id"` | ➖ | ❌ | ✅ | ✅ | `Expression::CodeRef` |
| ConceptRef | `"concept-id"` | ➖ | ❌ | ✅ | ✅ | `Expression::ConceptRef` |
| ValueSetRef | `"ValueSet"` | ➖ | ❌ | ✅ | ✅ | `Expression::ValueSetRef` |
| CodeSystemRef | `"CodeSystem"` | ➖ | ❌ | ✅ | ✅ | `Expression::CodeSystemRef` |
| CalculateAge¹ | *(clinical)* | ➖ | ❌ | ❌ | ❌ | See §13 Clinical |

---

## §13 Clinical Operators

*Spec ref: §9.12*

These operators depend on FHIR Patient context and are not yet implemented.

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| CalculateAge | `AgeInYears()` etc. | ➖ | ❌ | ❌ | ❌ | Requires patient context |
| CalculateAgeAt | `AgeInYearsAt(date)` | ➖ | ❌ | ❌ | ❌ | Requires patient context |
| AgeInYears | `AgeInYears()` | ➖ | ❌ | ❌ | ❌ | |
| AgeInMonths | `AgeInMonths()` | ➖ | ❌ | ❌ | ❌ | |
| AgeInWeeks | `AgeInWeeks()` | ➖ | ❌ | ❌ | ❌ | |
| AgeInDays | `AgeInDays()` | ➖ | ❌ | ❌ | ❌ | |
| AgeInHours | `AgeInHours()` | ➖ | ❌ | ❌ | ❌ | |
| AgeInMinutes | `AgeInMinutes()` | ➖ | ❌ | ❌ | ❌ | |

---

## §14 Error Operators

*Spec ref: §9.13*

| Operator | CQL Syntax | Parse | Semantic | Emit | Eval | Notes |
|----------|-----------|-------|----------|------|------|-------|
| Message | `Message(src, cond, code, sev, msg)` | ➖ | ❌ | ❌ | ❌ | Not implemented |

---

## §15 Keywords

All 130 keywords listed below are recognized by the parser (`src/parser/lexer.rs` `KEYWORDS` array) — each carries **✅ Parse** status. Unrecognized or future keywords would appear here with ❌.

### Declaration keywords
`library`, `version`, `using`, `include`, `called`, `public`, `private`,
`codesystem`, `valueset`, `code`, `concept`, `parameter`, `default`,
`context`, `define`, `function`, `fluent`, `returns`, `external`

### System types
`Boolean`, `Integer`, `Long`, `Decimal`, `String`, `Date`, `DateTime`, `Time`,
`Quantity`, `Ratio`, `Code`, `Concept`, `Any`, `List`, `Interval`, `Tuple`,
`Choice`

### Literals
`true`, `false`, `null`

### Logical operators
`and`, `or`, `xor`, `not`, `implies`

### Comparison / type operators
`is`, `as`, `between`

### Query keywords
`from`, `where`, `return`, `all`, `distinct`, `such`, `that`,
`with`, `without`, `let`, `sort`, `by`, `asc`, `ascending`, `desc`,
`descending`, `retrieve`, `aggregate`, `starting`

### Membership
`in`, `contains`, `properly`, `includes`, `included`, `during`

### Interval / timing operators
`starts`, `ends`, `occurs`, `same`, `before`, `after`, `overlaps`, `meets`,
`within`, `start`, `end`, `of`, `width`, `successor`, `predecessor`,
`singleton`, `point`, `per`

### Type constructors
`convert`, `to`

### Set operators
`union`, `intersect`, `except`

### List operators
`collapse`, `expand`, `flatten`

### Control flow
`if`, `then`, `else`, `case`, `when`

### Date/time precision units
`day`, `days`, `week`, `weeks`, `month`, `months`, `year`, `years`,
`hour`, `hours`, `minute`, `minutes`, `second`, `seconds`,
`millisecond`, `milliseconds`

### Display / value bounds
`display`, `minimum`, `maximum`

---

## §16 Grammar Productions

The following AST expression categories are recognized by the parser and lowered to ELM by the
emitter. All have corresponding ELM representations.

| Production | AST Variant | ELM Node | Status |
|-----------|-------------|----------|--------|
| Literal (all types) | `Expression::Literal` | `Literal` | ✅ |
| Identifier ref | `Expression::IdentifierRef` | `ExpressionRef` / `IdentifierRef` | ✅ |
| Qualified identifier | `Expression::QualifiedIdentifierRef` | `QualifiedIdentifierRef` | ✅ |
| Unary expression | `Expression::UnaryExpression` | various | ✅ |
| Binary expression | `Expression::BinaryExpression` | various | ✅ |
| Ternary expression | `Expression::TernaryExpression` | various | ✅ |
| Type expression | `Expression::TypeExpression` | `Is` / `As` | ✅ |
| Timing expression | `Expression::TimingExpression` | interval timing nodes | 🚧 |
| If-then-else | `Expression::IfThenElse` | `If` | ✅ |
| Case | `Expression::Case` | `Case` | ✅ |
| Query | `Expression::Query` | `Query` | ✅ |
| Retrieve | `Expression::Retrieve` | `Retrieve` | ✅ |
| Interval literal | `Expression::IntervalExpression` | `Interval` | ✅ |
| List literal | `Expression::ListExpression` | `List` | ✅ |
| Tuple literal | `Expression::TupleExpression` | `Tuple` | ✅ |
| Instance | `Expression::Instance` | `Instance` | ✅ |
| Function invocation | `Expression::FunctionInvocation` | `FunctionRef` / system expr | ✅ |
| Member invocation | `Expression::MemberInvocation` | `Property` | ✅ |
| Index invocation | `Expression::IndexInvocation` | `Indexer` | ✅ |
| Let clause | `Expression::LetClause` | `LetClause` | ✅ |
| Parenthesized | `Expression::Parenthesized` | transparent | ✅ |
| Min value | `Expression::MinValue` | `MinValue` | ✅ |
| Max value | `Expression::MaxValue` | `MaxValue` | ✅ |
| DateTime component | `Expression::DateTimeComponentFrom` | `DateTimeComponentFrom` | ✅ |

---

## §17 Literal Types

| CQL Type | AST Variant | Eval `Value` | Status |
|----------|-------------|--------------|--------|
| `null` | `Literal::Null` | `Value::Null` | ✅ |
| `Boolean` | `Literal::Boolean` | `Value::Boolean` | ✅ |
| `Integer` | `Literal::Integer` | `Value::Integer` | ✅ |
| `Long` | `Literal::Long` | `Value::Long` | ✅ |
| `Decimal` | `Literal::Decimal` | `Value::Decimal` | ✅ |
| `String` | `Literal::String` | `Value::String` | ✅ |
| `Date` | `Literal::Date` | `Value::Date` | ✅ |
| `DateTime` | `Literal::DateTime` | `Value::DateTime` | ✅ |
| `Time` | `Literal::Time` | `Value::Time` | ✅ |
| `Quantity` | `Literal::Quantity` | `Value::Quantity` | ✅ |
| `Ratio` | `Literal::Ratio` | `Value::Ratio` | 🚧 |
| `Code` | `Literal::Code` (inline) | `Value::Code` | ✅ |

---

## §18 Known Gaps Summary

The following operators are recognized by the CQL 1.5.3 specification but are **not yet
implemented** in `rh-cql` at any pipeline stage:

### High Priority (affect common clinical expressions)
- **Precision / LowBoundary / HighBoundary** — uncertainty arithmetic for imprecise dates
- **Substring** — basic string slicing
- **PositionOf / LastPositionOf** — string search (implementations exist in `string_ops.rs` but are not wired into the pipeline)
- **DateFrom / TimeFrom / TimezoneOffsetFrom** — extraction from DateTime (parse ✅, eval ❌)
- **TimeOfDay** — current time-of-day function
- **Children / Descendents** — FHIR model navigation nodes

### Medium Priority
- **Tail / Skip / Take / Slice** — list sub-sequence operators
- **ReplaceMatches** (eval) — parse ✅ but eval not dispatched
- **Repeat** (full) — stub only; fixpoint iteration not complete
- **Product / GeometricMean** — numeric aggregate functions
- **Size** — interval size operator

### Low Priority (clinical context-dependent)
- **CalculateAge / AgeIn\*** — require patient birthDate context
- **Message** — CQL error/trace operator
- **ToRatio** — ratio conversion from string
- **SplitOnMatches** — regex-based split (impl exists, not wired)

---

*Generated from codebase audit of `rh-cql` as of branch `cql-roadmap`.*
*Operator inventory sourced from: `src/parser/ast.rs`, `src/parser/lexer.rs`, `src/operators.rs`, `src/emit/mod.rs`, `src/emit/operators.rs`, `src/eval/engine.rs`, `src/eval/lists.rs`, `src/eval/intervals.rs`, `src/eval/operators/`.*
