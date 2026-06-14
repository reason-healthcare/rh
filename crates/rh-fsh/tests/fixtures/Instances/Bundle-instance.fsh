// @Name: BundleInstance
// @Description: A Bundle instance containing multiple resource entries

Instance: SampleBundle
InstanceOf: Bundle
Usage: #example
Title: "Sample Bundle"
Description: "A sample bundle with multiple resource entries."

* type = #collection
* entry[+].fullUrl = "https://example.org/fhir/Patient/pat-1"
* entry[=].resource = pat-1
* entry[+].fullUrl = "https://example.org/fhir/Observation/obs-1"
* entry[=].resource = obs-1

Instance: pat-1
InstanceOf: Patient
Usage: #inline
* id = "pat-1"
* name.family = "Smith"
* name.given = "John"
* birthDate = "1970-01-15"

Instance: obs-1
InstanceOf: Observation
Usage: #inline
* id = "obs-1"
* status = #final
* code = http://loinc.org#29463-7 "Body weight"
* subject = Reference(pat-1)
* valueQuantity.value = 72.5
* valueQuantity.unit = "kg"
* valueQuantity.system = "http://unitsofmeasure.org"
* valueQuantity.code = #kg
