#!/usr/bin/env python3
"""Run the HL7 FHIRPath R5 XML testlab suite through the rh CLI.

This is intentionally a dev-only harness, not part of the rh CLI. It parses
tests-fhir-r5.xml, invokes `rh fhirpath eval` for each selected test, and writes
a readable report showing the input fixture, expression, expectation, engine
result, and classification.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import xml.etree.ElementTree as ET
from dataclasses import dataclass, field
from decimal import Decimal, InvalidOperation
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[3]
DEFAULT_SUITE = Path(__file__).resolve().with_name("tests-fhir-r5.xml")
DEFAULT_BASELINE = Path(__file__).resolve().with_name("Rust-0.0.1.json")
DEFAULT_INPUT_DIRS = [
    Path(__file__).resolve().with_name("input"),
    REPO_ROOT / "crates/rh-fhirpath/tests/fixtures/hl7_fhirpath_tests/input",
]
DEFAULT_REPORT = REPO_ROOT / "target/fhirpath-r5-testlab-report.md"


@dataclass
class ExpectedOutput:
    type: str
    value: str


@dataclass
class TestCase:
    group: str
    name: str
    testing: str
    description: str
    inputfile: str
    expression: str
    invalid: str | None
    predicate: bool
    outputs: list[ExpectedOutput] = field(default_factory=list)


@dataclass
class CaseResult:
    group: str
    name: str
    testing: str
    description: str
    inputfile: str
    expression: str
    expectation: str
    actual: str
    detail: str
    outcome: str
    rationale: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--suite", type=Path, default=DEFAULT_SUITE)
    parser.add_argument(
        "--input-dir",
        dest="input_dirs",
        type=Path,
        action="append",
        default=[],
        help="Directory containing JSON input fixtures. Repeatable.",
    )
    parser.add_argument(
        "-n",
        "--name",
        action="append",
        default=[],
        help="Test name to run. Repeat or use comma-separated names.",
    )
    parser.add_argument("--rh", type=Path, help="Path to rh binary.")
    parser.add_argument(
        "--no-build",
        action="store_true",
        help="Do not build rh if no binary is found.",
    )
    parser.add_argument("--report", type=Path, default=DEFAULT_REPORT)
    parser.add_argument(
        "--report-format",
        choices=["markdown", "json", "text"],
        default="markdown",
    )
    parser.add_argument(
        "--baseline-report",
        type=Path,
        default=DEFAULT_BASELINE,
        help="Optional baseline JSON for aggregate non-pass comparison.",
    )
    parser.add_argument("--no-baseline-check", action="store_true")
    parser.add_argument(
        "--fail-on-baseline-mismatch",
        action="store_true",
        help="Exit 3 if current non-pass count differs from the baseline.",
    )
    parser.add_argument(
        "--fail-on-fail",
        action="store_true",
        help="Exit 3 if any selected test has outcome fail.",
    )
    return parser.parse_args()


def local_name(tag: str) -> str:
    return tag.rsplit("}", 1)[-1]


def parse_suite(path: Path) -> list[TestCase]:
    root = ET.parse(path).getroot()
    cases: list[TestCase] = []
    for group in root:
        if local_name(group.tag) != "group":
            continue
        group_name = group.attrib.get("name", "")
        for test in group:
            if local_name(test.tag) != "test":
                continue
            expression = ""
            invalid = test.attrib.get("invalid")
            outputs: list[ExpectedOutput] = []
            for child in test:
                child_name = local_name(child.tag)
                if child_name == "expression":
                    expression = child.text or ""
                    invalid = invalid or child.attrib.get("invalid")
                elif child_name == "output":
                    outputs.append(
                        ExpectedOutput(
                            type=child.attrib.get("type", ""),
                            value=child.text or "",
                        )
                    )
            cases.append(
                TestCase(
                    group=group_name,
                    name=test.attrib.get("name", ""),
                    testing=test.attrib.get("testing", ""),
                    description=test.attrib.get("description", ""),
                    inputfile=test.attrib.get("inputfile", ""),
                    expression=expression,
                    invalid=invalid,
                    predicate=test.attrib.get("predicate") == "true",
                    outputs=outputs,
                )
            )
    return cases


def selected_names(raw_names: list[str]) -> set[str]:
    names: set[str] = set()
    for raw in raw_names:
        for name in raw.split(","):
            if name.strip():
                names.add(name.strip())
    return names


def find_or_build_rh(explicit: Path | None, no_build: bool) -> Path:
    if explicit:
        return explicit

    candidate = REPO_ROOT / "target/debug/rh"
    if candidate.exists():
        return candidate

    if no_build:
        raise SystemExit(f"rh binary not found at {candidate}; pass --rh or omit --no-build")

    subprocess.run(["cargo", "build", "-p", "rh-cli"], cwd=REPO_ROOT, check=True)
    return candidate


def load_inputs(input_dirs: list[Path]) -> dict[str, Path]:
    dirs = input_dirs or [path for path in DEFAULT_INPUT_DIRS if path.is_dir()]
    inputs: dict[str, Path] = {}
    for directory in dirs:
        if not directory.is_dir():
            continue
        for path in directory.glob("*.json"):
            inputs[path.stem] = path
    return inputs


def input_path_for(case: TestCase, inputs: dict[str, Path]) -> Path | None:
    if not case.inputfile:
        return None
    stem = case.inputfile.removesuffix(".xml").removesuffix(".json")
    return inputs.get(stem)


def run_case(case: TestCase, rh: Path, inputs: dict[str, Path]) -> CaseResult:
    expectation = describe_expectation(case)
    data_path = input_path_for(case, inputs)
    if case.inputfile and data_path is None:
        return result(
            case,
            expectation,
            "",
            f"missing input fixture {case.inputfile}",
            "skipped",
            (
                f"Skipped because `{case.inputfile}` is referenced by the XML suite, "
                "but no matching JSON fixture was found in the configured input directories."
            ),
        )

    cmd = [
        str(rh),
        "--format",
        "json",
        "--quiet",
        "fhirpath",
        "eval",
        "--display-format",
        "json",
    ]
    if data_path:
        cmd.extend(["--data", str(data_path)])
    cmd.extend(["--", case.expression])

    completed = subprocess.run(
        cmd,
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    parsed = parse_rh_stdout(completed.stdout)
    ok = completed.returncode == 0 and parsed.get("ok") is True

    if case.invalid:
        if ok:
            return result(
                case,
                expectation,
                describe_actual(parsed.get("result")),
                "expected exception but expression evaluated successfully",
                "fail",
                (
                    "The XML test marks this expression as invalid; per harness rules, "
                    "any successful engine evaluation is a failure and declared outputs are ignored."
                ),
            )
        return result(
            case,
            expectation,
            error_text(parsed, completed),
            "expected exception",
            "pass",
            "The XML test marks this expression as invalid and the engine returned an error as expected.",
        )

    if not ok:
        detail = error_text(parsed, completed)
        outcome = "not_implemented" if is_not_implemented(detail) else "fail"
        return result(case, expectation, detail, detail, outcome, rationale_for_outcome(outcome, detail))

    actual = parsed.get("result")
    detail = compare_outputs(case, flatten_actual(actual))
    outcome = "pass" if not detail else "fail"
    return result(case, expectation, describe_actual(actual), detail, outcome, rationale_for_outcome(outcome, detail))


def parse_rh_stdout(stdout: str) -> dict[str, Any]:
    try:
        return json.loads(stdout)
    except json.JSONDecodeError:
        return {}


def error_text(parsed: dict[str, Any], completed: subprocess.CompletedProcess[str]) -> str:
    errors = parsed.get("errors")
    if isinstance(errors, list) and errors:
        message = errors[0].get("message")
        if isinstance(message, str):
            return message
    return (completed.stderr or completed.stdout or f"exit {completed.returncode}").strip()


def result(
    case: TestCase,
    expectation: str,
    actual: str,
    detail: str,
    outcome: str,
    rationale: str,
) -> CaseResult:
    return CaseResult(
        group=case.group,
        name=case.name,
        testing=case.testing,
        description=case.description,
        inputfile=case.inputfile,
        expression=case.expression,
        expectation=expectation,
        actual=actual,
        detail=detail,
        outcome=outcome,
        rationale=rationale,
    )


def rationale_for_outcome(outcome: str, detail: str) -> str:
    if outcome == "pass":
        return "Engine result matched the XML test expectation."
    if outcome == "not_implemented":
        return (
            "Categorized as not implemented because the engine error matched the harness "
            "not-implemented classifier: not implemented, unsupported, unknown function, "
            "unknown invocation, or unexpected characters."
        )
    if outcome == "skipped":
        return detail
    return "Engine result did not match the XML test expectation."


def describe_expectation(case: TestCase) -> str:
    if case.invalid:
        return f"exception ({case.invalid}); outputs ignored"
    if case.predicate:
        expected = case.outputs[0].value if case.outputs else "false"
        return f"predicate {expected}"
    if not case.outputs:
        return "empty"
    return ", ".join(f"{out.type} {out.value}" for out in case.outputs)


def flatten_actual(value: Any) -> list[Any]:
    if value is None:
        return []
    if isinstance(value, list):
        return value
    return [value]


def compare_outputs(case: TestCase, actual: list[Any]) -> str:
    if case.predicate:
        expected = case.outputs[0].value == "true" if case.outputs else False
        got = bool(actual)
        return "" if got == expected else f"predicate expected {expected}, got {got}"

    if len(actual) != len(case.outputs):
        return f"expected {len(case.outputs)} value(s), got {len(actual)} value(s)"

    for got, expected in zip(actual, case.outputs):
        if not matches_expected(got, expected):
            return f"expected {expected.type} {expected.value}, got {describe_actual(got)}"
    return ""


def matches_expected(actual: Any, expected: ExpectedOutput) -> bool:
    typ = expected.type or infer_type(expected.value)
    if typ == "boolean":
        return actual is (expected.value == "true")
    if typ in {"integer", "decimal"}:
        try:
            return Decimal(str(actual)) == Decimal(expected.value)
        except (InvalidOperation, TypeError, ValueError):
            return False
    if typ in {"string", "code", "id", "uri"}:
        return actual == expected.value
    if typ in {"date", "dateTime", "time"}:
        return normalize_temporal(str(actual)) == normalize_temporal(expected.value)
    if typ == "Quantity":
        if not isinstance(actual, dict):
            return False
        number, unit = parse_quantity(expected.value)
        try:
            numbers_match = Decimal(str(actual.get("value"))) == Decimal(number)
        except (InvalidOperation, TypeError, ValueError):
            return False
        actual_unit = actual.get("unit")
        return numbers_match and (actual_unit == unit or {actual_unit, unit} <= {None, "1"})
    return False


def infer_type(value: str) -> str:
    if value in {"true", "false"}:
        return "boolean"
    if value.startswith("@T"):
        return "time"
    if value.startswith("@") and "T" in value:
        return "dateTime"
    if value.startswith("@"):
        return "date"
    if "'" in value:
        return "Quantity"
    try:
        Decimal(value)
        return "decimal"
    except InvalidOperation:
        return "string"


def normalize_temporal(value: str) -> str:
    return value.removeprefix("@").removeprefix("T")


def parse_quantity(value: str) -> tuple[str, str | None]:
    if " " not in value:
        return value, None
    number, unit = value.split(" ", 1)
    return number, unit.strip().strip("'")


def describe_actual(value: Any) -> str:
    return json.dumps(value, ensure_ascii=False, sort_keys=True)


def is_not_implemented(message: str) -> bool:
    lowered = message.lower()
    return any(
        marker in lowered
        for marker in (
            "not implemented",
            "unsupported",
            "unknown function",
            "unknown invocation",
            "unexpected characters",
        )
    )


def baseline_counts(path: Path) -> dict[str, int] | None:
    if not path.exists():
        return None
    data = json.loads(path.read_text())
    counts = {"fail": 0, "not_implemented": 0}

    def visit(value: Any) -> None:
        if isinstance(value, dict):
            if value.get("Result") is False:
                counts["fail"] += 1
            if value.get("NotImplemented") is True:
                counts["not_implemented"] += 1
            for child in value.values():
                visit(child)
        elif isinstance(value, list):
            for item in value:
                visit(item)

    visit(data)
    counts["non_pass"] = counts["fail"] + counts["not_implemented"]
    return counts


def summarize(results: list[CaseResult], baseline: dict[str, int] | None, baseline_path: Path) -> dict[str, Any]:
    counts = {
        "pass": sum(1 for item in results if item.outcome == "pass"),
        "fail": sum(1 for item in results if item.outcome == "fail"),
        "not_implemented": sum(1 for item in results if item.outcome == "not_implemented"),
        "skipped": sum(1 for item in results if item.outcome == "skipped"),
    }
    summary: dict[str, Any] = {
        "total": len(results),
        **counts,
        "current_failures": counts["fail"] + counts["not_implemented"],
    }
    if baseline:
        summary["baseline"] = {
            "report": str(baseline_path),
            **baseline,
            "matches": baseline["non_pass"] == summary["current_failures"],
        }
    return summary


def render_report(results: list[CaseResult], summary: dict[str, Any], suite: Path, fmt: str) -> str:
    payload = {
        "suite": str(suite),
        "summary": summary,
        "results": [item.__dict__ for item in results],
    }
    if fmt == "json":
        return json.dumps(payload, indent=2, ensure_ascii=False)
    if fmt == "text":
        lines = [
            "FHIRPath R5 Testlab Report",
            f"Suite: {suite}",
            f"Total: {summary['total']} | Pass: {summary['pass']} | Fail: {summary['fail']} | "
            f"Not implemented: {summary['not_implemented']} | Skipped: {summary['skipped']}",
        ]
        if "baseline" in summary:
            baseline = summary["baseline"]
            lines.append(
                f"Baseline failures: {baseline['non_pass']} | Current failures: "
                f"{summary['current_failures']} | Match: {baseline['matches']}"
            )
        lines.append("")
        rationale_counts = grouped_rationales(results)
        if rationale_counts:
            lines.append("Outcome rationales:")
            for rationale, count in rationale_counts.items():
                lines.append(f"  {count}x {rationale}")
            lines.append("")
        for item in results:
            lines.extend(
                [
                    f"[{item.outcome}] {item.group}::{item.name}",
                    f"  Input: {item.inputfile}",
                    f"  Expression: {item.expression}",
                    f"  Expectation: {item.expectation}",
                    f"  Actual: {item.actual}",
                    f"  Detail: {item.detail}",
                    f"  Rationale: {item.rationale}",
                    "",
                ]
            )
        return "\n".join(lines)
    return render_markdown(payload)


def render_markdown(payload: dict[str, Any]) -> str:
    summary = payload["summary"]
    lines = [
        "# FHIRPath R5 Testlab Report",
        "",
        f"- Suite: `{payload['suite']}`",
        f"- Total selected: {summary['total']}",
        f"- Pass: {summary['pass']}",
        f"- Fail: {summary['fail']}",
        f"- Not implemented: {summary['not_implemented']}",
        f"- Skipped: {summary['skipped']}",
        "",
    ]
    if "baseline" in summary:
        baseline = summary["baseline"]
        lines.extend(
            [
                "## Baseline Comparison",
                "",
                f"- Baseline report: `{baseline['report']}`",
                f"- Baseline failures: {baseline['fail']}",
                f"- Baseline not implemented: {baseline['not_implemented']}",
                f"- Baseline failures: {baseline['non_pass']}",
                f"- Current failures: {summary['current_failures']}",
                f"- Match: {'yes' if baseline['matches'] else 'no'}",
                "",
            ]
        )

    by_group: dict[str, dict[str, int]] = {}
    for item in payload["results"]:
        group_counts = by_group.setdefault(item["group"], {})
        group_counts[item["outcome"]] = group_counts.get(item["outcome"], 0) + 1
    lines.extend(
        [
            "## Group Summary",
            "",
            "| Group | Pass | Fail | Not implemented | Skipped |",
            "|---|---:|---:|---:|---:|",
        ]
    )
    for group, counts in sorted(by_group.items()):
        lines.append(
            f"| {md(group)} | {counts.get('pass', 0)} | {counts.get('fail', 0)} | "
            f"{counts.get('not_implemented', 0)} | {counts.get('skipped', 0)} |"
        )

    rationale_counts = grouped_rationales_from_payload(payload["results"])
    if rationale_counts:
        lines.extend(
            [
                "",
                "## Outcome Rationales",
                "",
                "| Rationale | Count |",
                "|---|---:|",
            ]
        )
        for rationale, count in rationale_counts.items():
            lines.append(f"| {md(rationale)} | {count} |")

    lines.extend(["", "## Results", ""])
    for item in payload["results"]:
        lines.extend(
            [
                f"### {md(item['group'])}::{md(item['name'])} - {item['outcome']}",
                "",
                f"- Input: `{md(item['inputfile'])}`",
                f"- Expectation: {md(item['expectation'])}",
                f"- Detail: {md(item['detail'])}",
                f"- Rationale: {md(item['rationale'])}",
                "",
                "Expression:",
                "",
                "```fhirpath",
                item["expression"],
                "```",
                "",
                "Actual:",
                "",
                "```text",
                item["actual"],
                "```",
                "",
            ]
        )
    return "\n".join(lines)


def grouped_rationales(results: list[CaseResult]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for item in results:
        if item.outcome in {"skipped", "not_implemented"}:
            counts[item.rationale] = counts.get(item.rationale, 0) + 1
    return dict(sorted(counts.items(), key=lambda entry: (-entry[1], entry[0])))


def grouped_rationales_from_payload(results: list[dict[str, Any]]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for item in results:
        if item["outcome"] in {"skipped", "not_implemented"}:
            rationale = item["rationale"]
            counts[rationale] = counts.get(rationale, 0) + 1
    return dict(sorted(counts.items(), key=lambda entry: (-entry[1], entry[0])))


def md(value: str) -> str:
    return value.replace("|", "\\|")


def main() -> int:
    args = parse_args()
    cases = parse_suite(args.suite)
    names = selected_names(args.name)
    if names:
        cases = [case for case in cases if case.name in names]
        found = {case.name for case in cases}
        missing = sorted(names - found)
        if missing:
            raise SystemExit(f"test name(s) not found: {', '.join(missing)}")

    rh = find_or_build_rh(args.rh, args.no_build)
    inputs = load_inputs(args.input_dirs)
    results = [run_case(case, rh, inputs) for case in cases]
    baseline = None if args.no_baseline_check or names else baseline_counts(args.baseline_report)
    summary = summarize(results, baseline, args.baseline_report)

    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(render_report(results, summary, args.suite, args.report_format))

    print(
        f"FHIRPath R5 testlab: {summary['total']} selected, {summary['pass']} pass, "
        f"{summary['fail']} fail, {summary['not_implemented']} not implemented, "
        f"{summary['skipped']} skipped"
    )
    if "baseline" in summary:
        baseline_summary = summary["baseline"]
        print(
            f"Baseline failures: {baseline_summary['non_pass']} current failures: "
            f"{summary['current_failures']} "
            f"({'match' if baseline_summary['matches'] else 'mismatch'})"
        )
    print(f"Report: {args.report}")

    if args.fail_on_baseline_mismatch and summary.get("baseline", {}).get("matches") is False:
        return 3
    if args.fail_on_fail and summary["fail"] > 0:
        return 3
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
