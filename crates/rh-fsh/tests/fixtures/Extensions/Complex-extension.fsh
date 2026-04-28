// @Name: Complex Extensions
// @Description: Examples of extensions with sub-extensions. Note that an extension cannot have BOTH a value and extensions.

Extension: RadiationDoseDelivered
Id:        radiation-dose-delivered
Title:     "Radiotherapy Dose Delivered"
Description: "Dose delivered to a body structure divided into a number of fractional doses. This extension is used on Procedure resource."
* insert ExtensionContext(Procedure)
* extension contains
    bodyStructure 1..1 MS and
    totalDoseDelivered 0..1 MS and
    fractionsDelivered 0..1 MS
* extension[bodyStructure].value[x] only Reference(BodyStructure)
* extension[totalDoseDelivered].value[x] only Quantity
* extension[totalDoseDelivered].valueQuantity = UCUM#cGy
* extension[fractionsDelivered].value[x] only unsignedInt
// Definitions of in-line extensions
* insert DocumentExtension (
    bodyStructure,
    "Volume in the body where radiation was delivered",
    "A BodyStructure resource representing the body structure treated\, for example\, Chest Wall Lymph Nodes.")
* insert DocumentExtension (
    totalDoseDelivered, 
    "Total Radiation Dose Delivered",
    "The total amount of radiation delivered to this volume within the scope of this dose delivery.")
* insert DocumentExtension (
    fractionsDelivered,
    "Number of Fractions Delivered",
    "The number of fractions delivered to this volume.")


// This rule set limits the application of an extension to the given path
RuleSet: ExtensionContext(path)
* ^context[+].type = #element
* ^context[=].expression = "{path}"

// The strings defined for short and definition should not be quoted, and any comma must be escaped with a backslash.
RuleSet: DocumentExtension(path, short, definition)
* extension[{path}] ^short = {short}
* extension[{path}] ^definition = {definition}

Alias: UCUM = http://unitsofmeasure.org