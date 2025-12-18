# Refactoring Tasks for rh-foundation

This document identifies meaningful refactoring opportunities in the `rh-foundation` crate to improve maintainability, readability, and adherence to the DRY principle.

---

## 1. Extract IO Error Context Helper

**File(s):** `src/io.rs`, `src/cli.rs`

**Description:** The pattern of wrapping `std::io::Error` with path context is repeated 5+ times across `io.rs` (lines 27-31, 74-79, 94-98) and `cli.rs` (lines 42-44, 76-78). Each creates a new `std::io::Error` with `format!("Failed to {action} {path}: {e}")`.

**Proposed Solution:** Create a helper function in `error.rs`:
```rust
pub fn io_error_with_path(err: std::io::Error, path: &Path, action: &str) -> FoundationError
```
Replace all manual error wrapping calls with this helper. Alternatively, implement a macro `io_context!($result, $path, $action)` that handles the `map_err` pattern inline.

---

## 2. Consolidate Duplicate Serde Error Variants

**File(s):** `src/error.rs`

**Description:** `FoundationError` has two variants for serde_json errors: `Serde(serde_json::Error)` at line 42 and `Serialization(#[from] serde_json::Error)` at line 45. This creates ambiguity and prevents the `#[from]` attribute from working correctly.

**Proposed Solution:** Remove the `Serde` variant entirely. Keep only `Serialization` with the `#[from]` attribute. Update any code that explicitly uses `FoundationError::Serde(...)` to use `FoundationError::Serialization(...)` or rely on the automatic conversion via `?`.

---

## 3. Unify JSON Serialization Logic

**File(s):** `src/json.rs`, `src/cli.rs`

**Description:** The `if pretty { to_string_pretty } else { to_string }` pattern appears in `json.rs` (lines 21-26, 41-45) and similar logic exists in `cli.rs` output formatting (lines 217-232).

**Proposed Solution:** Create a single generic function that handles both String and Vec<u8> serialization:
```rust
fn serialize_json<T: Serialize, W: JsonWriter>(value: &T, pretty: bool) -> Result<W>
```
Or use a trait-based approach with a `JsonFormat` enum to centralize the pretty/compact decision.

---

## 4. Implement Display for ElementPath

**File(s):** `src/snapshot/path.rs`

**Description:** `ElementPath` has an `as_str()` method but lacks a `Display` implementation. This forces callers to use `.as_str()` instead of direct formatting with `{}`.

**Proposed Solution:** Add `Display` implementation:
```rust
impl std::fmt::Display for ElementPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.original)
    }
}
```

---

## 5. Fix Potential Panic in normalize_choice_type

**File(s):** `src/snapshot/path.rs`

**Description:** Line 85 uses `.chars().nth(0).unwrap()` which will panic on empty strings. This is a safety issue.

**Proposed Solution:** Replace with safe pattern:
```rust
if last.chars().next().map_or(false, |c| c.is_lowercase()) {
```
Consider extracting a helper function `fn is_lowercase_start(s: &str) -> bool` for clarity.

---

## 6. Reduce Cloning in expand_slice_children

**File(s):** `src/snapshot/merger.rs`

**Description:** The `expand_slice_children` function (lines 63-96) clones entire `ElementDefinition` objects multiple times (lines 69, 84). `ElementDefinition` contains multiple `Vec` and `Option` fields, making deep clones expensive.

**Proposed Solution:** Refactor to collect only the keys needed for iteration first, then operate on references where possible. Use indices or keys to avoid cloning the full elements during the filtering phase. Only clone when inserting new elements into the map.

---

## 7. Decompose merge_elements Function

**File(s):** `src/snapshot/merger.rs`

**Description:** `merge_elements` (lines 15-55) handles four distinct responsibilities: creating the base map, applying differential elements, expanding slice children, and sorting results. This violates single-responsibility principle.

**Proposed Solution:** Extract into smaller functions:
```rust
fn create_element_map(base: &[ElementDefinition]) -> HashMap<...>
fn apply_differential(map: &mut HashMap<...>, differential: &[ElementDefinition]) -> Result<()>
fn sort_elements_by_path(elements: &mut Vec<ElementDefinition>)
```

---

## 8. Consolidate Input Reading Functions

**File(s):** `src/cli.rs`

**Description:** `read_input` (lines 37-52) and `read_input_from_path` (lines 68-83) have nearly identical logic for reading from file or stdin, differing only in how the file path is accepted (`&str` vs `impl AsRef<Path>`).

**Proposed Solution:** Consolidate into a single function with a generic path parameter:
```rust
pub fn read_input<P: AsRef<Path>>(file: Option<P>, inline: Option<String>) -> Result<String>
```
Remove the duplicate function.

---

## 9. Use Arc for Snapshot Cache

**File(s):** `src/snapshot/generator.rs`

**Description:** The snapshot cache (line 23) stores `Snapshot` values directly, requiring deep clones when retrieving cached values (lines 59, 81). `Snapshot` contains nested structures that are expensive to clone.

**Proposed Solution:** Change cache type to:
```rust
snapshot_cache: RefCell<HashMap<String, Arc<Snapshot>>>
```
Return `Arc<Snapshot>` from cache lookups to share ownership without cloning.

---

## 10. Extract HTTP Retry Logic

**File(s):** `src/loader.rs`

**Description:** The retry loop for HTTP downloads (lines 320-345) is embedded in `download_tarball`. This pattern could be reused elsewhere and clutters the download logic.

**Proposed Solution:** Extract a generic retry helper in `http.rs`:
```rust
pub async fn with_retry<T, F, Fut>(
    operation: F,
    max_retries: u32,
    delay_ms: u64,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
```
Use this helper in `download_tarball` and make it available for other HTTP operations.

---

## 11. Reduce ElementPath Clone Operations

**File(s):** `src/snapshot/path.rs`

**Description:** Multiple methods clone the `parts` vector unnecessarily: `parent()` (line 56), `normalize_choice_type()` (line 83), `base_path()` (line 117). These create new allocations on each call.

**Proposed Solution:** For `parent()`, return an iterator or slice view instead of allocating a new vector. For `normalize_choice_type()`, avoid the string round-trip by constructing the new `ElementPath` directly from parts. Consider using `Cow<str>` for parts that rarely need modification.

---

## 12. Create Field Merge Macro for ElementDefinition

**File(s):** `src/snapshot/merger.rs`

**Description:** The `merge_element` function (lines 98-143) has repetitive patterns like `diff.field.clone().or_else(|| base.field.clone())` for 15+ fields.

**Proposed Solution:** Create a macro to reduce boilerplate:
```rust
macro_rules! merge_optional {
    ($diff:expr, $base:expr, $field:ident) => {
        $diff.$field.clone().or_else(|| $base.$field.clone())
    };
}
```
Apply to all optional field merges in `merge_element`.

---

## 13. Remove Redundant Default Implementations

**File(s):** `src/http.rs`, `src/snapshot/generator.rs`

**Description:** `HttpClientBuilder` (line 106) and `SnapshotGenerator` (line 124) have explicit `Default` implementations that simply call `Self::new()`. This is boilerplate.

**Proposed Solution:** Either derive `Default` if `new()` matches what derive would produce, or document why the explicit impl is needed. For `HttpClientBuilder`, the builder pattern with `new()` returning default values could potentially use `#[derive(Default)]` if field defaults match.

---

## 14. Reconsider WasmResult Default Implementation

**File(s):** `src/wasm.rs`

**Description:** `WasmResult::default()` (line 103) returns an error state ("No result"). This is semantically surprising as `Default` typically implies a valid/empty state.

**Proposed Solution:** Either remove the `Default` implementation entirely (forcing explicit construction), or change the default to represent an empty/uninitialized success state. Document the rationale if keeping the error default.

---

## Priority Summary

| Task | Severity | Type | Impact |
|------|----------|------|--------|
| 6. Reduce Cloning in expand_slice_children | HIGH | Performance | Major clone cost reduction |
| 7. Decompose merge_elements | HIGH | Complexity | Better maintainability |
| 1. Extract IO Error Context Helper | HIGH | Duplication | ~20 lines saved |
| 2. Consolidate Serde Error Variants | MEDIUM | Rust idiom | Cleaner error handling |
| 5. Fix Panic in normalize_choice_type | MEDIUM | Safety | Prevents runtime panic |
| 4. Implement Display for ElementPath | MEDIUM | Rust idiom | Better ergonomics |
| 9. Use Arc for Snapshot Cache | MEDIUM | Performance | Reduces deep clones |
| 8. Consolidate Input Reading Functions | MEDIUM | Duplication | ~15 lines saved |
| 3. Unify JSON Serialization Logic | MEDIUM | Duplication | ~8 lines saved |
| 10. Extract HTTP Retry Logic | MEDIUM | Duplication | Reusable pattern |
| 11. Reduce ElementPath Clone Operations | MEDIUM | Performance | Allocation reduction |
| 12. Create Field Merge Macro | LOW | Duplication | Cleaner code |
| 13. Remove Redundant Default Impls | LOW | Boilerplate | Style consistency |
| 14. Reconsider WasmResult Default | LOW | Semantics | API clarity |
