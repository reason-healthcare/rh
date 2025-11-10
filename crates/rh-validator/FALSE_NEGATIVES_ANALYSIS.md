# FHIR Validator Test Analysis - False Negatives

## Summary

The validator currently has a **54% pass rate** (27/50 tests), with **23 false negatives** where valid resources are incorrectly reported as INVALID.

## Root Cause: ext-1 Invariant Over-Application

The primary issue is that the **ext-1 invariant** is being applied too broadly:

- **Invariant**: `ext-1: Must have either extensions or value[x], not both`
- **Scope**: Should only apply to Extension resource types or extension arrays
- **Current Behavior**: Being applied to ALL elements in the resource
- **Impact**: Creates 6-12 false positive errors per resource

## Examples of False Negatives

### contained.json
- **Expected**: VALID (Java validator)
- **Actual**: INVALID (12E/2W)
- **Issues**:
  - 10× ext-1 errors (false positives - no Extension elements in resource)
  - 1× dom-3 error (contained resource reference - likely false positive)
  - 1× que-2 error (link ID uniqueness - needs investigation)
  - 2× warnings (FHIRPath parsing issues)

### Other Affected Resources
```
fixed-quantity-binding-observation-2: 1E/1W
bundle-document-versioned-references-good: 10E/1W
snomed-ca: 12E/1W
patient-example-ra4: 8E/1W
dr-example-org-2: 8E/1W
medstmt-ips: 3E/1W
mycommunication.invalid: 4E/1W
mycommunication.valid.example: 4E/1W
care-plan: 6E/1W
mr-covid-m4: 12E/1W
vs-jurisdiction: 21E/1W
qr-enablewhen-me: 6E/1W
pat-security-good1: 8E/1W
pat-security-good2: 8E/1W
pat-security-good3: 8E/1W
sp-diff-expression: 4E/1W
bb-obs-value-is-not-quantity: 1E/1W
bb-obs-value-is-not-in-valueset: 1E/0W
bb-obs-value-is-not-quantity-or-string: 1E/1W
obs-temp: 1E/0W
patient-ig-good: 8E/1W
patient-min-none: 9E/1W
```

## Pattern Analysis

- **High error counts (8-21E)**: Multiple ext-1 false positives
- **Low error counts (1-3E)**: Different validation issues (bindings, cardinality, etc.)
- **Common warning**: dom-6 FHIRPath parsing issue with backticks

## Required Fix

The invariant validation logic needs to:

1. **Scope invariants properly**: Only apply element-level invariants to the correct element types
2. **Filter ext-1**: Only apply to:
   - Elements with `resourceType: "Extension"`
   - Arrays/fields named `extension` or `modifierExtension`
3. **Context-aware validation**: Check the element path/type before applying invariants

## Impact on Test Results

If ext-1 false positives were eliminated:
- **Current**: 27/50 passing (54%)
- **Estimated**: 40-45/50 passing (80-90%)

The remaining failures would be legitimate validation gaps (terminology bindings, reference validation, etc.) rather than invariant application bugs.
