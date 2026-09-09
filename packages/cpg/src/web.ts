import initWasm, * as wasm from "../wasm/rh_cpg.js";
import {
  type CpgApplyOptions,
  type WasmCallResult,
  resourceToJson,
  toPlainResult
} from "./common.js";

export type { CpgApplyOptions, WasmCallResult } from "./common.js";

export const initCpg = initWasm;

function applyArguments(
  definition: unknown,
  subject: string,
  contentBundle: unknown,
  options?: CpgApplyOptions
): [string, string, string, string | undefined, string | undefined, string | undefined, string | undefined] {
  return [
    resourceToJson(definition),
    subject,
    resourceToJson(contentBundle),
    options?.data === undefined ? undefined : resourceToJson(options.data),
    options?.encounter,
    options?.practitioner,
    options?.organization
  ];
}

export function applyPlanDefinition<T = unknown>(
  planDefinition: unknown,
  subject: string,
  contentBundle: unknown,
  options?: CpgApplyOptions
): WasmCallResult<T> {
  return toPlainResult<T>(wasm.apply_plan_definition(...applyArguments(planDefinition, subject, contentBundle, options)));
}

export function applyActivityDefinition<T = unknown>(
  activityDefinition: unknown,
  subject: string,
  contentBundle: unknown,
  options?: CpgApplyOptions
): WasmCallResult<T> {
  return toPlainResult<T>(wasm.apply_activity_definition(...applyArguments(activityDefinition, subject, contentBundle, options)));
}
