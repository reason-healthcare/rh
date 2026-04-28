// @Name: Instance Example
// @Description: An example of an instance of a Patient resource

Instance: PatientExample
InstanceOf: Patient
Description: "Example of Patient"
* name.family = "Anyperson"
* name.given[0] = "John"
* name.given[1] = "B."
// The first element [0] can also be represented as [+] if it is not preceded by any hard index
* contact.telecom[+].system = #phone
* contact.telecom[=].value = "555-555-5555"
* contact.telecom[=].use = #home
* contact.telecom[+].system = #email
* contact.telecom[=].value = "john.anyperson@example.com"

* gender = #male

* birthDate = "1951-01-20"

* address.line = "123 Main St"
* address.city = "Anytown"
* address.postalCode = "12345"
* address.country = "US"

* communication.language = urn:ietf:bcp:47#en-US "English (Region=United States)"
* communication.language.text = "English"
