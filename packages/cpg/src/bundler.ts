import * as wasm from "../wasm-bundler/rh_cpg.js";
import {
  type CpgApplyOptions,
  type CpgContextOptions,
  type MeasurementPeriod,
  type QuestionnaireObservationExtraction,
  type SdcExtractionOptions,
  type WasmCallResult,
  type QuestionnaireValidationResult,
  resourceToJson,
  toPlainResult
} from "./common.js";

export type {
  CpgApplyOptions,
  CpgContextOptions,
  MeasurementPeriod,
  QuestionnaireObservationExtraction,
  QuestionnaireValidationResult,
  SdcExtractionOptions,
  WasmCallResult
} from "./common.js";

function applyArguments(
  definition: unknown,
  subject: string,
  contentBundle: unknown,
  options?: CpgApplyOptions
): [string, string, string, string | undefined, string | undefined, string | undefined, string | undefined, string | undefined, string | undefined, string | undefined] {
  return [
    resourceToJson(definition),
    subject,
    resourceToJson(contentBundle),
    options?.data === undefined ? undefined : resourceToJson(options.data),
    options?.encounter,
    options?.practitioner,
    options?.organization,
    options?.evaluationDate,
    options?.measurementPeriod === undefined ? undefined : resourceToJson(options.measurementPeriod),
    options?.parameters === undefined ? undefined : resourceToJson(options.parameters)
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
      options?.organization,
      options?.evaluationDate,
      options?.measurementPeriod === undefined ? undefined : resourceToJson(options.measurementPeriod),
      options?.parameters === undefined ? undefined : resourceToJson(options.parameters)
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
      options?.organization,
      options?.evaluationDate,
      options?.measurementPeriod === undefined ? undefined : resourceToJson(options.measurementPeriod),
      options?.parameters === undefined ? undefined : resourceToJson(options.parameters)
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

export function extractQuestionnaireObservations(
  questionnaire: unknown,
  response: unknown,
  subject: string,
  options: SdcExtractionOptions
): WasmCallResult<QuestionnaireObservationExtraction> {
  return toPlainResult<QuestionnaireObservationExtraction>(
    wasm.extract_questionnaire_observations(
      resourceToJson(questionnaire),
      resourceToJson(response),
      subject,
      options.encounter,
      options.workflowGated ?? true
    )
  );
}

export { reconcileExtractedObservations } from "./common.js";
