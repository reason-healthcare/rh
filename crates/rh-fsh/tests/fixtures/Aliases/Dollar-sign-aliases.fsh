// @Name: DollarSignAliases
// @Description: Demonstrates $alias usage in rules and bindings

Alias: $loinc = http://loinc.org
Alias: $snomed = http://snomed.info/sct
Alias: $ucum = http://unitsofmeasure.org
Alias: $v2-0203 = http://terminology.hl7.org/CodeSystem/v2-0203
Alias: $USCorePatient = http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient

Profile: AliasedProfile
Id: aliased-profile
Parent: Observation
Title: "Aliased Profile"
Description: "A profile using aliases for cleaner rule writing."

* code from $loinc|http://hl7.org/fhir/ValueSet/observation-codes (preferred)
* valueQuantity.system = $ucum
* category[+] = http://terminology.hl7.org/CodeSystem/observation-category#laboratory

Instance: AliasedObservation
InstanceOf: AliasedProfile
Usage: #example
* status = #final
* code = $loinc#2085-9 "HDL Cholesterol"
* valueQuantity.value = 55
* valueQuantity.unit = "mg/dL"
* valueQuantity.system = $ucum
* valueQuantity.code = #mg/dL
