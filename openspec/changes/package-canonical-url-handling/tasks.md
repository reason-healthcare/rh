## 1. Canonical URL Helpers

- [ ] 1.1 Add packager canonical helper functions for derived resource URLs.
- [ ] 1.2 Add warning detection for canonicals that look exactly like `.../ImplementationGuide/{id}`.
- [ ] 1.3 Use R4 generated metadata to detect resource types with root `url` fields.
- [ ] 1.4 Add focused unit tests for canonical helper behavior and metadata-backed detection.

## 2. Package Metadata And IG Sync

- [ ] 2.1 Keep `PackageJson` emitting and deserializing the optional `canonical` field.
- [ ] 2.2 Derive expected `ImplementationGuide.url` as `{canonical}/ImplementationGuide/{id}`.
- [ ] 2.3 Change `ImplementationGuide.url` mismatch handling from failure to warning.
- [ ] 2.4 Keep `packageId`, `version`, and `fhirVersion` mismatches as blocking errors.
- [ ] 2.5 Add build/check tests for canonical roots that look like IG URLs.
- [ ] 2.6 Update `rh package init` to generate `ImplementationGuide.url` from the generated IG `id`.

## 3. Resource URL Diagnostics

- [ ] 3.1 Warn when existing canonical resource URLs differ from `{canonical}/{resourceType}/{id}`.
- [ ] 3.2 Ignore missing resource `url` fields without writing generated URLs.
- [ ] 3.3 Exclude `ImplementationGuide` from non-IG canonical resource URL mismatch warnings.
- [ ] 3.4 Add integration tests for warning-only resource URL mismatches.

## 4. Validate Processor Error Details

- [ ] 4.1 Collect validation ERROR issues with resource labels.
- [ ] 4.2 Return the first concrete validation error in the processor failure message.
- [ ] 4.3 Include a count of additional validation errors when more than one is present.
- [ ] 4.4 Add tests for improved validation failure messages.

## 5. Audit And Documentation

- [ ] 5.1 Update `crossbeam-epoch` to a version that satisfies the current RustSec advisory.
- [ ] 5.2 Update `Cargo.lock` with minimal unrelated dependency churn.
- [ ] 5.3 Update architecture dependency docs for any new packager dependency.
- [ ] 5.4 Run `cargo audit`, `cargo check -p rh-packager`, focused packager tests, and docs sync.
