# rh-validator Performance Report

## Current Snapshot

- **Status:** Needs performance work
- **Measured:** July 3, 2026
- **Command:** `cargo bench -p rh-validator --bench validation`
- **Environment:** Apple M1 Max, 10 cores, 32 GiB RAM, macOS 26.4

The current validator is materially slower than the historical December 2024 Phase 8 report. The latest measured simple US Core Patient validation is about **82 ms/resource**, and warmed batch throughput is about **12 resources/sec**.

## Performance Targets & Results

| Metric | Target | Latest Result | Status |
|--------|--------|---------------|--------|
| Single validation latency | < 5 ms | 82.21 ms | Miss |
| Auto-detected validation latency | < 5 ms | 163.88 ms | Miss |
| Batch throughput | > 500 resources/sec | 12.05-12.31 resources/sec | Miss |
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
