# Agent Instructions for RH Repository

## Build/Test/Lint Commands
- **Build**: `cargo build` (or `just build`)
- **Test all**: `cargo test --workspace --all-features` (or `just test`)
- **Test single package**: `cargo test -p <package-name>` (e.g., `cargo test -p rh-fhirpath`)
- **Test single test**: `cargo test -p <package-name> <test-name>` (e.g., `cargo test -p rh-codegen test_rust_name_conversion`)
- **Lint**: `cargo clippy --workspace --all-targets --all-features -- -D warnings` (or `just lint`)
- **Format**: `cargo fmt --all` (or `just fmt`)
- **Format check**: `cargo fmt --all -- --check` (or `just fmt-check`)
- **Full check**: `just check` (runs fmt-check, lint, test, audit)

## Code Style
- **Formatting**: Run `cargo fmt` before committing. 100 char line length.
- **Linting**: Always run `cargo clippy` and fix warnings. Use `-D warnings` to treat as errors.
- **Naming**: `snake_case` (functions/vars/modules), `PascalCase` (types/structs/enums/traits), `SCREAMING_SNAKE_CASE` (constants)
- **Error handling**: Use `anyhow::Result<T>` for apps, `thiserror` for libraries. Never `.unwrap()` in production code - use `?` operator. Add context with `anyhow::Context`.
- **Imports**: Use workspace dependencies from root `Cargo.toml` when available. Check existing code for library usage before adding new deps.
- **Documentation**: Write `///` doc comments for all public APIs with examples. Use `cargo doc --open` to review.
- **Tests**: Place unit tests in `#[cfg(test)]` modules, integration tests in `tests/` dir. Use `#[tokio::test]` for async tests.
- **NO comments**: Do not add code comments unless explicitly requested. Doc comments for public APIs only.
- **Async**: Use `tokio` runtime, prefer `async fn`, use `reqwest` for HTTP, handle timeouts explicitly.

## Project-Specific Conventions
- **Commit format**: Conventional commits with `<type>(<scope>): <description>`. Types: feat/fix/docs/style/refactor/test/chore. Scopes: codegen/cli/fhirpath/vcl/validator/loader/deps/ci/docs.
- **FHIR types**: Map FHIR primitives consistently: `string`→`String`, `integer`→`i32`, `boolean`→`bool`, `decimal`→`f64`, `dateTime`→`String`
- **Generated code**: Includes serde annotations, uses `Option<T>` for optional fields, `Vec<T>` for arrays

## Task Completion
After completing tasks, always run: `cargo test`, `cargo clippy --all-targets --all-features`, `cargo fmt`
