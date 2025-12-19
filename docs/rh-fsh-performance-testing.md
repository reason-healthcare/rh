# rh-fsh Performance Testing Strategy

## Overview

Beyond correctness testing, rh-fsh needs dedicated performance benchmarks using very large synthetic IGs to validate scalability, memory efficiency, and compilation speed.

**Key Testing Requirements:**
- Test fixtures must include all three FHIR usage types: `#example`, `#definition`, `#inline`
- Definitional resources (Questionnaires, ActivityDefinitions, PlanDefinitions, Libraries, Measures) use `Usage: #definition`
- Example instances use `Usage: #example`
- Inline instances use `Usage: #inline` (typically for contained resources or intermediate representations)

## Synthetic IG Generation

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
    /// Number of instances with Usage: #example
    example_count: usize,
    /// Number of instances with Usage: #inline
    inline_count: usize,
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
            example_count: 150,
            inline_count: 50,
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
            example_count: 750,
            inline_count: 250,
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
            example_count: 3750,
            inline_count: 1250,
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
            example_count: 18750,
            inline_count: 6250,
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

    // Generate example instances (Usage: #example)
    for i in 0..config.example_count {
        let profile_ref = i % config.profile_count;
        fsh_content.push_str(&format!(r#"
Instance: PerfExample{i}
InstanceOf: PerfProfile{profile_ref}
Usage: #example
Title: "Performance Example Instance {i}"
Description: "Generated example instance for performance testing"
* status = #active
"#));
    }

    // Generate inline instances (Usage: #inline)
    for i in 0..config.inline_count {
        let profile_ref = i % config.profile_count;
        fsh_content.push_str(&format!(r#"
Instance: PerfInline{i}
InstanceOf: PerfProfile{profile_ref}
Usage: #inline
Title: "Performance Inline Instance {i}"
Description: "Generated inline instance for performance testing"
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

## Performance Benchmarks

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

## Performance Test Suite Structure

```
perf-test-data/
├── ig-small/          # 100 profiles, 65 #definition, 150 #example, 50 #inline, ~20MB FSH
│   ├── input/fsh/
│   │   ├── profiles.fsh
│   │   ├── extensions.fsh
│   │   ├── valuesets.fsh
│   │   ├── examples.fsh          # Usage: #example
│   │   ├── inline.fsh            # Usage: #inline
│   │   ├── questionnaires.fsh    # Usage: #definition
│   │   ├── activitydefinitions.fsh
│   │   ├── plandefinitions.fsh
│   │   ├── libraries.fsh
│   │   └── measures.fsh
│   └── sushi-config.yaml
├── ig-medium/         # 500 profiles, 325 #definition, 750 #example, 250 #inline, ~100MB FSH
│   ├── input/fsh/
│   └── sushi-config.yaml
├── ig-large/          # 2000 profiles, 1300 #definition, 3750 #example, 1250 #inline, ~400MB FSH
│   ├── input/fsh/
│   └── sushi-config.yaml
├── ig-xl/             # 10000 profiles, 6500 #definition, 18750 #example, 6250 #inline, ~2GB FSH
│   ├── input/fsh/
│   └── sushi-config.yaml
└── README.md
```

## Tracked Metrics

| Metric | Target (small) | Target (medium) | Target (large) | Target (xl) |
|--------|----------------|-----------------|----------------|-------------|
| Compilation time | <5s | <30s | <2min | <10min |
| Peak memory | <500MB | <2GB | <8GB | <32GB |
| Memory per profile | <5KB | <4KB | <4KB | <3KB |
| Throughput | 20 prof/s | 17 prof/s | 17 prof/s | 17 prof/s |

## CI Integration

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

## Comparison with SUSHI

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

## Performance Regression Detection

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

## Real-World Performance Testing

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
