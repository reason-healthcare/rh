# rh-fsh Regression Testing Strategy

## Overview

SUSHI maintains its position as the reference FSH compiler through rigorous regression testing against real-world FSH projects. This document outlines how rh-fsh should adopt and extend this testing approach.

## SUSHI's Regression Testing Approach

### Architecture

SUSHI's regression system (`regression/` directory) provides:

```
Regression Test Pipeline
=========================

┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│  FSH Finder  │───▶│  Repository  │───▶│   Compare    │
│  (discover)  │    │  (download)  │    │   (diff)     │
└──────────────┘    └──────────────┘    └──────────────┘
       │                   │                   │
       ▼                   ▼                   ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│ 1000+ repos  │    │ Run SUSHI A  │    │ HTML Report  │
│ JSON index   │    │ Run SUSHI B  │    │ JSON Diff    │
└──────────────┘    └──────────────┘    └──────────────┘
```

### Key Components

| File | Purpose |
|------|---------|
| `cli.ts` | Command-line interface for regression runs |
| `run.ts` | Core comparison logic (30KB) |
| `find.ts` | FSH Finder integration and repo discovery |
| `repos-all.txt` | Legacy curated repo list |
| `repos-select.txt` | Subset for quick testing |

### FSH Finder Integration

SUSHI uses [FSH Finder](https://fshschool.github.io/fsh-finder/) to discover test repositories:

```typescript
const FSH_FINDER_URL = 'https://fshschool.github.io/fsh-finder/fshy_repos.json';

// JSON structure
{
  "updated": "2025-12-18T...",
  "repos": [
    {
      "repo_host": "github.com",
      "repo_owner": "HL7",
      "repo_name": "fhir-mCODE-ig",
      "updated_at": "2025-12-15 12:00:00 +0000",
      "feature_assessments": {
        "FeatureSushiOne": {
          "branches_with_feature": ["master", "main"]
        }
      }
    }
  ]
}
```

### Comparison Methodology

For each repository, SUSHI:

1. **Downloads** the repo as a ZIP archive
2. **Extracts** two copies (one for each SUSHI version)
3. **Runs** both SUSHI versions independently
4. **Compares** output directories file-by-file
5. **Generates** diff reports (HTML and JSON formats)

Key comparison metrics:
- **Output differences** - Any changed files in `fsh-generated/`
- **Error count changes** - Regressions in error handling
- **Warning count changes** - New or resolved warnings
- **Performance** - Elapsed time comparison

### CLI Usage

```bash
# Compare local changes against master branch
npm run regression -- -a gh:master -b local -c 50

# Test specific repositories
npm run regression -- --repo HL7/fhir-mCODE-ig#master HL7/davinci-crd#master

# Run against recently updated repos
npm run regression -- -l 30 -c 50  # Last 30 days, max 50 repos
```

## rh-fsh Testing Strategy

### Level 1: Unit Tests

Test individual parser and compiler components:

```rust
// tests/parser_tests.rs

#[test]
fn test_parse_profile() {
    let input = r#"
Profile: MyPatient
Parent: Patient
* identifier 1..*
* name 1..1 MS
"#;

    let doc = parse_fsh(input).unwrap();
    assert_eq!(doc.entities.len(), 1);

    let Entity::Profile(profile) = &doc.entities[0] else { panic!() };
    assert_eq!(profile.name, "MyPatient");
    assert_eq!(profile.parent, Some("Patient".into()));
    assert_eq!(profile.rules.len(), 2);
}

#[test]
fn test_parse_valueset_include() {
    let input = r#"
ValueSet: MyValueSet
* include codes from system http://loinc.org
"#;
    let doc = parse_fsh(input).unwrap();
    // ... assertions
}
```

### Level 2: SUSHI Test Fixtures

Port SUSHI's test fixtures from `test/` directory.

**Important:** Ensure test fixtures include all three FHIR usage types:
- `Usage: #example` - Example instances for documentation
- `Usage: #definition` - Definitional resources (Questionnaires, ActivityDefinitions, etc.)
- `Usage: #inline` - Inline instances (contained resources, intermediate representations)

```
SUSHI Test Files (Partial List)
===============================

test/
├── import/
│   ├── FSHImporter.Profile.test.ts
│   ├── FSHImporter.Extension.test.ts
│   ├── FSHImporter.Instance.test.ts
│   └── ... (one per entity type)
├── export/
│   ├── StructureDefinitionExporter.test.ts
│   ├── ValueSetExporter.test.ts
│   └── ...
└── testhelpers/
    └── ... (test utilities)
```

Create equivalent Rust test files:

```rust
// tests/sushi_compat/profile_tests.rs

mod profile_tests {
    use rh_fsh::*;

    #[test]
    fn test_profile_with_parent() {
        // From SUSHI: FSHImporter.Profile.test.ts
        let input = include_str!("fixtures/profile_with_parent.fsh");
        let expected = include_str!("fixtures/profile_with_parent.json");

        let result = compile_fsh(input).unwrap();
        assert_json_eq!(result, expected);
    }
}
```

### Level 3: Regression Testing

Implement a regression test runner similar to SUSHI's:

```rust
// src/bin/regression.rs

use rh_fsh::compile_project;
use std::process::Command;

struct RegressionConfig {
    /// rh-fsh version (or "local")
    version_a: String,
    /// SUSHI version for comparison
    version_b: String,
    /// Repositories to test
    repos: Vec<RepoRef>,
}

struct RepoRef {
    owner: String,
    name: String,
    branch: String,
}

fn run_regression(config: RegressionConfig) -> RegressionReport {
    let repos = if config.repos.is_empty() {
        fetch_fsh_finder_repos()
    } else {
        config.repos
    };

    let mut report = RegressionReport::new();

    for repo in repos {
        let repo_dir = download_repo(&repo);

        // Run rh-fsh
        let rh_result = compile_project(&repo_dir);

        // Run SUSHI for comparison
        let sushi_result = run_sushi(&repo_dir);

        // Compare outputs
        let diff = compare_outputs(&rh_result, &sushi_result);
        report.add(repo, diff);
    }

    report
}
```

### Level 4: Continuous Integration

```yaml
# .github/workflows/regression.yml

name: FSH Regression Tests

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  regression:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Install Node.js (for SUSHI)
        uses: actions/setup-node@v4
        with:
          node-version: '18'

      - name: Install SUSHI
        run: npm install -g fsh-sushi

      - name: Run Regression Tests
        run: cargo run --bin regression -- --count 50

      - name: Upload Report
        uses: actions/upload-artifact@v4
        with:
          name: regression-report
          path: regression-output/
```

## Performance Testing

For performance testing strategy including synthetic IG generation, benchmarking, and stress testing, see [rh-fsh Performance Testing Strategy](rh-fsh-performance-testing.md).

## FSH Finder Repository Categories

FSH Finder categorizes repositories by several criteria useful for testing:

### By Organization

| Organization | Count | Description |
|--------------|-------|-------------|
| HL7 | 150+ | Official HL7 IGs |
| hl7-eu | 50+ | European IGs |
| IHE | 30+ | IHE profiles |
| WHO | 10+ | WHO standards |

### By Complexity

For testing progression, categorize by:

| Tier | Criteria | Example Repos |
|------|----------|---------------|
| Simple | 1-2 profiles, no extensions | Tutorial examples |
| Medium | Multiple profiles, extensions | US Core |
| Complex | Logical models, mappings, many rules | mCODE |
| Comprehensive | Full IG with all features | Da Vinci guides |

### Recommended Test Suite

**Smoke Tests (5 repos):**
- `HL7/fhir-shorthand` (the spec itself)
- `HL7/US-Core`
- `HL7/fhir-mCODE-ig`
- A simple tutorial repo
- One with known edge cases

**Extended Tests (50 repos):**
- All HL7 flagship IGs
- Regional variations (AU, EU, UK)
- IHE profiles
- Complex commercial IGs

**Full Suite (500+ repos):**
- All FSH Finder repositories
- Run weekly or pre-release

## Comparison Criteria

### Output Equivalence

For each FHIR artifact, compare:

```rust
enum ComparisonResult {
    Identical,
    EquivalentJson {
        // Same semantic content, different formatting
        whitespace_diff: bool,
        key_order_diff: bool,
    },
    SemanticDiff {
        // Different values but potentially valid
        path: String,
        expected: Value,
        actual: Value,
    },
    Missing {
        path: String,
    },
    Extra {
        path: String,
    },
}
```

### Known Acceptable Differences

Some differences are acceptable:

1. **Whitespace/formatting** - JSON formatting choices
2. **Key ordering** - JSON object key order
3. **Generated IDs** - UUIDs or timestamps
4. **Snapshot generation** - rh-fsh may generate different but valid snapshots

### Error Equivalence

Compare error reporting:

```rust
struct ErrorComparison {
    /// Errors reported by both
    common_errors: Vec<ErrorMatch>,
    /// Errors only in rh-fsh
    rh_only: Vec<FshError>,
    /// Errors only in SUSHI
    sushi_only: Vec<SushiError>,
}
```

## Recommended Implementation Order

### Phase 1: Basic Infrastructure
1. Download FSH Finder JSON
2. Clone/download GitHub repositories
3. Run SUSHI and capture output
4. Basic file comparison

### Phase 2: rh-fsh Integration
1. Run rh-fsh on downloaded repos
2. Compare JSON outputs
3. Generate diff reports

### Phase 3: CI Integration
1. GitHub Actions workflow
2. Artifact storage
3. Trend tracking

### Phase 4: Dashboard
1. Web-based report viewer
2. Historical comparison
3. Per-repo drill-down

## Repository Selection Heuristics

For efficient testing, prioritize repositories that:

1. **Recently updated** - More likely to use current FSH features
2. **High complexity** - Exercise more code paths
3. **Diverse organizations** - Different coding styles
4. **Known edge cases** - Previously found bugs
5. **Successfully built** - Currently valid FSH

Query FSH Finder with:
```rust
fn select_test_repos(count: usize, lookback_days: u32) -> Vec<RepoRef> {
    let all_repos = fetch_fsh_finder();

    all_repos
        .into_iter()
        .filter(|r| r.updated_within_days(lookback_days))
        .filter(|r| r.has_sushi_config())
        .take(count)
        .collect()
}
```

## Metrics and Reporting

Track over time:
- **Pass rate** - % of repos with identical output
- **Error parity** - % of repos with same error count
- **Performance** - Compile time comparison
- **Coverage** - % of FSH features exercised

Generate reports showing:
- Failing repos with diff details
- New failures since last run
- Performance trends
- Feature coverage gaps
