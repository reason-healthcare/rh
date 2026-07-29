// Regression fixture for the first-pass relational lowerer (`cql lower-check`).
// Mirrors a realistic hypertension decision-support library so we can track
// which ELM node kinds the first-pass lowerer recognizes.
//
// Companion test: crates/rh-cql/src/analytics.rs `lower_check_hyper_tension_fixture_*`.

library HypertensionManagement version '1.0.0'

using FHIR version '4.0.1'

include FHIRHelpers version '4.0.1' called FHIRHelpers

valueset "Essential Hypertension":
  'https://example.org/fhir/ValueSet/essential-hypertension'

valueset "Diabetes Mellitus":
  'https://example.org/fhir/ValueSet/diabetes'

valueset "Chronic Kidney Disease":
  'https://example.org/fhir/ValueSet/chronic-kidney-disease'

valueset "Pregnancy":
  'https://example.org/fhir/ValueSet/pregnancy'

valueset "Office Blood Pressure":
  'https://example.org/fhir/ValueSet/office-blood-pressure'

valueset "ACE Inhibitors":
  'https://example.org/fhir/ValueSet/ace-inhibitors'

valueset "Angiotensin Receptor Blockers":
  'https://example.org/fhir/ValueSet/angiotensin-receptor-blockers'

valueset "Calcium Channel Blockers":
  'https://example.org/fhir/ValueSet/calcium-channel-blockers'

valueset "Thiazide Diuretics":
  'https://example.org/fhir/ValueSet/thiazide-diuretics'

code "Systolic Blood Pressure":
  '8480-6' from "LOINC"

code "Diastolic Blood Pressure":
  '8462-4' from "LOINC"

codesystem "LOINC":
  'http://loinc.org'

parameter "Measurement Period" Interval<DateTime>
  default Interval[Today() - 6 months, Today()]

context Patient

/*
 * Basic population
 */

define "Adult":
  AgeInYearsAt(Today()) >= 18

define "Active Hypertension":
  [Condition: "Essential Hypertension"] C
    where C.clinicalStatus ~ ToConcept(
      Code {
        system: 'http://terminology.hl7.org/CodeSystem/condition-clinical',
        code: 'active'
      }
    )

define "Has Hypertension":
  exists "Active Hypertension"

define "Has Diabetes":
  exists (
    [Condition: "Diabetes Mellitus"] C
      where C.clinicalStatus ~ ToConcept(
        Code {
          system: 'http://terminology.hl7.org/CodeSystem/condition-clinical',
          code: 'active'
        }
      )
  )

define "Has Chronic Kidney Disease":
  exists (
    [Condition: "Chronic Kidney Disease"] C
      where C.clinicalStatus ~ ToConcept(
        Code {
          system: 'http://terminology.hl7.org/CodeSystem/condition-clinical',
          code: 'active'
        }
      )
  )

define "Is Pregnant":
  exists (
    [Condition: "Pregnancy"] C
      where C.clinicalStatus ~ ToConcept(
        Code {
          system: 'http://terminology.hl7.org/CodeSystem/condition-clinical',
          code: 'active'
        }
      )
  )

/*
 * Blood pressure observations
 */

define function "Observation Date"(O FHIR.Observation):
  Coalesce(
    FHIRHelpers.ToDateTime(O.effective as FHIR.dateTime),
    FHIRHelpers.ToDateTime(O.issued)
  )

define function "Systolic Value"(O FHIR.Observation):
  singleton from (
    O.component Component
      where Component.code ~ "Systolic Blood Pressure"
      return FHIRHelpers.ToQuantity(
        Component.value as FHIR.Quantity
      )
  )

define function "Diastolic Value"(O FHIR.Observation):
  singleton from (
    O.component Component
      where Component.code ~ "Diastolic Blood Pressure"
      return FHIRHelpers.ToQuantity(
        Component.value as FHIR.Quantity
      )
  )

define "Valid Blood Pressure Observations":
  [Observation: "Office Blood Pressure"] O
    where O.status.value in { 'final', 'amended', 'corrected' }
      and "Observation Date"(O) during "Measurement Period"
      and "Systolic Value"(O) is not null
      and "Diastolic Value"(O) is not null
    return {
      id: O.id.value,
      date: "Observation Date"(O),
      systolic: "Systolic Value"(O).value,
      diastolic: "Diastolic Value"(O).value
    }

define "Blood Pressure Observations By Date":
  "Valid Blood Pressure Observations" BP
    sort by date desc

define "Most Recent Blood Pressure":
  First("Blood Pressure Observations By Date")

define "Has Recent Blood Pressure":
  "Most Recent Blood Pressure" is not null

/*
 * Patient-specific target
 */

define "Systolic Target":
  if "Has Diabetes" or "Has Chronic Kidney Disease" then
    130
  else
    140

define "Diastolic Target":
  if "Has Diabetes" or "Has Chronic Kidney Disease" then
    80
  else
    90

define "Most Recent Blood Pressure Above Target":
  "Has Recent Blood Pressure"
    and (
      "Most Recent Blood Pressure".systolic >= "Systolic Target"
        or "Most Recent Blood Pressure".diastolic >= "Diastolic Target"
    )

define "Severely Elevated Blood Pressure":
  "Has Recent Blood Pressure"
    and (
      "Most Recent Blood Pressure".systolic >= 180
        or "Most Recent Blood Pressure".diastolic >= 120
    )

/*
 * Medication treatment
 *
 * This intentionally uses MedicationRequest status and authoredOn only.
 * It does not attempt to infer medication possession or adherence.
 */

define "Active ACE Inhibitor Orders":
  [MedicationRequest: "ACE Inhibitors"] M
    where M.status.value = 'active'
      and M.intent.value = 'order'

define "Active ARB Orders":
  [MedicationRequest: "Angiotensin Receptor Blockers"] M
    where M.status.value = 'active'
      and M.intent.value = 'order'

define "Active Calcium Channel Blocker Orders":
  [MedicationRequest: "Calcium Channel Blockers"] M
    where M.status.value = 'active'
      and M.intent.value = 'order'

define "Active Thiazide Orders":
  [MedicationRequest: "Thiazide Diuretics"] M
    where M.status.value = 'active'
      and M.intent.value = 'order'

define "Has ACE Inhibitor":
  exists "Active ACE Inhibitor Orders"

define "Has ARB":
  exists "Active ARB Orders"

define "Has Calcium Channel Blocker":
  exists "Active Calcium Channel Blocker Orders"

define "Has Thiazide":
  exists "Active Thiazide Orders"

define "Antihypertensive Class Count":
  (if "Has ACE Inhibitor" then 1 else 0)
    + (if "Has ARB" then 1 else 0)
    + (if "Has Calcium Channel Blocker" then 1 else 0)
    + (if "Has Thiazide" then 1 else 0)

define "ACE and ARB Duplication":
  "Has ACE Inhibitor"
    and "Has ARB"

define "On Standard Triple Therapy":
  ("Has ACE Inhibitor" or "Has ARB")
    and "Has Calcium Channel Blocker"
    and "Has Thiazide"

/*
 * Decision support
 */

define "Applicable":
  "Adult"
    and "Has Hypertension"

define "Recommendation Code":
  case
    when not "Applicable" then
      'not-applicable'

    when "Is Pregnant" then
      'pregnancy-specific-management'

    when not "Has Recent Blood Pressure" then
      'obtain-blood-pressure'

    when "Severely Elevated Blood Pressure" then
      'urgent-clinical-evaluation'

    when "ACE and ARB Duplication" then
      'review-ace-arb-duplication'

    when not "Most Recent Blood Pressure Above Target" then
      'continue-current-management'

    when "On Standard Triple Therapy" then
      'evaluate-resistant-hypertension'

    when "Antihypertensive Class Count" = 0 then
      'consider-starting-treatment'

    else
      'consider-treatment-intensification'
  end

define "Recommendation":
  case
    when "Recommendation Code" = 'not-applicable' then
      'The patient does not meet the adult hypertension criteria.'

    when "Recommendation Code" = 'pregnancy-specific-management' then
      'Use a pregnancy-specific hypertension management pathway.'

    when "Recommendation Code" = 'obtain-blood-pressure' then
      'No recent qualifying blood pressure was found.'

    when "Recommendation Code" = 'urgent-clinical-evaluation' then
      'The most recent blood pressure is severely elevated and requires prompt clinical evaluation.'

    when "Recommendation Code" = 'review-ace-arb-duplication' then
      'Both an ACE inhibitor and an ARB appear active. Review for therapeutic duplication.'

    when "Recommendation Code" = 'continue-current-management' then
      'The most recent blood pressure is below the patient-specific threshold.'

    when "Recommendation Code" = 'evaluate-resistant-hypertension' then
      'Blood pressure is above target despite standard triple therapy. Evaluate adherence, measurement accuracy, and secondary causes.'

    when "Recommendation Code" = 'consider-starting-treatment' then
      'Blood pressure is above target and no active antihypertensive order was identified.'

    else
      'Blood pressure is above target. Review adherence and consider treatment intensification.'
  end

define "Decision Support Result":
  {
    applicable: "Applicable",
    recommendationCode: "Recommendation Code",
    recommendation: "Recommendation",
    systolicTarget: "Systolic Target",
    diastolicTarget: "Diastolic Target",
    mostRecentSystolic: "Most Recent Blood Pressure".systolic,
    mostRecentDiastolic: "Most Recent Blood Pressure".diastolic,
    antihypertensiveClassCount: "Antihypertensive Class Count",
    aceArbDuplication: "ACE and ARB Duplication"
  }
