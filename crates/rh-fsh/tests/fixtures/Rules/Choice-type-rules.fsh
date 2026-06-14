// @Name: ChoiceTypeRules
// @Description: Rules constraining choice-type elements (value[x]) using only/binding

Profile: ChoiceTypeProfile
Id: choice-type-profile
Parent: Observation
Title: "Choice Type Profile"
Description: "A profile that constrains the value[x] choice element."

* value[x] only Quantity or CodeableConcept
* valueQuantity.system = "http://unitsofmeasure.org"
* valueQuantity.code from http://hl7.org/fhir/ValueSet/ucum-units (preferred)
