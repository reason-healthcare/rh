# CQL-to-ELM Translator Implementation Plan

## Overview

Implementation plan for `rh-cql`, a Rust CQL-to-ELM translator library integrated with rh CLI, and targeting WASM.

## Phases

### Phase 1: Foundation (ELM Types & Model Info)
**Goal**: Define ELM output types and model resolution infrastructure

- [x] **1.1 ELM Type Definitions**
  - Define all ELM expression types with serde serialization
  - Library, ExpressionDef, all Expression variants
  - TypeSpecifier variants (Named, List, Interval, Tuple, Choice)
  - Implement JSON serialization matching ELM JSON schema

- [x] **1.2 ModelInfo Types**
  - Parse ModelInfo XML schema
  - Define ModelInfo, TypeInfo, ClassInfo structs
  - System model (primitives: Boolean, Integer, Decimal, String, Date, DateTime, Time, Quantity)

- [x] **1.3 Model Manager**
  - ModelInfoProvider trait (in rh-cql)
  - MemoryStore pattern in rh-foundation (reusable for packages, models, libraries)
  - Implement MemoryModelInfoProvider using foundation pattern
  - Bundle FHIR R4 model info (fhir_r4_model_info, fhir_r4_provider)
  - Type resolution by name (resolve_type, resolve_class)
  - *Note: Memory provider pattern in rh-foundation for WASM compatibility and reuse across crates*

- [x] **1.4 DataType System**
  - Internal DataType enum (System, Model, List, Interval, Tuple, Choice, TypeParameter)
  - SystemType enum with all CQL primitives (Any, Boolean, Integer, Long, Decimal, String, Date, DateTime, Time, Quantity, Ratio, Code, Concept, Vocabulary)
  - Type compatibility checking (is_subtype_of, can_convert_to)
  - Common type resolution (common_type)
  - Implicit conversion map (Integer→Long→Decimal, Date→DateTime, Code→Concept)

### Phase 2: CQL Parser
**Goal**: Parse CQL source into AST using `nom` (consistent with rh-fhirpath)

- [x] **2.1 Parser Foundation**
  - Use `nom` parser combinators (same as rh-fhirpath)
  - Translate CQL grammar (CQL version 1.5.3) from ANTLR4/spec to nom combinators
  - Leverage patterns established in rh-fhirpath parser
  - Created parser module structure (mod.rs, lexer.rs, span.rs, ast.rs)
  - Source location tracking with Span type implementing nom traits

- [x] **2.2 Lexer**
  - CQL keywords and operators (130+ keywords)
  - Literals (string, number, date/time, quantity)
  - Identifiers and comments (line //, block /* */)
  - Source location tracking (Span, SourceLocation, SourceRange)

- [x] **2.3 CQL AST Types**
  - Library header (library, using, include, codesystem, valueset, code, concept, parameter, context)
  - Expression definitions and function definitions
  - All expression types (70+ types)
  - Type specifiers (Named, List, Interval, Tuple, Choice)

- [x] **2.4 Parser Implementation**
  - Expression parser with full operator precedence (10 levels)
  - Statement parser for library structure and definitions
  - Library parser integrating all components
  - CqlParser with parse() and parse_expression() methods
  - Comprehensive tests and cql_parser example

### Phase 3: Library Resolution
**Goal**: Resolve library dependencies

- [x] **3.1 LibrarySourceProvider Trait**
  - LibraryIdentifier (name + version key)
  - LibrarySource (source code with metadata)
  - LibrarySourceProvider trait (get_source, has_library, list_libraries, find_by_name)
  - MemoryLibrarySourceProvider using MemoryStore
  - FileLibrarySourceProvider for filesystem (non-WASM)
  - CompositeLibrarySourceProvider for layered providers

- [x] **3.2 CompiledLibrary**
  - Wrapper for ELM Library with convenient lookup methods
  - Usings, includes, parameters, expressions lookup
  - Code systems, value sets, codes, concepts lookup
  - Expression lookup by name and context
  - Public/private filtering
  - DefinitionRef enum for type-safe definition access
  - FunctionRef placeholder for function overload resolution

- [x] **3.3 LibraryManager**
  - Library caching using MemoryStore
  - Dependency resolution with resolve() and resolve_all()
  - Circular dependency detection (detect_cycle)
  - Topological sort for compilation order
  - Dependency graph generation
  - LibraryError enum for error handling

### Phase 4: Semantic Analysis & Translation
**Goal**: Translate CQL AST to typed ELM

- [x] **4.1 Preprocessor**
  - Collect LibraryInfo from AST (name, version, identifier)
  - Extract definitions without type resolution (expressions, functions, parameters)
  - Extract model and library dependencies for resolution
  - Extract terminology definitions (code systems, value sets, codes, concepts)
  - Definition lookup and classification (DefinitionKind)
  - Access control tracking (public/private)
  - Trigger library resolution via library_dependency_ids()

- [x] **4.2 LibraryBuilder Core**
  - Symbol and Scope types for symbol table management
  - SymbolKind enum (expression, function, parameter, terminology, alias, let, operand, library, context)
  - FunctionSignature for overload resolution with operand types
  - Context management (Patient, Practitioner, etc.) - auto-defines context symbol
  - Nested scope stack with shadowing semantics
  - Model registration (using declarations) with URI mapping
  - Included library management with qualified lookup
  - Identifier resolution through scope chain (innermost to outermost)
  - Qualified identifier resolution (library.name, model.type, alias.property)
  - Function overload resolution by name
  - Error and warning collection (BuilderError enum)
  - Integration with Preprocessor via from_library_info()

- [x] **4.3 Type Resolution**
  - Resolve type specifiers to DataTypes
  - Type inference for expressions
  - TypeBuilder (DataType ↔ TypeSpecifier)

- [x] **4.4 Operator Resolution**
  - System operator signatures (arithmetic, comparison, logical, string, list, interval, datetime)
  - Overload resolution with scoring (exact match, conversion, subtype)
  - Implicit conversions (Integer→Long→Decimal, Date→DateTime, Code→Concept)
  - Generic operator support (List<T>, Interval<T> operations)
  - Type parameter binding and substitution for result types
  - OperatorResolver with resolve_unary, resolve_binary, resolve_ternary
  - 21 tests + operator_resolver example

- [x] **4.5 Expression Translation** ✅
  - a. Literals (24 tests)
  - b. References - ExpressionRef, FunctionRef, ParameterRef (45 tests)
  - c. Operators - unary, binary, nary (101 tests)
  - d. Queries - source, where, return, sort (118 tests)
  - e. Retrieve expressions (131 tests)
  - f. Conditionals - if, case (142 tests)
  - g. Type operators - as, is, convert (165 tests)

- [x] **4.6 Function Resolution** ✅
  - System functions (nullary, aggregate, list, nullological, arithmetic, string, interval, clinical, type conversion)
  - User-defined functions (FunctionRef, FunctionDef translation)
  - Fluent function syntax (value.function(args) → function(value, args))
  - 46 new tests (211 total translator tests) + function_resolver example

### Phase 5: System Operators ✅
**Goal**: Implement all CQL system operators

> **Note**: Phase 5 is complete through work done in Phases 4.5c (Operator Translation), 4.5g (Type Operators), and 4.6 (Function Resolution). All system operators are now covered:
> - **Operators** (unary/binary/ternary) translate AST operators to ELM expressions
> - **System functions** (Abs, Sum, First, Now, etc.) resolve to specific ELM operator types

- [x] **5.1 Arithmetic**: Add, Subtract, Multiply, Divide, Modulo, Negate (binary/unary operators), Abs, Ceiling, Floor, Truncate, Round, Ln, Exp (system functions), Log, Power (binary operators)
- [x] **5.2 Comparison**: Equal, Equivalent, Less, Greater, LessOrEqual, GreaterOrEqual, NotEqual, NotEquivalent (binary operators)
- [x] **5.3 Logical**: And, Or, Not, Implies, Xor (unary/binary operators)
- [x] **5.4 String**: Concatenate (binary/nary), Length, Upper, Lower, ToChars (system functions), StartsWith, EndsWith, Matches (binary/system functions), ReplaceMatches (ternary)
- [x] **5.5 DateTime**: Now, Today, TimeOfDay (nullary system functions), Before, After, SameAs, SameOrBefore, SameOrAfter (time binary operators)
- [x] **5.6 Interval**: Contains, In, Includes, IncludedIn, ProperContains, ProperIn, ProperIncludes, ProperIncludedIn, Overlaps, OverlapsBefore, OverlapsAfter, Meets, MeetsBefore, MeetsAfter, Starts, Ends (binary operators), Start, End, Width, PointFrom, Collapse, Expand (unary/binary operators/system functions)
- [x] **5.7 List**: First, Last, Flatten, Distinct, Exists, SingletonFrom (system functions), Union, Intersect, Except, Indexer (binary operators/system functions)
- [x] **5.8 Aggregate**: Count, Sum, Min, Max, Avg, Median, Mode, StdDev, PopulationStdDev, Variance, PopulationVariance, AllTrue, AnyTrue (system functions)
- [x] **5.9 Type**: As, Is, Convert (type operators in 4.5g), ToBoolean, ToInteger, ToLong, ToDecimal, ToString, ToDate, ToDateTime, ToTime, ToQuantity, ToRatio, ToConcept, ToList, ToChars (unary operators/system functions)
- [x] **5.10 Null**: IsNull, IsTrue, IsFalse (unary operators), Coalesce (nary system function)

### Phase 6: Compiler Options & Output
**Goal**: Complete translation with options

- [x] **6.1 CompilerOptions** ✅
  - CompilerOption enum with all option flags (EnableAnnotations, EnableLocators, EnableResultTypes, DisableListDemotion, etc.)
  - SignatureLevel enum (None, Differing, Overloads, All)
  - ErrorSeverity enum (Info, Warning, Error)
  - CompilerOptions struct with default(), new(), strict(), debug() constructors
  - Builder pattern for fluent configuration
  - Integration with ExpressionTranslator
  - JSON serialization/deserialization
  - Options string parsing for compatibility with reference implementation

- [x] **6.2 Output Generation** ✅
  - ElmWriter struct for JSON ELM output with pretty/compact options
  - Automatic translator metadata injection (version, options)
  - Schema identifier added to output
  - Convenience functions: library_to_json, library_to_json_with_options, library_to_compact_json
  - Write to String, Vec<u8>, or any io::Write
  - XML output returns UnsupportedFormat (placeholder for future)

- [x] **6.3 Error Reporting**
  - Detailed error messages
  - Source locations
  - Warning levels

- [x] **Phase 6.4: Translation Integration** ✅
├── [x] 6.4a: translate_expression() - unified expression dispatch ✅
├── [x] 6.4b: Statement translation (expression_def, parameter_def, function_def) ✅
├── [x] 6.4c: Terminology translation (using, include, codesystem, valueset, code, concept) ✅
├── [x] 6.4d: LibraryBuilder.build() - assemble complete library ✅
└── [x] 6.4e: compile() entry point - public API ✅

- [x] **Phase 6.5: ELM Output Refinement** (Complete)
  
  - High Priority (ELM Semantic Correctness)
    - [x] Symbol Resolution Infrastructure: Added translate_expression_with_resolution() 
          and translate_expr_recursive() in builder.rs for resolution-aware translation
    - [x] Expression Resolution: IdentifierRef → ExpressionRef for define references
    - [x] Terminology References: Use CodeRef, ValueSetRef, CodeSystemRef based on symbol table
    - [x] Query Alias Resolution: IdentifierRef used correctly for query aliases (in-scope)
    - [x] Retrieve Enhancement: Add codeProperty, codeComparator, templateId from model info
          - templateId from ClassInfo.identifier (profile URL)
          - codeProperty from ClassInfo.primary_code_path (defaults from model)
          - codeComparator set to "~" (equivalent) when codes present
          - compile() now uses FHIR R4 model provider by default
  - Medium Priority (ELM Spec Compliance)
    - [x] Query Scope: Use scope attribute instead of nested source for query aliases
          - ExpressionTranslator: Added query_alias_scopes stack for tracking aliases
          - push_query_scope/pop_query_scope for entering/exiting query contexts
          - is_query_alias() checks if identifier is in scope
          - translate_query: Pushes aliases from sources and let clauses
          - translate_relationship_clause: Pushes alias for such_that condition
          - translate_member_invocation: Uses scope attribute when source is query alias
          - builder.rs: Updated MemberInvocation translation to use scope
          - Test: test_query_alias_scope_attribute verifies scope usage
    - [x] EnableLocators: Parser captures SourceLocation, builder emits "line:column" locator
          - Parser: parse_expression_def now captures start_loc before define keyword
          - Builder: ExpressionDef uses locator from expr_def.location when enabled
          - Test: test_enable_locators verifies enabled/disabled behavior
    - [x] EnableAnnotations: Implement annotation with "s" narrative structure ✓
          - Updated Annotation struct with source (Narrative) field
          - Narrative struct with local_id_ref (r) and segments (s)
          - NarrativeSegment enum: Value (text fragments) or Nested (recursive)
          - Builder: Generate annotation when annotations_enabled()
          - Test: test_enable_annotations verifies enabled/disabled behavior
    - [ ] Empty Arrays: Add let, relationship arrays where expected
          - Deferred: Requires structural changes to ELM types (Query.let, Query.relationship)
          - [x] Retrieve: include, codeFilter, dateFilter, otherFilter (completed in 6.6g)
    - [x] Patient Context: Generate implicit Patient definition ✓
          - Builder: Generates `define "Patient": SingletonFrom([Patient])` when context is Patient
          - Retrieve has dataType="{http://hl7.org/fhir}Patient" and templateId
          - Tests: test_implicit_patient_definition, test_no_implicit_definition_without_context
  - Low Priority (Metadata)
    - [x] Library Metadata: Add localId to Library (extends Element) ✓
          - Library struct: Added localId field for element tracking
          - Builder: Calls translator.next_local_id() when building library
    - [x] Using System: Add implicit System using declaration ✓
          - Builder: Always adds System using first with uri urn:hl7-org:elm-types:r1
          - Explicit usings from source follow after System using
    - [x] Sort Type: Add "type": "ByColumn" to sort by clauses ✓
          - SortByItem struct: Added sort_by_type field (ByColumn, ByExpression, ByDirection)
          - Translator: Sets type based on whether path is present

- [ ] **Phase 6.6: Type System & Implicit Conversions** (Planned)
  
  **Goal**: Implement proper type inference and implicit conversions based on ModelInfo
  
  **Background**:
  The Java reference translator inserts FHIRHelpers conversion calls (e.g., `FHIRHelpers.ToString`,
  `FHIRHelpers.ToConcept`) to convert FHIR types to CQL System types. This is driven by:
  1. **ModelInfo** - defines `conversionInfo` entries specifying fromType, toType, and functionName
  2. **FHIRHelpers library** - provides the actual conversion function implementations
  
  The official ModelInfo for FHIR 4.0.1 is in the `fhir.cqf.common@4.0.1` FHIR package:
  - `Library-FHIR-ModelInfo.json` - contains base64-encoded XML ModelInfo
  - `Library-FHIRHelpers.json` - contains CQL, ELM+XML, and ELM+JSON for conversion functions
  
  **Architecture**:
  
  ```
  ┌─────────────────────────────────────────────────────────────────────────────┐
  │  FHIR Package: fhir.cqf.common@4.0.1                                        │
  │  └── Library-FHIR-ModelInfo.json                                            │
  │      └── content[].data (base64 XML)                                        │
  │          └── <modelInfo>                                                    │
  │              ├── <typeInfo> (ClassInfo for each FHIR type)                  │
  │              └── <conversionInfo> (implicit conversion declarations)        │
  │                  ├── fromType="FHIR.Coding" toType="System.Code"            │
  │                  │   functionName="FHIRHelpers.ToCode"                      │
  │                  ├── fromType="FHIR.CodeableConcept" toType="System.Concept"│
  │                  │   functionName="FHIRHelpers.ToConcept"                   │
  │                  ├── fromType="FHIR.Period" toType="Interval<System.DateTime>│
  │                  │   functionName="FHIRHelpers.ToInterval"                  │
  │                  └── ... (many FHIR.* → System.String via FHIRHelpers.ToString)
  └─────────────────────────────────────────────────────────────────────────────┘
  
  ┌─────────────────────────────────────────────────────────────────────────────┐
  │  Compiler Pipeline                                                          │
  │                                                                             │
  │  1. Load ModelInfo from FHIR package (via rh-foundation loader)             │
  │  2. Parse XML ModelInfo → populate conversionInfo in ModelInfo struct       │
  │  3. During type checking, identify where implicit conversion is needed      │
  │  4. Resolve FHIRHelpers from author's `include FHIRHelpers` statement       │
  │  5. Insert FunctionRef to FHIRHelpers.* function (libraryName + name)       │
  └─────────────────────────────────────────────────────────────────────────────┘
  ```
  
  **Important**: The CQL author must explicitly `include FHIRHelpers version '4.0.1'` 
  in their library. The translator does NOT auto-add this include. What it does is:
  - See that FHIRHelpers is included
  - When a type conversion is needed, wrap the expression in a FunctionRef 
    to the appropriate FHIRHelpers function (e.g., `FHIRHelpers.ToConcept`)
  - If FHIRHelpers is NOT included but conversions are needed → warning/error
  
  **Implementation Steps**:
  
  - [x] **6.6a: ModelInfo XML Parsing** ✅
    - Added `quick-xml` to workspace dependencies
    - Created `modelinfo_xml.rs` module with XML parsing for all TypeInfo variants
    - Handles `xsi:type` attribute for polymorphic type discrimination
    - Parses all 264 `conversionInfo` elements from FHIR ModelInfo
    - 7 new tests for XML parsing including real FHIR ModelInfo file
    
  - [x] **6.6b: FHIR Package ModelInfo Provider** ✅
    - Added `load_modelinfo_from_package()` to load from package directories
    - Added `load_fhir_r4_modelinfo_from_package()` for fhir.cqf.common@4.0.1
    - Added `fhir_r4_provider_from_package()` for provider with full ModelInfo
    - Handles base64 decoding of XML content from Library resources
    - Falls back to built-in minimal model if package not available
    - 7 new tests for package loading
    
  - [x] **6.6c: Type Inference Infrastructure** ✓
    - Created `conversion.rs` module (578 lines)
    - `ConversionEntry`: Struct holding from_type, to_type, function_name, library_name
    - `ConversionRegistry`: HashMap<String, Vec<ConversionEntry>> indexed by from_type
    - `ConversionRegistry::from_model_info()`: Loads all 264 conversionInfo entries from ModelInfo
    - `ConversionContext`: Tracks FHIRHelpers availability and provides conversion lookup APIs
    - `find_conversion(from_type, to_type)`: Returns ConversionEntry if available
    - `needs_conversion(from, to)`: Checks if DataType A needs conversion to DataType B
    - `datatype_to_conversion_key()`: Converts DataType to "FHIR.Coding" string format
    - `conversion_key_to_datatype()`: Parses type string back to DataType
    - 10 unit tests including real ModelInfo integration test
    
  - [x] **6.6d: Implicit Conversion Insertion** ✓
    - `wrap_in_conversion(operand, entry, element_fields)`: Creates FunctionRef wrapper
    - `ConversionResult` enum: NotNeeded, Applied, LibraryNotIncluded, NoConversionDefined
    - `ConversionContext::try_convert()`: Full conversion workflow with library checking
    - `ConversionContext::convert_if_needed()`: Convenience method returning (expr, bool)
    - `LibraryBuilder` integration:
      - Added `conversion_context: ConversionContext` field
      - `set_conversion_registry()`: Initialize from ModelInfo
      - `register_conversion_library()`: Track included libraries (e.g., FHIRHelpers)
      - `apply_implicit_conversion()`: Entry point for conversion during translation
    - Emits warnings when conversion needed but library not included
    - 6 new tests for conversion wrapping and try_convert scenarios
    
  - [x] **6.6e: Conversion Validation** ✓
    - New compiler options in `options.rs`:
      - `DisableImplicitConversions`: Disables automatic FHIRHelpers conversion insertion
      - `StrictConversionLibraryCheck`: Treats missing conversion library as error (not warning)
    - Helper methods: `implicit_conversions_enabled()`, `strict_conversion_library_check()`
    - Updated `CompilerOptions::strict()` to include conversion-related strict settings
    - `apply_implicit_conversion()` now respects:
      - `DisableImplicitConversions` - skips conversion entirely
      - `StrictConversionLibraryCheck` - emits `BuilderError::SemanticError` instead of warning
    - Added `BuilderError::SemanticError` variant for generic semantic errors
    - `options_to_string()` and `parse_options()` updated for new options
    - 9 new tests for conversion validation behavior
    
  **Key Conversion Functions** (from ModelInfo):
  | FHIR Type | CQL Type | Converter |
  |-----------|----------|-----------|
  | FHIR.Coding | System.Code | FHIRHelpers.ToCode |
  | FHIR.CodeableConcept | System.Concept | FHIRHelpers.ToConcept |
  | FHIR.Quantity | System.Quantity | FHIRHelpers.ToQuantity |
  | FHIR.Period | Interval<DateTime> | FHIRHelpers.ToInterval |
  | FHIR.Range | Interval<Quantity> | FHIRHelpers.ToInterval |
  | FHIR.Ratio | System.Ratio | FHIRHelpers.ToRatio |
  | FHIR.* (codes/enums) | System.String | FHIRHelpers.ToString |
  | FHIR.string | System.String | FHIRHelpers.ToString |
  | FHIR.boolean | System.Boolean | FHIRHelpers.ToBoolean |
  | FHIR.integer | System.Integer | FHIRHelpers.ToInteger |
  | FHIR.decimal | System.Decimal | FHIRHelpers.ToDecimal |
  | FHIR.date | System.Date | FHIRHelpers.ToDate |
  | FHIR.dateTime | System.DateTime | FHIRHelpers.ToDateTime |
  | FHIR.time | System.Time | FHIRHelpers.ToTime |
  
  **Compiler Options**:
  - `EnableImplicitConversions` - Insert FHIRHelpers conversion calls (default: true for FHIR)

**6.6 continued**
  - [x] 6.6e Type conversions - FHIRHelpers.To* calls for implicit FHIR→System conversions ✓
        - Tracks alias types in queries (HashMap<alias, Option<type>> in translator)
        - Extracts type from Retrieve source expressions
        - Looks up property types from ModelInfo via lookup_property_type()
        - Applies FHIRHelpers conversion via maybe_apply_property_conversion()
        - Requires full ModelInfo from fhir.cqf.common package for proper types
  - [x] 6.6f CodeRef vs IdentifierRef - terminology references now use CodeRef, ValueSetRef ✓
        - Retrieve.codes uses CodeRef for code definitions
        - Retrieve.codes uses ValueSetRef for value set references
  - [x] 6.6g Retrieve enhancements ✓
        - [x] templateId from ClassInfo.identifier
        - [x] codeProperty from ClassInfo.primary_code_path
        - [x] codeComparator: "in" for ValueSetRef, "~" for CodeRef/other expressions
        - [x] ToList wrapper around single CodeRef (ValueSetRef used directly)
        - [x] Empty arrays: include, codeFilter, dateFilter, otherFilter
  - [x] 6.6h Equivalent type coercion - ToConcept wrapper on CodeRef operands ✓
        - CodeRef in Equivalent/NotEquivalent comparisons wrapped in ToConcept
        - Uses built-in CQL ToConcept (not FHIRHelpers) for Code→Concept conversion
  - [x] 6.6i Implicit Patient definition - generates `define "Patient": SingletonFrom([Patient])` ✓
  - [x] 6.6j System using - always added first with uri urn:hl7-org:elm-types:r1 ✓
  
  **Remaining Type System Work** (future enhancements):
  - [x] 6.6k ExpressionRef type tracking ✓
        - Track result types of definitions via `expression_result_types` HashMap
        - Infer FHIR type from `as` type casts (e.g., `.value as Quantity`)
        - For comparison operators, apply FHIRHelpers conversion on ExpressionRef operands
        - Example: `"Most Recent Tumor Size Quantity" > 1 'cm'` wraps left side in `FHIRHelpers.ToQuantity`
  - [ ] 6.6l Extended type inference
        - First/Last unwrapping list element types
        - Query return clause type inference
        - Retrieve source type propagation

- [ ] **Phase 6.7: Compiler Options Implementation**
  
  **Goal**: Implement all CompilerOption flags that affect translation behavior
  
  **Option Status Table**:
  | Option | Status | Implementation Location |
  |--------|--------|------------------------|
  | EnableDateRangeOptimization | ❌ Not implemented | Retrieve optimization |
  | EnableAnnotations | ✅ Implemented | translator.rs, builder.rs, output.rs |
  | EnableLocators | ✅ Implemented | translator.rs, builder.rs |
  | EnableResultTypes | ✅ Implemented | translator.rs |
  | EnableDetailedErrors | ❌ Not implemented | reporting.rs |
  | DisableListTraversal | ❌ Not implemented | translator.rs (property access) |
  | DisableListDemotion | ❌ Not implemented | operators.rs (type coercion) |
  | DisableListPromotion | ❌ Not implemented | operators.rs (type coercion) |
  | EnableIntervalDemotion | ❌ Not implemented | operators.rs (type coercion) |
  | EnableIntervalPromotion | ❌ Not implemented | operators.rs (type coercion) |
  | DisableMethodInvocation | ❌ Not implemented | parser (fluent syntax) |
  | RequireFromKeyword | ❌ Not implemented | parser (query syntax) |
  | DisableDefaultModelInfoLoad | ❌ Not implemented | compiler.rs |
  | DisableImplicitConversions | ✅ Implemented | builder.rs |
  | StrictConversionLibraryCheck | ✅ Implemented | builder.rs |
  
  **Implementation Tasks**:
  
  - [ ] **6.7a: EnableDateRangeOptimization**
    - Optimize Retrieve expressions with date range filters
    - Move date comparisons from Where clause into Retrieve.dateRange
    - Reference: Java translator's DateRangeOptimizer
    - Affects: Retrieve translation in builder.rs
    
  - [ ] **6.7b: EnableDetailedErrors**
    - Include additional context in error messages
    - Show source snippets in error output
    - Include suggestion/fix hints where possible
    - Affects: reporting.rs, ExceptionCollector
    
  - [ ] **6.7c: DisableListTraversal**
    - When disabled: property access on List<T> is an error
    - When enabled (default): `patients.name` traverses list, returns List<name type>
    - Affects: translate_member_invocation in translator.rs
    - Check: if source type is List<T> and option disabled, emit error
    
  - [ ] **6.7d: DisableListDemotion**
    - When disabled: no auto-conversion from List<T> to T (singleton)
    - When enabled (default): List<T> can demote to T in singleton context
    - Affects: operator resolution, function argument matching
    - Check: OperatorResolver.resolve_* should respect this option
    
  - [ ] **6.7e: DisableListPromotion**
    - When disabled: no auto-conversion from T to List<T>
    - When enabled (default): T can promote to List<T> where list expected
    - Affects: operator resolution, function argument matching
    - Check: OperatorResolver.resolve_* should respect this option
    
  - [ ] **6.7f: EnableIntervalDemotion**
    - When enabled: Interval<T> can demote to T (point) in point context
    - When disabled (default): no auto-conversion
    - Affects: operator resolution for interval/point comparisons
    
  - [ ] **6.7g: EnableIntervalPromotion**
    - When enabled: T (point) can promote to Interval<T>
    - When disabled (default): no auto-conversion
    - Affects: operator resolution for interval/point comparisons
    
  - [ ] **6.7h: DisableMethodInvocation**
    - When disabled: fluent syntax `value.function(args)` is an error
    - When enabled (default): `value.function(args)` → `function(value, args)`
    - Affects: parser or translator fluent function handling
    - Note: Parser currently accepts fluent syntax; need check before translation
    
  - [ ] **6.7i: RequireFromKeyword**
    - When enabled: queries MUST use `from` keyword
    - When disabled (default): `from` is optional
    - Example required: `from [Condition] C where ...`
    - Example error: `[Condition] C where ...` (missing from)
    - Affects: parser query_clause validation
    
  - [ ] **6.7j: DisableDefaultModelInfoLoad**
    - When enabled: don't auto-load FHIR R4 ModelInfo
    - Requires explicit model registration
    - Affects: compiler.rs compile() function
    - Add accessor: `default_model_info_load_enabled() -> bool`
  
  **Testing Strategy**:
  - Each option gets integration tests showing enabled vs disabled behavior
  - Test strict() preset enables appropriate restrictions
  - Test default() preset has expected defaults
  - Verify options_to_string() round-trips through parse_options()


### Phase 7: WASM & Integration
**Goal**: WASM build and JavaScript API

- [ ] **7.1 WASM Build**
  - wasm-bindgen setup
  - Bundle size optimization
  - Feature flags for optional components

- [ ] **7.2 JavaScript API**
  - `translateCql(source, options)` → ELM JSON
  - `validateCql(source)` → errors/warnings
  - Library registration API

- [ ] **7.3 Integration**
  - Consider rh-fhirpath integration for expression evaluation
  - FHIR Library resource support

### Phase 8: Testing & Validation
**Goal**: Ensure compatibility with Java/Kotlin reference implementation and CQL specification

#### 8A. Official CQL Conformance Tests (https://cql.hl7.org/tests.html)

The CQL specification provides an official conformance test suite at **https://cql.hl7.org/tests.zip**.

**Format:**
- XML files with test cases (same format as FHIRPath spec tests)
- Each test defines: expression, expected result, optional context
- XSD schema included for validation
- Tests should run with UTC timestamp to avoid timezone issues

**Test Categories (from spec):**
| Category | Description |
|----------|-------------|
| Aggregate Functions | Count, Sum, Min, Max, Avg, etc. |
| Aggregate Operator | aggregate clause in queries |
| Arithmetic Functions | Abs, Ceiling, Floor, Ln, Log, etc. |
| Comparison Operators | =, !=, <, >, <=, >=, ~, !~ |
| Conditional Operators | If, Case |
| Date/Time Operators | Date/Time construction, arithmetic, components |
| Errors And Messaging Operators | Message function |
| Interval Operators | Contains, In, Overlaps, Meets, etc. |
| List Operators | First, Last, Union, Intersect, etc. |
| Logical Operators | And, Or, Not, Xor, Implies |
| Nullological Operators | IsNull, IsTrue, IsFalse, Coalesce |
| String Operators | Concat, Substring, Length, etc. |
| Type Operators | As, Is, Convert, ToInteger, etc. |
| Types | Type system tests |
| Value Literals and Selectors | Literal parsing, constructors |

**Implementation:**
```rust
// Download and cache official tests
const CQL_TESTS_URL: &str = "https://cql.hl7.org/tests.zip";

// Test case structure (from XSD)
struct CqlTestCase {
    name: String,
    expression: String,
    expected: ExpectedResult,
    context: Option<String>,
    invalid: Option<bool>,  // Expects error
}

// Run against our translator + evaluator
fn run_conformance_test(test: &CqlTestCase) -> TestResult;
```

#### 8B. Reference Implementation Tests (cqframework/clinical_quality_language)

**CQL Input Files:**
```
Src/java/cql-to-elm/src/jvmTest/resources/org/cqframework/cql/cql2elm/
├── OperatorTests/           # Operator-specific tests
│   ├── ArithmeticOperators.cql
│   ├── ComparisonOperators.cql
│   ├── ListOperators.cql
│   ├── IntervalOperators.cql
│   ├── DateTimeOperators.cql
│   ├── StringOperators.cql
│   ├── LogicalOperators.cql
│   ├── TypeOperators.cql
│   ├── AggregateOperators.cql
│   └── NullologicalOperators.cql
├── LibraryTests/            # Library resolution tests
├── SignatureTests/          # Function signature tests
├── fhir/r4/                 # FHIR R4 specific tests
└── ErrorTests/              # Expected error cases
    ├── ParameterTestInvalid.cql    (17 expected errors)
    └── InvalidEquality.cql         (1 expected error)
```

**Expected ELM Output (Golden Files):**
```
Src/java/elm/src/jvmTest/resources/org/cqframework/cql/elm/
├── serializing/
│   ├── CMS146v2_Expected_SignatureLevel_None.json
│   ├── CMS146v2_Expected_SignatureLevel_All.json
│   └── ... (multiple signature levels)
└── ElmDeserialize/
    ├── fhir/*.json
    └── qdm/*.json
```

**Engine Test Manifests (XML):**
```
Src/java/engine-fhir/src/test/resources/org/hl7/fhirpath/cql/
├── CqlArithmeticFunctionsTest.xml
├── CqlComparisonOperatorsTest.xml
├── CqlListOperatorsTest.xml
├── CqlIntervalOperatorsTest.xml
├── CqlDateTimeOperatorsTest.xml
└── ... (per-operator test definitions with expected values)
```

#### Test Implementation Tasks

- [ ] **8.1 Test Infrastructure**
  - Download and cache official CQL spec tests (tests.zip)
  - Download and cache reference implementation tests
  - XML test manifest parser
  - ELM JSON comparison utilities (structural equality, ignoring order)

- [ ] **8.2 CQL Spec Conformance Tests**
  - Parse tests.zip XML format
  - Run each category systematically
  - Track pass rate per category
  - **Target: 100% conformance with spec tests**

- [ ] **8.3 Parser Tests**
  - Test each grammar construct against OperatorTests/*.cql
  - Verify AST structure matches expected patterns
  - Error recovery and location tracking tests

- [ ] **8.4 Translation Tests (Golden File Comparison)**
  - Translate CQL → ELM JSON
  - Compare against `CMS146v2_Expected_SignatureLevel_*.json`
  - Test with different CompilerOptions (SignatureLevel, annotations, etc.)
  - Track agreement percentage (like rh-validator)

- [ ] **8.5 Error Case Tests**
  - `ParameterTestInvalid.cql` → expect 17 errors
  - `InvalidEquality.cql` → expect 1 error
  - Verify error messages match reference implementation

- [ ] **8.6 Library Resolution Tests**
  - Multi-library compilation
  - Include/reference resolution
  - Circular dependency detection

- [ ] **8.7 Real-World CQL**
  - eCQM measures (CMS146, CMS149, etc.)
  - CDS Hooks libraries
  - WHO SMART Guidelines

- [ ] **8.8 Test Runner Infrastructure**
  - Create `tests/cql_tests/` module (similar to rh-validator's fhir_test_cases)
  - Implement test fixture downloader for:
    - CQL spec tests (https://cql.hl7.org/tests.zip)
    - Reference implementation tests (cqframework GitHub)
  - XML test manifest parser (reuse FHIRPath test format knowledge)
  - Test result comparison utilities
  - Summary reporter with pass/fail counts by category
  - Feature flag: `cql-tests` to enable conformance tests

- [ ] **8.9 Benchmarking**
  - Use `criterion` for benchmarks (consistent with rh-validator)
  - Create `benches/` directory with:
    - `translation_bench.rs` - CQL-to-ELM translation performance
    - `evaluation_bench.rs` - CQL/ELM evaluation performance
  - Benchmark scenarios:
    - Simple expression translation (literals, operators)
    - Complex query translation (from/where/return)
    - Full library translation (CMS146, etc.)
    - Expression evaluation against FHIR data
    - Batch evaluation (multiple patients)
  - Track metrics: throughput (ops/sec), latency (p50/p95/p99)
  - Compare WASM vs native performance

#### Test Runner Implementation
```rust
// tests/cql_tests/mod.rs
#[cfg(feature = "cql-tests")]
pub mod cql_tests {
    mod downloader;    // Download and cache test fixtures
    mod parser;        // Parse XML test manifests
    mod runner;        // Execute tests and collect results
    mod reporter;      // Summary reporting
    
    // Download CQL spec tests (tests.zip) on first run
    // Download reference implementation tests
    // Compare translation output
    // Track conformance percentage by category
    // Report disagreements
}

// Example test
#[test]
#[cfg(feature = "cql-tests")]
fn test_cql_spec_conformance() {
    let runner = CqlTestRunner::new();
    let results = runner.run_spec_tests();
    runner.print_summary(&results);
    assert!(results.pass_rate() > 0.95, "Conformance below 95%");
}
```

#### Benchmark Setup
```rust
// benches/translation_bench.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn translation_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("cql_translation");
    
    // Simple expression
    group.bench_function("simple_arithmetic", |b| {
        b.iter(|| translate("1 + 2 * 3"))
    });
    
    // Complex query
    group.bench_function("fhir_query", |b| {
        b.iter(|| translate(PATIENT_QUERY_CQL))
    });
    
    // Full library (CMS146)
    group.bench_function("cms146_library", |b| {
        b.iter(|| translate(CMS146_CQL))
    });
    
    group.finish();
}

fn evaluation_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("cql_evaluation");
    
    // Pre-translated ELM
    let elm = translate(PATIENT_QUERY_CQL);
    let patient_data = load_test_patient();
    
    group.bench_function("evaluate_patient_query", |b| {
        b.iter(|| evaluate(&elm, &patient_data))
    });
    
    // Batch evaluation
    let patients = load_test_patients(100);
    group.bench_function("batch_100_patients", |b| {
        b.iter(|| evaluate_batch(&elm, &patients))
    });
    
    group.finish();
}

criterion_group!(benches, translation_benchmarks, evaluation_benchmarks);
criterion_main!(benches);
```

#### Conformance Tracking

**CQL Spec Tests (https://cql.hl7.org/tests.zip):**
| Category | Status | Pass/Total |
|----------|--------|------------|
| Aggregate Functions | ⬜ | 0/? |
| Aggregate Operator | ⬜ | 0/? |
| Arithmetic Functions | ⬜ | 0/? |
| Comparison Operators | ⬜ | 0/? |
| Conditional Operators | ⬜ | 0/? |
| Date/Time Operators | ⬜ | 0/? |
| Errors And Messaging | ⬜ | 0/? |
| Interval Operators | ⬜ | 0/? |
| List Operators | ⬜ | 0/? |
| Logical Operators | ⬜ | 0/? |
| Nullological Operators | ⬜ | 0/? |
| String Operators | ⬜ | 0/? |
| Type Operators | ⬜ | 0/? |
| Types | ⬜ | 0/? |
| Value Literals | ⬜ | 0/? |
| **TOTAL** | ⬜ | **0/?** |

**Reference Implementation Alignment:**
| Category | Status | Pass/Total |
|----------|--------|------------|
| OperatorTests/*.cql | ⬜ | 0/? |
| ELM Golden Files | ⬜ | 0/? |
| Library Resolution | ⬜ | 0/? |
| Error Cases | ⬜ | 0/? |
| Real-World eCQMs | ⬜ | 0/? |

## Dependencies

```toml
[dependencies]
# Workspace crates - reuse existing functionality
rh-fhirpath = { path = "../rh-fhirpath" }   # FHIRPath parsing & evaluation
rh-foundation = { path = "../rh-foundation" } # Package loading, HTTP, config, MemoryProvider pattern

# Parser and serialization
nom = { workspace = true }        # Parser combinators (same as rh-fhirpath)
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
anyhow = { workspace = true }
indexmap = { workspace = true }   # Ordered maps for ELM
chrono = { workspace = true }     # Date/time handling
tokio = { workspace = true }      # Async runtime for library loading
tracing = { workspace = true }    # Logging/tracing

[dev-dependencies]
criterion = "0.5"                 # Benchmarking
tempfile = "3.0"                  # Test fixtures
tokio = { workspace = true, features = ["rt-multi-thread", "macros"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"

[features]
default = []
cql-tests = []                    # Enable conformance test suite

[[bench]]
name = "translation_bench"
harness = false

[[bench]]
name = "evaluation_bench"
harness = false
```

## Milestones

| Milestone | Description | Target |
|-----------|-------------|--------|
| M1 | ELM types + JSON serialization | Week 1-2 |
| M2 | CQL parser (basic expressions) | Week 3-4 |
| M3 | Simple CQL → ELM translation | Week 5-6 |
| M4 | Library resolution | Week 7 |
| M5 | Full operator support | Week 8-10 |
| M6 | Test runner + benchmarks | Week 11 |
| M7 | WASM build | Week 12 |
| M8 | Production ready | Week 13+ |

## Open Questions (All Resolved)

1. **Parser choice**: ✅ RESOLVED - Use `nom`
   - Consistent with rh-fhirpath implementation
   - Proven WASM compatibility in this codebase
   - No build step required, team familiarity

2. **Model bundling**: ✅ RESOLVED - Use existing FHIR ModelInfo
   - Use the standard FHIR ModelInfo from [CQF Common IG](https://www.fhir.org/guides/cqf/common/4.0.1/artifacts.html)
   - Key artifacts: `FHIR-ModelInfo`, `FHIRHelpers`, `FHIRCommon`
   - ModelInfo is now published in [Using CQL with FHIR IG](https://build.fhir.org/ig/HL7/cql-ig/en/using-modelinfo.html)
   - Canonical reference: `http://hl7.org/fhir/uv/cql/Library/FHIR-ModelInfo|4.0.1`
   - Support derived ModelInfo for IG-specific profiles (US Core, QI-Core, etc.)
   - Load ModelInfo from FHIR packages via rh-foundation PackageLoader

3. **FHIRPath integration**: ✅ RESOLVED - Reuse rh-fhirpath
   - CQL path expressions are FHIRPath-based
   - Reuse `rh-fhirpath` crate directly for:
     - Path expression parsing (`FhirPathParser`)
     - Path evaluation (`FhirPathEvaluator`, `EvaluationContext`)
     - AST types (`Expression`, `Invocation`, etc.)
   - CQL-specific operators extend FHIRPath in the translator
   - Shared WASM build infrastructure

4. **Async support**: ✅ RESOLVED - Use rh-foundation patterns
   - Follow patterns from [Using CQL with FHIR IG](https://build.fhir.org/ig/HL7/cql-ig/en/index.html)
   - Reuse `rh-foundation::loader::PackageLoader` for library/model loading
   - PackageLoader already handles:
     - Async FHIR package download from npm registries
     - Authentication (bearer tokens)
     - Tarball extraction and caching
     - Configurable timeout/retry
   - Library resolution via FHIR Library resources in packages
   - ModelInfo loading from `Library/<model>-ModelInfo` convention

## References

### CQL/ELM Specifications
- [CQL Specification](https://cql.hl7.org/)
- [ELM Specification](https://cql.hl7.org/elm.html)
- [CQL Grammar](https://github.com/cqframework/clinical_quality_language/blob/master/Src/grammar/cql.g4)
- [CQL Conformance Tests](https://cql.hl7.org/tests.html)

### Reference Implementation
- [Java Translator](https://github.com/cqframework/clinical_quality_language/tree/master/Src/java/cql-to-elm)
- [ELM JSON Schema](https://github.com/cqframework/clinical_quality_language/tree/master/Src/java/elm/schema)

### FHIR Integration
- [Using CQL with FHIR IG](https://build.fhir.org/ig/HL7/cql-ig/en/index.html) - Canonical patterns for CQL+FHIR
- [Using ModelInfo](https://build.fhir.org/ig/HL7/cql-ig/en/using-modelinfo.html) - ModelInfo conformance requirements
- [CQF Common IG](https://www.fhir.org/guides/cqf/common/4.0.1/artifacts.html) - FHIR ModelInfo, FHIRHelpers, FHIRCommon
- [FHIR-ModelInfo](https://build.fhir.org/ig/HL7/cql-ig/en/Library-FHIR-ModelInfo.html) - Canonical FHIR R4 ModelInfo
- [FHIRHelpers](https://build.fhir.org/ig/HL7/cql-ig/en/Library-FHIRHelpers.html) - Type conversion functions

### Workspace Crates (Reuse)
- `rh-fhirpath` - FHIRPath parsing and evaluation
- `rh-foundation` - Package loading, HTTP, configuration, MemoryProvider pattern (WASM-compatible in-memory storage)
