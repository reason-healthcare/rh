#!/usr/bin/env python3
"""Run expanded CQL corpus compile/translation checks."""

from __future__ import annotations

import argparse
import csv
import json
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).parent
CONFORMANCE_DIR = SCRIPT_DIR.parent
CRATE_DIR = CONFORMANCE_DIR.parent
WORKSPACE_ROOT = CRATE_DIR.parent.parent
TOOLS_DIR = CONFORMANCE_DIR / "tools"
GENERATED_DIR = CONFORMANCE_DIR / "corpus/generated"
RESULTS_DIR = CONFORMANCE_DIR / "results/corpus"
DEFAULT_RH_CLI = WORKSPACE_ROOT / "target/release/rh"
JAVA_CLI = TOOLS_DIR / "cql-java/Src/java/cql-to-elm-cli/build/install/cql-to-elm-cli/bin/cql-to-elm-cli"


@dataclass(frozen=True)
class CorpusSource:
    corpus: str
    root: Path
    pattern: str = "**/*.cql"


@dataclass(frozen=True)
class CorpusFile:
    corpus: str
    path: Path
    relative_path: str


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--results-dir", type=Path, default=RESULTS_DIR)
    parser.add_argument("--rh-cli", type=Path, default=DEFAULT_RH_CLI)
    parser.add_argument("--limit", type=int, help="limit files per corpus for smoke tests")
    parser.add_argument("--corpus", action="append", help="run only named corpus; repeatable")
    parser.add_argument("--skip-java", action="store_true")
    parser.add_argument("--require-java", action="store_true")
    parser.add_argument("--rh-timeout-seconds", type=int, default=10)
    parser.add_argument("--java-timeout-seconds", type=int, default=10)
    args = parser.parse_args()

    args.results_dir.mkdir(parents=True, exist_ok=True)
    sources = discover_sources()
    if args.corpus:
        allowed = set(args.corpus)
        sources = [source for source in sources if source.corpus in allowed]

    files = discover_files(sources, args.limit)
    rows = []
    for index, corpus_file in enumerate(files, start=1):
        if index == 1 or index % 100 == 0 or index == len(files):
            print(f"Corpus audit: {index}/{len(files)}", flush=True)
        rows.append(run_file(corpus_file, args))

    write_csv(args.results_dir / "corpus_matrix.csv", rows)
    write_json(args.results_dir / "corpus_matrix.json", rows)
    write_csv(args.results_dir / "java_pass_rh_fail.csv", java_pass_rh_fail_rows(rows))
    write_csv(args.results_dir / "java_non_pass.csv", java_non_pass_rows(rows))
    write_summary(args.results_dir / "corpus_summary.json", rows, sources)
    print(f"Wrote corpus audit results to {args.results_dir}")
    return 0


def discover_sources() -> list[CorpusSource]:
    return [
        CorpusSource("generated", GENERATED_DIR),
        CorpusSource(
            "cqframework_jvm_test",
            TOOLS_DIR / "cql-java/Src/java/cql-to-elm/src/jvmTest/resources",
        ),
        CorpusSource("cqframework_examples", TOOLS_DIR / "cql-java/Examples"),
        CorpusSource("cooking_with_cql", TOOLS_DIR / "cooking-with-cql/Source"),
        CorpusSource("cms_2025_ecqm", TOOLS_DIR / "ecqm-content-cms-2025"),
    ]


def discover_files(sources: list[CorpusSource], limit: int | None) -> list[CorpusFile]:
    files: list[CorpusFile] = []
    for source in sources:
        if not source.root.exists():
            continue
        source_files = sorted(path for path in source.root.glob(source.pattern) if path.is_file())
        if limit is not None:
            source_files = source_files[:limit]
        for path in source_files:
            files.append(
                CorpusFile(
                    corpus=source.corpus,
                    path=path,
                    relative_path=path.relative_to(source.root).as_posix(),
                )
            )
    return files


def run_file(corpus_file: CorpusFile, args: argparse.Namespace) -> dict[str, str]:
    rh_result = run_rh_compile(corpus_file.path, args.rh_cli.resolve(), args.rh_timeout_seconds)
    if args.skip_java:
        java_result = {"status": "not_run", "notes": "Java translation skipped"}
    else:
        java_result = run_java_translate(
            corpus_file.path,
            require=args.require_java,
            timeout_seconds=args.java_timeout_seconds,
        )

    rh_diagnostic_class = classify_diagnostic(rh_result["status"], rh_result["notes"])
    java_diagnostic_class = classify_diagnostic(java_result["status"], java_result["notes"])

    return {
        "corpus": corpus_file.corpus,
        "path": corpus_file.relative_path,
        "rh_cql_status": rh_result["status"],
        "rh_cql_diagnostic_class": rh_diagnostic_class,
        "rh_cql_notes": rh_result["notes"],
        "java_elm_status": java_result["status"],
        "java_elm_diagnostic_class": java_diagnostic_class,
        "java_elm_notes": java_result["notes"],
    }


def run_rh_compile(path: Path, rh_cli: Path, timeout_seconds: int) -> dict[str, str]:
    if not rh_cli.exists():
        return {"status": "not_run", "notes": f"rh CLI not found: {rh_cli}"}
    try:
        proc = subprocess.run(
            [str(rh_cli), "cql", "compile", str(path)],
            capture_output=True,
            text=True,
            timeout=timeout_seconds,
        )
    except subprocess.TimeoutExpired:
        return {"status": "timeout", "notes": f"rh-cql exceeded {timeout_seconds}s timeout"}
    if proc.returncode == 0:
        return {"status": "pass", "notes": ""}
    return {
        "status": "compile_error",
        "notes": truncate(first_non_empty(proc.stderr, proc.stdout, "rh-cql compile failed")),
    }


def run_java_translate(path: Path, require: bool, timeout_seconds: int) -> dict[str, str]:
    if not JAVA_CLI.exists():
        if require:
            raise SystemExit(f"Java translator CLI not found: {JAVA_CLI}")
        return {
            "status": "not_run",
            "notes": "Java translator CLI not found; run conformance/scripts/setup.sh",
        }
    with tempfile.TemporaryDirectory(prefix="rh-cql-corpus-java-") as tmp:
        output_dir = Path(tmp)
        try:
            proc = subprocess.run(
                [
                    str(JAVA_CLI),
                    "--input",
                    str(path),
                    "--format",
                    "JSON",
                    "--output",
                    str(output_dir),
                ],
                capture_output=True,
                text=True,
                timeout=timeout_seconds,
            )
        except subprocess.TimeoutExpired:
            return {
                "status": "timeout",
                "notes": f"Java translator exceeded {timeout_seconds}s timeout",
            }
    notes = first_non_empty(proc.stderr, proc.stdout, "")
    if proc.returncode == 0 and java_translation_succeeded(notes):
        return {"status": "pass", "notes": "Java translator produced ELM"}
    return {
        "status": "compile_error",
        "notes": truncate(first_non_empty(notes, "Java translator failed")),
    }


def write_csv(path: Path, rows: list[dict[str, str]]) -> None:
    fieldnames = [
        "corpus",
        "path",
        "rh_cql_status",
        "rh_cql_diagnostic_class",
        "rh_cql_notes",
        "java_elm_status",
        "java_elm_diagnostic_class",
        "java_elm_notes",
    ]
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def java_pass_rh_fail_rows(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    filtered = [
        row
        for row in rows
        if row["java_elm_status"] == "pass" and row["rh_cql_status"] != "pass"
    ]
    return sorted(
        filtered,
        key=lambda row: (
            row["corpus"],
            row["rh_cql_diagnostic_class"],
            row["path"],
        ),
    )


def java_non_pass_rows(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    filtered = [
        row
        for row in rows
        if row["java_elm_status"] not in {"pass", "not_run"}
    ]
    return sorted(
        filtered,
        key=lambda row: (
            row["corpus"],
            row["java_elm_status"],
            row["java_elm_diagnostic_class"],
            row["path"],
        ),
    )


def write_json(path: Path, rows: list[dict[str, str]]) -> None:
    path.write_text(json.dumps({"rows": rows}, indent=2))


def write_summary(path: Path, rows: list[dict[str, str]], sources: list[CorpusSource]) -> None:
    payload: dict[str, Any] = {
        "sources": [
            {
                "corpus": source.corpus,
                "root": str(source.root),
                "available": source.root.exists(),
            }
            for source in sources
        ],
        "row_count": len(rows),
        "java_pass_rh_fail_count": 0,
        "java_non_pass_count": 0,
        "java_pass_rh_fail_by_diagnostic_class": {},
        "java_non_pass_by_diagnostic_class": {},
        "by_corpus": {},
    }
    for row in rows:
        corpus = row["corpus"]
        corpus_summary = payload["by_corpus"].setdefault(
            corpus,
            {
                "row_count": 0,
                "rh_cql_status": {},
                "rh_cql_diagnostic_class": {},
                "java_elm_status": {},
                "java_elm_diagnostic_class": {},
                "java_pass_rh_fail_by_diagnostic_class": {},
                "java_non_pass_by_diagnostic_class": {},
            },
        )
        corpus_summary["row_count"] += 1
        increment(corpus_summary["rh_cql_status"], row["rh_cql_status"])
        increment(
            corpus_summary["rh_cql_diagnostic_class"],
            row["rh_cql_diagnostic_class"],
        )
        increment(corpus_summary["java_elm_status"], row["java_elm_status"])
        increment(
            corpus_summary["java_elm_diagnostic_class"],
            row["java_elm_diagnostic_class"],
        )

        if row["java_elm_status"] == "pass" and row["rh_cql_status"] != "pass":
            payload["java_pass_rh_fail_count"] += 1
            diagnostic_class = row["rh_cql_diagnostic_class"]
            increment(payload["java_pass_rh_fail_by_diagnostic_class"], diagnostic_class)
            increment(
                corpus_summary["java_pass_rh_fail_by_diagnostic_class"],
                diagnostic_class,
            )

        if row["java_elm_status"] not in {"pass", "not_run"}:
            payload["java_non_pass_count"] += 1
            diagnostic_class = row["java_elm_diagnostic_class"]
            increment(payload["java_non_pass_by_diagnostic_class"], diagnostic_class)
            increment(
                corpus_summary["java_non_pass_by_diagnostic_class"],
                diagnostic_class,
            )
    path.write_text(json.dumps(payload, indent=2))


def increment(counts: dict[str, int], status: str) -> None:
    counts[status] = counts.get(status, 0) + 1


def classify_diagnostic(status: str, notes: str) -> str:
    if status == "pass":
        return "none"
    if status == "timeout":
        return "timeout"
    if status == "not_run":
        if "not found" in notes.lower():
            return "tooling"
        return "not_run"

    normalized = notes.lower()
    if any(token in normalized for token in ("parse error", "syntax error", "unexpected")):
        return "parser"
    if any(
        token in normalized
        for token in (
            "could not resolve library",
            "library not found",
            "include",
            "included",
        )
    ):
        return "include_resolution"
    if any(
        token in normalized
        for token in (
            "modelinfo",
            "model info",
            "could not resolve property",
            "unknown property",
            "no such property",
        )
    ):
        return "model_info"
    if any(
        token in normalized
        for token in (
            "could not resolve",
            "unknown identifier",
            "ambiguous",
            "no overload",
            "type mismatch",
            "cannot convert",
        )
    ):
        return "semantic"
    return "unknown"


def java_translation_succeeded(notes: str) -> bool:
    normalized = notes.lower()
    if "translation failed" in normalized:
        return False
    if "syntax error" in normalized or "error:" in normalized:
        return False
    return "translation completed successfully" in normalized


def first_non_empty(*values: str) -> str:
    return next((value.strip() for value in values if value and value.strip()), "")


def truncate(value: str, max_len: int = 500) -> str:
    return value if len(value) <= max_len else value[: max_len - 3] + "..."


if __name__ == "__main__":
    raise SystemExit(main())
