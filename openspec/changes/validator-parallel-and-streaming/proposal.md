## Why

The validator roadmap and CLI docs both call out throughput and large-batch ergonomics as remaining work: current sequential execution limits throughput, and large NDJSON workflows require manual chunking. A dedicated proposal is needed to define thread-safe validation internals and first-class streaming workflows.

## What Changes

- Refactor validation internals (including FHIRPath evaluator usage boundaries) for safe parallel execution.
- Add parallel batch validation for independent resources with deterministic result aggregation.
- Add streaming validation mode for very large NDJSON inputs to reduce memory pressure and improve CLI usability.
- Add performance benchmark/regression checks to protect throughput gains.

## Capabilities

### New Capabilities
- `parallel-batch-validation`: Multi-core validation pipeline for independent resources with stable output ordering.
- `streaming-ndjson-validation`: Incremental read/validate/report flow for large files without loading full datasets into memory.
- `performance-regression-guardrails`: Benchmarks and CI thresholds for throughput and latency-sensitive paths.

### Modified Capabilities
- `validator-cli-output-modes`: Extend current CLI validation command behavior for streaming and parallel execution modes.

## Impact

- Affected crates: `crates/rh-validator`, `crates/rh-fhirpath`, `apps/rh-cli`.
- Affected quality assets: benchmarks and CI workflows.
- Expected outcome: improved resources/sec and better operator experience on large validation batches.
