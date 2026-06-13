// @Name: CaretPathsNested
// @Description: Profiles that use nested caret paths for metadata customization

Profile: CaretNestedProfile
Id: caret-nested-profile
Parent: Patient
Title: "Caret Nested Profile"
Description: "Uses nested caret paths to set StructureDefinition metadata."

// Set top-level SD metadata via caret
* ^url = "http://example.org/StructureDefinition/CaretNestedProfile"
* ^version = "1.0.0"
* ^status = #active
* ^publisher = "Example Organization"
* ^contact.name = "Example Contact"
* ^contact.telecom.system = #email
* ^contact.telecom.value = "contact@example.org"

// Set element-level metadata
* name ^short = "Patient's legal name"
* name ^definition = "The legal name(s) used by the patient."
* name ^comment = "May include multiple name representations."
* name.family ^short = "Family name (surname)"
