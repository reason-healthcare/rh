// @Name: Observation hasMember Slice on Code
// @Description: RuleSet for Slicing Observation.hasMember on an element's value in the referenced resource

// Slice the hasMember element on the code element of the referenced resource. Use this pattern if the resources referenced by the hasMember array can be discriminated based on a single element's value, in this case Observation.code.
RuleSet: ObservationHasMemberSliceOnCode
* hasMember ^slicing.discriminator.type = #pattern 
* hasMember ^slicing.discriminator.path = "$this.resolve().code"
* hasMember ^slicing.rules = #open
* hasMember ^slicing.description = "Slice based on referenced resource's code attribute."

Profile: StageGroup
Id:      stage-group
Parent:  Observation
Title:   "Stage Group"
Description: "The cancer stage group with related separate Observations representing the T Stage, N Stage, and M Stage."
* insert ObservationHasMemberSliceOnCode
* hasMember only Reference(Observation) 
// Optionally specific profiles could be specified, or we could set ^slicing.rules = #closed. As written, other Observations are allowed.
* hasMember contains
    TumorStage 0..1 MS and
    NodesStage 0..1 MS and
    MetastasesStage 0..1 MS


// Each of the T, N, and M stages must have a different code since they are all Observations and that is how they will be distinguished in an instance. In brief:
Profile: TumorStage
Parent: Observation
* code = LNC#21905-5 // Primary tumor.clinical [Class] Cancer

Profile: NodesStage
Parent: Observation
* code = LNC#21906-3 // Regional lymph nodes.clinical [Class] Cancer

Profile: MetastasesStage
Parent: Observation
* code = LNC#21907-1 // Distant metastases.clinical [Class] Cancer

Alias:   LNC = http://loinc.org