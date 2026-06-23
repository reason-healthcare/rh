library MainLibrary version '0.1.0'

include HelperLibrary version '0.1.0' called Helper

define UsesIncludedExpression: Helper.PublicValue + 1
define UsesIncludedFunction: Helper.AddOne(4)
