// @Name: Contained Instance Example
// @Description: An example of an instance that contains another instance which references its container

Alias: $SCT = http://snomed.info/sct

Instance: my-provenance
InstanceOf: Provenance
Usage: #inline
* target.reference = "#"
* recorded = "2023-08-01T00:00:00.0Z"
* agent.who = Reference(bob)

Instance: bob
InstanceOf: Practitioner
* name.given = "Bob"

Instance: sue
InstanceOf: Patient
* name.given = "Sue"

Instance: my-condition
InstanceOf: Condition
* contained[+] = my-provenance
* code = $SCT#134006 "Decreased hair growth"
* subject = Reference(sue)