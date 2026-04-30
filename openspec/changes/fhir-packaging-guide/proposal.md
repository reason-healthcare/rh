## Why

Users of `rh-packager` need a concrete, end-to-end worked example that demonstrates how to assemble a real, well-formed FHIR Package using every built-in processor (FSH, snapshot, validate, CQL, and shell). Without a practical guide, adopters must piece together the workflow from reference docs alone, which creates friction and leaves the processor pipeline underutilized.

## What Changes

- Add `GUIDE.md` to `crates/rh-packager/` — a step-by-step FHIR packaging walkthrough that takes a user from an empty directory to a publishable `.tgz` FHIR Package
- The guide covers:
  - Project layout and `package.json`
  - `packager.toml` configuration
  - Authoring FHIR Shorthand (`.fsh`) source files
  - Running the FSH processor to compile resources
  - Running the snapshot processor to expand profiles
  - Running the validate processor to enforce conformance
  - Authoring CQL libraries and running the CQL processor
  - Adding a shell processor step (e.g., a custom enrichment script)
  - Assembling the final FHIR Package tarball

## Capabilities

### New Capabilities

- `fhir-packaging-guide`: End-to-end guide spec covering the structure, content, and correctness requirements of the step-by-step packaging walkthrough document

### Modified Capabilities

(none)

## Impact

- New file: `crates/rh-packager/GUIDE.md`
- `crates/rh-packager/README.md` — add a link to `GUIDE.md` from the "Getting Started" section
- No API or behavioral changes to `rh-packager` itself
