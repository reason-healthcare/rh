## 1. Project Setup Section

- [x] 1.1 Write introduction paragraph explaining what the guide covers and prerequisites (`rh` installed, FHIR R4 packages cached)
- [x] 1.2 Show the complete example project directory layout with all files that will be created
- [x] 1.3 Write `package.json` example with all required fields (`name`, `version`, `fhirVersions`, `dependencies`)
- [x] 1.4 Write `ImplementationGuide.json` stub that syncs with `package.json`
- [x] 1.5 Write the complete `packager.toml` wiring all built-in processors in order: `fsh` → `snapshot` → `validate` → `cql` → `shell`

## 2. FSH Processor Step

- [x] 2.1 Write the FSH step intro explaining what FSH is and what the `fsh` processor does
- [x] 2.2 Show an example `.fsh` file defining a `BpObservation` profile (StructureDefinition + ValueSet)
- [x] 2.3 Explain that the processor scans for `*.fsh` recursively and emits FHIR JSON into the pipeline

## 3. Snapshot Processor Step

- [x] 3.1 Write the snapshot step intro explaining why snapshot expansion is needed
- [x] 3.2 Show the `[snapshot]` section in `packager.toml` (packages_dir)
- [x] 3.3 Explain what resources are affected and what the expanded output looks like

## 4. Validate Processor Step

- [x] 4.1 Write the validate step intro explaining what the validator checks
- [x] 4.2 Show the `[validate]` section in `packager.toml`
- [x] 4.3 Show example validator output for a passing build
- [x] 4.4 Describe pipeline-abort behavior on validation failure and what the error output looks like

## 5. CQL Processor Step

- [x] 5.1 Write the CQL step intro explaining that `.cql` files are compiled to ELM and upserted as `Library` resources
- [x] 5.2 Show an example `BpCheck.cql` library that references the `BpObservation` profile
- [x] 5.3 Show the `[cql]` section in `packager.toml`
- [x] 5.4 Explain the output `Library-BpCheck.json` resource that is added to the package

## 6. Shell Processor Step (Basic)

- [x] 6.1 Write the basic shell step intro showing the mechanism with a simple built-in example
- [x] 6.2 Show a `[processors.build-info]` entry that runs `echo` to write `other/build-info.txt`
- [x] 6.3 Document the key environment variables (`PACKAGER_SOURCE_DIR`, `PACKAGER_OUTPUT_DIR`, `PACKAGER_WORKDIR`, `PACKAGER_PACKAGE_NAME`)
- [x] 6.4 Explain the resource exchange protocol (`$PACKAGER_WORKDIR/resources/`)

## 7. Extended Example — Python Hooks

- [x] 7.1 Write intro for the "Extending the Pipeline" section explaining the Python hook pattern
- [x] 7.2 Write complete `stamp_date.py` script that reads resources from `$PACKAGER_WORKDIR/resources/`, adds a `meta.tag` build-date entry, and writes them back
- [x] 7.3 Write complete `check_date.py` script that reads resources from `$PACKAGER_WORKDIR/resources/` and exits non-zero if any resource is missing the build-date tag
- [x] 7.4 Show the `packager.toml` snippet registering both scripts as `[processors.stamp-date]` and `[processors.check-date]` with correct hook stage placement
- [x] 7.5 Show example pipeline output when `check_date.py` finds a resource missing the tag (non-zero exit, pipeline abort message)

## 8. Final Package Output

- [x] 8.1 Show the `rh publish build <dir>` command and expected terminal output
- [x] 8.2 Show the resulting `output/package/` directory tree with all compiled resources, narrative, and metadata
- [x] 8.3 Describe the generated `.tgz` filename convention (`<name>-<version>.tgz`)

## 9. README Link

- [x] 9.1 Add a "Getting Started" or "Quickstart" section to `crates/rh-packager/README.md` with a link to `GUIDE.md`
