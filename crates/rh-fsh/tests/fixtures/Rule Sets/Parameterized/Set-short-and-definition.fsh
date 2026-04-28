// @Name: Set Short and Definition
// @Description: Rule set to set the short name and definition for any element

// The strings defined for short and definition should not be quoted, and any comma must be escaped with a backslash.
RuleSet: ShortAndDefinition(path, short, definition)
* {path} ^short = "{short}"
* {path} ^definition = "{definition}"

RuleSet: NotUsed(path)
* {path} ^short = "Not used in this profile"
* {path} ^definition = "Not used in this profile"


Profile:    CancerGeneticVariant
Parent:     Observation
Id:         cancer-genetic-variant
Title:      "Cancer Genetic Variant"
Description: "Records an alteration in the most common DNA nucleotide sequence."
* insert NotUsed(bodySite)
* insert NotUsed(referenceRange)
* insert NotUsed(hasMember)
* insert ShortAndDefinition(value[x], Whether the variant is present, The overall result of the genetic test; specifically\, whether a variant is present\, absent\, no call\, or indeterminant.)
