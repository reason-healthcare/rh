#!/usr/bin/env python3
"""Populate Java and JavaScript columns in the HL7 implementation matrix."""

from __future__ import annotations

import argparse
import csv
import html
import json
import re
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path


SCRIPT_DIR = Path(__file__).parent
CONFORMANCE_DIR = SCRIPT_DIR.parent
CRATE_DIR = CONFORMANCE_DIR.parent
WORKSPACE_ROOT = CRATE_DIR.parent.parent
FIXTURES_DIR = CRATE_DIR / "tests/fixtures/hl7_cql_tests"
AUDIT_DIR = CONFORMANCE_DIR / "results/audit"
JAVA_CLI = CONFORMANCE_DIR / "tools/cql-java/Src/java/cql-to-elm-cli/build/install/cql-to-elm-cli/bin/cql-to-elm-cli"
JS_DIR = CONFORMANCE_DIR / "javascript"
JS_RUNNER = SCRIPT_DIR / "evaluate_with_cql_execution.mjs"
DEFAULT_RH_CLI = WORKSPACE_ROOT / "target/release/rh"


@dataclass(frozen=True)
class Hl7Case:
    suite: str
    group: str
    test: str
    expression: str
    expected_output: str | None
    invalid: bool

    @property
    def id(self) -> str:
        return row_id(self.suite, self.group, self.test)

    def json(self) -> dict:
        return {
            "id": self.id,
            "suite": self.suite,
            "group": self.group,
            "test": self.test,
            "expression": self.expression,
            "expected_output": self.expected_output,
            "invalid": self.invalid,
        }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--audit-dir", type=Path, default=AUDIT_DIR)
    parser.add_argument("--fixtures-dir", type=Path, default=FIXTURES_DIR)
    parser.add_argument("--rh-cli", type=Path, default=DEFAULT_RH_CLI)
    parser.add_argument("--limit", type=int, help="limit cases for local smoke tests")
    parser.add_argument("--skip-java", action="store_true")
    parser.add_argument("--skip-javascript", action="store_true")
    parser.add_argument("--setup-javascript", action="store_true")
    parser.add_argument("--require-java", action="store_true")
    parser.add_argument("--require-javascript", action="store_true")
    parser.add_argument("--java-timeout-seconds", type=int, default=10)
    args = parser.parse_args()

    matrix_csv = args.audit_dir / "implementation_matrix.csv"
    matrix_json = args.audit_dir / "implementation_matrix.json"
    if not matrix_csv.exists():
        print(f"missing matrix CSV: {matrix_csv}", file=sys.stderr)
        return 2

    cases = load_cases(args.fixtures_dir)
    rows = read_matrix(matrix_csv)
    selected_rows = rows[: args.limit] if args.limit else rows
    rows_by_id = {row_id(row["suite"], row["group"], row["test"]): row for row in rows}
    selected_ids = {row_id(row["suite"], row["group"], row["test"]) for row in selected_rows}
    selected_cases = [case for case in cases if case.id in selected_ids]

    if not args.skip_java:
        java_results = run_java(
            selected_cases,
            require=args.require_java,
            timeout_seconds=args.java_timeout_seconds,
        )
        apply_results(rows_by_id, java_results, "java_elm_status", "java_elm_notes")

    if not args.skip_javascript:
        if args.setup_javascript:
            setup_javascript()
        js_results = run_javascript(selected_cases, args.rh_cli, require=args.require_javascript)
        apply_results(
            rows_by_id,
            js_results,
            "javascript_eval_status",
            "javascript_eval_notes",
        )

    write_matrix_csv(matrix_csv, rows)
    write_matrix_json(matrix_json, rows)
    write_summary(args.audit_dir, rows)
    print(f"Updated reference implementation matrix: {matrix_csv}")
    return 0


def load_cases(fixtures_dir: Path) -> list[Hl7Case]:
    cases: list[Hl7Case] = []
    for xml_path in sorted(fixtures_dir.glob("*.xml")):
        cases.extend(scan_hl7_xml(xml_path.name, xml_path.read_text()))
    return cases


def scan_hl7_xml(suite: str, xml: str) -> list[Hl7Case]:
    """Scan HL7 test XML.

    The checked-in HL7 corpus has at least one unmatched closing group tag.
    A forgiving scanner matches the Rust streaming parser's behavior better
    than Python's strict ElementTree parser.
    """
    cases: list[Hl7Case] = []
    group_name = ""
    test_name = ""
    expression = ""
    output: str | None = None
    invalid = False
    in_test = False
    capture: str | None = None

    for token in re.finditer(r"<!--.*?-->|<!\[CDATA\[.*?\]\]>|<[^>]+>|[^<]+", xml, flags=re.S):
        text = token.group(0)
        if text.startswith("<!--"):
            continue
        if text.startswith("<![CDATA["):
            body = text.removeprefix("<![CDATA[").removesuffix("]]>")
            if capture == "expression":
                expression += body
            elif capture == "output" and output is not None:
                output += body
            continue
        if text.startswith("<"):
            if text.startswith("</"):
                tag = text[2:-1].strip()
                if tag == "expression":
                    capture = None
                elif tag == "output":
                    capture = None
                elif tag == "test" and in_test:
                    in_test = False
                    cases.append(
                        Hl7Case(
                            suite=suite,
                            group=group_name,
                            test=test_name,
                            expression=expression.strip(),
                            expected_output=output.strip() if output is not None else None,
                            invalid=invalid,
                        )
                    )
                continue

            tag, attrs = parse_start_tag(text)
            if tag == "group":
                group_name = attrs.get("name", "")
            elif tag == "test":
                in_test = True
                test_name = attrs.get("name", "")
                expression = ""
                output = None
                invalid = False
                capture = None
            elif tag == "expression" and in_test:
                capture = "expression"
                invalid = attrs.get("invalid") == "true"
            elif tag == "output" and in_test:
                capture = "output"
                output = ""
            continue

        body = html.unescape(text)
        if capture == "expression":
            expression += body
        elif capture == "output" and output is not None:
            output += body

    return cases


def parse_start_tag(raw: str) -> tuple[str, dict[str, str]]:
    body = raw.strip("<>/ \t\r\n")
    parts = body.split(None, 1)
    tag = parts[0] if parts else ""
    attrs: dict[str, str] = {}
    if len(parts) > 1:
        for match in re.finditer(r"([A-Za-z_:][-A-Za-z0-9_:.]*)\s*=\s*(['\"])(.*?)\2", parts[1], flags=re.S):
            attrs[match.group(1)] = html.unescape(match.group(3))
    return tag, attrs


def read_matrix(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle))


def write_matrix_csv(path: Path, rows: list[dict[str, str]]) -> None:
    fieldnames = [
        "suite",
        "group",
        "test",
        "rh_cql_status",
        "rh_cql_notes",
        "java_elm_status",
        "java_elm_notes",
        "javascript_eval_status",
        "javascript_eval_notes",
    ]
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def write_matrix_json(path: Path, rows: list[dict[str, str]]) -> None:
    payload = {
        "implementations": [
            {
                "id": "rh_cql",
                "label": "rh-cql evaluator",
                "notes": "CQL is compiled by rh-cql and evaluated by the rh-cql ELM runtime.",
            },
            {
                "id": "java_elm",
                "label": "Java CQL-to-ELM translator",
                "notes": "CQL is compiled by the pinned CQFramework Java translator.",
            },
            {
                "id": "javascript_eval",
                "label": "JavaScript cql-execution",
                "notes": "rh-cql ELM is evaluated by cql-execution.",
            },
        ],
        "rows": [
            {
                "suite": row["suite"],
                "group": row["group"],
                "test": row["test"],
                "implementations": {
                    "rh_cql": {
                        "status": row["rh_cql_status"],
                        "notes": row["rh_cql_notes"],
                    },
                    "java_elm": {
                        "status": row["java_elm_status"],
                        "notes": row["java_elm_notes"],
                    },
                    "javascript_eval": {
                        "status": row["javascript_eval_status"],
                        "notes": row["javascript_eval_notes"],
                    },
                },
            }
            for row in rows
        ],
    }
    path.write_text(json.dumps(payload, indent=2))


def write_summary(audit_dir: Path, rows: list[dict[str, str]]) -> None:
    summary = {
        "row_count": len(rows),
        "implementations": {
            key: count_statuses(rows, key)
            for key in ("rh_cql_status", "java_elm_status", "javascript_eval_status")
        },
    }
    (audit_dir / "implementation_matrix_summary.json").write_text(json.dumps(summary, indent=2))


def count_statuses(rows: list[dict[str, str]], key: str) -> dict[str, int]:
    counts: dict[str, int] = {}
    for row in rows:
        status = row.get(key, "")
        counts[status] = counts.get(status, 0) + 1
    return dict(sorted(counts.items()))


def run_java(
    cases: list[Hl7Case],
    require: bool,
    timeout_seconds: int,
) -> dict[str, dict[str, str]]:
    if not JAVA_CLI.exists():
        if require:
            raise SystemExit(f"Java translator CLI not found: {JAVA_CLI}")
        return {
            case.id: {
                "status": "not_run",
                "notes": "Java translator CLI not found; run conformance/scripts/setup.sh",
            }
            for case in cases
        }

    results: dict[str, dict[str, str]] = {}
    with tempfile.TemporaryDirectory(prefix="rh-cql-java-") as tmp:
        tmp_dir = Path(tmp)
        for index, case in enumerate(cases, start=1):
            if index == 1 or index % 100 == 0 or index == len(cases):
                print(f"Java translator: {index}/{len(cases)}", flush=True)
            cql_path = tmp_dir / "HlTest.cql"
            out_dir = tmp_dir / "out"
            out_dir.mkdir(exist_ok=True)
            java_output = out_dir / f"{cql_path.stem}.json"
            java_output.unlink(missing_ok=True)
            cql_path.write_text(f"library HlTest\ndefine Result: {case.expression}\n")
            try:
                proc = subprocess.run(
                    [
                        str(JAVA_CLI),
                        "--input",
                        str(cql_path),
                        "--format",
                        "JSON",
                        "--output",
                        str(out_dir),
                    ],
                    capture_output=True,
                    text=True,
                    timeout=timeout_seconds,
                )
            except subprocess.TimeoutExpired:
                results[case.id] = {
                    "status": "timeout",
                    "notes": f"Java translator exceeded {timeout_seconds}s timeout",
                }
                continue
            if proc.returncode == 0 and java_output.exists():
                results[case.id] = {
                    "status": "pass",
                    "notes": "Java translator produced ELM"
                    if not case.invalid
                    else "Java translator produced ELM for invalid test; evaluation may still reject it",
                }
            elif case.invalid:
                results[case.id] = {
                    "status": "pass",
                    "notes": "invalid expression rejected by Java translator",
                }
            else:
                results[case.id] = {
                    "status": "compile_error",
                    "notes": truncate(first_non_empty(proc.stderr, proc.stdout, "Java translator failed")),
                }
    return results


def setup_javascript() -> None:
    subprocess.run(["npm", "install", "--prefix", str(JS_DIR)], check=True)


def run_javascript(cases: list[Hl7Case], rh_cli: Path, require: bool) -> dict[str, dict[str, str]]:
    rh_cli = rh_cli.resolve()
    node_modules = JS_DIR / "node_modules/cql-execution"
    if not node_modules.exists():
        if require:
            raise SystemExit(f"JavaScript dependencies not installed: {node_modules}")
        return {
            case.id: {
                "status": "not_run",
                "notes": "JavaScript dependencies not installed; run with --setup-javascript",
            }
            for case in cases
        }
    if not rh_cli.exists():
        if require:
            raise SystemExit(f"rh CLI not found: {rh_cli}")
        return {
            case.id: {
                "status": "not_run",
                "notes": f"rh CLI not found: {rh_cli}",
            }
            for case in cases
        }

    with tempfile.TemporaryDirectory(prefix="rh-cql-js-cases-") as tmp:
        print(f"JavaScript cql-execution: 1/{len(cases)}", flush=True)
        tmp_dir = Path(tmp)
        cases_path = tmp_dir / "cases.json"
        output_path = tmp_dir / "results.json"
        cases_path.write_text(json.dumps([case.json() for case in cases]))
        proc = subprocess.run(
            [
                "node",
                str(JS_RUNNER),
                "--cases",
                str(cases_path),
                "--output",
                str(output_path),
                "--rh-cli",
                str(rh_cli),
            ],
            cwd=JS_DIR,
            capture_output=True,
            text=True,
        )
        if proc.returncode != 0:
            if require:
                raise SystemExit(first_non_empty(proc.stderr, proc.stdout, "JavaScript runner failed"))
            return {
                case.id: {
                    "status": "not_run",
                    "notes": truncate(first_non_empty(proc.stderr, proc.stdout, "JavaScript runner failed")),
                }
                for case in cases
            }
        payload = json.loads(output_path.read_text())
    print(f"JavaScript cql-execution: {len(cases)}/{len(cases)}", flush=True)
    return {
        result["id"]: {
            "status": result["status"],
            "notes": result.get("notes", ""),
        }
        for result in payload["results"]
    }


def apply_results(
    rows_by_id: dict[str, dict[str, str]],
    results: dict[str, dict[str, str]],
    status_key: str,
    notes_key: str,
) -> None:
    for id_, result in results.items():
        row = rows_by_id.get(id_)
        if not row:
            continue
        row[status_key] = result["status"]
        row[notes_key] = result.get("notes", "")


def row_id(suite: str, group: str, test: str) -> str:
    return f"{suite}::{group}::{test}"


def first_non_empty(*values: str) -> str:
    return next((value.strip() for value in values if value and value.strip()), "")


def truncate(value: str, max_len: int = 240) -> str:
    return value if len(value) <= max_len else value[: max_len - 3] + "..."


if __name__ == "__main__":
    raise SystemExit(main())
