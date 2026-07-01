# Validator Follow-up TODO

The following validation gaps are intentionally out of scope for the current
FHIR R4 validation tightening work. Do not add tests for these items until they
are actively being implemented.

- Recursively validate `Bundle.entry[].resource` for transaction and batch
  bundles, preserving path and resource identity context in diagnostics.
- Add a package-directory validation command such as
  `rh validate package output/package --fhir-version 4.0.1` that skips
  metadata JSON and validates every JSON object with `resourceType`.
- Validate generated package contents from package/check workflows so
  `rh package check` does not pass when generated resources would fail resource
  validation.
- Ensure package/check or package-validation workflows catch
  `ImplementationGuide.dependsOn` entries missing required `uri` fields when
  applying the relevant package validation mode.
