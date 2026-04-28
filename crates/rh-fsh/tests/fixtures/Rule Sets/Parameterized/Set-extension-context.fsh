// @Name: Set Extension Context
// @Description: Specify which resources or elements an extension can be applied to

// The path is either the name of a resource or path to an element (see examples below)
RuleSet: ExtensionContext(path)
* ^context[+].type = #element
* ^context[=].expression = "{path}"

// The extension can be applied to MedicationRequests, MedicationAdministrations, and Procedures
Extension: TreatmentIntent
Id:        treatment-intent
Title:    "Treatment Intent"
Description: "The purpose of a medication or procedure."
* insert ExtensionContext(MedicationRequest)
* insert ExtensionContext(MedicationAdministration)
* insert ExtensionContext(Procedure)
* value[x] only CodeableConcept
* value[x] from TreatmentIntentVS (required)

// The extension can be applied to bodySite element within Specimens, Procedures, Conditions, and Observations
Extension: BodySiteLocationQualifier
Id:        body-site-location-qualifier
Title:     "Body Site Location Qualifier"
Description: "Qualifier to refine an body location. These include qualifiers for laterality, relative location, directionality, number, and plane."
* insert ExtensionContext(Specimen.collection.bodySite)
* insert ExtensionContext(Procedure.bodySite)
* insert ExtensionContext(Condition.bodySite)
* insert ExtensionContext(Observation.bodySite)
* value[x] only CodeableConcept
* value[x] from http://hl7.org/fhir/ValueSet/bodystructure-relative-location (preferred)


// Extra stuff used in example
ValueSet: TreatmentIntentVS
Id:       treatment-intent-vs
Title:    "Treatment Intent Value Set"
Description: "The purpose of a treatment."
* ^copyright = "This value set includes content from SNOMED CT, which is copyright © 2002+ International Health Terminology Standards Development Organisation (IHTSDO), and distributed by agreement between IHTSDO and HL7. Implementer use of SNOMED CT is not covered by this agreement"
* SCT#373808002 "Curative - procedure intent"
* SCT#363676003 "Palliative - procedure intent"
* SCT#399707004 "Supportive - procedure intent"

Alias: SCT = http://snomed.info/sct