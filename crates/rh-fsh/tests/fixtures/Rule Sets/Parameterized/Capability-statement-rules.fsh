// @Name:CapabilityStatement Rules
// @Description: Rules to populate a CapabilityStatement


Alias: $exp = http://hl7.org/fhir/StructureDefinition/capabilitystatement-expectation

RuleSet: SupportResource (resource, expectation)
* rest.resource[+].type = #{resource}
* rest.resource[=].extension[0].url = $exp
* rest.resource[=].extension[0].valueCode = {expectation}

RuleSet: SupportProfile (profile, expectation)
// This rule set must follow a SupportResource rule set, and applies to that resource.
* rest.resource[=].supportedProfile[+] = "{profile}"
* rest.resource[=].supportedProfile[=].extension[0].url = $exp
* rest.resource[=].supportedProfile[=].extension[0].valueCode = {expectation}

RuleSet: SupportInteraction (interaction, expectation)
// This rule set must follow a SupportResource rule set, and applies to that resource.
* rest.resource[=].interaction[+].code = {interaction}
* rest.resource[=].interaction[=].extension[0].url = $exp
* rest.resource[=].interaction[=].extension[0].valueCode = {expectation}

RuleSet: SupportSearchParam (name, canonical, type, expectation)
// This rule set must follow a SupportResource rule set, and applies to that resource.
* rest.resource[=].searchParam[+].name = "{name}"
* rest.resource[=].searchParam[=].definition = "{canonical}"
* rest.resource[=].searchParam[=].type = {type}
* rest.resource[=].searchParam[=].extension[0].url = $exp
* rest.resource[=].searchParam[=].extension[0].valueCode = {expectation}


Instance: ExampleCapabilityStatement
InstanceOf: CapabilityStatement
Title: "Create a Capability Statement using RuleSets"
Usage: #definition
* name = "Example Capability Statement"
* rest.mode = #server
* status = #draft
* date = "2020-05-07"
* kind = #requirements
* fhirVersion = #4.0.1
* format = #json

// Measure requirements
* insert SupportResource(Measure, #SHALL)
* insert SupportProfile(http://hl7.org/fhir/StructureDefinition/shareablemeasure, #SHALL)
* insert SupportInteraction(#read, #SHALL)
* insert SupportInteraction(#search-type, #SHALL)
* insert SupportSearchParam(_id, http://hl7.org/fhir/SearchParameter/Resource-id, #token, #SHALL)
* insert SupportSearchParam(_lastUpdated, http://hl7.org/fhir/SearchParameter/Resource-lastUpdated, #date, #SHALL)
* insert SupportSearchParam(name, http://hl7.org/fhir/SearchParameter/Location-name, #string, #SHALL)

 // MeasureReport requirements
* insert SupportResource(MeasureReport, #SHALL)
* insert SupportInteraction(#read, #SHALL)
* insert SupportInteraction(#search-type, #SHALL)
* insert SupportSearchParam(_id, http://hl7.org/fhir/SearchParameter/Resource-id, #token, #SHALL)
* insert SupportSearchParam(_lastUpdated, http://hl7.org/fhir/SearchParameter/Resource-lastUpdated, #date, #SHALL)
* insert SupportSearchParam(date, http://hl7.org/fhir/SearchParameter/MeasureReport-date, #date, #SHALL)
* insert SupportSearchParam(subject, http://hl7.org/fhir/SearchParameter/MeasureReport-subject, #reference, #SHALL)
* insert SupportSearchParam(code, http://hl7.org/fhir/us/saner/SearchParameter/SearchParameter-code, #token, #SHOULD)
