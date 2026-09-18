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
