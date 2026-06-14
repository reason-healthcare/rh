// @Name: QuantityValues
// @Description: Instance demonstrating various quantity value assignments

Instance: WeightObservation
InstanceOf: Observation
Usage: #example
Title: "Weight Observation"
Description: "An observation with quantity, ratio, and coded values."

* status = #final
* category[+] = http://terminology.hl7.org/CodeSystem/observation-category#vital-signs
* code = http://loinc.org#29463-7 "Body weight"
* subject.reference = "Patient/example"
* effectiveDateTime = "2024-01-15T10:30:00Z"

// Quantity value
* valueQuantity.value = 72.5
* valueQuantity.unit = "kg"
* valueQuantity.system = "http://unitsofmeasure.org"
* valueQuantity.code = #kg

// Reference ranges
* referenceRange[+].low.value = 50
* referenceRange[=].low.unit = "kg"
* referenceRange[=].high.value = 120
* referenceRange[=].high.unit = "kg"
* referenceRange[=].type = http://terminology.hl7.org/CodeSystem/referencerange-meaning#normal
