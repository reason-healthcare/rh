# rh-validator Performance Report

## Current Snapshot

- **Status:** Current performance branch meets the documented smoke targets
- **Measured:** July 3, 2026
- **Command:** `cargo bench -p rh-validator --bench validation`
- **Environment:** Apple M1 Max, 10 cores, 32 GiB RAM, macOS 26.4

The full Criterion baseline from `origin/main` was materially slower than the
historical December 2024 Phase 8 report: simple US Core Patient validation was
about **82 ms/resource**, and warmed batch throughput was about **12
resources/sec**. On the current performance branch, the reduced smoke harness
measures simple US Core Patient validation at about **170 us/resource** and
warmed batch-50 throughput at about **5,807 resources/sec**.

## Performance Targets & Results

| Metric | Target | Latest Result | Status |
|--------|--------|---------------|--------|
| Single validation latency | < 5 ms | 169.87 us | Pass |
| Auto-detected validation latency | < 5 ms | 245.32 us | Pass |
| Batch throughput | > 500 resources/sec | ~5,807 resources/sec | Pass |
| Cache hit rate after warmup | > 90% | 100.0% | Pass |

## Benchmark Results

### Criterion Benchmarks

These results come from Criterion estimates saved under `target/criterion/*/new/estimates.json`.

| Benchmark | Mean | 95% CI |
|-----------|------|--------|
| `validate_simple_patient` | 82.21 ms | 81.92-82.48 ms |
| `validate_complex_patient` | 635.67 ms | 633.51-638.01 ms |
| `validate_auto_detect` | 163.88 ms | 163.09-164.74 ms |

The full Criterion batch group was not completed because the current timings would require a long run for 100 samples across all batch sizes. Batch measurements below were collected with the same validator setup and simple patient payload after one warmup validation.

### Perf Smoke After Initial Cache Sharing

The reduced smoke harness in `benches/perf_smoke.rs` uses sample size 10 and
omits the 100- and 500-resource batch cases. After changing snapshot and
compiled-rule caches to return shared `Arc` values, the smoke run measured:

| Benchmark | Mean | Change |
|-----------|------|--------|
| `validate_simple_patient` | 74.311 ms | -10.387% |
| `validate_auto_detect` | 149.39 ms | -7.2423% |
| `validate_complex_patient` | 611.96 ms | -4.4865% |
| `warmed_batch_validation/10` | 758.86 ms | -8.4037% |
| `warmed_batch_validation/50` | 3.7392 s | -4.3580% |

Warm cache probe from the same smoke run:

```text
Profile Cache: 6/6 hits (100.0% hit rate)
Rule Cache: 1/1 hits (100.0% hit rate)
```

### Perf Smoke After ValueSet Membership Caching

Binding validation was the dominant cost in the feature-enabled timing
breakdown, spending about 80 ms in the explicit simple-patient path. After
adding a local membership-result cache for repeated `(ValueSet, system, code)`
checks, the reduced smoke harness measured:

| Benchmark | Mean | Change |
|-----------|------|--------|
| `validate_simple_patient` | 535.39 us | approximately -99.3% from pre-cache binding path |
| `validate_auto_detect` | 801.16 us | -99.482% |
| `validate_complex_patient` | 198.07 ms | -67.634% |
| `warmed_batch_validation/10` | 5.4828 ms | -99.273% |
| `warmed_batch_validation/50` | 26.673 ms | -99.282% |

The feature-enabled simple-patient timing breakdown after this change shows
binding validation at 0.006 ms and invariant validation as the largest remaining
simple-patient section at about 0.445 ms.

### Perf Smoke After CodeSystem Membership Caching

Complex Patient timing showed the remaining latency was dominated by
`validate_known_coding_codes`, which repeatedly checked package CodeSystems.
After adding a local `(CodeSystem, code)` membership-result cache, the reduced
smoke harness measured:

| Benchmark | Mean |
|-----------|------|
| `validate_simple_patient` | 539.20 us |
| `validate_auto_detect` | 791.21 us |
| `validate_complex_patient` | 1.6895 ms |
| `warmed_batch_validation/10` | 5.3037 ms |
| `warmed_batch_validation/50` | 26.729 ms |

The feature-enabled complex-patient timing breakdown after this change shows
base validation at 0.205 ms, known coding code validation at 0.011 ms, and
invariant validation as the largest section at about 1.393 ms.

### Perf Smoke After Invariant Parse Caching And Native Core Invariants

Invariant validation became the largest measured section after terminology
membership caching. Adding a validator-level parsed FHIRPath expression cache
and native fast paths for the common `ele-1` and `ext-1` invariants produced:

| Benchmark | Mean |
|-----------|------|
| `validate_simple_patient` | 187.95 us |
| `validate_auto_detect` | 270.36 us |
| `validate_complex_patient` | 416.31 us |
| `warmed_batch_validation/10` | 1.8808 ms |
| `warmed_batch_validation/50` | 9.3392 ms |

The feature-enabled timing breakdown now shows invariant validation at about
0.095 ms for simple Patient and 0.162 ms for complex Patient.

### Perf Smoke After Native Period Invariant

Adding a native fast path for the `per-1` Period invariant preserved the
existing local precision/order semantics. The Patient smoke cases are not
Period-heavy, so this change was expected to be neutral for the headline
benchmarks:

| Benchmark | Mean |
|-----------|------|
| `validate_simple_patient` | 188.46 us |
| `validate_auto_detect` | 268.28 us |
| `validate_complex_patient` | 413.94 us |
| `warmed_batch_validation/10` | 1.8789 ms |
| `warmed_batch_validation/50` | 9.2203 ms |

The feature-enabled timing breakdown after this change shows invariant
validation at about 0.092 ms for simple Patient and 0.161 ms for complex
Patient.

### Perf Smoke After Precomputed Unknown-Property Maps

Unknown-property validation previously rebuilt allowed-child and choice-prefix
maps from compiled element paths for every resource validation. Moving those
maps into `CompiledValidationRules` reduced warmed validation cost:

| Benchmark | Mean | Change |
|-----------|------|--------|
| `validate_simple_patient` | 166.97 us | -9.7695% |
| `validate_auto_detect` | 240.74 us | -11.065% |
| `validate_complex_patient` | 396.41 us | -3.9710% |
| `warmed_batch_validation/10` | 1.6596 ms | -11.423% |
| `warmed_batch_validation/50` | 8.4175 ms | -11.058% |

The feature-enabled timing breakdown after this change shows unknown-property
validation at about 0.003 ms for simple Patient and 0.005 ms for complex
Patient.

### Full Criterion Benchmark After A4

The full `benches/validation.rs` Criterion suite was refreshed after the A4
changes. These measurements use 100 samples and include the larger batch groups
that the smoke harness omits:

| Benchmark | Mean | Approx throughput |
|-----------|------|-------------------|
| `validate_simple_patient` | 169.87 us | 5,887/sec |
| `validate_complex_patient` | 392.87 us | 2,545/sec |
| `validate_auto_detect` | 245.32 us | 4,076/sec |
| `batch_validation/10` | 1.7261 ms | 5,793/sec |
| `batch_validation/50` | 8.6101 ms | 5,807/sec |
| `batch_validation/100` | 16.987 ms | 5,887/sec |
| `batch_validation/500` | 85.514 ms | 5,847/sec |
| `validate_cached_profile` | 171.28 us | 5,838/sec |

Full benchmark cache metrics:

```text
Profile Cache: 378402/378402 hits (100.0% hit rate)
Rule Cache: 63067/63067 hits (100.0% hit rate)
```

### Batch Validation

| Batch Size | Elapsed | Per Resource | Throughput |
|------------|---------|--------------|------------|
| 10 | 819.11 ms | 81.91 ms | 12.21 resources/sec |
| 50 | 4.06 s | 81.22 ms | 12.31 resources/sec |
| 100 | 8.24 s | 82.36 ms | 12.14 resources/sec |
| 500 | 41.50 s | 83.00 ms | 12.05 resources/sec |

**Analysis:** Batch performance is linear and sequential. Cache effectiveness remains strong, but the current hot path is expensive enough that cache hits do not keep validation near the original latency or throughput targets.

### Cache Performance

Warmed batch probe across 660 measured validations:

```text
Profile Cache Hit Rate: 100.0% (3960 hits, 0 misses)
Rule Cache Hit Rate:    100.0% (660 hits, 0 misses)
```

## Implementation Details

### Cache Behavior

The validator still exposes cache instrumentation through `cache_metrics()` and `reset_cache_metrics()`. Current warm-cache measurements show no misses in the measured path:

- Profile/snapshot cache: 100-entry LRU
- Rule cache: 100-entry LRU
- Warm validation path: 100% hit rate in the latest batch probe

### Benchmark Suite

The Criterion suite lives in `benches/validation.rs` and includes:

- `bench_simple_patient_validation`: baseline US Core Patient validation
- `bench_complex_patient_validation`: Patient with extensions
- `bench_auto_detect_validation`: profile detection overhead
- `bench_batch_validation`: batch sizes 10, 50, 100, and 500
- `bench_cache_performance`: cache hit/miss measurement

## Performance Characteristics

- **Single-resource latency:** Current simple patient validation is ~82 ms.
- **Complex profile cost:** Extension-heavy patient validation is ~636 ms.
- **Auto-detect overhead:** `validate_auto` is about 2x the explicit simple profile path.
- **Batch scaling:** Per-resource time stays around 81-83 ms across 10 to 500 resources.
- **Primary bottleneck:** Sequential validation hot path, not cache miss behavior.

## Optimization Opportunities

1. **Profile validation hot-path profiling** (High Impact)
   - Profile where the ~82 ms simple-patient path is spent.
   - Focus first on snapshot traversal, rule evaluation, and repeated path/FHIRPath work.

2. **Auto-detect deduplication** (High Impact)
   - Investigate why `validate_auto` is roughly 2x explicit profile validation.
   - Avoid duplicate base/profile work when `meta.profile` contains one known profile.

3. **Compiled FHIRPath/rule reuse** (Medium to High Impact)
   - Cache parsed or compiled invariant expressions where possible.
   - Avoid repeated construction in warmed validations.

4. **Parallel batch validation** (High Impact after hot-path work)
   - Implement rayon-based batch validation once shared evaluator state is thread-safe.
   - Parallelism can improve throughput, but it will not fix single-resource latency.

5. **JSON/path allocation reduction** (Medium Impact)
   - Audit cloning and temporary allocation in profile traversal and path construction.

## Running Benchmarks

```bash
# Run all validator benchmarks
cargo bench -p rh-validator --bench validation

# Run one benchmark
cargo bench -p rh-validator --bench validation -- validate_simple_patient

# View HTML reports
open target/criterion/report/index.html
```

## Notes

These metrics are local-machine measurements and should be refreshed after any validator hot-path change. Because the current batch Criterion group is slow, use a reduced sample size or a focused batch probe when validating batch performance during iterative optimization.
