import initWasm, * as wasm from "../wasm/rh_fhirpath.js";
import {
  type FhirPathEvaluateOptions,
  type FhirPathParseOptions,
  type WasmCallResult,
  resourceToJson,
  toPlainResult
} from "./common.js";

export type { FhirPathEvaluateOptions, FhirPathParseOptions, WasmCallResult } from "./common.js";

export const initFhirPath = initWasm;

function parseOptions(options: FhirPathParseOptions = {}): wasm.ParseOptions {
  const raw = new wasm.ParseOptions();
  raw.format = options.format ?? "json";
  return raw;
}

function evaluateOptions(options: FhirPathEvaluateOptions = {}): wasm.EvaluateOptions {
  const raw = new wasm.EvaluateOptions();
  raw.format = options.format ?? "json";
  return raw;
}

export function parseExpression<T = unknown>(
  expression: string,
  options: FhirPathParseOptions = {}
): WasmCallResult<T> {
  const rawOptions = parseOptions(options);
  try {
    return toPlainResult<T>(
      wasm.parse_fhirpath_expression(expression, rawOptions),
      rawOptions.format === "json"
    );
  } finally {
    rawOptions.free();
  }
}

export function evaluateExpression<T = unknown>(
  expression: string,
  resource: unknown,
  options: FhirPathEvaluateOptions = {}
): WasmCallResult<T> {
  const rawOptions = evaluateOptions(options);
  try {
    return toPlainResult<T>(
      wasm.evaluate_fhirpath_expression(expression, resourceToJson(resource), rawOptions),
      rawOptions.format === "json"
    );
  } finally {
    rawOptions.free();
  }
}

export function validateExpression(expression: string): WasmCallResult<{ valid: boolean; message: string }> {
  return toPlainResult(wasm.validate_fhirpath_expression(expression), true);
}

export function version(): string {
  return wasm.get_version();
}
