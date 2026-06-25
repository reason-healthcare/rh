Alias: $USCorePractitionerProfile = http://hl7.org/fhir/us/core/StructureDefinition/us-core-practitioner

Profile: MyPatient
Parent: Patient
Id: my-patient
Title: "My Patient"
Description: "Local profile used to verify profile instance resource identity."

Instance: LocalProfilePatient
InstanceOf: MyPatient
Usage: #example
* active = true
* name[+].family = "Example"
* name[=].given[+] = "Pat"

Instance: DependencyProfilePractitioner
InstanceOf: $USCorePractitionerProfile
Usage: #example
* active = true
* identifier.system = "http://example.org/practitioners"
* identifier.value = "practitioner-1"
* name[+].family = "Example"
* name[=].given[+] = "Prac"
