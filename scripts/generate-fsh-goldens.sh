#!/usr/bin/env bash
# Generates golden FHIR JSON files using sushi for each FSH fixture.
# Run this script to (re)generate goldens when fixtures change or sushi updates.
#
# Usage: ./scripts/generate-fsh-goldens.sh
# Requires: npx, fsh-sushi (installed automatically via npx)
#
# Output structure: goldens/<category>/<fixture-stem>/<ResourceType-id>.json

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FIXTURES_DIR="$REPO_ROOT/crates/rh-fsh/tests/fixtures"
GOLDENS_DIR="$REPO_ROOT/crates/rh-fsh/tests/goldens"
SUSHI_CONFIG="$REPO_ROOT/crates/rh-fsh/tests/sushi-config.yaml"

mkdir -p "$GOLDENS_DIR"

pass=0
skip=0

while IFS= read -r fsh_file; do
    rel_path="${fsh_file#$FIXTURES_DIR/}"
    # Fixture stem: relative path without .fsh extension
    fixture_stem="${rel_path%.fsh}"
    echo "Processing: $rel_path"

    # Sushi v3+ requires FSH files to be in input/fsh/
    tmpdir=$(mktemp -d)
    mkdir -p "$tmpdir/input/fsh"
    cp "$fsh_file" "$tmpdir/input/fsh/input.fsh"
    cp "$SUSHI_CONFIG" "$tmpdir/sushi-config.yaml"

    # Run sushi; FSHOnly: true means output goes to fsh-generated/resources/
    npx --yes fsh-sushi "$tmpdir" >/dev/null 2>&1 || true

    output_dir="$tmpdir/fsh-generated/resources"
    found=0

    if [ -d "$output_dir" ]; then
        while IFS= read -r json_file; do
            json_basename=$(basename "$json_file")
            # Each fixture gets its own subdirectory to avoid collisions
            golden_subdir="$GOLDENS_DIR/$fixture_stem"
            mkdir -p "$golden_subdir"
            cp "$json_file" "$golden_subdir/$json_basename"
            echo "  → $json_basename"
            found=1
        done < <(find "$output_dir" -name "*.json" ! -name "package.json" ! -name "*.index.json" 2>/dev/null | sort)
    fi

    if [ "$found" -eq 1 ]; then
        pass=$((pass + 1))
    else
        echo "  ⚠ No JSON output — skipping (aliases-only or unsupported)"
        skip=$((skip + 1))
    fi

    rm -rf "$tmpdir"
done < <(find "$FIXTURES_DIR" -name "*.fsh" | sort)

echo ""
echo "Done! Goldens written to $GOLDENS_DIR"
echo "  Generated: $pass  Skipped: $skip"
