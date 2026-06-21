library Nullology version '0.1.0'

define IsNull: null is null
define IsNotNull: 1 is not null
define NullEqualNull: null = null
define NullEquivalentNull: null ~ null
define CoalesceMixed: Coalesce(null, 3, 4)
define NullInList: null in { null, 1 }
define AnyTrueWithNull: AnyTrue({ null, false, true })
define AllTrueWithNull: AllTrue({ true, null })
