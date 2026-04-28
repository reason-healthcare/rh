// @Name: Require Element XOR
// @Description: Invariant requiring one of two elements to be present (not both)

Invariant:   exactly-one-reason-required
Description: "Either reasonCode or reasonReference MUST be populated, but not both."
Expression: "(reasonCode.exists() or reasonReference.exists()) and (reasonCode.exists() and reasonReference.exists()).not()"
Severity: #error

Profile:  MedicationAdministrationExactlyOneReason
Parent:   MedicationAdministration
Id:       medication-administration-exactly-one-reason
Title:    "Medication Administration With Required Reason"
Description:    "The reason for this medication administration must be documented using a code or a reference, but not both."
* obeys exactly-one-reason-required
* reasonCode and reasonReference MS

