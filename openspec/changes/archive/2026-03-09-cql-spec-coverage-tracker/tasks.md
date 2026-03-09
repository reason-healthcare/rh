## 1. Document Scaffolding

- [x] 1.1 Create `crates/rh-cql/SPEC_COVERAGE.md` with title, CQL version reference, status legend (✅/❌/➖), and empty category dashboard table
- [x] 1.2 Add all 13 Appendix B section headers (§1 Types through §13 Errors and Messaging) with empty per-operator tables (columns: Operator | Parse | Semantic | Emit | Eval | Notes)
- [x] 1.3 Add §14 Keywords section with empty keyword status table
- [x] 1.4 Add §15 Grammar Productions section with empty productions table

## 2. Codebase Audit — Parse Stage

- [x] 2.1 Audit `parser/lexer.rs` KEYWORDS array and `Operator` enum to determine which operators have explicit parse support
- [x] 2.2 Audit `parser/ast.rs` UnaryOperator, BinaryOperator, TernaryOperator, and TypeOperator enums for operator coverage
- [x] 2.3 Audit `parser/precedence.rs` for inline-parsed operators (div, mod, cast, timezoneoffset)
- [x] 2.4 Populate Parse column for all operator rows based on audit findings

## 3. Codebase Audit — Semantic Stage

- [x] 3.1 Audit `operators.rs` OperatorResolver registration methods (9 methods) to map registered operators
- [x] 3.2 Audit `semantics/analyzer.rs` for type-checking and resolution of each operator category
- [x] 3.3 Populate Semantic column for all operator rows based on audit findings

## 4. Codebase Audit — Emit Stage

- [x] 4.1 Audit `emit/operators.rs` for unary, binary, and ternary operator emission mappings
- [x] 4.2 Audit `emit/operators.rs` `emit_system_function` for function-style operators (11 functions)
- [x] 4.3 Audit `emit/queries.rs`, `emit/clinical.rs`, `emit/types.rs`, `emit/literals.rs` for remaining expression emission
- [x] 4.4 Populate Emit column for all operator rows based on audit findings

## 5. Codebase Audit — Eval Stage

- [x] 5.1 Audit `eval/engine.rs` dispatch arms (~95+ expression types) to determine evaluated operators
- [x] 5.2 Audit `eval/operators/` submodules (arithmetic, comparison, string_ops, conversion, temporal, lists, intervals)
- [x] 5.3 Populate Eval column for all operator rows based on audit findings

## 6. Populate Operator Tables

- [x] 6.1 Fill §1 Types section (~13 type operators: Equal, Equivalent, Is, As, Convert, ToBoolean, ToInteger, ToDecimal, ToString, ToDateTime, ToDate, ToTime, ToQuantity, ToRatio, ToConcept)
- [x] 6.2 Fill §2 Logical Operators (And, Or, Not, Implies, Xor)
- [x] 6.3 Fill §3 Type Operators (Is, As, Convert, CanConvert, ToBoolean, ToDate, ToDateTime, ToDecimal, ToInteger, ToLong, ToQuantity, ToRatio, ToString, ToTime, ToConcept)
- [x] 6.4 Fill §4 Nullological Operators (IsNull, IsTrue, IsFalse, Coalesce, IfNull/Null propagation)
- [x] 6.5 Fill §5 Comparison Operators (Equal, Equivalent, NotEqual, Less, Greater, LessOrEqual, GreaterOrEqual)
- [x] 6.6 Fill §6 Arithmetic Operators (Add, Subtract, Multiply, Divide, TruncatedDivide, Modulo, Ceiling, Floor, Truncate, Abs, Negate, Round, Ln, Exp, Log, Power, MinValue, MaxValue, Successor, Predecessor)
- [x] 6.7 Fill §7 String Operators (Concatenate, Combine, Split, SplitOnMatches, Length, Upper, Lower, Indexer, PositionOf, LastPositionOf, Substring, StartsWith, EndsWith, Matches, ReplaceMatches)
- [x] 6.8 Fill §8 Date/Time Operators (DateTime, Date, Time, Now, Today, TimeOfDay, DateTimeComponentFrom, DateFrom, TimeFrom, TimezoneOffsetFrom, DurationBetween, DifferenceBetween, Add, Subtract, SameAs, SameOrBefore, SameOrAfter)
- [x] 6.9 Fill §9 Interval Operators (Interval, Start, End, Width, Size, PointFrom, Contains, ProperContains, In, ProperIn, Includes, ProperIncludes, IncludedIn, ProperIncludedIn, Before, After, Meets, MeetsBefore, MeetsAfter, Overlaps, OverlapsBefore, OverlapsAfter, Starts, Ends, Collapse, Expand, Union, Intersect, Except)
- [x] 6.10 Fill §10 List Operators (List, Exists, Equal, Equivalent, Contains, In, Includes, IncludedIn, ProperIncludes, ProperIncludedIn, Union, Intersect, Except, IndexOf, Flatten, Sort, ForEach, Filter, Repeat, Distinct, Current, First, Last, Tail, Skip, Take, Slice, SingletonFrom, Count)
- [x] 6.11 Fill §11 Aggregate Operators (Count, Sum, Min, Max, Avg, Median, Mode, Variance, StdDev, PopulationVariance, PopulationStdDev, AllTrue, AnyTrue, Product, GeometricMean)
- [x] 6.12 Fill §12 Clinical Operators (AgeInYears, AgeInMonths, AgeInWeeks, AgeInDays, AgeInHours, AgeInMinutes, AgeInSeconds, CalculateAge, CalculateAgeAt)
- [x] 6.13 Fill §13 Errors and Messaging (Message)

## 7. Keywords and Grammar Productions

- [x] 7.1 List all CQL 1.5.3 keywords from Appendix L and cross-reference against `KEYWORDS` array in `lexer.rs`
- [x] 7.2 Mark each keyword as recognized/unrecognized in parser
- [x] 7.3 List grammar productions and note any unsupported syntax constructs

## 8. Dashboard and Cross-references

- [x] 8.1 Compute per-category counts and coverage percentages; populate the summary dashboard table
- [x] 8.2 Update CONFORMANCE.md §4 to replace feature tables with a link to SPEC_COVERAGE.md
- [x] 8.3 Review entire document for accuracy — spot-check 10+ operators by verifying source code references

## 9. Notes Column Population

- [x] 9.1 Add source code references (enum variant, function name, file path) to Notes column for all ✅ entries
- [x] 9.2 Add brief gap explanations to Notes column for notable ❌ entries (e.g., "ELM variant exists but no dispatch")
