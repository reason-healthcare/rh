#!/bin/bash
# Setup script for CQL-to-ELM conformance testing
# Run this once to download and build the Java reference translator

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFORMANCE_DIR="$(dirname "$SCRIPT_DIR")"
TOOLS_DIR="$CONFORMANCE_DIR/tools"
WORKSPACE_ROOT="$CONFORMANCE_DIR/../../.."

echo "=== CQL-to-ELM Conformance Test Setup ==="
echo ""

# Check prerequisites
echo "Checking prerequisites..."

if ! command -v java &> /dev/null; then
    echo "❌ Java not found. Please install Java 17+."
    exit 1
fi

JAVA_VERSION=$(java -version 2>&1 | head -1)
echo "✓ Java: $JAVA_VERSION"

if ! command -v python3 &> /dev/null; then
    echo "❌ Python3 not found. Please install Python 3.8+."
    exit 1
fi

PYTHON_VERSION=$(python3 --version)
echo "✓ Python: $PYTHON_VERSION"

if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust toolchain."
    exit 1
fi

CARGO_VERSION=$(cargo --version)
echo "✓ Cargo: $CARGO_VERSION"

echo ""

# Create tools directory
mkdir -p "$TOOLS_DIR"
cd "$TOOLS_DIR"

# Clone Java translator if not present
if [ ! -d "cql-java" ]; then
    echo "Cloning cqframework/clinical_quality_language..."
    git clone --depth 1 https://github.com/cqframework/clinical_quality_language.git cql-java
else
    echo "✓ Java translator repo already cloned"
fi

# Build Java translator
JAVA_CLI="$TOOLS_DIR/cql-java/Src/java/cql-to-elm-cli/build/install/cql-to-elm-cli/bin/cql-to-elm-cli"

if [ ! -f "$JAVA_CLI" ]; then
    echo "Building Java translator..."
    cd "$TOOLS_DIR/cql-java/Src/java"
    ./gradlew :cql-to-elm-cli:installDist
else
    echo "✓ Java translator already built"
fi

# Verify Java translator works
echo ""
echo "Verifying Java translator..."
if "$JAVA_CLI" 2>&1 | grep -q "Missing required option"; then
    echo "✓ Java translator CLI is functional"
else
    echo "❌ Java translator CLI verification failed"
    exit 1
fi

# Build Rust translator
echo ""
echo "Building Rust translator..."
cd "$WORKSPACE_ROOT"
cargo build --release -p rh

RUST_CLI="$WORKSPACE_ROOT/target/release/rh"
if [ -f "$RUST_CLI" ]; then
    echo "✓ Rust translator built: $RUST_CLI"
else
    echo "❌ Rust translator build failed"
    exit 1
fi

# Verify Rust translator works
echo ""
echo "Verifying Rust translator..."
if "$RUST_CLI" cql compile --help &> /dev/null; then
    echo "✓ Rust translator CLI is functional"
else
    echo "❌ Rust translator CLI verification failed"
    exit 1
fi

# Create test directories
echo ""
echo "Creating test directories..."
mkdir -p "$CONFORMANCE_DIR/test-cases/simple"
mkdir -p "$CONFORMANCE_DIR/results"

# Create a simple test file if not present
SIMPLE_TEST="$CONFORMANCE_DIR/test-cases/simple/SimpleTest.cql"
if [ ! -f "$SIMPLE_TEST" ]; then
    cat > "$SIMPLE_TEST" << 'EOF'
library SimpleTest version '1.0.0'

using FHIR version '4.0.1'

context Patient

define TestExpression: 1 + 1
EOF
    echo "✓ Created simple test file"
else
    echo "✓ Simple test file exists"
fi

echo ""
echo "=== Setup Complete ==="
echo ""
echo "To run comparisons:"
echo "  cd $CONFORMANCE_DIR"
echo "  python3 scripts/compare_translators.py -t simple"
echo ""
echo "See README.md for more options."
