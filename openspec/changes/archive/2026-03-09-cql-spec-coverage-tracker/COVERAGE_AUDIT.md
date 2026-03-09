# CQL 1.5.3 Operator Coverage Audit — rh-cql

> Generated from source analysis of `crates/rh-cql/src/`

## Legend

| Symbol | Meaning |
|--------|---------|
| YES | Implemented at this stage |
| NO | Not implemented at this stage |
| N/A | Not applicable to this stage (e.g., function-call-based operators have no Parse-level AST node) |
| PARTIAL | Partially implemented or placeholder |

**Pipeline Stages:**
- **Parse** — Lexer keyword / AST enum variant exists (`parser/lexer.rs`, `parser/ast.rs`)
- **Semantic** — Operator registered in `OperatorResolver` (`operators.rs`)
- **Emit** — Emitted to ELM expression (`emit/operators.rs`, `emit/mod.rs`, `emit/types.rs`, `emit/references.rs`)
- **Eval** — Evaluated at runtime (`eval/engine.rs`, `eval/operators/*`, `eval/lists.rs`, `eval/intervals.rs`)

---

## §1 Logical Operators

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| And | YES — keyword `and`, `BinaryOperator::And` | YES — `register_logical_operators` | YES — `emit_binary_operator` | YES — `Expression::And` engine.rs |
| Or | YES — keyword `or`, `BinaryOperator::Or` | YES — `register_logical_operators` | YES — `emit_binary_operator` | YES — `Expression::Or` engine.rs |
| Xor | YES — keyword `xor`, `BinaryOperator::Xor` | YES — `register_logical_operators` | YES — `emit_binary_operator` | YES — `Expression::Xor` engine.rs |
| Not | YES — keyword `not`, `UnaryOperator::Not` | YES — `register_logical_operators` | YES — `emit_unary_operator` | YES — `Expression::Not` engine.rs |
| Implies | YES — keyword `implies`, `BinaryOperator::Implies` | YES — `register_logical_operators` | YES — `emit_binary_operator` | YES — `Expression::Implies` engine.rs |

---

## §2 Type Operators

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| Is | YES — keyword `is`, `TypeOperator::Is` | NO | YES — `emit_type_expression` types.rs | YES — `Expression::Is` engine.rs → conversion.rs |
| As | YES — keyword `as`, `TypeOperator::As` | NO | YES — `emit_type_expression` (strict=false) types.rs | YES — `Expression::As` engine.rs → conversion.rs |
| Cast | YES — `TypeOperator::Cast` | NO | YES — `emit_type_expression` → As (strict=true) types.rs | NO — maps to As at Emit |
| Convert | YES — keyword `convert`, `TypeOperator::Convert` | NO | YES — `emit_type_expression` types.rs | YES — `Expression::Convert` engine.rs → conversion.rs |
| CanConvert | NO | NO | NO | YES — `Expression::CanConvert` engine.rs → conversion.rs |
| CanConvertQuantity | NO | NO | NO | YES — `Expression::CanConvertQuantity` engine.rs |
| ToBoolean | YES — `UnaryOperator::ToBoolean` | YES — `register_type_operators` | YES — `emit_unary_operator` | YES — `Expression::ToBoolean` engine.rs |
| ToInteger | YES — `UnaryOperator::ToInteger` | YES — `register_type_operators` | YES — `emit_unary_operator` | YES — `Expression::ToInteger` engine.rs |
| ToLong | YES — `UnaryOperator::ToLong` | YES — `register_type_operators` | YES — `emit_unary_operator` | YES — `Expression::ToLong` engine.rs |
| ToDecimal | YES — `UnaryOperator::ToDecimal` | YES — `register_type_operators` | YES — `emit_unary_operator` | YES — `Expression::ToDecimal` engine.rs |
| ToString | YES — `UnaryOperator::ToString` | YES — `register_type_operators` | YES — `emit_unary_operator` → ToStringExpr | YES — `Expression::ToStringExpr` engine.rs |
| ToDate | YES — `UnaryOperator::ToDate` | YES — `register_type_operators` | YES — `emit_unary_operator` | YES — `Expression::ToDate` engine.rs |
| ToDateTime | YES — `UnaryOperator::ToDateTime` | YES — `register_type_operators` | YES — `emit_unary_operator` | YES — `Expression::ToDateTime` engine.rs |
| ToTime | YES — `UnaryOperator::ToTime` | YES — `register_type_operators` | YES — `emit_unary_operator` | YES — `Expression::ToTime` engine.rs |
| ToQuantity | YES — `UnaryOperator::ToQuantity` | YES — `register_type_operators` | YES — `emit_unary_operator` | YES — `Expression::ToQuantity` engine.rs |
| ToConcept | YES — `UnaryOperator::ToConcept` | YES — `register_type_operators` | YES — `emit_unary_operator` | YES — `Expression::ToConcept` engine.rs |
| ToList | NO | NO | NO | YES — `Expression::ToList` engine.rs |
| ToRatio | NO | NO | NO | NO |
| Children | NO | NO | NO | NO |
| Descendents | NO | NO | NO | NO |

---

## §3 Nullological Operators

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| IsNull | YES — `UnaryOperator::IsNull` | YES — `register_nullological_operators` | YES — `emit_unary_operator` | YES — `Expression::IsNull` engine.rs |
| IsTrue | YES — `UnaryOperator::IsTrue` | YES — `register_nullological_operators` | YES — `emit_unary_operator` | YES — `Expression::IsTrue` engine.rs |
| IsFalse | YES — `UnaryOperator::IsFalse` | YES — `register_nullological_operators` | YES — `emit_unary_operator` | YES — `Expression::IsFalse` engine.rs |
| Coalesce | N/A (function-call) | NO | N/A (emitted as FunctionRef) | YES — `Expression::Coalesce` engine.rs |

---

## §4 Comparison Operators

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| Equal | YES — `Operator::Equal`, `BinaryOperator::Equal` | YES — `register_comparison_operators` | YES — `emit_binary_operator` | YES — `Expression::Equal` engine.rs → comparison.rs |
| Equivalent | YES — `Operator::Equivalent`, `BinaryOperator::Equivalent` | YES — `register_comparison_operators` | YES — `emit_binary_operator` | YES — `Expression::Equivalent` engine.rs → comparison.rs |
| NotEqual | YES — `Operator::NotEqual`, `BinaryOperator::NotEqual` | NO (handled as Not(Equal)) | YES — `emit_binary_operator` → Not(Equal) | YES — `Expression::NotEqual` engine.rs |
| NotEquivalent | YES — `Operator::NotEquivalent`, `BinaryOperator::NotEquivalent` | NO (handled as Not(Equivalent)) | YES — `emit_binary_operator` → Not(Equivalent) | NO (should be lowered at Emit) |
| Less | YES — `Operator::Less`, `BinaryOperator::Less` | YES — `register_comparison_operators` | YES — `emit_binary_operator` | YES — `Expression::Less` engine.rs → comparison.rs |
| Greater | YES — `Operator::Greater`, `BinaryOperator::Greater` | YES — `register_comparison_operators` | YES — `emit_binary_operator` | YES — `Expression::Greater` engine.rs → comparison.rs |
| LessOrEqual | YES — `Operator::LessOrEqual`, `BinaryOperator::LessOrEqual` | YES — `register_comparison_operators` | YES — `emit_binary_operator` | YES — `Expression::LessOrEqual` engine.rs → comparison.rs |
| GreaterOrEqual | YES — `Operator::GreaterOrEqual`, `BinaryOperator::GreaterOrEqual` | YES — `register_comparison_operators` | YES — `emit_binary_operator` | YES — `Expression::GreaterOrEqual` engine.rs → comparison.rs |
| Between | YES — `TernaryOperator::Between`, keyword `between` | YES — `register_comparison_operators` | YES — `emit_ternary_operator` → And(>=, <=) | NO (lowered at Emit) |

---

## §5 Arithmetic Operators

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| Add | YES — `Operator::Plus`, `BinaryOperator::Add` | YES — `register_arithmetic_operators` | YES — `emit_binary_operator` | YES — `Expression::Add` engine.rs → arithmetic.rs |
| Subtract | YES — `Operator::Minus`, `BinaryOperator::Subtract` | YES — `register_arithmetic_operators` | YES — `emit_binary_operator` | YES — `Expression::Subtract` engine.rs → arithmetic.rs |
| Multiply | YES — `Operator::Star`, `BinaryOperator::Multiply` | YES — `register_arithmetic_operators` | YES — `emit_binary_operator` | YES — `Expression::Multiply` engine.rs → arithmetic.rs |
| Divide | YES — `Operator::Slash`, `BinaryOperator::Divide` | YES — `register_arithmetic_operators` | YES — `emit_binary_operator` (with Integer→Decimal promotion) | YES — `Expression::Divide` engine.rs → arithmetic.rs |
| TruncatedDivide | YES — `BinaryOperator::TruncatedDivide` | YES — `register_arithmetic_operators` | YES — `emit_binary_operator` | YES — `Expression::TruncatedDivide` engine.rs → arithmetic.rs |
| Modulo | YES — `BinaryOperator::Modulo` | YES — `register_arithmetic_operators` | YES — `emit_binary_operator` | YES — `Expression::Modulo` engine.rs → arithmetic.rs |
| Negate | YES — `UnaryOperator::Negate` | YES — `register_arithmetic_operators` | YES — `emit_unary_operator` | YES — `Expression::Negate` engine.rs → arithmetic.rs |
| Abs | N/A (function-call) | YES — `register_arithmetic_operators` | YES — `emit_system_function` | YES — `Expression::Abs` engine.rs → arithmetic.rs |
| Ceiling | N/A (function-call) | YES — `register_arithmetic_operators` | YES — `emit_system_function` | YES — `Expression::Ceiling` engine.rs → arithmetic.rs |
| Floor | N/A (function-call) | YES — `register_arithmetic_operators` | YES — `emit_system_function` | YES — `Expression::Floor` engine.rs → arithmetic.rs |
| Truncate | N/A (function-call) | YES — `register_arithmetic_operators` | YES — `emit_system_function` | YES — `Expression::Truncate` engine.rs → arithmetic.rs |
| Round | N/A (function-call) | YES — `register_arithmetic_operators` | YES — `emit_system_function` (1-arg only) | YES — `Expression::Round` engine.rs → arithmetic.rs |
| Ln | N/A (function-call) | YES — `register_arithmetic_operators` | YES — `emit_system_function` | YES — `Expression::Ln` engine.rs → arithmetic.rs |
| Exp | N/A (function-call) | YES — `register_arithmetic_operators` | YES — `emit_system_function` | YES — `Expression::Exp` engine.rs → arithmetic.rs |
| Log | N/A (function-call) | YES — `register_arithmetic_operators` | YES — `emit_system_function` (2-arg) | YES — `Expression::Log` engine.rs → arithmetic.rs |
| Power | YES — `Operator::Caret`, `BinaryOperator::Power` | YES — `register_arithmetic_operators` | YES — `emit_binary_operator` / `emit_system_function` | YES — `Expression::Power` engine.rs → arithmetic.rs |
| Predecessor | YES — keyword `predecessor`, `UnaryOperator::Predecessor` | YES — `register_arithmetic_operators` | YES — `emit_unary_operator` / `emit_system_function` | YES — `Expression::Predecessor` engine.rs → temporal.rs |
| Successor | YES — keyword `successor`, `UnaryOperator::Successor` | YES — `register_arithmetic_operators` | YES — `emit_unary_operator` / `emit_system_function` | YES — `Expression::Successor` engine.rs → temporal.rs |
| MinValue | YES — keyword `minimum`, `Expression::MinValue` | NO | YES — `emit_min_value` types.rs | YES — `Expression::MinValue` engine.rs → temporal.rs |
| MaxValue | YES — keyword `maximum`, `Expression::MaxValue` | NO | YES — `emit_max_value` types.rs | YES — `Expression::MaxValue` engine.rs → temporal.rs |

---

## §6 String Operators

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| Concatenate | YES — `Operator::Ampersand`, `BinaryOperator::Concatenate` | YES — `register_string_operators` | YES — `emit_binary_operator` | YES — `Expression::Concatenate` engine.rs |
| Combine | N/A (function-call) | NO | NO (emitted as FunctionRef) | YES — `Expression::Combine` engine.rs → string_ops.rs |
| Split | N/A (function-call) | NO | NO (emitted as FunctionRef) | YES — `Expression::Split` engine.rs → string_ops.rs |
| SplitOnMatches | N/A (function-call) | NO | NO | YES — string_ops.rs (fn `split_on_matches`) but NO engine.rs dispatch |
| Length | N/A (function-call) | YES — `register_string_operators` | NO (emitted as FunctionRef) | YES — `Expression::Length` engine.rs → string_ops.rs |
| Upper | N/A (function-call) | YES — `register_string_operators` | NO (emitted as FunctionRef) | YES — `Expression::Upper` engine.rs → string_ops.rs |
| Lower | N/A (function-call) | YES — `register_string_operators` | NO (emitted as FunctionRef) | YES — `Expression::Lower` engine.rs → string_ops.rs |
| Indexer | N/A (function-call) | YES — `register_list_operators` | YES — `emit_index_invocation` references.rs | YES — `Expression::Indexer` engine.rs |
| PositionOf | N/A (function-call) | NO | NO | YES — string_ops.rs (fn `position_of`) but NO engine.rs dispatch |
| LastPositionOf | N/A (function-call) | NO | NO | YES — string_ops.rs (fn `last_position_of`) but NO engine.rs dispatch |
| Substring | N/A (function-call) | NO | NO | YES — string_ops.rs (fn `substring`) but NO engine.rs dispatch |
| StartsWith | N/A (function-call) | YES — `register_string_operators` | NO (emitted as FunctionRef) | YES — `Expression::StartsWith` engine.rs → string_ops.rs |
| EndsWith | N/A (function-call) | YES — `register_string_operators` | NO (emitted as FunctionRef) | YES — `Expression::EndsWith` engine.rs → string_ops.rs |
| Matches | N/A (function-call) | YES — `register_string_operators` | NO (emitted as FunctionRef) | YES — `Expression::Matches` engine.rs → string_ops.rs |
| ReplaceMatches | YES — `TernaryOperator::ReplaceMatches` | YES — `register_string_operators` | YES — `emit_ternary_operator` | YES — string_ops.rs (fn `replace_matches`) but NO direct engine.rs dispatch |
| IndexOf | YES — `BinaryOperator::IndexOf` | YES — `register_string_operators` | YES — `emit_binary_operator` → Indexer | NO direct (maps to Indexer) |

---

## §7 DateTime Operators

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| Date (constructor) | YES — `Literal::Date` | N/A | YES — `emit_literal` literals.rs | YES — engine.rs |
| DateTime (constructor) | YES — `Literal::DateTime` | N/A | YES — `emit_literal` literals.rs | YES — engine.rs |
| Time (constructor) | YES — `Literal::Time` | N/A | YES — `emit_literal` literals.rs | YES — engine.rs |
| DateTimeComponentFrom | YES — `Expression::DateTimeComponentFrom` | NO | YES — `emit_datetime_component_from` operators.rs | NO direct engine.rs dispatch |
| DateFrom | YES — `UnaryOperator::DateFrom` | NO | YES — `emit_unary_operator` | NO direct engine.rs dispatch |
| TimeFrom | YES — `UnaryOperator::TimeFrom` | NO | YES — `emit_unary_operator` | NO direct engine.rs dispatch |
| TimezoneOffsetFrom | YES — `UnaryOperator::TimezoneOffsetFrom` | NO | YES — `emit_unary_operator` | NO direct engine.rs dispatch |
| SameAs | YES — keyword `same`, `BinaryOperator::SameAs` | YES — `register_datetime_operators` | YES — `emit_binary_operator` | YES — `Expression::SameAs` engine.rs → temporal.rs |
| SameOrBefore | YES — `BinaryOperator::SameOrBefore` | YES — `register_datetime_operators` | YES — `emit_binary_operator` | YES — `Expression::SameOrBefore` engine.rs → temporal.rs |
| SameOrAfter | YES — `BinaryOperator::SameOrAfter` | YES — `register_datetime_operators` | YES — `emit_binary_operator` | YES — `Expression::SameOrAfter` engine.rs → temporal.rs |
| Before | YES — keyword `before`, `BinaryOperator::Before` | YES — `register_datetime_operators` | YES — `emit_binary_operator` | NO direct engine.rs dispatch (see note) |
| After | YES — keyword `after`, `BinaryOperator::After` | YES — `register_datetime_operators` | YES — `emit_binary_operator` | NO direct engine.rs dispatch (see note) |
| DurationBetween | N/A (function-call) | YES — `register_datetime_operators` | NO (emitted as FunctionRef) | YES — `Expression::DurationBetween` engine.rs → temporal.rs |
| DifferenceBetween | N/A (function-call) | NO | NO | YES — `Expression::DifferenceBetween` engine.rs → temporal.rs |
| Add (DateTime) | YES — `BinaryOperator::Add` | YES — `register_datetime_operators` | YES — shared with arithmetic Add | YES — shared with arithmetic Add → temporal.rs |
| Subtract (DateTime) | YES — `BinaryOperator::Subtract` | YES — `register_datetime_operators` | YES — shared with arithmetic Subtract | YES — shared with arithmetic Subtract → temporal.rs |
| Today | N/A (function-call) | NO | NO (emitted as FunctionRef) | YES — `Expression::Today` engine.rs |
| Now | N/A (function-call) | NO | NO (emitted as FunctionRef) | YES — `Expression::Now` engine.rs |

---

## §8 Interval Operators

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| Interval (constructor) | YES — `Expression::IntervalExpression` | N/A | YES — `emit_interval_expression` types.rs | YES — `Expression::Interval` engine.rs |
| Start | YES — keyword `start`, `UnaryOperator::Start` | YES — `register_interval_operators` | YES — `emit_unary_operator` | YES — `Expression::Start` engine.rs → intervals.rs |
| End | YES — keyword `end`, `UnaryOperator::End` | YES — `register_interval_operators` | YES — `emit_unary_operator` | YES — `Expression::End` engine.rs → intervals.rs |
| Width | YES — keyword `width`, `UnaryOperator::Width` | YES — `register_interval_operators` | YES — `emit_unary_operator` | YES — `Expression::Width` engine.rs → intervals.rs |
| PointFrom | YES — `UnaryOperator::PointFrom` | YES — `register_interval_operators` | YES — `emit_unary_operator` | YES — `Expression::PointFrom` engine.rs → intervals.rs |
| Contains | YES — keyword `contains`, `BinaryOperator::Contains` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::Contains` engine.rs → intervals.rs |
| In | YES — keyword `in`, `BinaryOperator::In` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::In` engine.rs → intervals.rs |
| Includes | YES — keyword `includes`, `BinaryOperator::Includes` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::Includes` engine.rs → intervals.rs |
| IncludedIn | YES — keyword `included`, `BinaryOperator::IncludedIn` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::IncludedIn` engine.rs → intervals.rs |
| ProperlyIncludes | YES — keyword `properly`, `BinaryOperator::ProperlyIncludes` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::ProperIncludes` engine.rs → intervals.rs |
| ProperlyIncludedIn | YES — `BinaryOperator::ProperlyIncludedIn` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::ProperIncludedIn` engine.rs → intervals.rs |
| ProperContains | NO (no parse-level variant) | NO | NO | YES — `Expression::ProperContains` engine.rs → intervals.rs |
| ProperIn | NO (no parse-level variant) | NO | NO | YES — `Expression::ProperIn` engine.rs → intervals.rs |
| Overlaps | YES — keyword `overlaps`, `BinaryOperator::Overlaps` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::Overlaps` engine.rs → intervals.rs |
| OverlapsBefore | YES — `BinaryOperator::OverlapsBefore` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::OverlapsBefore` engine.rs → intervals.rs |
| OverlapsAfter | YES — `BinaryOperator::OverlapsAfter` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::OverlapsAfter` engine.rs → intervals.rs |
| Meets | YES — keyword `meets`, `BinaryOperator::Meets` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::Meets` engine.rs → intervals.rs |
| MeetsBefore | YES — `BinaryOperator::MeetsBefore` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::MeetsBefore` engine.rs → intervals.rs |
| MeetsAfter | YES — `BinaryOperator::MeetsAfter` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::MeetsAfter` engine.rs → intervals.rs |
| Starts | YES — keyword `starts`, `BinaryOperator::Starts` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::Starts` engine.rs → intervals.rs |
| Ends | YES — keyword `ends`, `BinaryOperator::Ends` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — `Expression::Ends` engine.rs → intervals.rs |
| During | YES — keyword `during`, `BinaryOperator::During` | NO (handled as IncludedIn) | YES — `emit_binary_operator` → IncludedIn | NO (lowered at Emit) |
| Within | YES — keyword `within`, `BinaryOperator::Within` | NO (handled as IncludedIn) | YES — `emit_binary_operator` → IncludedIn | NO (lowered at Emit) |
| Union | YES — keyword `union`, `BinaryOperator::Union` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — engine.rs → intervals.rs |
| Intersect | YES — keyword `intersect`, `BinaryOperator::Intersect` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — engine.rs → intervals.rs |
| Except | YES — keyword `except`, `BinaryOperator::Except` | YES — `register_interval_operators` | YES — `emit_binary_operator` | YES — engine.rs → intervals.rs |
| Collapse | YES — keyword `collapse`, `UnaryOperator::Collapse` | YES — `register_interval_operators` | YES — `emit_unary_operator` | YES — `Expression::Collapse` engine.rs → intervals.rs |
| Expand | YES — keyword `expand`, `UnaryOperator::Expand` | NO | YES — `emit_unary_operator` | YES — `Expression::Expand` engine.rs → intervals.rs |

---

## §9 List Operators

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| List (constructor) | YES — `Expression::ListExpression` | N/A | YES — `emit_list_expression` types.rs | YES — `Expression::List` engine.rs |
| Exists | YES — `UnaryOperator::Exists` | YES — `register_list_operators` | YES — `emit_unary_operator` | YES — `Expression::Exists` engine.rs → lists.rs |
| Count | N/A (function-call) | NO | NO (emitted as FunctionRef) | YES — `Expression::Count` engine.rs → lists.rs |
| Sum | N/A (function-call) | NO | NO (emitted as FunctionRef) | YES — `Expression::Sum` engine.rs → lists.rs |
| Min | N/A (function-call) | NO | NO (emitted as FunctionRef) | YES — `Expression::Min` engine.rs → lists.rs |
| Max | N/A (function-call) | NO | NO (emitted as FunctionRef) | YES — `Expression::Max` engine.rs → lists.rs |
| Avg | N/A (function-call) | NO | NO (emitted as FunctionRef) | YES — `Expression::Avg` engine.rs → lists.rs |
| Median | N/A (function-call) | NO | NO | YES — `Expression::Median` engine.rs → lists.rs |
| Mode | N/A (function-call) | NO | NO | YES — `Expression::Mode` engine.rs → lists.rs |
| Variance | N/A (function-call) | NO | NO | YES — `Expression::Variance` engine.rs → lists.rs |
| PopulationVariance | N/A (function-call) | NO | NO | YES — `Expression::PopulationVariance` engine.rs → lists.rs |
| StdDev | N/A (function-call) | NO | NO | YES — `Expression::StdDev` engine.rs → lists.rs |
| PopulationStdDev | N/A (function-call) | NO | NO | YES — `Expression::PopulationStdDev` engine.rs → lists.rs |
| AllTrue | N/A (function-call) | NO | NO | YES — `Expression::AllTrue` engine.rs → lists.rs |
| AnyTrue | N/A (function-call) | NO | NO | YES — `Expression::AnyTrue` engine.rs → lists.rs |
| First | N/A (function-call) | YES — `register_list_operators` | NO (emitted as FunctionRef) | YES — `Expression::First` engine.rs → lists.rs |
| Last | N/A (function-call) | YES — `register_list_operators` | NO (emitted as FunctionRef) | YES — `Expression::Last` engine.rs → lists.rs |
| IndexOf | N/A (function-call) | YES — `register_list_operators` | NO (emitted as FunctionRef) | NO direct (eval/lists.rs has fn `index_of`) |
| Distinct | YES — keyword `distinct`, `UnaryOperator::Distinct` | YES — `register_list_operators` | YES — `emit_unary_operator` | YES — `Expression::Distinct` engine.rs → lists.rs |
| Flatten | YES — keyword `flatten`, `UnaryOperator::Flatten` | YES — `register_list_operators` | YES — `emit_unary_operator` | YES — `Expression::Flatten` engine.rs → lists.rs |
| SingletonFrom | YES — `UnaryOperator::Singleton` | YES — `register_list_operators` | YES — `emit_unary_operator` → SingletonFrom | YES — `Expression::SingletonFrom` engine.rs → lists.rs |
| Union (List) | YES — keyword `union`, `BinaryOperator::Union` | YES — `register_list_operators` | YES — `emit_binary_operator` | YES — engine.rs → lists.rs |
| Intersect (List) | YES — keyword `intersect`, `BinaryOperator::Intersect` | YES — `register_list_operators` | YES — `emit_binary_operator` | YES — engine.rs → lists.rs |
| Except (List) | YES — keyword `except`, `BinaryOperator::Except` | YES — `register_list_operators` | YES — `emit_binary_operator` | YES — engine.rs → lists.rs |
| In (List) | YES — keyword `in`, `BinaryOperator::In` | YES — `register_list_operators` | YES — `emit_binary_operator` | YES — engine.rs → lists.rs |
| Contains (List) | YES — keyword `contains`, `BinaryOperator::Contains` | YES — `register_list_operators` | YES — `emit_binary_operator` | YES — engine.rs → lists.rs |
| Includes (List) | YES — keyword `includes`, `BinaryOperator::Includes` | YES — `register_list_operators` | YES — `emit_binary_operator` | YES — engine.rs → lists.rs |
| IncludedIn (List) | YES — keyword `included`, `BinaryOperator::IncludedIn` | YES — `register_list_operators` | YES — `emit_binary_operator` | YES — engine.rs → lists.rs |
| ProperlyIncludes (List) | YES — `BinaryOperator::ProperlyIncludes` | YES — `register_list_operators` | YES — `emit_binary_operator` | YES — engine.rs |
| ProperlyIncludedIn (List) | YES — `BinaryOperator::ProperlyIncludedIn` | YES — `register_list_operators` | YES — `emit_binary_operator` | YES — engine.rs |
| Sort | YES — keyword `sort` | N/A (query clause) | YES — `emit_sort_clause` queries.rs | YES — `Expression::Sort` engine.rs → lists.rs |
| ForEach | NO | NO | NO | NO engine.rs dispatch (lists.rs has fn `for_each`) |
| Filter | NO | NO | NO | NO engine.rs dispatch (lists.rs has fn `filter`) |
| Repeat | NO | NO | NO | YES — `Expression::Repeat` engine.rs |
| Current | NO | NO | NO | NO |
| Tail | NO | NO | NO | NO |
| Skip | NO | NO | NO | NO |
| Take | NO | NO | NO | NO |
| Slice | NO | NO | NO | NO |
| Product | NO | NO | NO | NO |
| GeometricMean | NO | NO | NO | NO |

---

## §10 Aggregate Operators

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| Aggregate (query clause) | YES — keyword `aggregate` | N/A (query clause) | YES — `emit_aggregate_clause` queries.rs | YES — engine.rs query eval |

> Note: Individual aggregate functions (Count, Sum, Min, Max, Avg, etc.) are listed under §9 List Operators.

---

## §11 Clinical Operators

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| InValueSet | NO | NO | NO | YES — `Expression::InValueSet` engine.rs |
| InCodeSystem | NO | NO | NO | YES — `Expression::InCodeSystem` engine.rs |
| AnyInValueSet | NO | NO | NO | YES — `Expression::AnyInValueSet` engine.rs |
| AnyInCodeSystem | NO | NO | NO | YES — `Expression::AnyInCodeSystem` engine.rs |
| CalculateAge | NO | NO | NO | NO |
| CalculateAgeAt | NO | NO | NO | NO |
| AgeInYears | NO | NO | NO | NO |
| AgeInMonths | NO | NO | NO | NO |
| AgeInWeeks | NO | NO | NO | NO |
| AgeInDays | NO | NO | NO | NO |
| AgeInHours | NO | NO | NO | NO |
| AgeInMinutes | NO | NO | NO | NO |
| AgeInSeconds | NO | NO | NO | NO |

---

## §12 Errors and Messaging

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| Message | NO | NO | NO | NO |

---

## §13 Query Operators

| Operator | Parse | Semantic | Emit | Eval |
|----------|-------|----------|------|------|
| Query | YES — `Expression::Query` | YES — analyzer.rs | YES — `emit_query` queries.rs | YES — `Expression::Query` engine.rs |
| Retrieve | YES — `Expression::Retrieve`, keyword `retrieve` | YES — analyzer.rs | YES — `emit_retrieve` queries.rs | YES — `Expression::Retrieve` engine.rs |
| AliasRef | N/A (generated) | N/A | N/A | YES — `Expression::AliasRef` engine.rs |
| QueryLetRef | N/A (generated) | N/A | N/A | YES — engine.rs query let handling |
| Property | N/A (generated) | N/A | YES — `emit_member_invocation` references.rs → Property | YES — `Expression::Property` engine.rs |
| Tuple (constructor) | YES — `Expression::TupleExpression` | N/A | YES — `emit_tuple_expression` types.rs | YES — `Expression::Tuple` engine.rs |

---

## Expression / Literal Types (not operators)

| Type | Parse | Semantic | Emit | Eval |
|------|-------|----------|------|------|
| Null | YES — keyword `null`, `Literal::Null` | N/A | YES — literals.rs | YES — engine.rs |
| Boolean | YES — keywords `true`/`false`, `Literal::Boolean` | N/A | YES — literals.rs | YES — engine.rs |
| Integer | YES — `Literal::Integer` | N/A | YES — literals.rs | YES — engine.rs |
| Long | YES — `Literal::Long` | N/A | YES — literals.rs | YES — engine.rs |
| Decimal | YES — `Literal::Decimal` | N/A | YES — literals.rs | YES — engine.rs |
| String | YES — `Literal::String` | N/A | YES — literals.rs | YES — engine.rs |
| Date | YES — `Literal::Date` | N/A | YES — literals.rs | YES — engine.rs |
| DateTime | YES — `Literal::DateTime` | N/A | YES — literals.rs | YES — engine.rs |
| Time | YES — `Literal::Time` | N/A | YES — literals.rs | YES — engine.rs |
| Quantity | YES — `Literal::Quantity` | N/A | YES — literals.rs | YES — engine.rs |
| Ratio | YES — `Literal::Ratio` | N/A | YES — literals.rs | NO engine.rs dispatch (literal only) |
| Code | YES — `Literal::Code`, keyword `code` | N/A | YES — clinical.rs | YES — `Expression::Code` engine.rs |
| Concept | YES — keyword `concept` | N/A | YES — clinical.rs | YES — `Expression::ConceptRef` engine.rs |
| Instance | YES — `Expression::Instance` | N/A | YES — `emit_instance` types.rs | NO |

---

## Control Flow

| Construct | Parse | Semantic | Emit | Eval |
|-----------|-------|----------|------|------|
| If-Then-Else | YES — keywords `if`/`then`/`else`, `Expression::IfThenElse` | YES — analyzer.rs | YES — emit/mod.rs | YES — `Expression::If` engine.rs |
| Case | YES — keyword `case`/`when`, `Expression::Case` | YES — analyzer.rs | YES — emit/mod.rs | YES — `Expression::Case` engine.rs |

---

## Declaration / Definition (Library-level)

| Construct | Parse | Emit | Eval |
|-----------|-------|------|------|
| Library | YES — keyword `library` | YES — `emit()` mod.rs | N/A |
| Using | YES — keyword `using` | YES — `emit()` mod.rs → UsingDef | N/A |
| Include | YES — keyword `include` | NO (TODO in emit) | N/A |
| Parameter | YES — keyword `parameter` | YES — `emit()` mod.rs → ParameterDef | YES — ParameterRef engine.rs |
| CodeSystem | YES — keyword `codesystem` | YES — `emit()` mod.rs → CodeSystemDef | YES — CodeSystemRef engine.rs |
| ValueSet | YES — keyword `valueset` | YES — `emit()` mod.rs → ValueSetDef | YES — ValueSetRef engine.rs |
| Code (def) | YES — keyword `code` | YES — `emit()` mod.rs → CodeDef | YES — CodeRef engine.rs |
| Concept (def) | YES — keyword `concept` | YES — `emit()` mod.rs → ConceptDef | YES — ConceptRef engine.rs |
| Context | YES — keyword `context` | YES — `emit()` mod.rs → ContextDef | N/A |
| Define | YES — keyword `define` | YES — `emit()` mod.rs → ExpressionDef | YES — ExpressionRef engine.rs |
| Function (def) | YES — keyword `function` | YES — `emit()` mod.rs → FunctionDef | YES — FunctionRef engine.rs |

---

## KEYWORDS Array (lexer.rs lines 34–157)

All 118 keywords in the `KEYWORDS` array:

```
library, version, using, include, called,
public, private,
codesystem, valueset, code, concept,
parameter, default,
context, define, function, fluent, returns, external,
Boolean, Integer, Long, Decimal, String, Date, DateTime, Time,
Quantity, Ratio, Code, Concept, Any, List, Interval, Tuple, Choice,
true, false, null,
and, or, xor, not, implies,
between, is, as,
from, where, return, all, distinct,
such, that, with, without, let,
sort, by, asc, ascending, desc, descending,
in, contains, properly, includes, included, during,
starts, ends, occurs, same, before, after,
overlaps, meets, within,
start, end, of, width,
successor, predecessor,
singleton, point,
aggregate, starting,
retrieve,
if, then, else, case, when,
day, days, week, weeks, month, months,
year, years, hour, hours, minute, minutes,
second, seconds, millisecond, milliseconds,
convert, to,
union, intersect, except,
collapse, expand, flatten, per,
display,
minimum, maximum
```

---

## Lexer Operator Tokens (lexer.rs Operator enum)

```
Plus (+), Minus (-), Star (*), Slash (/), Caret (^),
Equal (=), NotEqual (!=), Equivalent (~), NotEquivalent (!~),
Less (<), LessOrEqual (<=), Greater (>), GreaterOrEqual (>=),
Ampersand (&), Pipe (|), Dot (.), Comma (,), Colon (:), Semicolon (;),
LeftParen, RightParen, LeftBracket, RightBracket, LeftBrace, RightBrace,
Arrow (->)
```

---

## Coverage Summary

| Category | Total Operators | Full Pipeline (P+S+E+V) | Eval Only | Parse+Emit Only | Not Implemented |
|----------|----------------|------------------------|-----------|-----------------|-----------------|
| §1 Logical | 5 | 5 | 0 | 0 | 0 |
| §2 Type | 16 | 10 | 3 (CanConvert, CanConvertQuantity, ToList) | 1 (Cast) | 2 (ToRatio, Children/Descendents) |
| §3 Nullological | 4 | 3 | 1 (Coalesce) | 0 | 0 |
| §4 Comparison | 9 | 6 | 0 | 1 (Between—lowered) | 0 (NotEqual/NotEquivalent handled via lowering) |
| §5 Arithmetic | 19 | 17 | 0 | 0 | 0 (MinValue/MaxValue skip Semantic but covered) |
| §6 String | 16 | 3 (Concatenate, ReplaceMatches—partial, IndexOf→Indexer) | 5 (Combine, Split, PositionOf, LastPositionOf, Substring—partial) | 0 | 3 (SplitOnMatches dispatch gap) |
| §7 DateTime | 14 | 6 (SameAs/Before/After, Add/Sub) | 4 (DurationBetween, DifferenceBetween, Today, Now) | 3 (DateFrom, TimeFrom, TimezoneOffsetFrom) | 0 |
| §8 Interval | 25 | 21 | 2 (ProperContains, ProperIn) | 2 (During, Within—lowered) | 0 |
| §9 List | 35 | 11 | 9 (Count, Sum, Min, Max, Avg, Median, Mode, etc.) | 0 | 8 (ForEach*, Filter*, Tail, Skip, Take, Slice, Product, GeometricMean) |
| §11 Clinical | 13 | 0 | 4 (InValueSet, InCodeSystem, AnyIn*) | 0 | 9 (CalculateAge*, AgeIn*) |
| §12 Messaging | 1 | 0 | 0 | 0 | 1 (Message) |
| §13 Query | 6 | 3 (Query, Retrieve, Tuple) | 2 (AliasRef, QueryLetRef) | 0 | 0 |

**Totals across all categories:**
- **Full pipeline coverage**: ~85 operators
- **Eval-only** (missing Parse/Semantic/Emit): ~30 operators
- **Not implemented anywhere**: ~23 operators

> *ForEach and Filter have eval/lists.rs implementations but no engine.rs dispatch.
