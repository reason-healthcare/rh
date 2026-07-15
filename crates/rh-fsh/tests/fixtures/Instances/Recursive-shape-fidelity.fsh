// @Name: Recursive Shape Fidelity
// @Description: Focused recursive instance shapes for extensions, primitive shadows, Bundle entries, Parameters parts, and repeating BackboneElements.

Alias: $LNC = http://loinc.org

Profile: RecursiveObservation
Parent: Observation
* status = #final

Profile: RelatedCondition
Parent: Condition
* extension contains condition-related named relatedPrimaryCancerCondition 0..1

Instance: RecursiveObservationExample
InstanceOf: RecursiveObservation
Usage: #inline
* id = "recursive-observation"
* code = $LNC#55423-8 "Number of steps in 24 hour Measured"
* component[+].code = $LNC#41950-7 "Number of steps"
* component[=].valueQuantity.value = 190.00
* component[=].valueQuantity.unit = "steps"
* component[+].code = $LNC#41953-1 "Number of flights of stairs"
* component[=].valueQuantity.value = 2

Instance: RecursiveBundle
InstanceOf: Bundle
Usage: #example
* type = #collection
* entry[+].fullUrl = "http://example.org/fhir/Observation/recursive-observation"
* entry[=].resource = RecursiveObservationExample

Instance: RecursiveParameters
InstanceOf: Parameters
Usage: #example
* parameter[+].name = "outer"
* parameter[=].part[+].name = "inner"
* parameter[=].part[=].valueString = "nested value"

Instance: PrimitiveExtensionPatient
InstanceOf: Patient
Usage: #example
* name[+].family = "Anyperson"
* name[=].family.extension[+].url = "http://example.org/StructureDefinition/family-note"
* name[=].family.extension[=].valueString = "preferred family name"
* extension[+].url = "http://example.org/StructureDefinition/outer-note"
* extension[=].extension[+].url = "inner-note"
* extension[=].extension[=].valueString = "nested extension"

Instance: RelatedConditionExample
InstanceOf: RelatedCondition
Usage: #example
* subject = Reference(Patient/example)
* extension[relatedPrimaryCancerCondition].valueReference = Reference(Condition/primary-cancer)
