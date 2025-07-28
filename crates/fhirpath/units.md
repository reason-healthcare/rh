## Unit Conversion System

The FHIRPath library includes a comprehensive unit conversion system that automatically handles conversions between compatible UCUM units during arithmetic operations. This enables seamless calculations between different units of the same physical quantity, with special support for temperature units that require offset-based conversions.

### Supported Unit Categories

#### Mass Units
- `g` (gram) - base unit
- `kg` (kilogram) = 1000 g
- `mg` (milligram) = 0.001 g  
- `ug` (microgram) = 0.000001 g
- `lb` (pound) = 453.592 g

#### Length Units
- `m` (meter) - base unit
- `cm` (centimeter) = 0.01 m
- `mm` (millimeter) = 0.001 m
- `km` (kilometer) = 1000 m
- `in` or `[in_i]` (inch) = 0.0254 m
- `ft` (foot) = 0.3048 m

#### Volume Units
- `L` (liter) - base unit
- `mL` (milliliter) = 0.001 L
- `dL` (deciliter) = 0.1 L
- `uL` (microliter) = 0.000001 L

#### Time Units
- `s` (second) - base unit
- `min` (minute) = 60 s
- `h` (hour) = 3600 s
- `d` (day) = 86400 s
- `wk` (week) = 604800 s
- `mo` (month) = 2629746 s (average)
- `a` (year) = 31556952 s (average)

#### Pressure Units
- `Pa` (pascal) - base unit
- `kPa` (kilopascal) = 1000 Pa
- `mm[Hg]` (millimeter of mercury) = 133.322 Pa
- `bar` (bar) = 100000 Pa

#### Temperature Units (with Offset Conversion)
- `Cel` (celsius) - base unit
- `K` (kelvin) - offset conversion: K = Cel + 273.15
- `[degF]` (fahrenheit) - offset conversion: °F = (Cel × 9/5) + 32

Temperature units are handled with special offset-based conversion logic to ensure accurate conversions between Celsius, Kelvin, and Fahrenheit scales. Unlike linear unit conversions, temperature conversions account for the different zero points of each scale.

### Unit Conversion Examples

```rust
// Addition and Subtraction with automatic conversion
parser.parse("1.0'kg' + 500.0'g'").unwrap();      // → 1.5 kg (500g → 0.5kg)
parser.parse("2.0'L' + 250.0'mL'").unwrap();      // → 2.25 L (250mL → 0.25L)
parser.parse("1.0'h' + 30.0'min'").unwrap();      // → 1.5 h (30min → 0.5h)
parser.parse("1.5'm' + 50.0'cm'").unwrap();       // → 2.0 m (50cm → 0.5m)

// Temperature conversions with offset handling (Celsius as base unit)
parser.parse("0.0'Cel' + 273.15'K'").unwrap();    // → 273.15 Cel (273.15K → 273.15°C, then add)
parser.parse("32.0'[degF]' + 0.0'Cel'").unwrap(); // → 32.0 [degF] (0°C → 32°F, then add)
parser.parse("20.0'Cel' + 5.0'Cel'").unwrap();    // → 25.0 Cel (same unit addition)
parser.parse("100.0'Cel' - 273.15'K'").unwrap();  // → 100.0 Cel (273.15K → 0°C, then subtract)

// Cross-temperature scale conversions  
parser.parse("0.0'K' + 0.0'Cel'").unwrap();       // → -273.15 K (0°C → -273.15K in Kelvin result)
parser.parse("32.0'[degF]' - 32.0'[degF]'").unwrap(); // → 0.0 [degF] (same unit subtraction)

// Pressure unit conversions
parser.parse("120.0'mm[Hg]' + 10.0'kPa'").unwrap(); // → Automatic conversion and addition
parser.parse("1.0'bar' / 2.0").unwrap();           // → 0.5 bar (scalar division)

// Multiplication and Division by scalars
parser.parse("5.0'mg' * 3.0").unwrap();           // → 15.0 mg
parser.parse("100.0'mL' / 4.0").unwrap();         // → 25.0 mL
parser.parse("37.0'Cel' * 2.0").unwrap();         // → 74.0 Cel

// Division of compatible quantities (dimensionless result)
parser.parse("10.0'kg' / 2.0'kg'").unwrap();      // → 5.0 (dimensionless)
parser.parse("1.0'kg' / 500.0'g'").unwrap();      // → 2.0 (1kg = 1000g, 1000/500 = 2)
parser.parse("212.0'[degF]' / 100.0'Cel'").unwrap(); // → 2.12 (212°F = 100°C, ratio calculation)

// Error cases - incompatible units
parser.parse("1.0'kg' + 1.0'm'").unwrap();        // → Error: incompatible units
parser.parse("20.0'Cel' + 5.0'kg'").unwrap();     // → Error: cannot add temperature and mass
```

### Conversion Process

The unit conversion system follows different processes depending on the unit type:

#### Linear Unit Conversions (Mass, Length, Volume, Time, Pressure)
1. **Compatibility Check**: Verify both quantities are of the same physical type (mass, length, volume, etc.)
2. **Base Unit Conversion**: Convert both values to their respective base units using multiplicative factors
3. **Arithmetic Operation**: Perform the operation on base unit values  
4. **Result Conversion**: Convert back to the left operand's unit system

#### Temperature Unit Conversions (Offset-Based)
1. **Compatibility Check**: Verify both quantities are temperature units
2. **Base Unit Conversion**: Convert both values to Celsius using offset formulas:
   - Kelvin → Celsius: `Cel = K - 273.15`
   - Fahrenheit → Celsius: `Cel = (°F - 32) × 5/9`
   - Celsius → Celsius: `Cel = Cel` (identity)
3. **Arithmetic Operation**: Perform the operation on Celsius values
4. **Result Conversion**: Convert back to the left operand's unit using inverse offset formulas:
   - Celsius → Kelvin: `K = Cel + 273.15` 
   - Celsius → Fahrenheit: `°F = (Cel × 9/5) + 32`
   - Celsius → Celsius: `Cel = Cel` (identity)

This dual approach ensures accurate conversions for both proportional quantities (where zero means "nothing") and interval scales like temperature (where zero is arbitrary).

### Base Units by Category
- **Mass**: gram (g)
- **Length**: meter (m) 
- **Volume**: liter (L)
- **Time**: second (s)
- **Pressure**: pascal (Pa)
- **Temperature**: celsius (Cel) - with special offset-based conversion handling