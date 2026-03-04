# FHIR Validator Test Analysis - False Negatives

**Last Updated**: February 6, 2026  
**Test Suite**: 100 FHIR R4 tests  
**Pass Rate**: 70/100 (70.0%)  
**Failures**: 30 tests  
**False Negatives**: 1 test (pat-security-good2)

## Summary

After the ext-1 fix (Phase 12, Nov 24, 2025), the validator achieved **70% pass rate**, a **+15% improvement** from the 55% baseline. The ext-1 over-application issue has been **completely resolved**.

## Current Status

### Phase 12 Success: ext-1 Fix ✅

**Problem (Pre-Nov 24, 2025)**: The ext-1 invariant was being applied too broadly:  
- **Invariant**: `ext-1: Must have either extensions or value[x], not both`
- **Scope**: Should only apply to Extension resource types or extension arrays
- **Old Behavior**: Being applied to ALL elements in the resource
- **Impact**: Created 6-12 false positive errors per resource

**Solution (Commit 22259e8)**: Fixed invariant scoping to only apply ext-1 to resources/elements with extensions

**Result**: Eliminated massive false positive issue, improved pass rate from 55% to 70%

## Phase 15 Analysis: Remaining 30 Failures

See [PHASE_15_ANALYSIS.md](PHASE_15_ANALYSIS.md) for comprehensive breakdown.

### Critical False Negative: pat-security-good2 🔴

**Test**: pat-security-good2  
**Expected**: VALID (Java validator)  
**Actual**: INVALID (rh-validator)  
**Status**: **CRITICAL BUG - requires immediate fix**

**Description**: Patient resource with HTML-like tags in name.text field:
```json
{
  "resourceType": "Patient",
  "id": "pat-good",
  "name": [{"text": "Standard <script>somescript</script>"}]
}
```

**Java Validator Behavior**: Reports as VALID with INFORMATION-level issue about potential security risk

**Our Behavior**: Incorrectly reports as INVALID (error details TBD)

**Root Cause**: Under investigation - likely overly strict validation on narrative content

**Impact**: 1/100 tests (1%), but critical as it's our only false negative (incorrectly rejecting valid resource)

### Failure Categories Summary

| Category | Count | Percentage | Priority |
|----------|-------|------------|----------|
| Terminology/Display | 9 | 30% | 🔴 HIGH |
| Profile Constraints | 7 | 23% | 🟡 MEDIUM |
| Questionnaire/QR | 6 | 20% | 🟡 MEDIUM |
| Bundle/References | 2 | 7% | 🟢 LOW |
| Other/Edge Cases | 5 | 17% | Mixed |
| **False Negatives** | **1** | **3%** | **🔴 CRITICAL** |

### No More ext-1 False Positives! ✅

All tests that previously failed due to ext-1 over-application now pass:
- contained.json: Now VALID ✅
- snomed-ca: Now VALID ✅  
- patient-example-ra4: Now VALID ✅
- dr-example-org-2: Now VALID ✅
- medstmt-ips: Now VALID ✅
- And 20+ more tests fixed!

The ext-1 fix was **100% successful** - no remaining ext-1 false positives detected.
