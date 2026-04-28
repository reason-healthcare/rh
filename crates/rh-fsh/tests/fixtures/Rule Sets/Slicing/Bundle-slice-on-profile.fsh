// @Name: Bundle Slice on Profile
// @Description: RuleSet for slicing a Bundle on the profile of referenced resource

// Slice on profile when it is not possible to distinguish the resources in the sliced array by the value of a element, and cannot be distinguished by the resource type.
RuleSet: BundleSliceOnProfile
* entry ^slicing.discriminator.type = #profile
* entry ^slicing.discriminator.path = "resource"
* entry ^slicing.rules = #open
* entry ^slicing.description = "Slicing based on the profile conformance of the sliced element"


