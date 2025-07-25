# Rust Workspace Configuration

This document explains the workspace configuration and organization.

## Workspace Structure

```
.
├── Cargo.toml              # Workspace root configuration
├── crates/                 # Library crates
│   └── common/            # Shared utilities
├── apps/                  # Executable applications
│   └── example-cli/       # Example CLI app
├── docs/                  # Documentation
├── tools/                 # Development tools
├── setup.sh              # Development setup script
├── build.sh              # Build script for CI/CD
└── README.md             # Project overview
```

## Workspace Benefits

1. **Unified Dependencies**: Shared dependencies across all crates
2. **Consistent Versioning**: Coordinated releases
3. **Cross-crate Testing**: Easy integration testing
4. **Simplified Build Process**: Single `cargo build` command

## Adding Dependencies

### Workspace Dependencies

Add common dependencies to the root `Cargo.toml`:

```toml
[workspace.dependencies]
serde = "1.0"
tokio = "1.0"
```

Then reference them in individual crates:

```toml
[dependencies]
serde = { workspace = true }
```

### Crate-specific Dependencies

Add directly to the crate's `Cargo.toml`:

```toml
[dependencies]
regex = "1.0"
```

## Development Workflow

1. **Setup**: Run `./setup.sh` once
2. **Development**: Work in individual crates
3. **Testing**: `cargo test` runs all tests
4. **Building**: `cargo build` builds everything
5. **Linting**: `cargo clippy` checks all code

## Release Management

The workspace supports coordinated releases of all crates or individual crate releases as needed.
