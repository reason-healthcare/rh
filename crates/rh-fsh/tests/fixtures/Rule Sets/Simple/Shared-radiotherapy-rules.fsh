// @Name: Shared Radiotherapy Rules
// @Description: Share a set of rules among multiple profiles

// Rules applied to all types of radiotherapy procedures
RuleSet: RadiotherapyProcedureCommonRules
* category 1..
* category = http://snomed.info/sct#108290001 // "Radiation oncology AND/OR radiotherapy (procedure)"
* performed[x] only Period
* category and status and subject and performed[x] MS

// Note that this could also be achieved by defining an abstract parent profile, e.g., RadiotherapyProcedure, and then using that as a parent. There are pros and cons to each approach. Using the common rule set approach, the Differential View is easier to understand what changes have been made to a base resource, and obviously, one less profile. Also, multiple rule sets can be applied (whereas you can only have one parent profile). On the other hand, having a parent profile can make it easier for others to extend your IG.

Profile: ExternalBeamRadiotherapyProcedure
Parent: Procedure
Id: external-beam-radiotherapy-procedure
Description: "Procedures involving external radiation sources"
* insert RadiotherapyProcedureCommonRules
// etc.


Profile: BrachytherapyProcedure
Parent: Procedure
Id: brachytherapy-procedure
Description: "Procedures involving implanted radiation sources or sources in body cavities"
* insert RadiotherapyProcedureCommonRules
// etc.