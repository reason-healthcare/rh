import * as wasm from "../wasm-bundler/rh_cql.js";
import {
  type CompileOptions,
  type EvaluateOptions,
  type WasmCallResult,
  toPlainResult
} from "./common.js";

export type { CompileOptions, EvaluateOptions, WasmCallResult } from "./common.js";

function compileOptions(options: CompileOptions = {}): wasm.CompileOptions {
  const raw = new wasm.CompileOptions();
  raw.pretty = options.pretty ?? true;
  raw.source_map = options.sourceMap ?? false;
  return raw;
}

function evaluateOptions(options: EvaluateOptions): wasm.EvaluateOptions {
  const raw = new wasm.EvaluateOptions(options.expression);
  raw.data_json = JSON.stringify(options.data ?? {});
  raw.parameters_json = JSON.stringify(options.parameters ?? {});
  return raw;
}

export function compile<T = unknown>(
  source: string,
  options: CompileOptions = {}
): WasmCallResult<T> {
  const rawOptions = compileOptions(options);
  try {
    return toPlainResult<T>(wasm.compile_cql(source, rawOptions), true);
  } finally {
    rawOptions.free();
  }
}

export function evaluate<T = unknown>(
  elmJson: string | unknown,
  options: EvaluateOptions
): WasmCallResult<T> {
  const rawOptions = evaluateOptions(options);
  const elm = normalizeElmInput(elmJson);
  try {
    return toPlainResult<T>(wasm.evaluate_cql_elm(elm, rawOptions), true);
  } finally {
    rawOptions.free();
  }
}

export function version(): string {
  return wasm.get_version();
}

function normalizeElmInput(elmJson: string | unknown): string {
  const value = typeof elmJson === "string" ? JSON.parse(elmJson) : elmJson;
  if (value && typeof value === "object" && "library" in value) {
    return JSON.stringify((value as { library: unknown }).library);
  }
  return JSON.stringify(value);
}
