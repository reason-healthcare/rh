import { createRequire } from "node:module";
import type * as Raw from "../wasm-node/rh_vcl.js";
import {
  type VclParseOptions,
  type VclTranslateOptions,
  type WasmCallResult,
  toPlainResult
} from "./common.js";

export type { VclParseOptions, VclTranslateOptions, WasmCallResult } from "./common.js";

const require = createRequire(import.meta.url);
const wasm = require("../wasm-node/rh_vcl.js") as typeof Raw;

function parseOptions(options: VclParseOptions = {}): Raw.ParseOptions {
  const raw = new wasm.ParseOptions();
  raw.format = options.format ?? "json";
  return raw;
}

function translateOptions(options: VclTranslateOptions = {}): Raw.TranslateOptions {
  const raw = new wasm.TranslateOptions();
  raw.format = options.format ?? "json";
  raw.default_system = options.defaultSystem;
  raw.separate_conjunction_includes = options.separateConjunctionIncludes ?? false;
  return raw;
}

export function parseExpression<T = unknown>(
  expression: string,
  options: VclParseOptions = {}
): WasmCallResult<T> {
  const rawOptions = parseOptions(options);
  try {
    return toPlainResult<T>(
      wasm.parse_vcl_expression(expression, rawOptions),
      rawOptions.format === "json"
    );
  } finally {
    rawOptions.free();
  }
}

export function translateExpression<T = unknown>(
  expression: string,
  options: VclTranslateOptions = {}
): WasmCallResult<T> {
  const rawOptions = translateOptions(options);
  try {
    return toPlainResult<T>(
      wasm.translate_vcl_expression(expression, rawOptions),
      rawOptions.format === "json"
    );
  } finally {
    rawOptions.free();
  }
}

export function explainExpression<T = unknown>(
  expression: string,
  defaultSystem?: string
): WasmCallResult<T> {
  return toPlainResult<T>(
    defaultSystem === undefined
      ? wasm.explain_vcl_simple(expression)
      : wasm.explain_vcl_with_system(expression, defaultSystem),
    true
  );
}

export function validateExpression(expression: string): WasmCallResult<{ valid: boolean; message: string }> {
  return toPlainResult(wasm.validate_vcl_expression(expression), true);
}

export function version(): string {
  return wasm.get_version();
}
