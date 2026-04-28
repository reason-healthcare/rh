// @Name: Exclude Single Codes
// @Description: Value set with includes and excludes
Alias: SCT = http://snomed.info/sct

ValueSet: HandednessVS
Id: handedness-vs
Title: "Handedness Value Set"
Description: "Indicates the dominant hand."
* ^copyright = "This value set includes content from SNOMED CT, which is copyright © 2002+ International Health Terminology Standards Development Organisation (IHTSDO), and distributed by agreement between IHTSDO and HL7. Implementer use of SNOMED CT is not covered by this agreement"
// Note the "concept is-a" filter is specific to SNOMED CT
// A list of filters is given here: https://www.hl7.org/fhir/valueset-filter-operator.html
// To find what filters are supported by a particular code system, go to that code system's page (see https://www.hl7.org/fhir/terminologies-systems.html)
* include codes from system SCT where concept is-a #64940007 "Handedness finding (finding)"
* exclude SCT#709880000 "Unknown handedness (finding)"