## Why

`rh-foundation` docs outline multiple planned enhancements in package loading and snapshot generation (offline resilience, integrity and maintenance tooling, snapshot validation, and incremental/parallel generation). Capturing this as a proposal enables coordinated work across foundation services and CLI-facing operations.

## What Changes

- Add resilient package loading behavior for constrained/offline environments.
- Add package maintenance utilities (cleanup/pruning/integrity workflows) to reduce operational friction.
- Add snapshot validation and incremental regeneration workflows for profile-heavy pipelines.

## Capabilities

### New Capabilities
- `resilient-package-loading`: Offline/stale-cache tolerant package access with clear freshness and fallback behavior.
- `package-maintenance-utilities`: Operational utilities for pruning caches, verifying package integrity, and managing local package state.
- `snapshot-validation-and-incremental-generation`: Validate snapshot correctness and regenerate snapshots incrementally where inputs changed.

### Modified Capabilities
- `foundation-cli-surface`: Extend CLI-facing package/profile operations to expose the new loader and snapshot capabilities.

## Impact

- Affected crate: `crates/rh-foundation`.
- Affected integrations: package/snapshot workflows consumed by `rh-validator`, `rh-codegen`, and `apps/rh-cli`.
- Expected outcome: more reliable package/snapshot infrastructure and less manual recovery work in development and CI.
