# RH Workspace Refactoring Plan

## Overview

This document outlines a comprehensive refactoring plan for the RH workspace, focusing on consolidation opportunities to reduce duplication, improve maintainability, and establish clearer architectural boundaries.

**Current State:**
- 8 crates + 1 app (9 workspace members)
- 249,020 total LOC (185K generated FHIR types, 64K hand-written)
- Identified duplication: error handling, HTTP/IO utilities, WASM boilerplate, test infrastructure
- Empty rh-core crate (14 LOC stub) and minimal rh-common (83 LOC)

**Expected Impact:**
- 800-1,070 LOC reduction
- Significant complexity reduction through consolidation
- Improved developer experience with consistent patterns
- Better test infrastructure across all crates

---

## Priority 1: Merge rh-core + rh-common → rh-foundation

**Status:** Not Started  
**Effort:** Low (1-2 weeks)  
**Impact:** ~100 LOC reduction, 5 crates affected

### Current State
- `rh-core`: 14 LOC stub with empty add() function - essentially unused
- `rh-common`: 83 LOC with CommonError, Config trait, 2 utility functions
- 4 crates depend on rh-common: rh-loader, rh-codegen, rh-validator, rh-cli
- 0 crates depend on rh-core

### Goal
Create `rh-foundation` as the true foundation crate with clear module structure:
```
rh-foundation/
├── error/         # Error types and traits
├── config/        # Configuration traits
├── io/            # File operations
├── http/          # HTTP client utilities
├── json/          # JSON helpers
├── validation/    # Common validation logic
└── testing/       # Shared test utilities
```

### Steps
1. Create new `rh-foundation` crate
2. Migrate content from `rh-common` (83 LOC)
3. Add new utility modules (HTTP, IO, JSON, validation)
4. Update dependencies in 5 crates:
   - `rh-loader/Cargo.toml`: Replace `rh-common` with `rh-foundation`
   - `rh-codegen/Cargo.toml`: Replace `rh-common` with `rh-foundation`
   - `rh-validator/Cargo.toml`: Replace `rh-common` with `rh-foundation`
   - `rh-cli/Cargo.toml`: Replace `rh-common` with `rh-foundation`
   - Update import paths in ~20 files
5. Delete `rh-core` and `rh-common` crates
6. Update workspace `Cargo.toml`
7. Run tests: `cargo test --workspace --all-features`

### Benefits
- Eliminates confusion about which foundation crate to use
- Establishes clear architectural base for all other crates
- Provides logical home for shared utilities
- Better discoverability of common functionality

---

## Priority 2: Unified Error Handling

**Status:** Not Started  
**Effort:** Medium (1-2 weeks)  
**Impact:** ~150-200 LOC reduction, 8 crates affected

### Current State
**Identified Error Enums (9 total):**
1. `rh-vcl/src/error.rs`: VclError (19 variants, 145 LOC)
2. `rh-validator/src/error.rs`: ValidationError (8 variants)
3. `rh-validator/src/fhir_validation.rs`: FhirValidationError (6 variants) - **DUPLICATE!**
4. `rh-codegen/src/lib.rs`: CodegenError (6 variants)
5. `rh-fhirpath/src/evaluator/core/error.rs`: FhirPathError (30+ variants, 350+ LOC)
6. `rh-loader/src/lib.rs`: LoaderError (10 variants, 54 LOC)
7. `rh-common/src/lib.rs`: CommonError (4 variants)
8. Generated code errors (should not be refactored)

**Duplication Patterns:**
- Similar variants across enums: Io, Parse, InvalidInput, Http, Serde
- Each crate implements own error-to-string formatting
- No consistent error context/metadata patterns
- rh-validator has TWO error enums (ValidationError + FhirValidationError)

### Goal
Create unified error handling foundation with:
- `FoundationError` base enum in `rh-foundation/error.rs`
- Trait-based error extension for domain-specific errors
- Consistent error context and metadata handling
- Standard error conversion patterns

### Architecture
```rust
// rh-foundation/src/error.rs
pub enum FoundationError {
    Io(std::io::Error),
    Parse(String),
    Http(String),
    Serde(serde_json::Error),
    InvalidInput(String),
    // ... common variants
}

pub trait ErrorContext {
    fn add_context(&self, context: &str) -> Self;
    fn with_metadata(&self, key: &str, value: &str) -> Self;
}

// Domain-specific errors extend foundation
// rh-loader/src/error.rs
pub enum LoaderError {
    Foundation(FoundationError),
    PackageNotFound(String),
    InvalidManifest(String),
    // ... domain-specific variants
}
```

### Steps
1. Design `FoundationError` enum with common variants
2. Implement `ErrorContext` trait
3. Refactor each crate's error enum to extend foundation:
   - Start with `rh-loader` (simplest, 10 variants)
   - Then `rh-codegen` (6 variants, depends on loader)
   - Then `rh-validator` (merge 2 enums into 1, ~10 variants)
   - Then `rh-vcl` (19 variants, complex)
   - Finally `rh-fhirpath` (30+ variants, most complex)
4. Update error handling in ~20-30 files across crates
5. Add error handling examples to documentation
6. Run tests: `cargo test --workspace --all-features`

### Benefits
- Single source of truth for common error patterns
- Reduced boilerplate (150-200 LOC)
- Consistent error messages and formatting
- Easier error handling across crate boundaries
- Better error context for debugging

---

## Priority 3: HTTP & I/O Utilities Consolidation

**Status:** Not Started  
**Effort:** Medium (1 week)  
**Impact:** ~200-300 LOC reduction, 5 crates affected

### Current State
**HTTP Patterns Found:**
- `rh-loader/src/lib.rs`: ~75 LOC HTTP download logic (reqwest, timeout handling)
- `rh-codegen/download.rs`: Similar HTTP download patterns
- Both use `reqwest` but configure independently
- Timeout handling duplicated (30 seconds hardcoded in multiple places)

**I/O Patterns Found:**
- File reading/writing duplicated across:
  - `rh-loader`: Package manifest loading
  - `rh-codegen`: Structure definition reading/writing
  - `rh-validator`: Profile loading
- Path canonicalization patterns repeated
- JSON serialization/deserialization boilerplate

### Goal
Extract to `rh-foundation` modules:
- `rh-foundation/http.rs`: HttpClient with configuration
- `rh-foundation/io.rs`: FileOps trait with common operations
- `rh-foundation/json.rs`: JSON helpers

### Architecture
```rust
// rh-foundation/src/http.rs
pub struct HttpClient {
    client: reqwest::Client,
    timeout: Duration,
}

impl HttpClient {
    pub fn new() -> Result<Self>;
    pub fn with_timeout(timeout: Duration) -> Result<Self>;
    pub async fn download(&self, url: &str) -> Result<Vec<u8>>;
    pub async fn download_to_file(&self, url: &str, path: &Path) -> Result<()>;
}

// rh-foundation/src/io.rs
pub struct FileOps;

impl FileOps {
    pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T>;
    pub fn write_json<T: Serialize>(path: &Path, data: &T) -> Result<()>;
    pub fn ensure_dir(path: &Path) -> Result<()>;
    pub fn canonical_path(path: &Path) -> Result<PathBuf>;
}
```

### Steps
1. Implement `HttpClient` in `rh-foundation/http.rs`
   - Extract from `rh-loader` (use as reference)
   - Add configuration options (timeout, retries, headers)
2. Implement `FileOps` in `rh-foundation/io.rs`
   - Extract common patterns from loader, codegen, validator
   - Add JSON helpers
3. Migrate `rh-loader` to use `HttpClient` and `FileOps`
   - Remove ~75 LOC of HTTP code
   - Remove file I/O boilerplate
4. Migrate `rh-codegen` to use shared utilities
   - Update download.rs
   - Update file operations
5. Migrate `rh-validator` to use shared utilities
6. Update `rh-cli` if needed
7. Add unit tests for `HttpClient` and `FileOps`
8. Run tests: `cargo test --workspace --all-features`

### Benefits
- Single HTTP client configuration (200-300 LOC saved)
- Consistent timeout and retry logic
- Easier to add features (caching, rate limiting)
- Simplified testing with mockable HTTP layer
- Reduced dependency duplication

---

## Priority 4: WASM Boilerplate Consolidation

**Status:** Not Started  
**Effort:** Low-Medium (3-5 days)  
**Impact:** ~100-120 LOC reduction, 2 crates affected

### Current State
**WASM Crates:**
- `rh-fhirpath/src/wasm.rs`: 550+ LOC
- `rh-vcl/src/wasm.rs`: 200+ LOC

**Shared Patterns (100-120 LOC duplicated):**
- wasm-bindgen setup boilerplate
- console_error_panic_hook initialization
- JSON serialization/deserialization with serde_json
- Result type wrapping (String results for JS interop)
- Error message formatting for JavaScript

### Goal
Extract to `rh-foundation/wasm.rs`:
- Common WASM initialization
- Result wrapper types
- Error formatting utilities
- JSON conversion helpers

### Architecture
```rust
// rh-foundation/src/wasm.rs
#[cfg(target_arch = "wasm32")]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

pub struct WasmResult<T> {
    pub value: Option<T>,
    pub error: Option<String>,
}

impl<T> WasmResult<T> {
    pub fn to_json(&self) -> String where T: Serialize;
    pub fn from_result(result: Result<T, E>) -> Self where E: Display;
}
```

### Steps
1. Create `rh-foundation/wasm.rs` with common utilities
2. Add `wasm` feature flag to `rh-foundation/Cargo.toml`
3. Refactor `rh-fhirpath/src/wasm.rs`:
   - Use shared init, result types, error formatting
   - Keep domain-specific evaluation logic
4. Refactor `rh-vcl/src/wasm.rs`:
   - Use shared utilities
   - Keep VCL-specific parsing logic
5. Add WASM tests to `rh-foundation`
6. Run WASM builds: `cd crates/rh-fhirpath && just wasm-build`
7. Test WASM demos

### Benefits
- 100-120 LOC reduction
- Consistent WASM patterns across crates
- Easier to add new WASM bindings
- Single point for WASM-specific improvements

---

## Priority 5: CLI Framework Extraction

**Status:** Not Started  
**Effort:** Medium (1 week)  
**Impact:** ~150-200 LOC reduction, 1 app affected

### Current State
**CLI Structure:**
- `apps/rh-cli/src/main.rs`: 50 LOC (arg parsing setup)
- `apps/rh-cli/src/fhirpath.rs`: 400+ LOC (subcommands: eval, repl, test, compile)
- `apps/rh-cli/src/vcl.rs`: 150+ LOC (subcommands: parse, eval, convert)
- `apps/rh-cli/src/validator.rs`: 200+ LOC (validate command)
- `apps/rh-cli/src/codegen.rs`: 300+ LOC (generate command)
- `apps/rh-cli/src/download.rs`: 150+ LOC (download command)
- `apps/rh-cli/src/ffq.rs`: 100+ LOC (ffq command)

**Shared Patterns (150-200 LOC):**
- File reading boilerplate (repeated in every command)
- JSON parsing patterns
- Error formatting for CLI output
- Input/output handling
- REPL-style input loops (fhirpath.rs has full REPL)

### Goal
Extract to `rh-foundation/cli.rs`:
- Input/output utilities
- File handling helpers
- Error formatting for CLI
- REPL utilities (if generalizable)

### Architecture
```rust
// rh-foundation/src/cli.rs
pub struct CliInput;

impl CliInput {
    pub fn read_file_or_stdin(path: Option<&str>) -> Result<String>;
    pub fn read_json<T: DeserializeOwned>(path: &str) -> Result<T>;
}

pub struct CliOutput;

impl CliOutput {
    pub fn print_result<T: Display>(result: Result<T>);
    pub fn print_json<T: Serialize>(value: &T) -> Result<()>;
    pub fn format_error(error: &dyn Error) -> String;
}
```

### Steps
1. Analyze common patterns across all 7 CLI command files
2. Create `rh-foundation/cli.rs` with shared utilities
3. Refactor `rh-cli` commands to use shared utilities:
   - Start with `download.rs` (simplest)
   - Then `ffq.rs`
   - Then `codegen.rs`
   - Then `validator.rs`
   - Then `vcl.rs`
   - Finally `fhirpath.rs` (most complex with REPL)
4. Keep domain-specific logic in each command file
5. Add CLI utility tests
6. Run CLI tests: `cargo test -p rh-cli`
7. Manual testing of each command

### Benefits
- 150-200 LOC reduction
- Consistent CLI UX across commands
- Easier to add new commands
- Better error messages
- Potential for future REPL consolidation

---

## Priority 6: Test Utilities Consolidation

**Status:** Not Started  
**Effort:** Low-Medium (3-5 days)  
**Impact:** ~100-150 LOC reduction, improved test coverage

### Current State
**Test Coverage:**
- `rh-fhirpath`: 51 test files, 49 examples (excellent coverage)
- `rh-codegen`: 2 test files, 15 examples
- `rh-vcl`: 2 test files, 3 examples
- `rh-validator`: 1 test file
- Others: 0-1 test files

**Test Patterns Found:**
- JSON fixture loading repeated (rh-fhirpath, rh-validator)
- Test data creation boilerplate
- Assertion helpers duplicated
- Mock HTTP responses (rh-loader tests could use this)

### Goal
Create `rh-foundation/testing.rs` with:
- Fixture loading utilities
- Common test data builders
- Assertion helpers
- Mock utilities for HTTP and I/O

### Architecture
```rust
// rh-foundation/src/testing.rs
#[cfg(test)]
pub mod fixtures {
    pub fn load_json<T: DeserializeOwned>(name: &str) -> T;
    pub fn load_test_file(name: &str) -> String;
}

#[cfg(test)]
pub mod builders {
    pub struct TestDataBuilder<T> {
        // ... builder pattern
    }
}

#[cfg(test)]
pub mod mocks {
    pub struct MockHttpClient {
        // ... mock responses
    }
}
```

### Steps
1. Create `rh-foundation/testing.rs` module
2. Extract fixture loading from `rh-fhirpath` tests
3. Create test data builders based on common patterns
4. Implement mock HTTP client
5. Migrate existing tests to use shared utilities:
   - Start with `rh-fhirpath` (refactor existing tests)
   - Add tests to `rh-loader` using mock HTTP
   - Add tests to `rh-codegen`
   - Add tests to `rh-validator`
   - Add tests to `rh-vcl`
6. Document testing patterns
7. Run all tests: `cargo test --workspace --all-features`

### Benefits
- 100-150 LOC reduction in test code
- Improved test coverage across all crates
- Easier to write new tests
- Consistent testing patterns
- Better mock utilities for testing

---

## Implementation Phasing

### Phase 1: Foundation (Weeks 1-2)
**Focus:** Establish solid base
- ✅ Priority 1: Merge rh-core + rh-common → rh-foundation
- ✅ Priority 2: Unified Error Handling

**Rationale:** Must establish foundation before other consolidations. Error handling affects everything.

**Validation:**
- All existing tests pass: `cargo test --workspace --all-features`
- No clippy warnings: `cargo clippy --workspace --all-targets --all-features`
- Documentation builds: `cargo doc --workspace --no-deps`

### Phase 2: Utilities (Weeks 3-4)
**Focus:** Extract shared utilities
- ✅ Priority 3: HTTP & I/O Utilities Consolidation
- ✅ Priority 4: WASM Boilerplate Consolidation
- ✅ Priority 6: Test Utilities Consolidation

**Rationale:** Build on foundation. Can be done in parallel (different domains).

**Validation:**
- All tests pass including new utility tests
- WASM demos still work: Test both fhirpath and vcl demos
- HTTP operations work in loader and codegen

### Phase 3: Polish (Weeks 5-6)
**Focus:** CLI improvements and final touches
- ✅ Priority 5: CLI Framework Extraction
- ✅ Documentation updates
- ✅ Performance validation
- ✅ Migration guide

**Rationale:** CLI changes are user-facing. Do last to ensure foundation is solid.

**Validation:**
- Manual testing of all CLI commands
- Performance benchmarks (ensure no regression)
- Documentation review
- Final comprehensive test run

---

## Success Metrics

### Quantitative
- **LOC Reduction:** 800-1,070 lines removed (target: >700)
- **Test Coverage:** Increase from 51 tests (fhirpath only) to 100+ across all crates
- **Build Time:** Should not increase (may decrease slightly)
- **Crate Count:** Reduced from 9 to 8 (remove rh-core, rh-common; add rh-foundation)

### Qualitative
- **Developer Experience:** Easier to find and use common utilities
- **Code Quality:** More consistent patterns across crates
- **Maintainability:** Single source of truth for shared functionality
- **Testability:** Better mocking and test utilities

### Validation Checklist
- [ ] All 470+ tests passing
- [ ] Zero clippy warnings with `-D warnings`
- [ ] Code formatted: `cargo fmt --all -- --check`
- [ ] Documentation complete: `cargo doc --workspace --no-deps --open`
- [ ] WASM demos working (fhirpath and vcl)
- [ ] CLI commands manually tested
- [ ] No performance regressions
- [ ] Migration guide written

---

## Risk Assessment

### Low Risk
- **Priority 1 (Foundation merge):** Straightforward refactoring, well-understood scope
- **Priority 4 (WASM):** Isolated to 2 crates, good test coverage
- **Priority 6 (Test utilities):** Only affects test code

### Medium Risk
- **Priority 3 (HTTP/IO):** Touches multiple crates, need careful error handling
- **Priority 5 (CLI):** User-facing changes, needs thorough manual testing

### Higher Risk
- **Priority 2 (Error handling):** Touches ~20-30 files across 8 crates, complex dependencies

**Mitigation Strategy:**
1. Start with low-risk items to build confidence
2. Do higher-risk items in smaller increments
3. Run full test suite after each change
4. Keep git commits atomic and well-documented
5. Create feature branches for each priority
6. Get code review before merging higher-risk changes

---

## Notes

### What NOT to Refactor
- **rh-hl7_fhir_r4_core** (185K LOC): Generated code - do not touch
- Generated code in other crates: Leave as-is
- Core FHIRPath evaluation logic: Already well-structured
- Core VCL parsing logic: Already well-structured

### Dependencies to Add
**rh-foundation will need:**
- reqwest (HTTP client)
- tokio (async runtime)
- serde, serde_json (JSON handling)
- thiserror (error types)
- anyhow (error context)

**Optional (for testing):**
- mockito (HTTP mocking)
- tempfile (test fixtures)

### Post-Refactoring Opportunities
Once foundation is solid:
1. Consider extracting REPL framework (fhirpath has full REPL, could be generalized)
2. Consider caching layer for HTTP downloads
3. Consider plugin architecture for validators
4. Consider OpenAPI/CLI documentation generation

---

## Getting Started

**To begin Phase 1:**
```bash
# Create feature branch
git checkout -b refactor/phase1-foundation

# Create rh-foundation crate
cargo new --lib crates/rh-foundation

# Add to workspace Cargo.toml
# ... follow Priority 1 steps ...

# Run tests frequently
cargo test --workspace --all-features

# Check for issues
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Format code
cargo fmt --all
```

**Commit Message Format:**
```
refactor(foundation): merge rh-core and rh-common into rh-foundation

- Create new rh-foundation crate with modular structure
- Migrate 83 LOC from rh-common
- Update dependencies in 5 crates
- Delete rh-core and rh-common crates

BREAKING CHANGE: rh-common and rh-core no longer exist, use rh-foundation
```

---

## Questions for Discussion

Before starting implementation:

1. **Naming:** Is `rh-foundation` the right name? Alternatives: `rh-base`, `rh-shared`, `rh-utils`?
2. **Error Strategy:** Should we use `thiserror` for all errors or mix with `anyhow`?
3. **WASM Strategy:** Should foundation have WASM-specific code or keep separate?
4. **Testing:** What's the target test coverage percentage?
5. **Documentation:** Should we create a migration guide for external users?
6. **Versioning:** Will this be 0.2.0 or 1.0.0 given breaking changes?

---

**Last Updated:** 2025-10-23  
**Status:** Planning - Not Started  
**Next Action:** Review plan with team and answer discussion questions
