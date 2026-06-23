library CoreOperators version '0.1.0'

define BooleanNullAndTrue: null and true
define BooleanNullOrFalse: null or false
define IntegerBoundaryAdd: 2147483647 + 0
define DecimalPromotion: 1 + 2.5
define QuantitySameUnitCompare: 5 'mg' > 3 'mg'
define EqualityVsEquivalenceNulls: null ~ null
define CoalesceChain: Coalesce(null, null, 'fallback')
define NestedIf: if true then if false then 'inner' else 'outer' else 'miss'
define SimpleCase:
  case
    when 1 = 2 then 'no'
    when 2 = 2 then 'yes'
    else 'else'
  end
