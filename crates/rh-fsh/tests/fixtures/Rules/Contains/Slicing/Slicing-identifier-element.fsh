// @Name: Slicing Identifier Element
// @Description: Slice the identifier to require an implementer to support lab results accession number, filler order number and placer order number

Profile: LaboratoryResultWithMultipleIdentfiers
Parent: Observation
// Slice on the value of the identifier type code
* identifier ^slicing.discriminator.type = #value
* identifier ^slicing.discriminator.path = "type.coding.code"
* identifier ^slicing.rules = #open
* identifier contains
    AccessionIdentifier 0..1 MS and
    FillerOrderNumber 0..1 MS and
    PlacerOrderNumber 0..1 MS
* identifier[AccessionIdentifier].type = IDTYPE#ACSN
* identifier[FillerOrderNumber].type = IDTYPE#FILL
* identifier[PlacerOrderNumber].type = IDTYPE#PLAC


Alias:   IDTYPE = http://terminology.hl7.org/CodeSystem/v2-0203