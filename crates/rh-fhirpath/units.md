# Quantity units and conversion

The evaluator uses checked decimal conversion for a subset of UCUM units
and for the FHIRPath calendar units. Supported compound expressions use
products, division, and integer powers of recognized linear units.
[Converter implementation](src/evaluator/operations/units.rs)

| Dimension | Supported unit codes and aliases |
| --- | --- |
| Dimensionless | `1` |
| Mass | `g`, `kg`, `mg`, `ug`, `ng`, `lb`, `[lb_av]`, `lbs`, `lbs.` |
| Length | `m`, `cm`, `mm`, `km`, `in`, `in_i`, `[in_i]`, `ft`, `[ft_i]` |
| Volume | `L`, `mL`, `dL`, `uL` and their lowercase aliases |
| Pressure | `Pa`, `kPa`, `mm[Hg]`, `bar` |
| Definite time | `ms`, `s`, `min`, `h`, `d`, `wk`, `mo`, `a` |
| Calendar time | Millisecond, second, minute, hour, day, week, month, and year, in singular or plural form |
| Temperature | `K`, `Cel`, and `[degF]` for conversion and comparison |

Explicit conversion uses `toQuantity(unit)`. Comparison returns an empty
collection when conversion fails. Addition and subtraction apply the
calendar and UCUM rules for the operation. Convert affine temperatures to
Kelvin before arithmetic. Dividing by zero returns an empty collection.

```fhirpath
1 'kg' + 500 'g'
1 'mg/dL'.toQuantity('g/L')
0 'Cel'.toQuantity('K')
1 'a'.toQuantity('d')
60 / 1 's'
1 year > 2 months
```

The FHIR models use unit codes and systems during Quantity comparison.
Where both nodes lack a UCUM conversion, their available unit labels still
matter. Calendar years and months have separate equality, equivalence,
and arithmetic rules. [FHIRPath Quantity rules](https://hl7.org/fhirpath/STU3/en/index.html#quantity) ·
[Comparison rules](https://hl7.org/fhirpath/STU3/en/index.html#comparison)
