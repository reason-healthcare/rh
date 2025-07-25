# Copilot Instructions for FHIR Rust Monorepo

## Project Overview

This is a Rust monorepo focused on FHIR (Fast Healthcare Interoperability Resources) code generation and processing. The project includes:

- **Library crates** (`crates/`): Core functionality for FHIR processing and shared utilities
- **Applications** (`apps/`): CLI tools for FHIR code generation and processing
- **Documentation** (`docs/`): Project documentation and examples
- **Tools** (`tools/`): Development scripts and utilities

## Core Functionality

- **FHIR Code Generation**: Parse FHIR StructureDefinition JSON files and generate idiomatic Rust types
- **Package Management**: Download FHIR packages from npm-style registries (https://packages.fhir.org)
- **CLI Tools**: Command-line interfaces for batch processing and package management
- **Workspace Management**: Shared dependencies and coordinated development across crates

## Rust Conventions and Best Practices

### Code Style and Formatting

- **Use `cargo fmt`** before committing any code
- **Follow standard Rust naming conventions**:
  - `snake_case` for functions, variables, modules, and files
  - `PascalCase` for types, structs, enums, and traits
  - `SCREAMING_SNAKE_CASE` for constants and static variables
- **Use `cargo clippy`** to catch common issues and enforce best practices
- **Maximum line length**: 100 characters (configured in rustfmt.toml if present)

### Error Handling

- **Use `anyhow::Result<T>`** for application-level error handling
- **Use `thiserror`** for library-level custom error types
- **Avoid `.unwrap()`** in production code - use proper error propagation with `?`
- **Provide meaningful error messages** with context using `anyhow::Context`

Example:
```rust
use anyhow::{Context, Result};

fn load_structure_definition(path: &Path) -> Result<StructureDefinition> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    
    serde_json::from_str(&content)
        .with_context(|| "Failed to parse FHIR StructureDefinition JSON")
}
```

### Async Programming

- **Use `tokio`** as the async runtime (already configured in workspace)
- **Prefer `async fn`** over manual `Future` implementations
- **Use `reqwest`** for HTTP client operations
- **Handle timeouts explicitly** for network operations
- **Use structured concurrency** with `tokio::spawn` and `join!` macros

### Dependency Management

- **Use workspace dependencies** defined in root `Cargo.toml` when possible
- **Specify exact feature requirements** for dependencies
- **Avoid unnecessary dependencies** - prefer standard library when sufficient
- **Use `cargo-deny`** to check for security advisories and license compliance (if configured)

### Documentation

- **Write doc comments** (`///`) for all public APIs
- **Include examples** in doc comments using code blocks with `rust` annotation
- **Use `cargo doc --open`** to review generated documentation
- **Keep README.md updated** with current functionality and examples

Example:
```rust
/// Generate Rust types from a FHIR StructureDefinition.
///
/// This function parses a FHIR StructureDefinition JSON file and generates
/// corresponding Rust struct definitions with proper field types and serde annotations.
///
/// # Arguments
///
/// * `structure_def` - The FHIR StructureDefinition to process
///
/// # Returns
///
/// A `RustStruct` containing the generated type definition
///
/// # Example
///
/// ```rust
/// use codegen::{CodeGenerator, CodegenConfig};
///
/// let config = CodegenConfig::default();
/// let mut generator = CodeGenerator::new(config);
/// let structure_def = generator.load_structure_definition("patient.json")?;
/// let rust_struct = generator.generate_struct(&structure_def)?;
/// ```
pub fn generate_struct(&mut self, structure_def: &StructureDefinition) -> CodegenResult<RustStruct> {
    // Implementation...
}
```

## Testing Guidelines

### Test Organization

- **Unit tests**: Place in same file using `#[cfg(test)]` modules
- **Integration tests**: Place in `tests/` directory in each crate
- **Documentation tests**: Include in doc comments with `rust` code blocks
- **Use descriptive test names** that explain what is being tested

### Test Conventions

- **Use `#[tokio::test]`** for async tests
- **Use `tempfile`** crate for temporary files in tests
- **Mock external dependencies** using test doubles or feature flags
- **Test both success and failure paths**
- **Use `cargo test`** to run all tests
- **Use `cargo test --package <crate-name>`** for specific crate tests

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio_test;

    #[test]
    fn test_rust_name_conversion() {
        assert_eq!(to_rust_name("Patient"), "Patient");
        assert_eq!(to_rust_name("patient-name"), "PatientName");
        assert_eq!(to_rust_name("someField"), "SomeField");
    }

    #[tokio::test]
    async fn test_package_download() {
        let temp_dir = TempDir::new().unwrap();
        let config = PackageDownloadConfig::default();
        let downloader = PackageDownloader::new(config).unwrap();
        
        // Test implementation...
    }
}
```

### Test Coverage

- **Aim for high test coverage** especially for core business logic
- **Test error conditions** and edge cases
- **Use `cargo tarpaulin`** for coverage reports (if configured)
- **Don't test third-party dependencies** - focus on your logic

## Linting and Code Quality

### Clippy Configuration

Always run and address clippy warnings:
```bash
cargo clippy --all-targets --all-features
```

### Common Quality Checks

- **No `unwrap()` in production code** - use proper error handling
- **Avoid `clone()` unless necessary** - prefer borrowing
- **Use `const` for constant values**
- **Prefer iterators over manual loops** when appropriate
- **Use `match` exhaustively** - avoid catch-all arms when possible

### Performance Considerations

- **Profile before optimizing** using `cargo bench` or profiling tools
- **Prefer `&str` over `String`** for read-only string operations
- **Use `Vec::with_capacity()`** when size is known
- **Consider `Arc<T>` for shared immutable data**
- **Use `tokio::spawn`** for CPU-intensive tasks in async contexts

## FHIR-Specific Guidelines

### FHIR Type Mapping

- **Map FHIR primitives consistently**:
  - `string` → `String`
  - `integer` → `i32`
  - `boolean` → `bool`
  - `decimal` → `f64`
  - `dateTime` → `String` (consider `chrono::DateTime` for parsing)

### Code Generation

- **Generate idiomatic Rust code** with proper naming conventions
- **Include serde annotations** for JSON serialization/deserialization
- **Handle optional fields** using `Option<T>`
- **Handle arrays** using `Vec<T>`
- **Add documentation** to generated types when available

### Package Management

- **Follow npm package conventions** for FHIR package downloading
- **Handle network failures gracefully** with retries and timeouts
- **Validate package integrity** when possible (checksums, etc.)
- **Support authentication** for private registries

## Git Commit Guidelines

Always propose a commit message that follows the project's conventions. Use the following format:

### Commit Message Format

Use conventional commits format:
```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring without feature changes
- `test`: Adding or updating tests
- `chore`: Maintenance tasks, dependency updates

### Scopes

Use crate names or functional areas:
- `codegen`: FHIR code generation functionality
- `cli`: Command-line interface changes
- `common`: Shared utilities
- `core`: Core library functionality
- `docs`: Documentation updates
- `deps`: Dependency updates
- `ci`: CI/CD pipeline changes

### Examples

```
feat(codegen): add FHIR package downloading from npm registries

- Implement PackageDownloader with HTTP client for registry interaction
- Add download and install CLI commands with authentication support
- Support custom registries and bearer token authentication
- Extract tarballs and process StructureDefinition JSON files automatically
- Update documentation with package management examples

Closes #123
```

```
fix(cli): handle missing configuration file gracefully

When no codegen.json exists, use default configuration instead of failing.
Adds warning message to inform user about missing config.
```

```
test(codegen): add comprehensive tests for FHIR type mapping

- Test all primitive type conversions
- Add edge cases for optional and array fields
- Include integration tests for StructureDefinition parsing
```

## Task Completion Protocol

At the end of each development task, always:

1. **Run the full test suite**: `cargo test`
2. **Check for clippy warnings**: `cargo clippy --all-targets --all-features`
3. **Format the code**: `cargo fmt`
4. **Update documentation** if APIs changed
5. **Propose a git commit message** following the conventions above

### Example Task Completion

After implementing a new feature, provide:

```
## Summary
✅ Implemented FHIR package downloading capability
✅ Added CLI commands for download and install workflows  
✅ All tests passing (5 tests, 0 failures)
✅ No clippy warnings
✅ Documentation updated

## Proposed Commit Message
feat(codegen): add FHIR package downloading from npm registries

- Implement PackageDownloader with HTTP client for registry interaction
- Add download and install CLI commands with authentication support  
- Support custom registries and bearer token authentication
- Extract tarballs and process StructureDefinition JSON files automatically
- Update README and docs with package management examples
- All tests passing with comprehensive error handling

Resolves #feature-request-package-management
```

## Development Environment

### Required Tools

- **Rust toolchain**: Install via rustup (1.70+)
- **IDE extensions**: rust-analyzer for VS Code/other editors
- **Optional tools**: cargo-watch, cargo-deny, cargo-tarpaulin

### Workspace Commands

```bash
# Build entire workspace
cargo build

# Test entire workspace  
cargo test

# Run specific binary
cargo run --bin fhir-codegen -- --help

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy --all-targets --all-features

# Generate documentation
cargo doc --open
```

Remember: This project serves the healthcare interoperability community. Code quality, reliability, and clear documentation are essential for adoption and trust in healthcare environments.
