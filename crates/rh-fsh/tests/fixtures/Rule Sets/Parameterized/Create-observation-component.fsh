// @Name: Create Observation Component
// @Description: Create Observation components easily using RuleSets

RuleSet: CreateComponent(sliceName, min, max, code, short, definition)
* component contains {sliceName} {min}..{max} MS
* component[{sliceName}].code = {code}
* component[{sliceName}] ^short = {short}
* component[{sliceName}] ^definition = {definition}


Alias:   LNC = http://loinc.org

// Put these rules into action
Profile: TumorSize
Parent:  Observation
Id:      tumor-size
Title:   "Tumor Size"
Description:  "Records the dimensions of a tumor"
* insert ObservationComponentSlicingRules
// Require 1 dimension; up to two additional dimensions are optional
* insert CreateComponent(maxDimension, 1, 1, LNC#33728-7, "Maximum dimension of tumor", "The longest tumor dimension") 
* insert CreateComponent(otherDimension, 0, 2, LNC#33729-5, "Other tumor dimension", "Additional tumor dimensions should be ordered from largest to smallest") 


// Slice the component element on the component.code element
RuleSet: ObservationComponentSlicingRules
* component ^slicing.discriminator.type = #pattern
* component ^slicing.discriminator.path = "code"
* component ^slicing.rules = #open
* component ^slicing.description = "Slice based on the component.code pattern"