# Coding Standards

## Code Style & Conventions

- **Formatting**: Run `cargo fmt` before committing. 100 char line length.
- **Naming**: `snake_case` (functions/vars/modules), `PascalCase` (types/structs/enums/traits), `SCREAMING_SNAKE_CASE` (constants).
- **Error Handling**: Use `anyhow::Result<T>` for apps, `thiserror` for libraries. Never `.unwrap()` in production code - use `?` operator. Add context with `anyhow::Context`.
- **Imports**: Use workspace dependencies from root `Cargo.toml` when available. Check existing code for library usage before adding new deps.
- **Documentation**: Write `///` doc comments for all public APIs with examples. Use `cargo doc --open` to review.
- **Comments**: **NO code comments** unless explicitly requested. Focus on code clarity. Doc comments for public APIs only.
- **Async**: Use `tokio` runtime, prefer `async fn`, use `reqwest` for HTTP, handle timeouts explicitly.
- **Tests**: Place unit tests in `#[cfg(test)]` modules, integration tests in `tests/` dir. Use `#[tokio::test]` for async tests.

## Project-Specific Conventions

- **Commit Format**: Conventional commits with `<type>(<scope>): <description>`.
    - Types: feat/fix/docs/style/refactor/test/chore.
    - Scopes: codegen/cli/fhirpath/vcl/validator/loader/deps/ci/docs.
- **FHIR Types Mapping**:
    - `string` → `String`
    - `integer` → `i32`
    - `boolean` → `bool`
    - `decimal` → `f64`
    - `dateTime` → `String`
- **Generated Code**: Includes serde annotations, uses `Option<T>` for optional fields, `Vec<T>` for arrays.
