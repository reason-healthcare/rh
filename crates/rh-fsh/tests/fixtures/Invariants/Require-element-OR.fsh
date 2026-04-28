// @Name: Require Element OR
// @Description: Invariant requiring one or both of two elements to be present

Profile:  MedicationAdministrationWithReason
Parent:   MedicationAdministration
Id:       medication-administration-with-reason
Title:    "Medication Administration With Reason Required"
Description:    "The reason for this medication administration must be documented using a code or a reference, or both."
* obeys reason-required
* reasonCode and reasonReference MS

Invariant:  reason-required
Description: "Either reasonCode or reasonReference MUST be populated"
Expression: "reasonCode.exists() or reasonReference.exists()"
Severity:   #error