library ArithmeticTests version '1.0.0'

// Basic arithmetic
define IntegerAdd: 1 + 1
define DecimalAdd: 1.0 + 1.0
define IntegerSubtract: 2 - 1
define IntegerMultiply: 2 * 3
define IntegerDivide: 6 / 2
define IntegerModulo: 7 mod 3
define DecimalDivide: 6.0 / 2.0

// Unary operators
define IntegerNegate: -5
define DecimalNegate: -5.5

// Functions
define IntegerAbs: Abs(-5)
define DecimalAbs: Abs(-5.5)
define DecimalCeiling: Ceiling(4.1)
define DecimalFloor: Floor(4.9)
define DecimalTruncate: Truncate(4.9)
define DecimalRound: Round(4.5, 0)

// Power and logarithms
define IntegerPower: Power(2, 3)
define DecimalPower: Power(2.0, 3.0)
define DecimalLn: Ln(10.0)
define DecimalExp: Exp(1.0)
define DecimalLog: Log(100.0, 10.0)

// Minimum/Maximum (using functions)
define SmallestInteger: Minimum({ 1, 2, 3 })
define LargestInteger: Maximum({ 1, 2, 3 })

