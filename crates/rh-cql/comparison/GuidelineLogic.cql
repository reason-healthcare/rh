library GuidelineLogic version '0.1.0'

using FHIR version '4.0.1'

include FHIRHelpers version '4.0.1'

codesystem "LOINC": 'http://loinc.org'
codesystem "Condition Clinical Status Codesystem": 'http://terminology.hl7.org/CodeSystem/condition-clinical'
codesystem "Condition Verification Status Codesystem": 'http://terminology.hl7.org/CodeSystem/condition-ver-status'
codesystem "RxNorm": 'http://www.nlm.nih.gov/research/umls/rxnorm'

valueset "Anti-Anemic Ferrous Sulfate Branded Pack Drugs": 
  'http://example.org/fhir/ValueSet/AntiAnemicFurrousSulfateBrandedPacks'

code "Active Condition Code": 'active' from "Condition Clinical Status Codesystem"
code "Confirmed Condition Code": 'confirmed' from "Condition Verification Status Codesystem"

code "Tumor Size Code": '21889-1' from "LOINC"
code "Size.maximum Code": '33728-7' from "LOINC"

code "ERStatus Code": '85337-4' from "LOINC"
code "PRStatus Code": '85339-0' from "LOINC"
code "HER2Status Code": '48676-1' from "LOINC"

code "Negative Status Code": 'LA6577-6' from "LOINC"
code "Positive Status Code": 'LA6576-8' from "LOINC"

valueset "Breast Cancer VS": 'http://example.org/cancer-guideline/fhir/ValueSet/BreastCancerICD10'

context Patient

define "Has TNBC and at least T1c":
  "Is TNBC" and "Is at least T1c"

define "Has breast cancer":
  exists(
    [Condition: "Breast Cancer VS"] C
      where C.clinicalStatus ~ "Active Condition Code"
      and C.verificationStatus ~ "Confirmed Condition Code"
  )

define "Is TNBC":
  "Is ER negative" and "Is PR negative" and "Is HER2 negative"


define "Tumor Size":
  [Observation: "Tumor Size Code"] TS where TS.status = 'final'

define "Most Recent Tumor Size":
  First("Tumor Size" TS sort by issued descending)

define "Most Recent Tumor Size Quantity":
  First("Most Recent Tumor Size".component C where C.code ~ "Size.maximum Code").value as Quantity

define "Is at least T1c":
  "Most Recent Tumor Size Quantity" > 1 'cm'


define "Is ER negative":
  First(
    [Observation: "ERStatus Code"] ER 
      where ER.status = 'final' sort by issued descending
  ).value ~ "Negative Status Code"

define "Is PR negative":
  First(
    [Observation: "PRStatus Code"] ER 
      where ER.status = 'final' sort by issued descending
  ).value ~ "Negative Status Code"

define "Is HER2 negative":
  First(
    [Observation: "HER2Status Code"] ER 
      where ER.status = 'final' sort by issued descending
  ).value ~ "Negative Status Code"

// Iron supplementation functions using the branded ferrous sulfate ValueSet
define "Active Ferrous Sulfate Medications":
  [MedicationStatement: "Anti-Anemic Ferrous Sulfate Branded Pack Drugs"] MS
    where MS.status = 'active'
      and MS.effective overlaps Interval[Today() - 30 days, Today()]

define "Has Iron Deficiency Indication for Ferrous Sulfate":
  exists([Condition] C
    where C.code.coding contains Code { 
      code: 'D50.9', 
      system: 'http://hl7.org/fhir/sid/icd-10-cm', 
      display: 'Iron deficiency anemia, unspecified' 
    }
      and C.clinicalStatus ~ "Active Condition Code"
      and C.verificationStatus ~ "Confirmed Condition Code"
  )

define "Should Consider Iron Supplementation":
  "Has Iron Deficiency Indication for Ferrous Sulfate"
    and not exists("Active Ferrous Sulfate Medications")