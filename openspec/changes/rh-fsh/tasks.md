## 1. Crate + Workspace Setup

- [ ] 1.1 Add `crates/rh-fsh/` directory with `Cargo.toml` (deps: nom, thiserror, serde, serde_json, indexmap, rayon, rh-foundation, rh-hl7_fhir_r4_core)
- [ ] 1.2 Add `rayon = "1.10"` to root workspace `Cargo.toml` dependencies
- [ ] 1.3 Create `src/lib.rs` with public re-exports and stub `FshCompiler`, `CompilerOptions`, `FhirPackage`, `compile_fsh()`, `compile_fsh_files()`
- [ ] 1.4 Create `src/error.rs` with `FshError` enum (Parse, DuplicateEntity, UnresolvedAlias, RuleSetCycle, UnknownParent, Export, Foundation)

## 2. Parser Foundation

- [ ] 2.1 Copy and adapt `Span<'a>`, `SourceLocation`, `SourceRange` from `rh-cql` into `src/parser/span.rs`
- [ ] 2.2 Create `src/parser/lexer.rs`: FSH keyword constants (`KW_PROFILE`, `KW_EXTENSION`, etc.), `ws`, `line_comment`, `block_comment`, `identifier`, `quoted_string`, `multiline_string`, `integer`, `decimal`, `cardinal`, `fsh_path`, `coded_value`, `quantity_value` helpers
- [ ] 2.3 Create `src/parser/mod.rs` with `FshParser::parse(input, source_name) -> Result<FshDocument, FshError>` entry point

## 3. AST Types

- [ ] 3.1 Create `src/parser/ast.rs`: `FshDocument`, `Spanned<T>`, `FshEntity` enum with all 11 variants
- [ ] 3.2 Define structural entity types: `Profile`, `Extension`, `Logical`, `Resource` (shared metadata shape + `Vec<Spanned<SdRule>>`)
- [ ] 3.3 Define `Instance`, `ValueSet`, `CodeSystem`, `Invariant`, `Mapping` entity types
- [ ] 3.4 Define `RuleSet` and `ParamRuleSet` entity types (with parameter name list for ParamRuleSet)
- [ ] 3.5 Define `SdRule` enum (Card, Flag, Binding, Assignment, Contains, Only, Obeys, CaretValue, Insert, AddElement, Path) with typed rule structs
- [ ] 3.6 Define `FshValue` enum (Str, Bool, Integer, Decimal, Code, Quantity, Ratio, Reference, Canonical)
- [ ] 3.7 Define `FshPath`, `FshPathSegment` (Name, Index, Slice, ChoiceType, Extension)
- [ ] 3.8 Define remaining rule types: `ConceptRule`, `ValueSetComponentRule`, `MappingRule`

## 4. Entity Parsers

- [ ] 4.1 Create `src/parser/entity.rs` with `parse_document` and `parse_entity` entry points
- [ ] 4.2 Implement `parse_profile`, `parse_extension`, `parse_logical`, `parse_resource` (shared metadata + SD rules loop)
- [ ] 4.3 Implement `parse_instance` (InstanceOf, Usage metadata + instance rules loop)
- [ ] 4.4 Implement `parse_value_set` (metadata + VS component rules loop)
- [ ] 4.5 Implement `parse_code_system` (metadata + concept rules loop)
- [ ] 4.6 Implement `parse_invariant` (metadata only: Description, Expression, Severity, XPath)
- [ ] 4.7 Implement `parse_rule_set` and `parse_param_rule_set` (parameter list parsing for ParamRuleSet)
- [ ] 4.8 Implement `parse_mapping` (Source, Target metadata + mapping rules loop)
- [ ] 4.9 Implement `parse_alias`
- [ ] 4.10 Write unit tests covering all 11 entity types (parse round-trip assertions)

## 5. Rule Parsers

- [ ] 5.1 Create `src/parser/rules.rs` with `parse_sd_rule` dispatcher
- [ ] 5.2 Implement `parse_card_rule` (path + cardinality + optional flags)
- [ ] 5.3 Implement `parse_flag_rule` (path + one or more flags: MS, SU, ?!, N, TU, D)
- [ ] 5.4 Implement `parse_binding_rule` (path + `from` + VS reference + optional strength)
- [ ] 5.5 Implement `parse_assignment_rule` (path + `=` + FshValue; handle `(exactly)` modifier)
- [ ] 5.6 Implement `parse_contains_rule` (path + `contains` + named items + cardinalities)
- [ ] 5.7 Implement `parse_only_rule` (path + `only` + type list)
- [ ] 5.8 Implement `parse_obeys_rule` (path + `obeys` + invariant refs)
- [ ] 5.9 Implement `parse_caret_value_rule` (path + caret path + `=` + value)
- [ ] 5.10 Implement `parse_insert_rule` (`insert` + RuleSet name + optional params)
- [ ] 5.11 Implement `parse_add_element_rule` (path + cardinality + type(s) + short description)
- [ ] 5.12 Implement `parse_path_rule` (path only — sets context path)
- [ ] 5.13 Implement `parse_concept_rule` and `parse_value_set_component_rule`, `parse_mapping_rule`
- [ ] 5.14 Write unit tests for all rule types including complex values (Ratio, Reference, Canonical)

## 6. FshTank

- [ ] 6.1 Create `src/tank/mod.rs` with `FshTank` struct (IndexMap per entity type + aliases HashMap)
- [ ] 6.2 Implement `FshTank::add_document(&mut self, doc: FshDocument) -> Result<(), Vec<FshError>>` with duplicate-name detection
- [ ] 6.3 Implement `FshTank::fish(&self, name: &str) -> Option<FshEntityRef<'_>>` searching all collections
- [ ] 6.4 Implement `FshTank::all_entities(&self) -> impl Iterator<Item = FshEntityRef<'_>>`
- [ ] 6.5 Write unit tests for tank indexing, fish lookup, and duplicate detection

## 7. Resolver

- [ ] 7.1 Create `src/resolver/mod.rs` with `FshResolver::resolve(tank: &mut FshTank) -> Result<(), Vec<FshError>>`
- [ ] 7.2 Implement alias expansion pass: replace alias names with canonical URLs across all rule values
- [ ] 7.3 Implement RuleSet dependency graph builder and topological sort
- [ ] 7.4 Implement RuleSet inlining pass with cycle detection → `FshError::RuleSetCycle`
- [ ] 7.5 Implement ParamRuleSet expansion pass (parameter substitution then inline)
- [ ] 7.6 Write unit tests: simple insert, nested inserts, cycle detection, alias substitution, param substitution

## 8. FhirDefs

- [ ] 8.1 Create `src/fhirdefs/mod.rs` with `FhirDefs`, `SdSummary`, `ElementSummary`, `BindingSummary`, `SlicingSummary` types
- [ ] 8.2 Implement `FhirDefs::r4()` loading all base StructureDefinitions from `rh-hl7_fhir_r4_core` and building the element `HashMap`
- [ ] 8.3 Implement `FhirDefs::get_sd(name_or_url)` and `FhirDefs::get_element(sd, path)` lookups
- [ ] 8.4 Write unit tests confirming O(1) element lookup for representative R4 resources (Observation, Patient, Condition)

## 9. ValueSet + CodeSystem Exporters

- [ ] 9.1 Create `src/export/mod.rs` with `FshExporter` struct, `FhirPackage` output type, and parallel dispatch skeleton
- [ ] 9.2 Implement `src/export/value_set.rs`: compose.include/exclude from component rules
- [ ] 9.3 Implement `src/export/code_system.rs`: concept hierarchy from ConceptRules, count/content fields
- [ ] 9.4 Write unit tests using FSH input → expected FHIR JSON golden output

## 10. Instance Exporter

- [ ] 10.1 Implement `src/export/instance.rs`: walk assignment rules, build nested JSON via path segments
- [ ] 10.2 Handle contained resources, extension URLs, `ofType()` choice slices
- [ ] 10.3 Wire `instanceOf` SD lookup via `FhirDefs` for resource type resolution
- [ ] 10.4 Write unit tests: simple assignments, nested paths, extension assignments

## 11. StructureDefinition Exporter

- [ ] 11.1 Implement `src/export/structure_def.rs`: load parent SD via `FhirDefs`, derive metadata
- [ ] 11.2 Build `differential.element[]` from rules in declaration order: CardRule, FlagRule, BindingRule, AssignmentRule (fixed/pattern), OnlyRule
- [ ] 11.3 Handle ContainsRule → slicing discriminator + slice element entries
- [ ] 11.4 Handle CaretValueRule → caret-path property injection on element or SD root
- [ ] 11.5 Handle AddElementRule → new element entries in differential
- [ ] 11.6 Cache derived SDs for use by Instance exporter
- [ ] 11.7 Write unit tests: basic Profile → differential, Extension structure, slicing, caret values

## 12. Mapping Exporter

- [ ] 12.1 Implement `src/export/mapping.rs`: Mapping → FHIR ConceptMap JSON with group/element structure from MappingRules
- [ ] 12.2 Write unit tests

## 13. Parallel Export Wiring

- [ ] 13.1 Wire `rayon::par_iter()` in `FshExporter::export()` across all entity type collections
- [ ] 13.2 Ensure `FhirDefs` is wrapped in `Arc` and cloned into parallel closures
- [ ] 13.3 Verify thread-safety: confirm all shared state is `Arc<T>` or `Arc<RwLock<T>>`
- [ ] 13.4 Add `benches/compile.rs` with criterion benchmarks: small (10 entities), medium (100 entities), large (1000 entities) FSH inputs

## 14. CLI Integration

- [ ] 14.1 Create `apps/rh-cli/src/fsh.rs` with `rh fsh compile`, `rh fsh parse`, `rh fsh tank` subcommands using clap
- [ ] 14.2 Wire `fsh` subcommand into `apps/rh-cli/src/main.rs`
- [ ] 14.3 Add `rh-fsh` as a dependency in `apps/rh-cli/Cargo.toml`
- [ ] 14.4 Test compile command: single file, multiple files, error output format
