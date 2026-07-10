export interface CompileOptions {
  pretty?: boolean;
  sourceMap?: boolean;
}

export interface EvaluateOptions {
  expression: string;
  data?: Record<string, unknown[] | unknown>;
  parameters?: Record<string, unknown>;
}

export interface AnalyticsOptions extends CompileOptions {
  target?: string;
  canonicalBase?: string;
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

export function toPlainResult<T>(result: RawWasmResult, parseJson: boolean): WasmCallResult<T> {
  try {
    const data = result.data;
    const output: WasmCallResult<T> = {
      success: result.success,
      data,
      error: result.error
    };

    if (result.success && parseJson && data !== undefined) {
      output.value = JSON.parse(data) as T;
    }

    return output;
  } finally {
    result.free?.();
  }
}
