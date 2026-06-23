#!/usr/bin/env node
import { createRequire } from "node:module";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawn } from "node:child_process";

const require = createRequire(`${process.cwd()}/package.json`);
const cql = require("cql-execution");

const args = parseArgs(process.argv.slice(2));
const casesPath = requiredArg(args, "--cases");
const outputPath = requiredArg(args, "--output");
const rhCli = args["--rh-cli"] ?? "rh";
const limit = args["--limit"] ? Number(args["--limit"]) : undefined;

const cases = JSON.parse(await readFile(casesPath, "utf8"));
const selectedCases = Number.isFinite(limit) ? cases.slice(0, limit) : cases;
const results = [];

for (const testCase of selectedCases) {
  results.push(await runCase(testCase));
}

await writeFile(outputPath, JSON.stringify({ results }, null, 2));

async function runCase(testCase) {
  const id = testCase.id;
  const cqlSource = `library HlTest\ncontext Patient\ndefine Result: ${testCase.expression}\n`;
  const workDir = await mkdtemp(join(tmpdir(), "rh-cql-js-"));
  const cqlPath = join(workDir, "HlTest.cql");

  try {
    let elm = testCase.elm;
    if (!elm) {
      await writeFile(cqlPath, cqlSource);
      const compile = await runProcess(rhCli, ["cql", "compile", cqlPath]);
      if (compile.code !== 0) {
        return {
          id,
          status: testCase.invalid ? "pass" : "compile_error",
          category: testCase.invalid ? "invalid_rejected" : "rh_compile_failure",
          notes: testCase.invalid
            ? "invalid expression rejected by rh-cql before JavaScript evaluation"
            : truncate(firstNonEmpty(compile.stderr, compile.stdout, "rh-cql compile failed"))
        };
      }
      elm = JSON.parse(compile.stdout);
    }

    const library = new cql.Library(elm);
    const executor = new cql.Executor(library);
    const patientSource = new cql.PatientSource([{ id: "1", recordType: "Patient" }]);
    const executionDateTime = new cql.DateTime(2023, 1, 1, 0, 0, 0, 0, 0);

    try {
      const execution = await executor.exec(patientSource, executionDateTime);
      const actual = execution.patientResults?.["1"]?.Result;
      if (testCase.invalid) {
        return {
          id,
          status: "fail",
          category: "invalid_accepted",
          notes: `invalid expression evaluated in JavaScript: ${summarize(actual)}`
        };
      }

      const expected = parseExpected(testCase.expected_output);
      if (expected.kind === "unsupported") {
        return {
          id,
          status: "skip",
          category: "unsupported_expected_output",
          notes: expected.reason
        };
      }

      if (valuesMatch(actual, expected.value)) {
        return { id, status: "pass", category: "pass", notes: "" };
      }

      return {
        id,
        status: "fail",
        category: "value_mismatch",
        expected: expected.value,
        actual,
        notes: `expected ${summarize(expected.value)}; actual ${summarize(actual)}`
      };
    } catch (error) {
      return {
        id,
        status: testCase.invalid ? "pass" : "eval_error",
        category: testCase.invalid ? "invalid_rejected" : "js_runtime_error",
        notes: testCase.invalid
          ? "invalid expression raised JavaScript evaluation error"
          : truncate(error?.stack ?? String(error))
      };
    }
  } catch (error) {
    return {
      id,
      status: "error",
      category: "js_harness_error",
      notes: truncate(error?.stack ?? String(error))
    };
  } finally {
    await rm(workDir, { recursive: true, force: true });
  }
}

function parseArgs(argv) {
  const parsed = {};
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (arg.startsWith("--")) {
      parsed[arg] = argv[i + 1];
      i += 1;
    }
  }
  return parsed;
}

function requiredArg(args, name) {
  if (!args[name]) {
    console.error(`missing required argument ${name}`);
    process.exit(2);
  }
  return args[name];
}

function runProcess(command, args) {
  return new Promise((resolve) => {
    const child = spawn(command, args, { stdio: ["ignore", "pipe", "pipe"] });
    let stdout = "";
    let stderr = "";
    child.stdout.setEncoding("utf8");
    child.stderr.setEncoding("utf8");
    child.stdout.on("data", (chunk) => {
      stdout += chunk;
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk;
    });
    child.on("close", (code) => resolve({ code, stdout, stderr }));
    child.on("error", (error) => resolve({ code: 127, stdout, stderr: String(error) }));
  });
}

function parseExpected(raw) {
  if (raw == null || raw.trim() === "") {
    return { kind: "unsupported", reason: "no expected output" };
  }
  const value = raw.trim();
  if (value === "null") return { kind: "value", value: null };
  if (value === "true") return { kind: "value", value: true };
  if (value === "false") return { kind: "value", value: false };
  if (/^-?\d+$/.test(value)) return { kind: "value", value: Number(value) };
  if (/^-?\d+\.\d+$/.test(value)) return { kind: "value", value: Number(value) };
  if (value.startsWith("'") && value.endsWith("'")) {
    return { kind: "value", value: decodeCqlString(value.slice(1, -1)) };
  }
  return { kind: "unsupported", reason: `unsupported expected output: ${truncate(value, 120)}` };
}

function decodeCqlString(value) {
  let decoded = "";
  for (let index = 0; index < value.length; index += 1) {
    const char = value[index];
    if (char !== "\\" || index + 1 >= value.length) {
      decoded += char;
      continue;
    }

    const escaped = value[index + 1];
    if (escaped === "u" && /^[0-9a-fA-F]{4}$/.test(value.slice(index + 2, index + 6))) {
      decoded += String.fromCharCode(Number.parseInt(value.slice(index + 2, index + 6), 16));
      index += 5;
      continue;
    }

    const escapes = { n: "\n", r: "\r", t: "\t", "\\": "\\", "'": "'" };
    decoded += escapes[escaped] ?? escaped;
    index += 1;
  }
  return decoded;
}

function valuesMatch(actual, expected) {
  if (actual === expected) return true;
  if (typeof actual === "number" && typeof expected === "number") {
    return Math.abs(actual - expected) < 1e-9;
  }
  return false;
}

function summarize(value) {
  return truncate(JSON.stringify(value));
}

function truncate(value, max = 240) {
  const text = String(value ?? "");
  return text.length <= max ? text : `${text.slice(0, max - 3)}...`;
}

function firstNonEmpty(...values) {
  return values.find((value) => value && value.trim()) ?? "";
}
