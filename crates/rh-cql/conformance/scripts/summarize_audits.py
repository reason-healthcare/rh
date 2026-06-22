#!/usr/bin/env python3
"""Generate a date-stamped summary of CQL audit and corpus outputs."""

from __future__ import annotations

import argparse
import datetime as dt
import json
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).parent
CONFORMANCE_DIR = SCRIPT_DIR.parent
DEFAULT_AUDIT_DIR = CONFORMANCE_DIR / "results/audit"
DEFAULT_CORPUS_DIR = CONFORMANCE_DIR / "results/corpus"
DEFAULT_OUT_DIR = CONFORMANCE_DIR / "results/summaries"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--audit-dir", type=Path, default=DEFAULT_AUDIT_DIR)
    parser.add_argument("--corpus-dir", type=Path, default=DEFAULT_CORPUS_DIR)
    parser.add_argument("--out-dir", type=Path, default=DEFAULT_OUT_DIR)
    parser.add_argument(
        "--date",
        help="Date stamp to use in output names. Defaults to local current date.",
    )
    args = parser.parse_args()

    date_stamp = args.date or dt.datetime.now().astimezone().strftime("%Y-%m-%d")
    snapshot_dir = args.out_dir / date_stamp
    snapshot_dir.mkdir(parents=True, exist_ok=True)

    audit_summary = read_json(args.audit_dir / "hl7_eval_summary.json")
    matrix_summary = read_json(args.audit_dir / "implementation_matrix_summary.json")
    corpus_summary = read_json(args.corpus_dir / "corpus_summary.json")

    summary_json = build_summary_json(
        date_stamp=date_stamp,
        audit_dir=args.audit_dir,
        corpus_dir=args.corpus_dir,
        audit_summary=audit_summary,
        matrix_summary=matrix_summary,
        corpus_summary=corpus_summary,
    )
    summary_md = build_summary_markdown(summary_json)

    json_path = snapshot_dir / "summary.json"
    md_path = snapshot_dir / "summary.md"
    latest_json_path = args.out_dir / "latest-summary.json"
    latest_md_path = args.out_dir / "latest-summary.md"

    json_text = json.dumps(summary_json, indent=2)
    json_path.write_text(json_text)
    latest_json_path.write_text(json_text)
    md_path.write_text(summary_md)
    latest_md_path.write_text(summary_md)

    print(f"Wrote audit summary: {md_path}")
    print(f"Wrote audit summary JSON: {json_path}")
    return 0


def read_json(path: Path) -> dict[str, Any]:
    if not path.exists():
        raise SystemExit(f"missing required input: {path}")
    return json.loads(path.read_text())


def build_summary_json(
    *,
    date_stamp: str,
    audit_dir: Path,
    corpus_dir: Path,
    audit_summary: dict[str, Any],
    matrix_summary: dict[str, Any],
    corpus_summary: dict[str, Any],
) -> dict[str, Any]:
    return {
        "date": date_stamp,
        "inputs": {
            "audit_dir": str(audit_dir),
            "corpus_dir": str(corpus_dir),
            "hl7_eval_summary": str(audit_dir / "hl7_eval_summary.json"),
            "implementation_matrix_summary": str(
                audit_dir / "implementation_matrix_summary.json"
            ),
            "corpus_summary": str(corpus_dir / "corpus_summary.json"),
        },
        "audit_full": {
            "hl7_eval": {
                "suite_count": audit_summary.get("suite_count", 0),
                "totals": audit_summary.get("totals", {}),
            },
            "implementation_matrix": {
                "row_count": matrix_summary.get("row_count", 0),
                "implementations": matrix_summary.get("implementations", {}),
            },
        },
        "corpus_audit": {
            "row_count": corpus_summary.get("row_count", 0),
            "sources": corpus_summary.get("sources", []),
            "by_corpus": corpus_summary.get("by_corpus", {}),
        },
    }


def build_summary_markdown(summary: dict[str, Any]) -> str:
    lines: list[str] = []
    lines.append(f"# CQL Audit Summary - {summary['date']}")
    lines.append("")
    lines.append("## Inputs")
    inputs = summary["inputs"]
    lines.append(f"- Audit directory: `{inputs['audit_dir']}`")
    lines.append(f"- Corpus directory: `{inputs['corpus_dir']}`")
    lines.append("")

    lines.append("## audit-full")
    hl7 = summary["audit_full"]["hl7_eval"]
    totals = hl7["totals"]
    lines.append(f"- HL7 suites: `{hl7['suite_count']}`")
    lines.append(f"- HL7 parsed cases: `{totals.get('total', 0)}`")
    lines.append(f"- HL7 pass: `{totals.get('pass', 0)}`")
    lines.append(f"- HL7 wrong-answer fail: `{totals.get('fail', 0)}`")
    lines.append(f"- HL7 skip: `{totals.get('skip', 0)}`")
    lines.append(f"- HL7 compile errors: `{totals.get('compile_err', 0)}`")
    lines.append(f"- HL7 eval errors: `{totals.get('eval_err', 0)}`")
    lines.append(f"- HL7 invalid pass: `{totals.get('invalid_pass', 0)}`")
    lines.append(f"- HL7 invalid fail: `{totals.get('invalid_fail', 0)}`")
    lines.append(f"- HL7 unimplemented: `{totals.get('unimplemented', 0)}`")
    lines.append("")

    lines.append("### Implementation Matrix")
    lines.append(
        "| Implementation | Rows | Pass | Compile Error | Eval Error | Fail | Skip | Unimplemented | Timeout | Not Run |"
    )
    lines.append("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|")
    matrix = summary["audit_full"]["implementation_matrix"]
    for implementation, counts in matrix["implementations"].items():
        lines.append(status_table_row(implementation, matrix["row_count"], counts))
    lines.append("")

    lines.append("## corpus-audit")
    corpus = summary["corpus_audit"]
    lines.append(f"- Corpus rows: `{corpus['row_count']}`")
    lines.append(
        f"- Java-pass/RH-fail rows: `{corpus.get('java_pass_rh_fail_count', 0)}`"
    )
    lines.append(f"- Java non-pass quarantine rows: `{corpus.get('java_non_pass_count', 0)}`")
    lines.append("")
    lines.append(
        "| Corpus | Files | RH Pass | RH Compile Error | RH Timeout | Java Pass | Java Compile Error | Java Timeout | Java Not Run |"
    )
    lines.append("|---|---:|---:|---:|---:|---:|---:|---:|---:|")
    for corpus_name, corpus_counts in corpus["by_corpus"].items():
        rh_counts = corpus_counts.get("rh_cql_status", {})
        java_counts = corpus_counts.get("java_elm_status", {})
        lines.append(
            "| {name} | {rows} | {rh_pass} | {rh_compile} | {rh_timeout} | {java_pass} | {java_compile} | {java_timeout} | {java_not_run} |".format(
                name=corpus_name,
                rows=corpus_counts.get("row_count", 0),
                rh_pass=rh_counts.get("pass", 0),
                rh_compile=rh_counts.get("compile_error", 0),
                rh_timeout=rh_counts.get("timeout", 0),
                java_pass=java_counts.get("pass", 0),
                java_compile=java_counts.get("compile_error", 0),
                java_timeout=java_counts.get("timeout", 0),
                java_not_run=java_counts.get("not_run", 0),
            )
        )
    lines.append("")

    diagnostic_counts = corpus.get("java_pass_rh_fail_by_diagnostic_class", {})
    if diagnostic_counts:
        lines.append("### Java-Pass/RH-Fail Diagnostics")
        lines.append("| Diagnostic Class | Count |")
        lines.append("|---|---:|")
        for diagnostic_class, count in sorted(diagnostic_counts.items()):
            lines.append(f"| `{diagnostic_class}` | {count} |")
        lines.append("")

    java_non_pass_counts = corpus.get("java_non_pass_by_diagnostic_class", {})
    if java_non_pass_counts:
        lines.append("### Java Non-Pass Quarantine Diagnostics")
        lines.append("| Diagnostic Class | Count |")
        lines.append("|---|---:|")
        for diagnostic_class, count in sorted(java_non_pass_counts.items()):
            lines.append(f"| `{diagnostic_class}` | {count} |")
        lines.append("")

    unavailable = [
        source["corpus"]
        for source in corpus.get("sources", [])
        if not source.get("available", False)
    ]
    if unavailable:
        lines.append("## Unavailable Corpus Sources")
        for corpus_name in unavailable:
            lines.append(f"- `{corpus_name}`")
        lines.append("")

    return "\n".join(lines)


def status_table_row(name: str, rows: int, counts: dict[str, int]) -> str:
    return (
        f"| `{name}` | {rows} | {counts.get('pass', 0)} | "
        f"{counts.get('compile_error', 0)} | {counts.get('eval_error', 0)} | "
        f"{counts.get('fail', 0)} | {counts.get('skip', 0)} | "
        f"{counts.get('unimplemented', 0)} | {counts.get('timeout', 0)} | "
        f"{counts.get('not_run', 0)} |"
    )


if __name__ == "__main__":
    raise SystemExit(main())
