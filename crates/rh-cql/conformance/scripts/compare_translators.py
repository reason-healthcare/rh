#!/usr/bin/env python3
"""
Compare CQL-to-ELM translation output between:
1. Reference Java implementation (cqframework/clinical_quality_language)
2. Our Rust implementation (rh cql compile)

Usage:
    python compare_translators.py [--test-case NAME] [--options OPTIONS]
    python compare_translators.py --cooking [SESSION_NUMBERS]
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
COOKING_DIR = TOOLS_DIR / "cooking-with-cql/Source/Cooking With CQL"
CQL_TO_ELM_TESTS_DIR = TOOLS_DIR / "cql-java/Src/java/cql-to-elm/src/jvmTest/resources/org/cqframework/cql/cql2elm"

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
    
    # Check that output file exists
    if not output_file.exists():
        print(f"Java translator produced no output", file=sys.stderr)
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


def run_cooking_session(session: str, java_options: list[str] = None, rust_options: list[str] = None, model_filter: str = None) -> list:
    """Run all CQL files in a Cooking with CQL session."""
    session_dir = COOKING_DIR / session
    if not session_dir.exists():
        print(f"Error: Session directory not found: {session_dir}", file=sys.stderr)
        return []
    
    results = []
    for cql_file in sorted(session_dir.glob("*.cql")):
        # Filter by model if specified
        if model_filter:
            content = cql_file.read_text()
            if model_filter == 'fhir':
                if 'using FHIR' not in content:
                    continue
            elif model_filter == 'none':
                if 'using ' in content:
                    continue
            elif model_filter == 'qdm':
                if 'using QDM' not in content:
                    continue
        
        result = run_test_case(cql_file, java_options, rust_options, session_prefix=f"cooking-{session}")
        if result:
            results.append({
                'session': session,
                'file': cql_file.name,
                'total_differences': result['total_differences'],
                'status': 'compared'
            })
        else:
            results.append({
                'session': session,
                'file': cql_file.name,
                'total_differences': -1,
                'status': 'failed'
            })
    return results


def run_operator_tests(test_names: list[str] = None, java_options: list[str] = None, rust_options: list[str] = None) -> list:
    """Run the Java cql-to-elm OperatorTests."""
    operator_tests_dir = CQL_TO_ELM_TESTS_DIR / "OperatorTests"
    if not operator_tests_dir.exists():
        print(f"Error: OperatorTests not found. Run setup.sh first.", file=sys.stderr)
        return []
    
    results = []
    
    # Get list of CQL files to test
    if test_names:
        cql_files = []
        for name in test_names:
            # Try exact match first
            test_file = operator_tests_dir / f"{name}.cql"
            if test_file.exists():
                cql_files.append(test_file)
            else:
                # Try case-insensitive glob
                matches = list(operator_tests_dir.glob(f"*{name}*.cql"))
                cql_files.extend(matches)
    else:
        cql_files = sorted(operator_tests_dir.glob("*.cql"))
    
    for cql_file in cql_files:
        result = run_test_case(cql_file, java_options, rust_options, session_prefix="operator-tests")
        if result:
            results.append({
                'test': cql_file.stem,
                'file': cql_file.name,
                'total_differences': result['total_differences'],
                'status': 'compared'
            })
        else:
            results.append({
                'test': cql_file.stem,
                'file': cql_file.name,
                'total_differences': -1,
                'status': 'failed'
            })
    
    return results


def run_test_case(cql_file: Path, java_options: list[str] = None, rust_options: list[str] = None, session_prefix: str = None):
    """Run both translators on a CQL file and compare outputs."""
    print(f"\n{'='*60}")
    print(f"Test case: {cql_file.name}")
    print(f"{'='*60}")
    
    # Create results directory
    if session_prefix:
        results_dir = RESULTS_DIR / session_prefix / cql_file.stem
    else:
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
    parser.add_argument('--cooking', '-c', nargs='*', metavar='SESSION',
                        help='Run Cooking with CQL sessions (e.g., --cooking 01 02 03, or --cooking all)')
    parser.add_argument('--operator-tests', '-o', nargs='*', metavar='TEST',
                        help='Run Java OperatorTests (e.g., --operator-tests ArithmeticOperators, or --operator-tests for all)')
    parser.add_argument('--model', '-m', choices=['fhir', 'qdm', 'none'], 
                        help='Filter Cooking with CQL by model type (fhir, qdm, or none)')
    parser.add_argument('--summary-only', '-s', action='store_true', 
                        help='Only print summary at end (for batch runs)')
    args = parser.parse_args()
    
    # Ensure directories exist
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    
    if args.operator_tests is not None:
        # Operator tests mode
        if not CQL_TO_ELM_TESTS_DIR.exists():
            print(f"Error: cql-to-elm tests not found. Run setup.sh first.", file=sys.stderr)
            sys.exit(1)
        
        test_names = args.operator_tests if args.operator_tests else None
        all_results = run_operator_tests(test_names, args.java_options, args.rust_options)
        
        failed_files = []
        passed_files = []
        translation_failures = []
        
        for r in all_results:
            if r['status'] == 'failed':
                translation_failures.append(r)
            elif r['total_differences'] > 0:
                failed_files.append(r)
            else:
                passed_files.append(r)
        
        # Print summary
        print(f"\n{'='*60}")
        print("OPERATOR TESTS SUMMARY")
        print(f"{'='*60}")
        print(f"Total files tested: {len(all_results)}")
        print(f"Translation failures: {len(translation_failures)}")
        print(f"Compared successfully: {len(passed_files) + len(failed_files)}")
        print(f"  - Passed (0 differences): {len(passed_files)}")
        print(f"  - Failed (differences found): {len(failed_files)}")
        
        if translation_failures and not args.summary_only:
            print(f"\nTranslation failures:")
            for r in translation_failures:
                print(f"  - {r['file']}")
        
        if failed_files and not args.summary_only:
            print(f"\nFiles with differences:")
            for r in failed_files[:20]:
                print(f"  - {r['file']}: {r['total_differences']} differences")
            if len(failed_files) > 20:
                print(f"  ... and {len(failed_files) - 20} more")
        
        # Save summary to file
        summary_file = RESULTS_DIR / "operator-tests-summary.json"
        with open(summary_file, 'w') as f:
            json.dump({
                'total': len(all_results),
                'translation_failures': len(translation_failures),
                'passed': len(passed_files),
                'failed': len(failed_files),
                'translation_failure_files': translation_failures,
                'failed_files': failed_files,
                'passed_files': passed_files
            }, f, indent=2)
        print(f"\nSummary saved to: {summary_file}")
    
    elif args.cooking is not None:
        # Cooking with CQL mode
        if not COOKING_DIR.exists():
            print(f"Error: Cooking with CQL not found. Run setup.sh first.", file=sys.stderr)
            sys.exit(1)
        
        sessions = args.cooking
        if not sessions or 'all' in sessions:
            # Get all sessions
            sessions = sorted([d.name for d in COOKING_DIR.iterdir() if d.is_dir()])
        
        all_results = []
        failed_files = []
        passed_files = []
        translation_failures = []
        
        for session in sessions:
            results = run_cooking_session(session, args.java_options, args.rust_options, args.model)
            all_results.extend(results)
            for r in results:
                if r['status'] == 'failed':
                    translation_failures.append(r)
                elif r['total_differences'] > 0:
                    failed_files.append(r)
                else:
                    passed_files.append(r)
        
        # Print summary
        print(f"\n{'='*60}")
        print("COOKING WITH CQL SUMMARY")
        if args.model:
            print(f"(filtered by model: {args.model})")
        print(f"{'='*60}")
        print(f"Total files tested: {len(all_results)}")
        print(f"Translation failures: {len(translation_failures)}")
        print(f"Compared successfully: {len(passed_files) + len(failed_files)}")
        print(f"  - Passed (0 differences): {len(passed_files)}")
        print(f"  - Failed (differences found): {len(failed_files)}")
        
        if translation_failures and not args.summary_only:
            print(f"\nTranslation failures (first 10):")
            for r in translation_failures[:10]:
                print(f"  - {r['session']}/{r['file']}")
            if len(translation_failures) > 10:
                print(f"  ... and {len(translation_failures) - 10} more")
        
        if failed_files and not args.summary_only:
            print(f"\nFiles with differences (first 20):")
            for r in failed_files[:20]:
                print(f"  - {r['session']}/{r['file']}: {r['total_differences']} differences")
            if len(failed_files) > 20:
                print(f"  ... and {len(failed_files) - 20} more")
        
        # Save summary to file
        summary_file = RESULTS_DIR / "cooking-summary.json"
        with open(summary_file, 'w') as f:
            json.dump({
                'model_filter': args.model,
                'total': len(all_results),
                'translation_failures': len(translation_failures),
                'passed': len(passed_files),
                'failed': len(failed_files),
                'translation_failure_files': translation_failures,
                'failed_files': failed_files,
                'passed_files': passed_files
            }, f, indent=2)
        print(f"\nSummary saved to: {summary_file}")
    
    elif args.cql_file:
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
