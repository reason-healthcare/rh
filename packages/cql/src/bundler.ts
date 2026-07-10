import * as wasm from "../wasm-bundler/rh_cql.js";
import {
  type AnalyticsOptions,
  type CompileOptions,
  type EvaluateOptions,
  type WasmCallResult,
  toPlainResult
} from "./common.js";

export type { AnalyticsOptions, CompileOptions, EvaluateOptions, WasmCallResult } from "./common.js";

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

const defaultSqlOnFhirTarget = "sql-on-fhir";
const defaultCanonicalBase = "https://reason.health/rh/generated/sql-on-fhir";

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

export function explainParse(source: string): WasmCallResult<string> {
  return toPlainResult<string>(wasm.explain_cql_parse(source), false);
}

export function explainCompile(
  source: string,
  options: CompileOptions = {}
): WasmCallResult<string> {
  const rawOptions = compileOptions(options);
  try {
    return toPlainResult<string>(wasm.explain_cql_compile(source, rawOptions), false);
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

export function inspect<T = unknown>(
  source: string,
  options: CompileOptions = {}
): WasmCallResult<T> {
  const rawOptions = compileOptions(options);
  try {
    return toPlainResult<T>(wasm.inspect_cql(source, rawOptions), true);
  } finally {
    rawOptions.free();
  }
}

export function dataRequirements<T = unknown>(
  source: string,
  options: CompileOptions = {}
): WasmCallResult<T> {
  const rawOptions = compileOptions(options);
  try {
    return toPlainResult<T>(wasm.cql_data_requirements(source, rawOptions), true);
  } finally {
    rawOptions.free();
  }
}

export function relationalPlan<T = unknown>(
  source: string,
  options: AnalyticsOptions = {}
): WasmCallResult<T> {
  const rawOptions = compileOptions(options);
  try {
    return toPlainResult<T>(
      wasm.cql_relational_plan(source, options.target ?? defaultSqlOnFhirTarget, rawOptions),
      true
    );
  } finally {
    rawOptions.free();
  }
}

export function lowerCheck<T = unknown>(
  source: string,
  options: AnalyticsOptions = {}
): WasmCallResult<T> {
  const rawOptions = compileOptions(options);
  try {
    return toPlainResult<T>(
      wasm.cql_lower_check(source, options.target ?? defaultSqlOnFhirTarget, rawOptions),
      true
    );
  } finally {
    rawOptions.free();
  }
}

export function emitViewDefinitions<T = unknown>(
  source: string,
  options: AnalyticsOptions = {}
): WasmCallResult<T> {
  const rawOptions = compileOptions(options);
  try {
    return toPlainResult<T>(
      wasm.cql_emit_view_definitions(source, options.canonicalBase ?? defaultCanonicalBase, rawOptions),
      true
    );
  } finally {
    rawOptions.free();
  }
}

export function emitSql<T = unknown>(
  source: string,
  options: AnalyticsOptions = {}
): WasmCallResult<T> {
  const rawOptions = compileOptions(options);
  try {
    return toPlainResult<T>(
      wasm.cql_emit_sql(source, options.canonicalBase ?? defaultCanonicalBase, rawOptions),
      true
    );
  } finally {
    rawOptions.free();
  }
}

export function emitSqlQueryLibrary<T = unknown>(
  source: string,
  options: AnalyticsOptions = {}
): WasmCallResult<T> {
  const rawOptions = compileOptions(options);
  try {
    return toPlainResult<T>(
      wasm.cql_emit_sql_query_library(source, options.canonicalBase ?? defaultCanonicalBase, rawOptions),
      true
    );
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
