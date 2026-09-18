# @reasonhealth/cpg

Typed TypeScript wrapper for the Reason Health Clinical Practice Guidelines WebAssembly build.

```ts
import { applyPlanDefinition } from "@reasonhealth/cpg/node";

const result = applyPlanDefinition(
  planDefinition,
  "Patient/123",
  contentBundle
);
```

Exports:

- `@reasonhealth/cpg/node` for Node.js.
- `@reasonhealth/cpg/web` for direct browser loading. Call `initCpg()` before invoking wrapper functions.
- `@reasonhealth/cpg/bundler` for Vite, webpack, Rollup, and similar bundlers.

### Measure evaluation

```ts
import { evaluateMeasure } from "@reasonhealth/cpg/node";

const report = evaluateMeasure(
  measure,
  "Patient/123",
  contentBundle,
  {
    data: dataBundle,
    evaluationDate: "2026-06-15T09:20:00Z",
    measurementPeriod: {
      start: "2026-01-01T00:00:00Z",
      end: "2026-12-31T23:59:59Z"
    },
    parameters: { "Example Flag": true }
  }
);
```

`CpgContextOptions` accepts `data`, `encounter`, `practitioner`,
`organization`, `evaluationDate`, `measurementPeriod`, and `parameters`.
`evaluationDate` accepts RFC 3339 date-times or FHIR dates. A supplied
`measurementPeriod` is passed to CQL as `Measurement Period` and is emitted in
the individual MeasureReport. It must have valid, non-reversed bounds.

For backwards compatibility, an omitted CPG context uses the historical
2026-01-01 clock and a single-day MeasureReport period. Connectathon fixture
adapters must always supply the fixture's explicit patient, evaluation clock,
and measurement period; the compatibility fallback is not demo evidence.

### Questionnaire assembly

```ts
import { assembleQuestionnaire } from "@reasonhealth/cpg/node";

const assembled = assembleQuestionnaire(questionnaire, contentBundle);
```

### Questionnaire population

```ts
import { populateQuestionnaire } from "@reasonhealth/cpg/node";

const response = populateQuestionnaire(
  questionnaire,
  "Patient/123",
  contentBundle,
  { encounter: "Encounter/456" }
);
```

### Questionnaire response validation

```ts
import { validateQuestionnaireResponse } from "@reasonhealth/cpg/node";

const { value } = validateQuestionnaireResponse(questionnaire, response);
// value.issues contains validation messages.
```
