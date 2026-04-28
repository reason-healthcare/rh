// @Name: Must Support on Selected Referenced Resource
// @Description: Apply an MS flag to a Reference, for example, in Reference(Patient or Practitioner), put MS on Practitioner without a MS on Patient. 
// Using an "only" rule will forbid that type of resource. This method doesn't forbid anything, but just says the implementer is obliged to implement certain referenced resource (or profile) types.

/* Take Observation.performer as an example. It is defined as Reference(Practitioner | PractitionerRole | Organization | CareTeam | Patient | RelatedPerson). We would like to apply MS to Practitioner and Patient. Unfortunately, this has to be done by index number in the array, rather than by name. The StructureDefinition (see http://hl7.org/fhir/R4/observation.profile.json.html) uses the order above. The rules themselves are messy, so it's best to use the RuleSet provided.
*/

Profile: ObservationPerformerMustSupport
Parent: Observation
* insert MustSupportOnReference(performer, 0)
* insert MustSupportOnReference(performer, 4)


RuleSet: MustSupportOnReference(path, refIndex)
* {path} ^type[0].targetProfile[{refIndex}].extension[0].url = "http://hl7.org/fhir/StructureDefinition/elementdefinition-type-must-support"
* {path} ^type[0].targetProfile[{refIndex}].extension[0].valueBoolean = true