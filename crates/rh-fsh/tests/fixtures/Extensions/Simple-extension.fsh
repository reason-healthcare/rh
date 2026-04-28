// @Name: Simple Extensions
// @Description: Examples of extensions with values (no sub-extensions)

Extension: Laterality
Description: "Body side of a body location."
* value[x] only CodeableConcept
* value[x] from http://hl7.org/fhir/ValueSet/bodysite-laterality (required)

Extension: TherapySessionsCompleted
Id:        therapy-sessions-completed
Title:     "Therapy Sessions Completed"
Description: "The number of sessions of some therapy."
// Limit the context to Procedures -- Also see Rule Sets for a context-setting rule set
* ^context[+].type = #element
* ^context[=].expression = "Procedure"
* value[x] only unsignedInt  // 0 or more

Extension: RelatedCondition
Id:        related-condition
Title:    "Condition related to the current resource"
Description: "The resource has an unspecified relationship with a Condition."
* value[x] only Reference(Condition)