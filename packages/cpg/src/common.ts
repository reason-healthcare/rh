export interface MeasurementPeriod {
  start: string;
  end: string;
  startInclusive?: boolean;
  endInclusive?: boolean;
}

export interface CpgContextOptions {
  data?: unknown;
  encounter?: string;
  practitioner?: string;
  organization?: string;
  /** RFC 3339 date-time or FHIR date used for deterministic CQL clock evaluation. */
  evaluationDate?: string;
  /** FHIR period passed to CQL as `Measurement Period` and emitted in MeasureReports. */
  measurementPeriod?: MeasurementPeriod;
  /** Additional CQL parameter values keyed by their declared CQL names. */
  parameters?: Record<string, unknown>;
}

export type CpgApplyOptions = CpgContextOptions;

export interface QuestionnaireValidationResult {
  issues: string[];
}

export interface SdcExtractionOptions {
  /** Explicit encounter binding required by the constrained SDC extraction subset. */
  encounter: string;
  /** Defaults to true, applying completed-response and required-answer gating. */
  workflowGated?: boolean;
}

export interface QuestionnaireObservationExtraction {
  status: "extracted" | "not-invoked";
  reason?: string;
  transaction?: unknown;
  observations?: unknown[];
}

export interface WasmCallResult<T = unknown> {
  success: boolean;
  data?: string;
  error?: string;
  value?: T;
}

export interface RawWasmResult {
  readonly success: boolean;
  readonly data?: string;
  readonly error?: string;
  free?: () => void;
}

export function toPlainResult<T>(result: RawWasmResult): WasmCallResult<T> {
  try {
    const data = result.data;
    const output: WasmCallResult<T> = {
      success: result.success,
      data,
      error: result.error
    };

    if (result.success && data !== undefined) {
      output.value = JSON.parse(data) as T;
    }

    return output;
  } finally {
    result.free?.();
  }
}

export function resourceToJson(resource: unknown): string {
  return typeof resource === "string" ? resource : JSON.stringify(resource);
}

/**
 * Replace only generated Observations derived solely from
 * `questionnaireResponseReference` with the Observation resources in an SDC
 * transaction Bundle. Transaction request wrappers are intentionally not
 * copied into clinical data; their `fullUrl` values are retained.
 */
export function reconcileExtractedObservations(
  dataBundle: unknown,
  questionnaireResponseReference: string,
  transaction?: unknown
): Record<string, unknown> {
  const data = parseBundle(dataBundle, "data bundle");
  const existingEntries = Array.isArray(data.entry) ? data.entry : [];
  const extractedEntries = transaction === undefined
    ? []
    : extractionEntries(transaction, questionnaireResponseReference);

  const retainedEntries = existingEntries.filter((entry) => {
    if (!isRecord(entry) || !isRecord(entry.resource) || entry.resource.resourceType !== "Observation") {
      return true;
    }
    return !isGeneratedObservation(entry.resource, questionnaireResponseReference);
  });

  return {
    ...data,
    entry: [...retainedEntries, ...extractedEntries]
  };
}

function extractionEntries(
  transaction: unknown,
  questionnaireResponseReference: string
): Array<Record<string, unknown>> {
  const bundle = parseBundle(transaction, "SDC transaction Bundle");
  if (bundle.type !== "transaction" || !Array.isArray(bundle.entry)) {
    throw new Error("SDC transaction Bundle must have type=transaction and an entry array");
  }
  return bundle.entry.map((entry, index) => {
    if (!isRecord(entry) || typeof entry.fullUrl !== "string" || !isRecord(entry.resource)) {
      throw new Error(`SDC transaction entry ${index} must contain fullUrl and resource`);
    }
    if (entry.resource.resourceType !== "Observation") {
      throw new Error(`SDC transaction entry ${index} must contain an Observation`);
    }
    if (!isGeneratedObservation(entry.resource, questionnaireResponseReference)) {
      throw new Error(`SDC transaction entry ${index} does not have the expected exact derivedFrom reference`);
    }
    return { fullUrl: entry.fullUrl, resource: entry.resource };
  });
}

function isGeneratedObservation(
  resource: Record<string, unknown>,
  questionnaireResponseReference: string
): boolean {
  const derivedFrom = resource.derivedFrom;
  return Array.isArray(derivedFrom)
    && derivedFrom.length === 1
    && isRecord(derivedFrom[0])
    && derivedFrom[0].reference === questionnaireResponseReference;
}

function parseBundle(value: unknown, label: string): Record<string, unknown> {
  const parsed: unknown = typeof value === "string" ? JSON.parse(value) : value;
  if (!isRecord(parsed) || parsed.resourceType !== "Bundle") {
    throw new Error(`${label} must be a FHIR Bundle`);
  }
  return parsed;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}
