# VCL (ValueSet Compose Language) Examples

This document provides comprehensive examples of all VCL language features supported by the rh-vcl parser. Each example includes the VCL expression, a description of what it does, and the resulting FHIR ValueSet.compose structure.

## Table of Contents

1. [Basic Expressions](#basic-expressions)
2. [System URIs](#system-uris)
3. [Filters](#filters)
4. [Operations](#operations)
5. [Complex Expressions](#complex-expressions)
6. [ValueSet Inclusions](#valueset-inclusions)
7. [Of Operations](#of-operations)
8. [Code List Usage](#code-list-usage)
9. [Filter Lists](#filter-lists)
10. [Error Cases](#error-cases)

---

## Note on Default Systems

Many VCL expressions require a system URI to be valid. When no system URI is explicitly provided (like `(http://snomed.info/sct)`), you can use a default system. In the CLI tools:
- Translate: Use `--default-system "http://snomed.info/sct"` or `-s "http://snomed.info/sct"`
- REPL: Use `--default-system "http://snomed.info/sct"` 
- Library: Use `VclTranslator::with_default_system("http://snomed.info/sct".to_string())`

Examples marked as requiring a default system will only work in these contexts.

---

## Basic Expressions

### Wildcard with System

**VCL:** `(http://snomed.info/sct)*`

**Description:** Matches all codes from the specified system. Wildcard requires a system URI.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct"
    }
  ]
}
```

### Simple Code (with Default System)

**VCL:** `123456` *(with default system: http://snomed.info/sct)*

**Description:** A specific code without an explicit system URI. Requires a default system to be specified (e.g., using the REPL's `--default-system` option).

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "123456"
        }
      ]
    }
  ]
}
```

### Quoted Code (with Default System)

**VCL:** `"special-code with-spaces"` *(with default system: http://snomed.info/sct)*

**Description:** A code containing spaces or special characters, enclosed in quotes. Requires a default system.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "special-code with-spaces"
        }
      ]
    }
  ]
}
```

---

## System URIs

### Code with System URI

**VCL:** `(http://snomed.info/sct)123456789`

**Description:** A SNOMED CT code with its system URI explicitly specified.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "123456789"
        }
      ]
    }
  ]
}
```

### Wildcard with System URI

**VCL:** `(http://loinc.org)*`

**Description:** All codes from the LOINC code system.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://loinc.org"
    }
  ]
}
```

### Multiple Codes with System URI (Conjunction)

**VCL:** `(http://snomed.info/sct)123456, (http://snomed.info/sct)789012, (http://snomed.info/sct)"quoted-code"`

**Description:** Multiple specific codes from the same system using conjunction. Each code requires its own system URI.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "123456"
        }
      ]
    },
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "789012"
        }
      ]
    },
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "quoted-code"
        }
      ]
    }
  ]
}
```

### System URI with Version Specifier

**VCL:** `(http://snomed.info/sct|20250901)123456789`

**Description:** A SNOMED CT code with an explicit version specifier. The version follows the pipe character `|` after the system URI.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "version": "20250901",
      "concept": [
        {
          "code": "123456789"
        }
      ]
    }
  ]
}
```

### Complex Version Format

**VCL:** `(http://loinc.org|2.76)8302-2`

**Description:** LOINC code with a complex version string including dots and version prefixes.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://loinc.org",
      "version": "2.76",
      "concept": [
        {
          "code": "8302-2"
        }
      ]
    }
  ]
}
```

### Date-based Version

**VCL:** `(http://acme.org/cs|2025-01-01)74400008`

**Description:** Version specifier using a date format.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://acme.org/cs", 
      "version": "2025-01-01",
      "concept": [
        {
          "code": "74400008"
        }
      ]
    }
  ]
}
```

### Wildcard with Version

**VCL:** `(http://loinc.org|2.76)*`

**Description:** All codes from LOINC version 2.76.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://loinc.org",
      "version": "2.76"
    }
  ]
}
```

---

## Filters

### Equality Filter

**VCL:** `(http://snomed.info/sct)status = "active"`

**Description:** All SNOMED CT codes where the status property equals "active".

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "status",
          "op": "=",
          "value": "active"
        }
      ]
    }
  ]
}
```

### Is-a Filter (Subsumption)

**VCL:** `(http://snomed.info/sct)category << 123456789`

**Description:** All SNOMED CT codes where the category is a child/descendant of concept 123456789.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "category",
          "op": "is-a",
          "value": "123456789"
        }
      ]
    }
  ]
}
```

### Descendant-of Filter

**VCL:** `(http://snomed.info/sct)concept < 123456789`

**Description:** All SNOMED CT codes that are descendants of concept 123456789.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "concept",
          "op": "descendent-of",
          "value": "123456789"
        }
      ]
    }
  ]
}
```

### Regular Expression Filter

**VCL:** `(http://loinc.org)code / "^[0-9]+-[0-9]+$"`

**Description:** LOINC codes where the code property matches the regular expression pattern.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://loinc.org",
      "filter": [
        {
          "property": "code",
          "op": "regex",
          "value": "^[0-9]+-[0-9]+$"
        }
      ]
    }
  ]
}
```

### In Filter (Set Membership)

**VCL:** `(http://snomed.info/sct)status ^ {active, "in-review", deprecated}`

**Description:** Codes where the status property is one of the specified values.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "status",
          "op": "in",
          "value": "active,in-review,deprecated"
        }
      ]
    }
  ]
}
```

### Multiple Filter Operators

**VCL:** `(http://snomed.info/sct)category ~<< 123456789`

**Description:** Using the "is-not-a" operator (~<<).

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "category",
          "op": "is-not-a",
          "value": "123456789"
        }
      ]
    }
  ]
}
```

---

## Operations

### Conjunction (AND)

**VCL:** `(http://snomed.info/sct)123456, 789012, 345678`

**Description:** Include multiple specific codes from the same system.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "123456"
        }
      ]
    },
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "789012"
        }
      ]
    },
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "345678"
        }
      ]
    }
  ]
}
```

### Disjunction (OR)

**VCL:** `(http://snomed.info/sct)123456; (http://snomed.info/sct)789012; (http://snomed.info/sct)345678`

**Description:** A set of alternative codes - any one of these codes. Each code requires its own system URI.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "123456"
        },
        {
          "code": "789012"
        },
        {
          "code": "345678"
        }
      ]
    }
  ]
}
```

### Exclusion (NOT)

**VCL:** `(http://snomed.info/sct)* - (http://snomed.info/sct)inactive`

**Description:** All SNOMED CT codes except for the specified code. Exclusions require individual system URIs.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct"
    }
  ],
  "exclude": [
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "inactive"
        }
      ]
    }
  ]
}
```

### Mixed Operations

**VCL:** `(http://snomed.info/sct)status = "active", (http://snomed.info/sct)category << 123456`

**Description:** Codes with active status AND codes with category under 123456 (conjunction of filters).

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "status",
          "op": "=",
          "value": "active"
        }
      ]
    },
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "category",
          "op": "is-a",
          "value": "123456"
        }
      ]
    }
  ]
}
```

---

## Complex Expressions

### Filter with Exclusion

**VCL:** `(http://snomed.info/sct)status = "active" - (http://snomed.info/sct)deprecated`

**Description:** Codes with active status, excluding deprecated ones.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "status",
          "op": "=",
          "value": "active"
        }
      ]
    }
  ],
  "exclude": [
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "deprecated"
        }
      ]
    }
  ]
}
```

### Cross-System Expression

**VCL:** `(http://snomed.info/sct)123456; (http://loinc.org)LA123-4`

**Description:** Including codes from multiple terminology systems.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "123456"
        }
      ]
    },
    {
      "system": "http://loinc.org",
      "concept": [
        {
          "code": "LA123-4"
        }
      ]
    }
  ]
}
```

### Complex Filter Expression

**VCL:** `(http://snomed.info/sct)status = "active", (http://snomed.info/sct)category << 123456, (http://snomed.info/sct)type ~^ {deprecated, "not applicable"}`

**Description:** Multiple filters combined with exclusions - active codes under category 123456 that are procedures or observations, excluding deprecated ones.

**FHIR Output:**
```json
  {
    "include": [
      {
        "system": "http://snomed.info/sct",
        "filter": [
          {
            "property": "status",
            "op": "=",
            "value": "active"
          }
        ]
      },
      {
        "system": "http://snomed.info/sct",
        "filter": [
          {
            "property": "category",
            "op": "is-a",
            "value": "123456"
          }
        ]
      },
      {
        "system": "http://snomed.info/sct",
        "filter": [
          {
            "property": "type",
            "op": "not-in",
            "value": "deprecated,not applicable"
          }
        ]
      }
    ]
  }
```

---

## ValueSet Inclusions

### Simple ValueSet Inclusion

**VCL:** `^http://example.org/fhir/ValueSet/my-valueset`

**Description:** Include all codes from another ValueSet by reference.

**FHIR Output:**
```json
{
  "include": [
    {
      "valueSet": [
        "http://example.org/fhir/ValueSet/my-valueset"
      ]
    }
  ]
}
```

### ValueSet with System URI

**VCL:** `^(http://snomed.info/sct)`

**Description:** Include all codes from a specific code system (equivalent to wildcard with system).

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct"
    }
  ]
}
```

### Mixed ValueSet and Code Inclusions

**VCL:** `^http://example.org/fhir/ValueSet/base-codes, (http://snomed.info/sct)123456, 789012`

**Description:** Include a ValueSet plus additional specific codes.

**FHIR Output:**
```json
{
  "include": [
    {
      "valueSet": [
        "http://example.org/fhir/ValueSet/base-codes"
      ]
    },
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "123456"
        }
      ]
    },
    {
      "system": "http://snomed.info/sct",
      "concept": [
        {
          "code": "789012"
        }
      ]
    }
  ]
}
```

---

## Of Operations

### Wildcard Of Operation

**VCL:** `(http://snomed.info/sct)*.category`

**Description:** Get the category property of all codes in the system.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "category",
          "op": "=",
          "value": "*"
        }
      ]
    }
  ]
}
```

### Code List Of Operation

**VCL:** `(http://snomed.info/sct){123456, 789012, 345678}.status`

**Description:** Get the status property of specific codes.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "status",
          "op": "in",
          "value": "123456,789012,345678"
        }
      ]
    }
  ]
}
```

### Single Code Of Operation

**VCL:** `(http://snomed.info/sct)123456.status`

**Description:** Get the status property of a specific code.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "status",
          "op": "=",
          "value": "123456"
        }
      ]
    }
  ]
}
```

---

## Code List Usage

### Code Lists in Filters

**VCL:** `(http://snomed.info/sct)status ^ {active, "in-review", deprecated}`

**Description:** Code lists can be used in property filters with the "in" operator to match multiple values.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "status",
          "op": "in",
          "value": "active,in-review,deprecated"
        }
      ]
    }
  ]
}
```

### Code Lists with Properties (Of Operations)

**VCL:** `(http://snomed.info/sct){123456, 789012, 345678}.status`

**Description:** Code lists can be used with properties to filter by property values of specific codes.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "status",
          "op": "in",
          "value": "123456,789012,345678"
        }
      ]
    }
  ]
}
```

---

## Filter Lists

Filter lists allow you to specify multiple filters that must all match (logical AND). They can be used with or without explicit system URIs.

### Complex Filter Lists

**VCL:** `(http://snomed.info/sct){has_ingredient=1886, has_dose_form^{concept<<1151133}}`

**Description:** Complex filter lists with multiple filters. When using the `^{filter}` syntax, the inner filter's operator is extracted and applied directly.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "has_ingredient",
          "op": "=",
          "value": "1886"
        },
        {
          "property": "has_dose_form",
          "op": "is-a",
          "value": "1151133"
        }
      ]
    }
  ]
}
```

### Simple Filter Lists

**VCL:** `(http://snomed.info/sct){status="active", category<<12345}`

**Description:** Multiple filters combined in a single filter list for logical AND operations.

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "status",
          "op": "=",
          "value": "active"
        },
        {
          "property": "category",
          "op": "is-a",
          "value": "12345"
        }
      ]
    }
  ]
}
```

### Filter Lists with Default System (requires default system)

**VCL:** `{has_ingredient=1886, has_dose_form=317541}`

**Description:** Filter lists without explicit system URI. Requires using `--default-system` CLI option or `VclTranslator::with_default_system()` in library usage.

**CLI Usage:**
```bash
rh vcl translate --default-system "http://snomed.info/sct" "{has_ingredient=1886, has_dose_form=317541}"
```

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "has_ingredient",
          "op": "=",
          "value": "1886"
        },
        {
          "property": "has_dose_form",
          "op": "=",
          "value": "317541"
        }
      ]
    }
  ]
}
```

### Parentheses Filter Conjunction (requires default system)

**VCL:** `(has_ingredient=1886, has_dose_form=317541)`

**Description:** Parentheses syntax creates nested expressions with conjunctions. Filters with the same system are automatically merged into a single filter list for logical AND operations.

**CLI Usage:**
```bash
rh vcl translate --default-system "http://snomed.info/sct" "(has_ingredient=1886, has_dose_form=317541)"
```

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "has_ingredient",
          "op": "=",
          "value": "1886"
        },
        {
          "property": "has_dose_form",
          "op": "=",
          "value": "317541"
        }
      ]
    }
  ]
}
```

**Note:** Both `{filter, filter}` and `(filter, filter)` syntaxes produce equivalent results when all filters use the same system URI.

---

## Error Cases

### Empty Expression

**VCL:** *(empty string)*

**Description:** Empty input is not valid VCL.

**Error:** `Parse error at position 0: Expected expression`

### Invalid Operator

**VCL:** `code1 $ code2`

**Description:** Unknown operator `$` is not supported.

**Error:** `Parse error at position 6: Unknown operator '$'`

### Unclosed Parentheses

**VCL:** `(http://example.org code1`

**Description:** Missing closing parenthesis for system URI.

**Error:** `Parse error at position 20: Expected closing parenthesis`

### Unclosed Quote

**VCL:** `"unclosed string`

**Description:** Missing closing quote for quoted string.

**Error:** `Parse error at position 16: Unclosed quoted string`

### Invalid URI

**VCL:** `(not-a-valid-uri)code1`

**Description:** System URIs must be valid URIs.

**Error:** `Parse error: Invalid URI format`

---

## Real-World Examples

### Clinical Conditions

**VCL:** `(http://snomed.info/sct)category << 404684003, status = "active" - {inactive, "entered-in-error"}`

**Description:** Active clinical findings (SNOMED CT), excluding inactive or erroneous entries.

### Laboratory Results

**VCL:** `(http://loinc.org)CLASS = "CHEM", STATUS = "ACTIVE"; (http://snomed.info/sct)category << 108252007`

**Description:** Active chemistry lab tests from LOINC or laboratory procedures from SNOMED CT.

### Medication Codes

**VCL:** `(http://www.nlm.nih.gov/research/umls/rxnorm)TTY ^ {SCD, SBD, GPCK}, status = "active"`

**Description:** Active RxNorm semantic clinical drugs, branded drugs, and generic packs.

### Comprehensive ValueSet

**VCL:** `^http://hl7.org/fhir/ValueSet/condition-clinical, (http://snomed.info/sct)category << 404684003, status = "active" - {inactive, deprecated, "not applicable"}`

**Description:** Base condition clinical codes plus active SNOMED CT clinical findings, excluding various inactive states.

---

## Usage with rh-vcl

All these examples can be parsed and translated using the rh-vcl library:

```rust
use rh_vcl::{parse_vcl, translate_vcl_string_to_fhir, VclTranslator};

// Parse any VCL expression
let vcl_expr = parse_vcl("(http://snomed.info/sct)status = \"active\"")?;

// Translate to FHIR
let fhir_compose = translate_vcl_string_to_fhir("(http://snomed.info/sct)123456")?;

// Use custom translator with default system
let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
let fhir_compose = translator.translate(&vcl_expr)?;

// Analyze the parsed expression
println!("System URIs: {:?}", vcl_expr.system_uris());
println!("Codes: {:?}", vcl_expr.codes());
```

For more information, see the [main documentation](README.md) and the [API reference](https://docs.rs/rh-vcl).