#!/usr/bin/env python3
"""
Compare CQL-to-ELM translation output between:
1. Reference Java implementation (cqframework/clinical_quality_language)
2. Our Rust implementation (rh cql compile)

Usage:
    python compare_translators.py [--test-case NAME] [--options OPTIONS]
"""

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any

# Paths
SCRIPT_DIR = Path(__file__).parent
CONFORMANCE_DIR = SCRIPT_DIR.parent
TOOLS_DIR = CONFORMANCE_DIR / "tools"
TEST_CASES_DIR = CONFORMANCE_DIR / "test-cases"
RESULTS_DIR = CONFORMANCE_DIR / "results"

# Java CLI path
JAVA_CLI = TOOLS_DIR / "cql-java/Src/java/cql-to-elm-cli/build/install/cql-to-elm-cli/bin/cql-to-elm-cli"

# Rust CLI (relative to workspace root)
WORKSPACE_ROOT = CONFORMANCE_DIR.parent.parent.parent
RUST_CLI = WORKSPACE_ROOT / "target/release/rh"


def normalize_elm(elm: dict) -> dict:
    """Normalize ELM JSON for comparison by removing variable fields."""
    def recurse(obj: Any) -> Any:
        if isinstance(obj, dict):
            result = {}
            for k, v in obj.items():
                # Skip fields that vary between implementations
                if k in ('translatorVersion', 'translatorOptions', 'signatureLevel'):
                    continue
                # Keep localId but note it may differ
                result[k] = recurse(v)
            return result
        elif isinstance(obj, list):
            return [recurse(item) for item in obj]
        else:
            return obj
    return recurse(elm)


def compare_values(path: str, java_val: Any, rust_val: Any, differences: list):
    """Recursively compare values and collect differences."""
    if type(java_val) != type(rust_val):
        differences.append({
            'path': path,
            'type': 'type_mismatch',
            'java': f"{type(java_val).__name__}: {java_val}",
            'rust': f"{type(rust_val).__name__}: {rust_val}"
        })
        return
    
    if isinstance(java_val, dict):
        java_keys = set(java_val.keys())
        rust_keys = set(rust_val.keys())
        
        # Missing in rust
        for key in java_keys - rust_keys:
            # Skip empty arrays
            if isinstance(java_val[key], list) and len(java_val[key]) == 0:
                continue
            differences.append({
                'path': f"{path}.{key}",
                'type': 'missing_in_rust',
                'java': java_val[key]
            })
        
        # Extra in rust
        for key in rust_keys - java_keys:
            # Skip empty arrays
            if isinstance(rust_val[key], list) and len(rust_val[key]) == 0:
                continue
            differences.append({
                'path': f"{path}.{key}",
                'type': 'extra_in_rust',
                'rust': rust_val[key]
            })
        
        # Compare common keys
        for key in java_keys & rust_keys:
            compare_values(f"{path}.{key}", java_val[key], rust_val[key], differences)
    
    elif isinstance(java_val, list):
        if len(java_val) != len(rust_val):
            differences.append({
                'path': path,
                'type': 'array_length_mismatch',
                'java_len': len(java_val),
                'rust_len': len(rust_val)
            })
            # Compare up to the shorter length
            for i in range(min(len(java_val), len(rust_val))):
                compare_values(f"{path}[{i}]", java_val[i], rust_val[i], differences)
        else:
            for i, (j, r) in enumerate(zip(java_val, rust_val)):
                compare_values(f"{path}[{i}]", j, r, differences)
    
    else:
        if java_val != rust_val:
            differences.append({
                'path': path,
                'type': 'value_mismatch',
                'java': java_val,
                'rust': rust_val
            })


def run_java_translator(cql_file: Path, output_dir: Path, options: list[str] = None) -> Path:
    """Run the Java CQL-to-ELM translator."""
    output_file = output_dir / f"{cql_file.stem}-java.json"
    
    cmd = [
        str(JAVA_CLI),
        "--input", str(cql_file),
        "--format", "JSON",
        "--output", str(output_dir)
    ]
    
    if options:
        cmd.extend(options)
    
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    # Java CLI outputs to <stem>.json, rename to -java.json
    java_output = output_dir / f"{cql_file.stem}.json"
    if java_output.exists():
        java_output.rename(output_file)
    
    if result.returncode != 0:
        print(f"Java translator error: {result.stderr}", file=sys.stderr)
        return None
    
    return output_file


def run_rust_translator(cql_file: Path, output_dir: Path, options: list[str] = None) -> Path:
    """Run the Rust CQL-to-ELM translator."""
    output_file = output_dir / f"{cql_file.stem}-rust.json"
    
    cmd = [str(RUST_CLI), "cql", "compile", str(cql_file)]
    
    if options:
        cmd.extend(options)
    
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    if result.returncode != 0:
        print(f"Rust translator error: {result.stderr}", file=sys.stderr)
        return None
    
    with open(output_file, 'w') as f:
        f.write(result.stdout)
    
    return output_file


def compare_outputs(java_file: Path, rust_file: Path) -> dict:
    """Compare Java and Rust translator outputs."""
    with open(java_file) as f:
        java_elm = json.load(f)
    
    with open(rust_file) as f:
        rust_elm = json.load(f)
    
    # Normalize for comparison
    java_norm = normalize_elm(java_elm)
    rust_norm = normalize_elm(rust_elm)
    
    differences = []
    compare_values('library', java_norm.get('library', {}), rust_norm.get('library', {}), differences)
    
    return {
        'total_differences': len(differences),
        'differences': differences,
        'java_file': str(java_file),
        'rust_file': str(rust_file)
    }


def summarize_differences(comparison: dict) -> str:
    """Generate a human-readable summary of differences."""
    lines = []
    lines.append(f"Total differences: {comparison['total_differences']}")
    lines.append("")
    
    if comparison['total_differences'] == 0:
        lines.append("✅ Outputs match!")
        return "\n".join(lines)
    
    # Group by difference type
    by_type = {}
    for diff in comparison['differences']:
        dtype = diff['type']
        if dtype not in by_type:
            by_type[dtype] = []
        by_type[dtype].append(diff)
    
    for dtype, diffs in sorted(by_type.items()):
        lines.append(f"## {dtype} ({len(diffs)} occurrences)")
        for diff in diffs[:10]:  # Show first 10
            lines.append(f"  - {diff['path']}")
            if 'java' in diff:
                lines.append(f"    Java: {diff['java']}")
            if 'rust' in diff:
                lines.append(f"    Rust: {diff['rust']}")
        if len(diffs) > 10:
            lines.append(f"  ... and {len(diffs) - 10} more")
        lines.append("")
    
    return "\n".join(lines)


def run_test_case(cql_file: Path, java_options: list[str] = None, rust_options: list[str] = None):
    """Run both translators on a CQL file and compare outputs."""
    print(f"\n{'='*60}")
    print(f"Test case: {cql_file.name}")
    print(f"{'='*60}")
    
    # Create results directory
    results_dir = RESULTS_DIR / cql_file.stem
    results_dir.mkdir(parents=True, exist_ok=True)
    
    # Run translators
    print("Running Java translator...")
    java_output = run_java_translator(cql_file, results_dir, java_options)
    
    print("Running Rust translator...")
    rust_output = run_rust_translator(cql_file, results_dir, rust_options)
    
    if not java_output or not rust_output:
        print("❌ Translation failed")
        return None
    
    # Compare outputs
    print("Comparing outputs...")
    comparison = compare_outputs(java_output, rust_output)
    
    # Save comparison result
    comparison_file = results_dir / "comparison.json"
    with open(comparison_file, 'w') as f:
        json.dump(comparison, f, indent=2)
    
    # Print summary
    summary = summarize_differences(comparison)
    print(summary)
    
    # Save summary
    summary_file = results_dir / "summary.txt"
    with open(summary_file, 'w') as f:
        f.write(summary)
    
    return comparison


def main():
    parser = argparse.ArgumentParser(description='Compare CQL-to-ELM translators')
    parser.add_argument('--test-case', '-t', help='Name of test case to run')
    parser.add_argument('--cql-file', '-f', help='Path to CQL file to test')
    parser.add_argument('--java-options', '-j', nargs='*', help='Java translator options')
    parser.add_argument('--rust-options', '-r', nargs='*', help='Rust translator options')
    parser.add_argument('--all', '-a', action='store_true', help='Run all test cases')
    args = parser.parse_args()
    
    # Ensure directories exist
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    
    if args.cql_file:
        cql_file = Path(args.cql_file)
        if not cql_file.exists():
            print(f"Error: CQL file not found: {cql_file}", file=sys.stderr)
            sys.exit(1)
        run_test_case(cql_file, args.java_options, args.rust_options)
    
    elif args.test_case:
        test_dir = TEST_CASES_DIR / args.test_case
        if not test_dir.exists():
            print(f"Error: Test case directory not found: {test_dir}", file=sys.stderr)
            sys.exit(1)
        
        for cql_file in test_dir.glob("*.cql"):
            run_test_case(cql_file, args.java_options, args.rust_options)
    
    elif args.all:
        for test_dir in sorted(TEST_CASES_DIR.iterdir()):
            if test_dir.is_dir():
                for cql_file in test_dir.glob("*.cql"):
                    run_test_case(cql_file, args.java_options, args.rust_options)
    
    else:
        parser.print_help()


if __name__ == '__main__':
    main()
