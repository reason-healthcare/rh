library R4RetrieveStress version '0.1.0'

using FHIR version '4.0.1'

include FHIRHelpers version '4.0.1' called FHIRHelpers

codesystem "LOINC": 'http://loinc.org'
codesystem "ConditionClinicalStatus": 'http://terminology.hl7.org/CodeSystem/condition-clinical'

valueset "Diabetes": 'http://example.org/fhir/ValueSet/diabetes'

code "A1c": '4548-4' from "LOINC" display 'Hemoglobin A1c/Hemoglobin.total in Blood'
code "active": 'active' from "ConditionClinicalStatus"

parameter "Measurement Period" Interval<DateTime>

context Patient

define "Conditions in Measurement Period":
  [Condition: "Diabetes"] C
    where FHIRHelpers.ToInterval(C.onset) during "Measurement Period"
      and C.clinicalStatus ~ FHIRHelpers.ToConcept("active")

define "Most Recent A1c":
  Last(
    [Observation: "A1c"] O
      where O.status in { 'final', 'amended', 'corrected' }
        and FHIRHelpers.ToInterval(O.effective) during "Measurement Period"
      sort by start of FHIRHelpers.ToInterval(effective)
  )

define "A1c Value":
  "Most Recent A1c".value as Quantity

define "Poor Control":
  "A1c Value" is not null
    and "A1c Value" >= 9.0 '%'
