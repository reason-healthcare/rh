library LastCbcPanelReportDateFeatureLogic version '0.1.0'

using FHIR version '4.0.1'

include FHIRHelpers version '0.1.0'
include Common version '0.1.0'

codesystem "CaseFeatureCodes": 'http://example.org/CodeSystem/CaseFeatureCodes'
code "Last CBC Report Date": 'last-cbc-panel-report-date' from "CaseFeatureCodes"

codesystem "LOINC": 'http://loinc.org'
code "CBC Panel": '58410-2' from "LOINC"

context Patient

define "CBC Reports":
  [DiagnosticReport: "CBC Panel"] CBCReport
    where CBCReport.status ~ 'final'

define "Last CBC report":
  First("CBC Reports" C sort by effective descending)

/**
 * @description
 * Inference if the patient is taking drug with a Methotrexate code
 */
define "Last CBC Panel Report Date Inferred":
    if exists("Last CBC report") then
      Observation {
        meta: Meta {
          profile: {
            canonical { value: 'http://example.org/StructureDefinition/LastCbcPanelReportDateFeature' }
          }
        },
        extension: {
          Extension {
            url: url { value: 'http://hl7.org/fhir/uv/cpg/StructureDefinition/cpg-instantiatesCaseFeature' },
            value: canonical { value: 'http://example.org/StructureDefinition/LastCbcPanelReportDateFeature' }
          },
          Extension {
            url: url { value: 'http://hl7.org/fhir/uv/cpg/StructureDefinition/cpg-caseFeatureType' },
            value: code { value: 'inferred' }
          }
        },
        status: ObservationStatus { value: 'final' },
        effective: dateTime { value: Now() },
        code: CodeableConcept {
          coding: {
            Coding {
              system: uri { value: 'http://example.org/CodeSystem/CaseFeatureCodes' },
              code: code { value: 'last-cbc-panel-report-date' }
            }
          }
        },
        value: dateTime { value: "Last CBC report".effective }
      }
    else null

/**
 * @description
 * Assertion is if there is a case feature directly documented
 */
define "Last CBC Panel Report Date Asserted":
  Common.MostRecent([Observation: "Last CBC Report Date"])

/**
 * @description
 * Feature is asserted followed by inferred
 */
define "Last CBC Panel Report Date":
  Coalesce(
    "Last CBC Panel Report Date Asserted",
    "Last CBC Panel Report Date Inferred"
  )

define "Value":
  ("Last CBC Panel Report Date".value as dateTime).value
