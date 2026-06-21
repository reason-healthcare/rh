# CQL-to-ELM Conformance Testing

This directory contains infrastructure for comparing our Rust CQL-to-ELM translator against the reference Java implementation.

For the full conformance/spec coverage map, start at
[`../CONFORMANCE_INDEX.md`](../CONFORMANCE_INDEX.md).

## Directory Structure

```
conformance/
├── README.md                 # This file
├── CQL_ENGINE_TEST_AUDIT.md  # Audit workflow and generated report locations
├── CQL_TEST_CORPUS.md        # Corpus/source strategy
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
- Node.js and npm for JavaScript `cql-execution` matrix checks

## Setup

Run the setup script once to download and build the Java translator:

```bash
cd crates/rh-cql/conformance
./scripts/setup.sh
```

This will:
1. Clone the cqframework/clinical_quality_language repository at the pinned reference `v4.2.0`
2. Build the cql-to-elm-cli using Gradle
3. Verify both translators work
4. Write `reference-version.json` with the Java translator repository, ref, and commit

To override the pinned reference for an experiment:

```bash
CQL_JAVA_REF=v4.7.0 ./scripts/setup.sh
```

## Running Comparisons

### Populate the Three-Engine Implementation Matrix

```bash
cd crates/rh-cql
just audit-full
```

This regenerates the Rust HL7 audit and then updates
`conformance/results/audit/implementation_matrix.csv` / `.json` with Java ELM
and JavaScript `cql-execution` status columns.

To run only the external-reference phase after `just audit-strict`:

```bash
cd crates/rh-cql
just audit-references
```

### Run Expanded Corpus Checks

```bash
cd crates/rh-cql
just corpus-audit-rh
```

This writes the fast RH-only baseline:

- `conformance/results/corpus/corpus_matrix.csv`
- `conformance/results/corpus/corpus_matrix.json`
- `conformance/results/corpus/corpus_summary.json`

Use `just corpus-audit` for the heavier Java-inclusive reference pass.

For a fast smoke run:

```bash
cd crates/rh-cql
just corpus-audit-smoke
```

Smoke output is written under `conformance/results/corpus-smoke/`.

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

Comparison reports include Java reference commit metadata when `tools/cql-java`
has been set up.

### Difference Types

| Type | Description |
|------|-------------|
| `missing_in_rust` | Field present in Java output but missing in ours |
| `extra_in_rust` | Field present in our output but not in Java |
| `value_mismatch` | Same field, different value |
| `type_mismatch` | Same path, different JSON types |
| `array_length_mismatch` | Arrays have different lengths |

## Adding Test Cases

For source selection and corpus layering, see
[`CQL_TEST_CORPUS.md`](CQL_TEST_CORPUS.md).

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

See [`../CONFORMANCE.md`](../CONFORMANCE.md) for the current known intentional
differences between our translator and the Java reference implementation.

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
