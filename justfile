# Justfile for common development tasks
# Install just: https://github.com/casey/just

# Default recipe - show available commands
default:
    @just --list

# Initial workspace setup (install tools, run checks)
setup:
    ./setup.sh

# Run all tests (skipping doctests due to incremental compilation issues)
test:
    cargo test --workspace --all-features --lib --bins --tests

# Run tests with nextest if available
test-fast:
    #!/usr/bin/env bash
    if command -v cargo-nextest &> /dev/null; then
        cargo nextest run --all-features
    else
        cargo test --workspace --all-features
    fi

# Test that all examples compile and run
test-examples:
    @echo "Building all examples..."
    cargo build --examples --workspace
    # Validator examples not yet created (Phase 8 - TODO)
    # cargo run -p rh-validator --example basic_validation > /dev/null
    @echo "✅ All examples compiled successfully!"

# Run the dev-only FHIRPath R5 XML testlab harness through the rh CLI
test-fhirpath-r5-testlab:
    python3 crates/rh-fhirpath/testlab/run_r5_testlab.py --no-baseline-check

# Run FHIR validation test cases (quick - 5 tests)
# For more options, see: just --justfile crates/rh-validator/justfile --list
# Or run from validator dir: cd crates/rh-validator && just --list
test-fhir:
    #!/usr/bin/env bash
    echo "╔════════════════════════════════════════════════════════════════════╗"
    echo "║ FHIR Validation Test Suite - Quick Test (5 cases)                 ║"
    echo "╚════════════════════════════════════════════════════════════════════╝"
    echo ""
    cargo test --features fhir-test-cases -p rh-validator fhir_test_cases::runner::tests::test_runner_basic -- --nocapture

# Run FHIR validation tests for a specific module
# Usage: just test-fhir-module general
#        just test-fhir-module profile
#        just test-fhir-module questionnaire
test-fhir-module module:
    #!/usr/bin/env bash
    echo "╔════════════════════════════════════════════════════════════════════╗"
    echo "║ FHIR Validation Test Suite - Module: {{module}}                   ║"
    echo "╚════════════════════════════════════════════════════════════════════╝"
    echo ""
    cargo test --features fhir-test-cases -p rh-validator fhir_test_cases::runner::tests::test_runner_with_module_filter -- --nocapture

# Run FHIR validation test suite with 50 tests
test-fhir-50:
    #!/usr/bin/env bash
    echo "╔════════════════════════════════════════════════════════════════════╗"
    echo "║ FHIR Validation Test Suite - Extended (50 tests)                  ║"
    echo "╚════════════════════════════════════════════════════════════════════╝"
    echo ""
    cargo test --features fhir-test-cases -p rh-validator fhir_test_cases::runner::tests::test_runner_extended_50 -- --nocapture --ignored

# Run FHIR validation test suite with 100 tests
test-fhir-100:
    #!/usr/bin/env bash
    echo "╔════════════════════════════════════════════════════════════════════╗"
    echo "║ FHIR Validation Test Suite - Extended (100 tests)                 ║"
    echo "╚════════════════════════════════════════════════════════════════════╝"
    echo ""
    cargo test --features fhir-test-cases -p rh-validator fhir_test_cases::runner::tests::test_runner_extended_100 -- --nocapture --ignored

# Run ALL FHIR validation test cases (~570 R4 tests - takes several minutes)
test-fhir-all:
    #!/usr/bin/env bash
    echo "╔════════════════════════════════════════════════════════════════════╗"
    echo "║ FHIR Validation Test Suite - ALL R4 TESTS (~570 tests)            ║"
    echo "╚════════════════════════════════════════════════════════════════════╝"
    echo ""
    echo "⚠️  This will run ALL R4 test cases and may take several minutes..."
    echo ""
    cargo test --features fhir-test-cases -p rh-validator fhir_test_cases::runner::tests::test_runner_all -- --nocapture --ignored --test-threads=1

# Build all packages
build:
    cargo build --workspace --all-targets --all-features

# Check code formatting
fmt-check:
    cargo fmt --all -- --check

# Format all code
fmt:
    cargo fmt --all

# Run clippy lints
lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run security audit
audit:
    cargo audit || (echo "cargo audit failed; retrying once..." >&2; sleep 2; cargo audit)

# Clean build artifacts
clean:
    cargo clean

# Check everything (format, lint, test, audit, examples, conformance harnesses)
check: fmt-check lint test test-examples test-fhirpath-r5-testlab audit

# Watch for changes and run tests
watch:
    cargo watch -x test

# Watch for changes and run clippy
watch-lint:
    cargo watch -x clippy

# Create a new library crate
new-lib name:
    cd crates && cargo new {{name}} --lib
    @echo "Don't forget to update the Cargo.toml to use workspace dependencies!"

# Create a new binary crate
new-bin name:
    cd apps && cargo new {{name}} --bin
    @echo "Don't forget to update the Cargo.toml to use workspace dependencies!"

# Install rh CLI to ~/.cargo/bin
install:
    cargo install --path apps/rh-cli

# Build release versions
build-release:
    cargo build --release --all-targets --all-features

# Run the example CLI
run-example *args:
    cargo run -p example-cli -- {{args}}

# Generate documentation
docs:
    cargo doc --all-features --no-deps --open

# Check machine-verifiable documentation sections against current workspace facts.
docs-sync:
    scripts/check-docs-sync.sh

# Update all dependencies
update:
    cargo update

# Show current versions for all crates
show-versions:
    @python3 scripts/bump-version show

# Bump all crates to the same version (default release workflow)
# Usage: just bump-version 0.2.0-beta.1
bump-version new-version:
    python3 scripts/bump-version {{new-version}}

# Compute the Nix source and Cargo vendor hashes for an immutable release tag.
# Usage: just nix-release-hashes v0.2.1
nix-release-hashes tag:
    scripts/nix-release-hashes {{tag}}

# [exceptional] Bump only the shared workspace version, leaving rh-validator unchanged
# Use when releasing workspace crates independently of rh-validator
# Usage: just bump-workspace-version 0.2.0-beta.1
bump-workspace-version new-version:
    python3 scripts/bump-version workspace {{new-version}}

# [exceptional] Bump rh-validator's standalone version only, leaving workspace crates unchanged
# Use when releasing rh-validator independently of the rest of the workspace
# Usage: just bump-validator-version 0.3.0-beta.1
bump-validator-version new-version:
    python3 scripts/bump-version validator {{new-version}}

# Regenerate R4 crate from hl7.fhir.r4.core@4.0.1
# Preserves tests/ directory. Requires network for first download; uses cache thereafter.
regen-r4:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Regenerating R4 crate (hl7.fhir.r4.core 4.0.1)..."
    OUTPUT="crates/rh-hl7_fhir_r4_core"
    # Preserve hand-authored test files
    TEST_BACKUP=$(mktemp -d)
    if [ -d "$OUTPUT/tests" ]; then
        cp -r "$OUTPUT/tests"/* "$TEST_BACKUP/" 2>/dev/null || true
    fi
    # Remove generated src/ content (Cargo.toml left intact so workspace resolves)
    rm -rf "$OUTPUT/src"
    rm -f "$OUTPUT/README.md"
    # Ensure workspace can resolve: need minimal Cargo.toml + src/lib.rs
    mkdir -p "$OUTPUT/src"
    if [ ! -f "$OUTPUT/Cargo.toml" ]; then
        printf '[package]\nname = "rh-hl7-fhir-r4-core"\nversion = "0.2.0"\nedition = "2021"\n' > "$OUTPUT/Cargo.toml"
    fi
    cat > "$OUTPUT/src/lib.rs" <<'EOF'
    pub mod metadata;
    EOF
    cat > "$OUTPUT/src/metadata.rs" <<'EOF'
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FhirPrimitiveType {
        Boolean, Integer, String, Date, DateTime, Instant, Time, Decimal, Uri, Url, Canonical,
        Code, Oid, Id, Markdown, Base64Binary, UnsignedInt, PositiveInt,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum FhirFieldType {
        Primitive(FhirPrimitiveType),
        Complex(&'static str),
        Reference,
        BackboneElement(&'static str),
    }
    #[derive(Debug, Clone)]
    pub struct FieldInfo {
        pub field_type: FhirFieldType,
        pub min: u32,
        pub max: Option<u32>,
        pub is_choice_type: bool,
        pub choice_types: &'static [&'static str],
    }
    pub fn get_field_info(_type_name: &str, _field_name: &str) -> Option<&'static FieldInfo> {
        None
    }
    EOF
    # Regenerate
    cargo run -p rh-cli -- codegen hl7.fhir.r4.core 4.0.1 \
        --output "$OUTPUT" \
        --crate-name rh-hl7-fhir-r4-core \
        --force
    # Restore tests
    if [ -d "$TEST_BACKUP" ] && [ "$(ls -A "$TEST_BACKUP" 2>/dev/null)" ]; then
        mkdir -p "$OUTPUT/tests"
        cp -r "$TEST_BACKUP"/* "$OUTPUT/tests/"
    fi
    rm -rf "$TEST_BACKUP"
    echo "Building regenerated R4 crate..."
    cargo build -p rh-hl7-fhir-r4-core
    echo "Testing regenerated R4 crate..."
    cargo test -p rh-hl7-fhir-r4-core
    echo "R4 regeneration complete."

# Regenerate R5 crate from hl7.fhir.r5.core@5.0.0
# Preserves tests/ directory. Requires network for first download; uses cache thereafter.
regen-r5:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Regenerating R5 crate (hl7.fhir.r5.core 5.0.0)..."
    OUTPUT="crates/rh-hl7_fhir_r5_core"
    # Preserve hand-authored test files
    TEST_BACKUP=$(mktemp -d)
    if [ -d "$OUTPUT/tests" ]; then
        cp -r "$OUTPUT/tests"/* "$TEST_BACKUP/" 2>/dev/null || true
    fi
    # Remove generated src/ content (Cargo.toml left intact so workspace resolves)
    rm -rf "$OUTPUT/src"
    rm -f "$OUTPUT/README.md"
    # Ensure workspace can resolve: need minimal Cargo.toml + src/lib.rs
    mkdir -p "$OUTPUT/src"
    if [ ! -f "$OUTPUT/Cargo.toml" ]; then
        printf '[package]\nname = "rh-hl7-fhir-r5-core"\nversion = "0.2.0"\nedition = "2021"\n' > "$OUTPUT/Cargo.toml"
    fi
    cat > "$OUTPUT/src/lib.rs" <<'EOF'
    pub mod metadata;
    EOF
    cat > "$OUTPUT/src/metadata.rs" <<'EOF'
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FhirPrimitiveType {
        Boolean, Integer, String, Date, DateTime, Instant, Time, Decimal, Uri, Url, Canonical,
        Code, Oid, Id, Markdown, Base64Binary, UnsignedInt, PositiveInt,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum FhirFieldType {
        Primitive(FhirPrimitiveType),
        Complex(&'static str),
        Reference,
        BackboneElement(&'static str),
    }
    #[derive(Debug, Clone)]
    pub struct FieldInfo {
        pub field_type: FhirFieldType,
        pub min: u32,
        pub max: Option<u32>,
        pub is_choice_type: bool,
        pub choice_types: &'static [&'static str],
    }
    pub fn get_field_info(_type_name: &str, _field_name: &str) -> Option<&'static FieldInfo> {
        None
    }
    EOF
    # Regenerate core types
    cargo run -p rh-cli -- codegen hl7.fhir.r5.core 5.0.0 \
        --output "$OUTPUT" \
        --crate-name rh-hl7-fhir-r5-core \
        --force
    # Regenerate R5 extensions in isolation, then merge extension modules into the core crate.
    # Running the extension package in-place with --force would delete the generated core crate.
    echo "Adding R5 extensions from hl7.fhir.uv.extensions..."
    EXT_OUTPUT=$(mktemp -d)
    if cargo run -p rh-cli -- codegen hl7.fhir.uv.extensions 5.1.0-snapshot1 \
        --output "$EXT_OUTPUT" \
        --crate-name rh-hl7-fhir-r5-core \
        --force 2>/dev/null; then
        mkdir -p "$OUTPUT/src/extensions"
        find "$EXT_OUTPUT/src/extensions" -maxdepth 1 -type f -name '*.rs' ! -name 'mod.rs' \
            -exec cp {} "$OUTPUT/src/extensions/" \;
        {
            echo "//! FHIR extension types"
            echo
            find "$OUTPUT/src/extensions" -maxdepth 1 -type f -name '*.rs' ! -name 'mod.rs' \
                -exec basename {} .rs \; | sort | sed 's/^/pub mod /; s/$/;/'
        } > "$OUTPUT/src/extensions/mod.rs"
    else
        echo "Warning: R5 extensions package not available, skipping"
    fi
    rm -rf "$EXT_OUTPUT"
    # Restore tests
    if [ -d "$TEST_BACKUP" ] && [ "$(ls -A "$TEST_BACKUP" 2>/dev/null)" ]; then
        mkdir -p "$OUTPUT/tests"
        cp -r "$TEST_BACKUP"/* "$OUTPUT/tests/"
    fi
    rm -rf "$TEST_BACKUP"
    echo "Building regenerated R5 crate..."
    cargo build -p rh-hl7-fhir-r5-core
    echo "Testing regenerated R5 crate..."
    cargo test -p rh-hl7-fhir-r5-core
    echo "R5 regeneration complete."

# Check that R4/R5 generated crates are up to date (no drift)
# Run this in CI to detect stale generated code.
regen-check:
    #!/usr/bin/env bash
    set -euo pipefail
    PKG_DIR=$(mktemp -d)
    EXT_OUTPUT=""
    trap 'rm -rf "$PKG_DIR" /tmp/rh-regen-check-r4 /tmp/rh-regen-check-r5 "$EXT_OUTPUT"' EXIT
    echo "Checking R4 crate for drift..."
    cargo run -p rh-cli -- codegen hl7.fhir.r4.core 4.0.1 \
        --package-dir "$PKG_DIR" \
        --overwrite \
        --output /tmp/rh-regen-check-r4 \
        --crate-name rh-hl7-fhir-r4-core \
        --force 2>/dev/null
    echo "Checking R5 crate for drift..."
    cargo run -p rh-cli -- codegen hl7.fhir.r5.core 5.0.0 \
        --package-dir "$PKG_DIR" \
        --overwrite \
        --output /tmp/rh-regen-check-r5 \
        --crate-name rh-hl7-fhir-r5-core \
        --force 2>/dev/null
    echo "Adding R5 extensions for drift check..."
    EXT_OUTPUT=$(mktemp -d)
    if cargo run -p rh-cli -- codegen hl7.fhir.uv.extensions 5.1.0-snapshot1 \
        --package-dir "$PKG_DIR" \
        --overwrite \
        --output "$EXT_OUTPUT" \
        --crate-name rh-hl7-fhir-r5-core \
        --force 2>/dev/null; then
        mkdir -p /tmp/rh-regen-check-r5/src/extensions
        find "$EXT_OUTPUT/src/extensions" -maxdepth 1 -type f -name '*.rs' ! -name 'mod.rs' \
            -exec cp {} /tmp/rh-regen-check-r5/src/extensions/ \;
        {
            echo "//! FHIR extension types"
            echo
            find /tmp/rh-regen-check-r5/src/extensions -maxdepth 1 -type f -name '*.rs' ! -name 'mod.rs' \
                -exec basename {} .rs \; | sort | sed 's/^/pub mod /; s/$/;/'
        } > /tmp/rh-regen-check-r5/src/extensions/mod.rs
    else
        echo "Warning: R5 extensions package not available, skipping"
    fi
    # Compare only src/ (generated content), not tests/ (hand-authored)
    R4_DIFF=0; R5_DIFF=0
    diff -rq crates/rh-hl7_fhir_r4_core/src /tmp/rh-regen-check-r4/src || R4_DIFF=$?
    diff -rq crates/rh-hl7_fhir_r5_core/src /tmp/rh-regen-check-r5/src || R5_DIFF=$?
    if [ "$R4_DIFF" -ne 0 ] || [ "$R5_DIFF" -ne 0 ]; then
        echo "Drift diff preview:"
        for checked in crates/rh-hl7_fhir_r4_core/src/metadata/datatypes.rs crates/rh-hl7_fhir_r4_core/src/metadata/primitives.rs crates/rh-hl7_fhir_r5_core/src/metadata/datatypes.rs crates/rh-hl7_fhir_r5_core/src/metadata/primitives.rs; do
            generated="${checked/crates\/rh-hl7_fhir_r4_core\/src/\/tmp\/rh-regen-check-r4\/src}"
            generated="${generated/crates\/rh-hl7_fhir_r5_core\/src/\/tmp\/rh-regen-check-r5\/src}"
            if [ -f "$generated" ] && ! cmp -s "$checked" "$generated"; then
                echo "--- $checked"
                diff -u "$checked" "$generated" | sed -n '1,220p' || true
            fi
        done
        echo "ERROR: Generated crates have drifted from codegen output."
        echo "Run just regen-r4 and just regen-r5 to update, then commit."
        exit 1
    fi
    echo "No drift detected. Generated crates are up to date."

# Build all WASM packages for WASM-bindgen crates.
wasm:
    just wasm-build fhirpath all
    just wasm-build vcl all
    just wasm-build cql all

# Build one WASM-bindgen crate for a target: web, node, bundler, or all.
wasm-build crate target="all":
    #!/usr/bin/env bash
    set -euo pipefail
    case "{{crate}}" in
      fhirpath) crate_dir="crates/rh-fhirpath" ;;
      vcl) crate_dir="crates/rh-vcl" ;;
      cql) crate_dir="crates/rh-cql" ;;
      *) echo "unknown WASM crate: {{crate}} (expected fhirpath, vcl, or cql)" >&2; exit 2 ;;
    esac

    case "{{target}}" in
      all) recipe="wasm" ;;
      web) recipe="wasm-web" ;;
      node|nodejs) recipe="wasm-node" ;;
      bundler) recipe="wasm-bundler" ;;
      *) echo "unknown WASM target: {{target}} (expected web, node, bundler, or all)" >&2; exit 2 ;;
    esac

    cd "$crate_dir"
    just "$recipe"

# Compile-check all WASM-capable crates without running wasm-pack.
wasm-check:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo check -p rh-foundation --target wasm32-unknown-unknown --no-default-features --features wasm
    cargo check -p rh-fhirpath --target wasm32-unknown-unknown --features wasm
    cargo check -p rh-vcl --target wasm32-unknown-unknown --features wasm
    cargo check -p rh-cql --target wasm32-unknown-unknown --no-default-features --features wasm

# Build WASM web packages.
build-wasm:
    #!/usr/bin/env bash
    set -euo pipefail
    just wasm-build fhirpath web
    just wasm-build vcl web
    just wasm-build cql web
    mkdir -p pkg/wasm/fhirpath pkg/wasm/vcl pkg/wasm/cql
    cp -R crates/rh-fhirpath/pkg/web/. pkg/wasm/fhirpath/
    cp -R crates/rh-vcl/pkg/web/. pkg/wasm/vcl/
    cp -R crates/rh-cql/pkg/web/. pkg/wasm/cql/
    echo "WASM web build complete. Output in pkg/wasm/ and crate pkg/web/ directories."

# Build WASM Node.js packages.
build-wasm-node:
    #!/usr/bin/env bash
    set -euo pipefail
    just wasm-build fhirpath node
    just wasm-build vcl node
    just wasm-build cql node
    mkdir -p pkg/wasm-node/fhirpath pkg/wasm-node/vcl pkg/wasm-node/cql
    cp -R crates/rh-fhirpath/pkg/node/. pkg/wasm-node/fhirpath/
    cp -R crates/rh-vcl/pkg/node/. pkg/wasm-node/vcl/
    cp -R crates/rh-cql/pkg/node/. pkg/wasm-node/cql/
    echo "WASM Node.js build complete. Output in pkg/wasm-node/ and crate pkg/node/ directories."

# Build WASM bundler packages.
build-wasm-bundler:
    #!/usr/bin/env bash
    set -euo pipefail
    just wasm-build fhirpath bundler
    just wasm-build vcl bundler
    just wasm-build cql bundler
    mkdir -p pkg/wasm-bundler/fhirpath pkg/wasm-bundler/vcl pkg/wasm-bundler/cql
    cp -R crates/rh-fhirpath/pkg/bundler/. pkg/wasm-bundler/fhirpath/
    cp -R crates/rh-vcl/pkg/bundler/. pkg/wasm-bundler/vcl/
    cp -R crates/rh-cql/pkg/bundler/. pkg/wasm-bundler/cql/
    echo "WASM bundler build complete. Output in pkg/wasm-bundler/ and crate pkg/bundler/ directories."

# Test WASM Node.js builds.
test-wasm:
    #!/usr/bin/env bash
    set -euo pipefail
    cd crates/rh-fhirpath && just test-wasm
    cd ../../crates/rh-vcl && just test-wasm
    cd ../../crates/rh-cql && just test-wasm
