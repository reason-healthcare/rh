import { createRequire } from "node:module";
import type * as Raw from "../wasm-node/rh_cpg.js";
import {
  type CpgApplyOptions,
  type CpgContextOptions,
  type WasmCallResult,
  type QuestionnaireValidationResult,
  resourceToJson,
  toPlainResult
} from "./common.js";

export type {
  CpgApplyOptions,
  CpgContextOptions,
  QuestionnaireValidationResult,
  WasmCallResult
} from "./common.js";

const require = createRequire(import.meta.url);
const wasm = require("../wasm-node/rh_cpg.js") as typeof Raw;

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

export function evaluateMeasure<T = unknown>(
  measure: unknown,
  subject: string,
  contentBundle: unknown,
  options?: CpgContextOptions
): WasmCallResult<T> {
  return toPlainResult<T>(
    wasm.evaluate_measure(
      resourceToJson(measure),
      subject,
      resourceToJson(contentBundle),
      options?.data === undefined ? undefined : resourceToJson(options.data),
      options?.encounter,
      options?.practitioner,
      options?.organization
    )
  );
}

export function assembleQuestionnaire<T = unknown>(
  questionnaire: unknown,
  contentBundle: unknown
): WasmCallResult<T> {
  return toPlainResult<T>(
    wasm.assemble_questionnaire(resourceToJson(questionnaire), resourceToJson(contentBundle))
  );
}

export function populateQuestionnaire<T = unknown>(
  questionnaire: unknown,
  subject: string,
  contentBundle: unknown,
  options?: CpgContextOptions
): WasmCallResult<T> {
  return toPlainResult<T>(
    wasm.populate_questionnaire(
      resourceToJson(questionnaire),
      subject,
      resourceToJson(contentBundle),
      options?.data === undefined ? undefined : resourceToJson(options.data),
      options?.encounter,
      options?.practitioner,
      options?.organization
    )
  );
}

export function validateQuestionnaireResponse(
  questionnaire: unknown,
  response: unknown
): WasmCallResult<QuestionnaireValidationResult> {
  return toPlainResult<QuestionnaireValidationResult>(
    wasm.validate_questionnaire_response(resourceToJson(questionnaire), resourceToJson(response))
  );
}
