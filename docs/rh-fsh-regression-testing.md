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

Port SUSHI's test fixtures from `test/` directory:

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

### Level 5: Performance Testing

Beyond correctness testing, rh-fsh needs dedicated performance benchmarks using very large synthetic IGs to validate scalability, memory efficiency, and compilation speed.

#### Synthetic IG Generation

Create test IGs at different scales to stress-test the compiler:

```rust
// src/bin/gen-perf-ig.rs

use rand::Rng;

struct PerfIgConfig {
    /// Number of profiles to generate
    profile_count: usize,
    /// Number of extensions
    extension_count: usize,
    /// Number of value sets
    valueset_count: usize,
    /// Number of code systems
    codesystem_count: usize,
    /// Number of instances/examples
    instance_count: usize,
    /// Number of questionnaires (definitional resources using profiles)
    questionnaire_count: usize,
    /// Number of activity definitions (definitional resources)
    activitydefinition_count: usize,
    /// Number of plan definitions (definitional resources)
    plandefinition_count: usize,
    /// Number of libraries with CQL content
    library_count: usize,
    /// Number of measure definitions
    measure_count: usize,
    /// Average rules per profile
    rules_per_profile: usize,
    /// Average codes per value set
    codes_per_valueset: usize,
    /// Average items per questionnaire
    items_per_questionnaire: usize,
}

impl PerfIgConfig {
    fn small() -> Self {
        Self {
            profile_count: 100,
            extension_count: 50,
            valueset_count: 50,
            codesystem_count: 10,
            instance_count: 200,
            questionnaire_count: 20,
            activitydefinition_count: 15,
            plandefinition_count: 10,
            library_count: 10,
            measure_count: 10,
            rules_per_profile: 20,
            codes_per_valueset: 50,
            items_per_questionnaire: 10,
        }
    }

    fn medium() -> Self {
        Self {
            profile_count: 500,
            extension_count: 200,
            valueset_count: 200,
            codesystem_count: 50,
            instance_count: 1000,
            questionnaire_count: 100,
            activitydefinition_count: 75,
            plandefinition_count: 50,
            library_count: 50,
            measure_count: 50,
            rules_per_profile: 30,
            codes_per_valueset: 100,
            items_per_questionnaire: 15,
        }
    }

    fn large() -> Self {
        Self {
            profile_count: 2000,
            extension_count: 800,
            valueset_count: 800,
            codesystem_count: 200,
            instance_count: 5000,
            questionnaire_count: 400,
            activitydefinition_count: 300,
            plandefinition_count: 200,
            library_count: 200,
            measure_count: 200,
            rules_per_profile: 40,
            codes_per_valueset: 200,
            items_per_questionnaire: 20,
        }
    }

    fn xl() -> Self {
        Self {
            profile_count: 10000,
            extension_count: 4000,
            valueset_count: 4000,
            codesystem_count: 1000,
            instance_count: 25000,
            questionnaire_count: 2000,
            activitydefinition_count: 1500,
            plandefinition_count: 1000,
            library_count: 1000,
            measure_count: 1000,
            rules_per_profile: 50,
            codes_per_valueset: 300,
            items_per_questionnaire: 25,
        }
    }
}

fn generate_perf_ig(config: &PerfIgConfig, output_dir: &Path) {
    let mut fsh_content = String::new();

    // Generate extensions
    for i in 0..config.extension_count {
        fsh_content.push_str(&format!(r#"
Extension: PerfExt{i}
Id: perf-ext-{i}
Title: "Performance Extension {i}"
Description: "Generated extension for performance testing"
* value[x] only string
"#));
    }

    // Generate profiles with many rules
    for i in 0..config.profile_count {
        fsh_content.push_str(&format!(r#"
Profile: PerfProfile{i}
Parent: {parent}
Id: perf-profile-{i}
Title: "Performance Profile {i}"
Description: "Generated profile for performance testing"
"#, parent = select_parent(i)));

        // Add rules
        for j in 0..config.rules_per_profile {
            let rule = generate_rule(i, j);
            fsh_content.push_str(&format!("* {rule}\n"));
        }
    }

    // Generate value sets
    for i in 0..config.valueset_count {
        fsh_content.push_str(&format!(r#"
ValueSet: PerfVS{i}
Id: perf-vs-{i}
Title: "Performance ValueSet {i}"
Description: "Generated value set for performance testing"
"#));

        // Add codes
        for j in 0..config.codes_per_valueset {
            fsh_content.push_str(&format!(
                "* http://example.org/cs#{code} \"Code {code}\"\n",
                code = format!("CODE{i}_{j}")
            ));
        }
    }

    // Generate code systems
    for i in 0..config.codesystem_count {
        fsh_content.push_str(&format!(r#"
CodeSystem: PerfCS{i}
Id: perf-cs-{i}
Title: "Performance CodeSystem {i}"
Description: "Generated code system for performance testing"
"#));

        let code_count = config.codes_per_valueset * 2;
        for j in 0..code_count {
            fsh_content.push_str(&format!(
                "* #{code} \"Display for {code}\"\n",
                code = format!("CODE{i}_{j}")
            ));
        }
    }

    // Generate instances
    for i in 0..config.instance_count {
        let profile_ref = i % config.profile_count;
        fsh_content.push_str(&format!(r#"
Instance: PerfInstance{i}
InstanceOf: PerfProfile{profile_ref}
Usage: #example
Title: "Performance Instance {i}"
Description: "Generated instance for performance testing"
* status = #active
"#));
    }

    // Generate questionnaires (definitional resources using profiles)
    for i in 0..config.questionnaire_count {
        fsh_content.push_str(&format!(r#"
Instance: PerfQuestionnaire{i}
InstanceOf: Questionnaire
Usage: #definition
Title: "Performance Questionnaire {i}"
Description: "Generated questionnaire for performance testing"
* status = #active
* subjectType = #Patient
"#));

        // Add items referencing profiles
        for j in 0..config.items_per_questionnaire {
            let profile_ref = (i * config.items_per_questionnaire + j) % config.profile_count;
            fsh_content.push_str(&format!(r#"
* item[{j}].linkId = "item-{j}"
* item[{j}].text = "Question {j}"
* item[{j}].type = #reference
* item[{j}].extension[+].url = "http://hl7.org/fhir/StructureDefinition/questionnaire-allowedProfile"
* item[{j}].extension[=].valueCanonical = "http://example.org/perf-ig/StructureDefinition/perf-profile-{profile_ref}"
"#));
        }
    }

    // Generate activity definitions
    for i in 0..config.activitydefinition_count {
        let profile_ref = i % config.profile_count;
        fsh_content.push_str(&format!(r#"
Instance: PerfActivityDefinition{i}
InstanceOf: ActivityDefinition
Usage: #definition
Title: "Performance ActivityDefinition {i}"
Description: "Generated activity definition for performance testing"
* status = #active
* kind = #Task
* profile = "http://example.org/perf-ig/StructureDefinition/perf-profile-{profile_ref}"
"#));
    }

    // Generate plan definitions
    for i in 0..config.plandefinition_count {
        fsh_content.push_str(&format!(r#"
Instance: PerfPlanDefinition{i}
InstanceOf: PlanDefinition
Usage: #definition
Title: "Performance PlanDefinition {i}"
Description: "Generated plan definition for performance testing"
* status = #active
* action[0].title = "Action 1"
* action[0].definitionCanonical = "http://example.org/perf-ig/ActivityDefinition/perf-activitydefinition-{activity}"
* action[1].title = "Action 2"
* action[1].definitionCanonical = "http://example.org/perf-ig/ActivityDefinition/perf-activitydefinition-{activity2}"
"#, activity = i % config.activitydefinition_count,
   activity2 = (i + 1) % config.activitydefinition_count));
    }

    // Generate libraries with CQL content
    for i in 0..config.library_count {
        let cql_content = generate_cql_library(i, config);
        let cql_base64 = base64_encode(&cql_content);

        fsh_content.push_str(&format!(r#"
Instance: PerfLibrary{i}
InstanceOf: Library
Usage: #definition
Title: "Performance Library {i}"
Description: "Generated library with CQL for performance testing"
* status = #active
* type = http://terminology.hl7.org/CodeSystem/library-type#logic-library
* content.contentType = #text/cql
* content.data = "{cql_base64}"
"#));
    }

    // Generate measures
    for i in 0..config.measure_count {
        let library_ref = i % config.library_count;
        let vs_ref = i % config.valueset_count;

        fsh_content.push_str(&format!(r#"
Instance: PerfMeasure{i}
InstanceOf: Measure
Usage: #definition
Title: "Performance Measure {i}"
Description: "Generated measure for performance testing"
* status = #active
* library = "http://example.org/perf-ig/Library/perf-library-{library_ref}"
* scoring = http://terminology.hl7.org/CodeSystem/measure-scoring#proportion
* group[0].population[0].code = http://terminology.hl7.org/CodeSystem/measure-population#initial-population
* group[0].population[0].criteria.language = #text/cql
* group[0].population[0].criteria.expression = "Initial Population"
* supplementalData[0].code.text = "Supplemental Data"
* supplementalData[0].criteria.language = #text/cql
* supplementalData[0].criteria.expression = "SDE"
"#));
    }

    // Write to file
    std::fs::write(
        output_dir.join("performance-test.fsh"),
        fsh_content
    ).expect("Failed to write FSH file");

    // Generate sushi-config.yaml
    let config_yaml = format!(r#"
id: performance-test-ig
canonical: http://example.org/perf-ig
name: PerformanceTestIG
title: "Performance Test IG"
status: draft
version: 1.0.0
fhirVersion: 4.0.1
copyrightYear: 2025+
releaseLabel: ci-build
"#);

    std::fs::write(
        output_dir.join("sushi-config.yaml"),
        config_yaml
    ).expect("Failed to write config");
}

fn select_parent(index: usize) -> &'static str {
    match index % 10 {
        0 => "Patient",
        1 => "Observation",
        2 => "Condition",
        3 => "Procedure",
        4 => "Medication",
        5 => "Encounter",
        6 => "Organization",
        7 => "Practitioner",
        8 => "Device",
        9 => "Specimen",
        _ => unreachable!(),
    }
}

fn generate_rule(profile_idx: usize, rule_idx: usize) -> String {
    match rule_idx % 5 {
        0 => format!("identifier {}..*", rule_idx % 3),
        1 => "name 1..1 MS".to_string(),
        2 => format!("extension[PerfExt{}] 0..1", (profile_idx + rule_idx) % 100),
        3 => "active = true".to_string(),
        4 => format!("status from PerfVS{}", (profile_idx + rule_idx) % 50),
        _ => unreachable!(),
    }
}

fn generate_cql_library(index: usize, config: &PerfIgConfig) -> String {
    let profile_ref = index % config.profile_count;
    let vs_ref = index % config.valueset_count;

    format!(r#"
library PerfLibrary{index} version '1.0.0'

using FHIR version '4.0.1'

include FHIRHelpers version '4.0.1'

valueset "Performance ValueSet": 'http://example.org/perf-ig/ValueSet/perf-vs-{vs_ref}'

context Patient

define "Initial Population":
  exists([Observation: "Performance ValueSet"])

define "Denominator":
  "Initial Population"

define "Numerator":
  exists([Observation: "Performance ValueSet"] O
    where O.status = 'final'
  )

define "SDE":
  Patient.gender

define "Profile Reference":
  [Observation] O
    where O.meta.profile contains 'http://example.org/perf-ig/StructureDefinition/perf-profile-{profile_ref}'
"#)
}

fn base64_encode(content: &str) -> String {
    // Simplified - use actual base64 encoding in real implementation
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(content.as_bytes())
}
```

#### Performance Benchmarks

Track key metrics across different scales:

```rust
// tests/performance_benchmarks.rs

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rh_fsh::compile_project;

fn benchmark_compilation(c: &mut Criterion) {
    let mut group = c.benchmark_group("compilation");

    for size in ["small", "medium", "large", "xl"] {
        let ig_path = format!("perf-test-data/ig-{size}");

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &ig_path,
            |b, path| {
                b.iter(|| {
                    compile_project(path).expect("Compilation failed")
                });
            }
        );
    }

    group.finish();
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory");
    group.sample_size(10); // Fewer iterations for memory tests

    for size in ["small", "medium", "large"] {
        let ig_path = format!("perf-test-data/ig-{size}");

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &ig_path,
            |b, path| {
                b.iter_custom(|iters| {
                    let start = std::time::Instant::now();
                    let start_mem = current_memory_usage();

                    for _ in 0..iters {
                        compile_project(path).expect("Compilation failed");
                    }

                    let peak_mem = peak_memory_usage();
                    let elapsed = start.elapsed();

                    // Log memory stats
                    println!("Peak memory: {} MB", peak_mem / 1024 / 1024);
                    println!("Memory delta: {} MB", (peak_mem - start_mem) / 1024 / 1024);

                    elapsed
                });
            }
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_compilation, benchmark_memory_usage);
criterion_main!(benches);
```

#### Performance Test Suite Structure

```
perf-test-data/
├── ig-small/          # 100 profiles, 65 definitional resources, ~20MB FSH
│   ├── input/fsh/
│   │   ├── profiles.fsh
│   │   ├── extensions.fsh
│   │   ├── valuesets.fsh
│   │   ├── questionnaires.fsh
│   │   ├── activitydefinitions.fsh
│   │   ├── plandefinitions.fsh
│   │   ├── libraries.fsh
│   │   └── measures.fsh
│   └── sushi-config.yaml
├── ig-medium/         # 500 profiles, 325 definitional resources, ~100MB FSH
│   ├── input/fsh/
│   └── sushi-config.yaml
├── ig-large/          # 2000 profiles, 1300 definitional resources, ~400MB FSH
│   ├── input/fsh/
│   └── sushi-config.yaml
├── ig-xl/             # 10000 profiles, 6500 definitional resources, ~2GB FSH
│   ├── input/fsh/
│   └── sushi-config.yaml
└── README.md
```

#### Tracked Metrics

| Metric | Target (small) | Target (medium) | Target (large) | Target (xl) |
|--------|----------------|-----------------|----------------|-------------|
| Compilation time | <5s | <30s | <2min | <10min |
| Peak memory | <500MB | <2GB | <8GB | <32GB |
| Memory per profile | <5KB | <4KB | <4KB | <3KB |
| Throughput | 20 prof/s | 17 prof/s | 17 prof/s | 17 prof/s |

#### CI Integration

```yaml
# .github/workflows/performance.yml

name: Performance Benchmarks

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  performance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Generate Performance Test IGs
        run: |
          cargo run --release --bin gen-perf-ig -- --size small
          cargo run --release --bin gen-perf-ig -- --size medium
          cargo run --release --bin gen-perf-ig -- --size large

      - name: Run Benchmarks
        run: cargo bench --bench performance_benchmarks

      - name: Compare with SUSHI
        run: |
          npm install -g fsh-sushi
          ./scripts/compare-performance.sh

      - name: Upload Results
        uses: actions/upload-artifact@v4
        with:
          name: performance-results
          path: target/criterion/

      - name: Check Performance Regression
        run: |
          # Fail if performance degrades by >20%
          ./scripts/check-perf-regression.sh
```

#### Comparison with SUSHI

For each scale, compare:

```rust
struct PerformanceComparison {
    ig_size: String,
    rh_fsh_time: Duration,
    sushi_time: Duration,
    rh_fsh_memory: usize,
    sushi_memory: usize,
    speedup: f64,
    memory_ratio: f64,
}

fn compare_performance(ig_path: &Path) -> PerformanceComparison {
    // Run rh-fsh with profiling
    let (rh_time, rh_mem) = profile_compilation(|| {
        Command::new("rh-fsh")
            .arg(ig_path)
            .output()
            .expect("rh-fsh failed")
    });

    // Run SUSHI with profiling
    let (sushi_time, sushi_mem) = profile_compilation(|| {
        Command::new("sushi")
            .arg(ig_path)
            .output()
            .expect("sushi failed")
    });

    PerformanceComparison {
        ig_size: ig_path.file_name().unwrap().to_string_lossy().to_string(),
        rh_fsh_time: rh_time,
        sushi_time: sushi_time,
        rh_fsh_memory: rh_mem,
        sushi_memory: sushi_mem,
        speedup: sushi_time.as_secs_f64() / rh_time.as_secs_f64(),
        memory_ratio: rh_mem as f64 / sushi_mem as f64,
    }
}
```

#### Performance Regression Detection

Track performance over time:

```rust
// Track baseline performance
struct PerformanceBaseline {
    commit: String,
    timestamp: DateTime<Utc>,
    benchmarks: HashMap<String, BenchmarkResult>,
}

// Fail CI if performance regresses significantly
fn check_regression(current: &BenchmarkResult, baseline: &BenchmarkResult) -> bool {
    let time_regression = (current.time - baseline.time) / baseline.time;
    let mem_regression = (current.memory as f64 - baseline.memory as f64) / baseline.memory as f64;

    // Fail if >20% slower or >30% more memory
    time_regression > 0.20 || mem_regression > 0.30
}
```

#### Real-World Performance Testing

Beyond synthetic IGs, test against the largest real-world IGs:

**Large Real IGs for Testing:**
- `HL7/fhir-mCODE-ig` (~200 profiles)
- `HL7/davinci-pas` (~150 profiles)
- `HL7/US-Core` (~60 profiles, complex rules)
- Combined multi-IG projects

**Stress Testing Scenarios:**
1. **Deep inheritance chains** - Profile inheriting from profile 10+ levels deep
2. **Wide value sets** - Value sets with 10,000+ codes
3. **Complex rules** - Profiles with 100+ constraints
4. **Circular references** - Detecting and handling circular dependencies
5. **Massive instances** - Instance examples with deeply nested structures
6. **Complex CQL libraries** - Libraries with many dependencies and complex logic
7. **Questionnaires with deep nesting** - Questionnaires with 50+ nested item groups
8. **PlanDefinitions with action graphs** - Plans with complex action dependencies
9. **Measures with multiple populations** - Measure definitions with stratifiers and supplemental data
10. **Cross-resource dependencies** - ActivityDefinitions → Questionnaires → Profiles chains

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
