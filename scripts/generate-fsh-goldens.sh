#!/usr/bin/env bash
# Generates golden FHIR JSON files using sushi for each FSH fixture.
# Run this script to (re)generate goldens when fixtures change or sushi updates.
#
# Usage: ./scripts/generate-fsh-goldens.sh
# Requires: npx, fsh-sushi (installed automatically via npx)
#
# Output structure: goldens/<category>/<fixture-stem>/<ResourceType-id>.json

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FIXTURES_DIR="$REPO_ROOT/crates/rh-fsh/tests/fixtures"
GOLDENS_DIR="$REPO_ROOT/crates/rh-fsh/tests/goldens"
SUSHI_CONFIG="$REPO_ROOT/crates/rh-fsh/tests/sushi-config.yaml"

mkdir -p "$GOLDENS_DIR"

generated=0
empty=0
failed=0

fixture_files=()
if [ "$#" -gt 0 ]; then
    for fixture in "$@"; do
        fixture_files+=("$FIXTURES_DIR/$fixture")
    done
else
    while IFS= read -r fixture; do
        fixture_files+=("$fixture")
    done < <(find "$FIXTURES_DIR" -name "*.fsh" | sort)
fi

for fsh_file in "${fixture_files[@]}"; do
    rel_path="${fsh_file#$FIXTURES_DIR/}"
    # Fixture stem: relative path without .fsh extension
    fixture_stem="${rel_path%.fsh}"
    echo "Processing: $rel_path"

    # Sushi v3+ requires FSH files to be in input/fsh/
    tmpdir=$(mktemp -d)
    mkdir -p "$tmpdir/input/fsh"
    cp "$fsh_file" "$tmpdir/input/fsh/input.fsh"
    cp "$SUSHI_CONFIG" "$tmpdir/sushi-config.yaml"

    # Run pinned SUSHI; FSHOnly: true means output goes to fsh-generated/resources/
    if ! npx --yes fsh-sushi@3.19.0 "$tmpdir" >"$tmpdir/sushi.log" 2>&1; then
        echo "  ERROR: SUSHI failed"
        sed -n '1,120p' "$tmpdir/sushi.log"
        failed=$((failed + 1))
        rm -rf "$tmpdir"
        continue
    fi

    output_dir="$tmpdir/fsh-generated/resources"
    found=0

    if [ -d "$output_dir" ]; then
        while IFS= read -r json_file; do
            json_basename=$(basename "$json_file")
            # Each fixture gets its own subdirectory to avoid collisions
            golden_subdir="$GOLDENS_DIR/$fixture_stem"
            mkdir -p "$golden_subdir"
            rm -f "$golden_subdir/.sushi-empty"
            cp "$json_file" "$golden_subdir/$json_basename"
            echo "  → $json_basename"
            found=1
        done < <(find "$output_dir" -name "*.json" ! -name "package.json" ! -name "*.index.json" 2>/dev/null | sort)
    fi

    if [ "$found" -eq 1 ]; then
        generated=$((generated + 1))
    else
        golden_subdir="$GOLDENS_DIR/$fixture_stem"
        mkdir -p "$golden_subdir"
        find "$golden_subdir" -type f -name '*.json' -delete
        touch "$golden_subdir/.sushi-empty"
        echo "  → verified empty output"
        empty=$((empty + 1))
    fi

    rm -rf "$tmpdir"
done

echo ""
echo "Done! Goldens written to $GOLDENS_DIR"
echo "  Generated: $generated  Empty: $empty  Failed: $failed"

if [ "$failed" -ne 0 ]; then
    exit 1
fi
