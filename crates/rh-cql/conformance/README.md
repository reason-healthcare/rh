# CQL-to-ELM Conformance Testing

This directory contains infrastructure for comparing our Rust CQL-to-ELM translator against the reference Java implementation.

## Directory Structure

```
conformance/
├── README.md                 # This file
├── CONFORMANCE_REPORT.md     # Summary of differences found
├── scripts/
│   ├── setup.sh              # One-time setup script
│   └── compare_translators.py # Comparison tool
├── test-cases/               # CQL test files
│   ├── simple/               # Basic tests
│   ├── arithmetic/           # Arithmetic operator tests
│   └── ...
├── results/                  # Generated comparison results
│   └── <test-name>/
│       ├── <test>-java.json  # Java translator output
│       ├── <test>-rust.json  # Rust translator output
│       ├── comparison.json   # Structured differences
│       └── summary.txt       # Human-readable summary
└── tools/                    # External tools (git-ignored)
    └── cql-java/             # Java translator repo
```

## Prerequisites

- Java 17+ (`java -version`)
- Python 3.8+ (`python3 --version`)
- Rust toolchain (`cargo --version`)

## Setup

Run the setup script once to download and build the Java translator:

```bash
cd crates/rh-cql/conformance
./scripts/setup.sh
```

This will:
1. Clone the cqframework/clinical_quality_language repository
2. Build the cql-to-elm-cli using Gradle
3. Verify both translators work

## Running Comparisons

### Compare a single CQL file

```bash
python3 scripts/compare_translators.py -f test-cases/simple/SimpleTest.cql
```

### Compare all files in a test directory

```bash
python3 scripts/compare_translators.py -t simple
python3 scripts/compare_translators.py -t arithmetic
```

### Compare all test cases

```bash
python3 scripts/compare_translators.py -a
```

### With translator options

```bash
# Pass options to Java translator
python3 scripts/compare_translators.py -f test.cql -j --annotations --locators

# Pass options to Rust translator (when supported)
python3 scripts/compare_translators.py -f test.cql -r --no-annotations
```

## Understanding Results

Results are saved to `results/<test-name>/`:

- `*-java.json` - Output from reference Java translator
- `*-rust.json` - Output from our Rust translator
- `comparison.json` - Structured list of differences
- `summary.txt` - Human-readable summary

### Difference Types

| Type | Description |
|------|-------------|
| `missing_in_rust` | Field present in Java output but missing in ours |
| `extra_in_rust` | Field present in our output but not in Java |
| `value_mismatch` | Same field, different value |
| `type_mismatch` | Same path, different JSON types |
| `array_length_mismatch` | Arrays have different lengths |

## Adding Test Cases

1. Create a directory under `test-cases/`
2. Add `.cql` files to the directory
3. Run comparison to generate baseline

```bash
mkdir -p test-cases/my-test
echo 'library MyTest version "1.0.0"

define TestExpr: 1 + 1' > test-cases/my-test/MyTest.cql

python3 scripts/compare_translators.py -t my-test
```

## Updating the Conformance Report

After making changes to the translator, re-run comparisons and update the report:

```bash
# Run all tests
python3 scripts/compare_translators.py -a

# Review results
cat results/*/summary.txt
```

## Known Differences

See [CONFORMANCE_REPORT.md](CONFORMANCE_REPORT.md) for a detailed analysis of differences between our translator and the reference implementation.

## Troubleshooting

### Java translator not found

```bash
# Rebuild the Java translator
cd tools/cql-java/Src/java
./gradlew :cql-to-elm-cli:installDist
```

### Rust translator not built

```bash
# From workspace root
cargo build --release
```

### Python script errors

Ensure you're running from the conformance directory:

```bash
cd crates/rh-cql/conformance
python3 scripts/compare_translators.py --help
```
