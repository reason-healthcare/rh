#!/usr/bin/env python3
"""Compare rh-fsh output to fsh-sushi for real IG input/fsh projects."""

from __future__ import annotations

import argparse
import difflib
import json
import shutil
import subprocess
import sys
import tempfile
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any
from urllib.parse import quote


SCRIPT_DIR = Path(__file__).resolve().parent
CONFORMANCE_DIR = SCRIPT_DIR.parent
CRATE_DIR = CONFORMANCE_DIR.parent
WORKSPACE_ROOT = CRATE_DIR.parent.parent
PROJECTS_DIR = CONFORMANCE_DIR / "projects"
RESULTS_DIR = CONFORMANCE_DIR / "results"
FIXTURES_DIR = CONFORMANCE_DIR / "fixtures"
PROJECTS_MANIFEST = CONFORMANCE_DIR / "projects.json"
DEFAULT_RH_CLI = WORKSPACE_ROOT / "target/conformance/debug/rh"
DEFAULT_SUSHI_BIN = "npx --yes fsh-sushi@3.19.0"

PROJECT_SPECS = json.loads(PROJECTS_MANIFEST.read_text())
DEFAULT_PROJECTS = {name: spec["repo"] for name, spec in PROJECT_SPECS.items()}
DEFAULT_REVISIONS = {name: spec["revision"] for name, spec in PROJECT_SPECS.items()}

FIXTURE_PROJECTS = {
    "profile-identity-smoke": FIXTURES_DIR / "profile-identity-smoke",
}

DEFAULT_THRESHOLDS = {
    "carin-bb": {"missing": 0, "extra": 0, "mismatch": 53},
    "mcode": {"missing": 0, "extra": 0, "mismatch": 155},
    "davinci-crd": {"missing": 0, "extra": 0, "mismatch": 51},
    "davinci-dtr": {"missing": 0, "extra": 0, "mismatch": 33},
    "davinci-pas": {"missing": 0, "extra": 0, "mismatch": 96},
    "fhir-ips": {"missing": 0, "extra": 0, "mismatch": 71},
    "profile-identity-smoke": {"missing": 0, "extra": 0, "mismatch": 1},
}

GAP_CATEGORIES = [
    "resource_identity",
    "json_shape",
    "structure_definition",
    "metadata",
    "terminology",
    "ig_generation",
    "other",
]

IGNORED_JSON_NAMES = {"package.json"}
IGNORED_JSON_SUFFIXES = (".index.json",)
NORMALIZATION_IGNORED_KEYS = {
    "date",
    "experimental",
    "meta",
    "publisher",
    "text",
    "version",
}


@dataclass
class ToolResult:
    ok: bool
    duration_seconds: float
    stdout: str = ""
    stderr: str = ""
    error: str | None = None


@dataclass
class ProjectResult:
    name: str
    repo: str
    path: str
    fsh_files: int
    sushi: ToolResult
    rh_fsh: ToolResult
    reference_resources: int = 0
    actual_resources: int = 0
    skipped_reason: str | None = None
    missing: list[str] = field(default_factory=list)
    extra: list[str] = field(default_factory=list)
    mismatched: list[dict[str, Any]] = field(default_factory=list)
    category_summary: dict[str, int] = field(default_factory=dict)
    thresholds: dict[str, int] = field(default_factory=dict)
    artifacts_path: str | None = None

    @property
    def status(self) -> str:
        if self.skipped_reason:
            return "skipped"
        if not self.sushi.ok:
            return "sushi-failed"
        if not self.rh_fsh.ok:
            return "rh-fsh-failed"
        if self.missing or self.extra or self.mismatched:
            return "different"
        return "match"

    @property
    def within_threshold(self) -> bool:
        if self.skipped_reason:
            return True
        if not self.sushi.ok or not self.rh_fsh.ok:
            return False
        if not self.thresholds:
            return True
        return (
            len(self.missing) <= self.thresholds.get("missing", 0)
            and len(self.extra) <= self.thresholds.get("extra", 0)
            and len(self.mismatched) <= self.thresholds.get("mismatch", 0)
        )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--projects-dir", type=Path, default=PROJECTS_DIR)
    parser.add_argument("--results-dir", type=Path, default=RESULTS_DIR)
    parser.add_argument("--artifacts-dir", type=Path)
    parser.add_argument("--rh-cli", type=Path, default=DEFAULT_RH_CLI)
    parser.add_argument("--sushi-bin", default=DEFAULT_SUSHI_BIN)
    parser.add_argument("--limit-files", type=int)
    parser.add_argument("--project", action="append", choices=sorted(DEFAULT_PROJECTS))
    parser.add_argument("--fixture", action="append", choices=sorted(FIXTURE_PROJECTS))
    parser.add_argument("--update-projects", action="store_true")
    parser.add_argument("--thresholds-file", type=Path)
    parser.add_argument("--timeout-seconds", type=int, default=300)
    args = parser.parse_args()

    args.projects_dir.mkdir(parents=True, exist_ok=True)
    args.results_dir.mkdir(parents=True, exist_ok=True)
    if args.artifacts_dir:
        args.artifacts_dir.mkdir(parents=True, exist_ok=True)

    selected = args.project or ([] if args.fixture else list(DEFAULT_PROJECTS))
    thresholds = load_thresholds(args.thresholds_file)
    results: list[ProjectResult] = []
    for name in selected:
        repo = DEFAULT_PROJECTS[name]
        project_dir = ensure_project(
            name,
            repo,
            DEFAULT_REVISIONS[name],
            args.projects_dir,
            args.update_projects,
        )
        results.append(run_project(name, repo, project_dir, args, thresholds.get(name, {})))
    for name in args.fixture or []:
        project_dir = FIXTURE_PROJECTS[name]
        results.append(
            run_project(
                name,
                f"fixture:{name}",
                project_dir,
                args,
                thresholds.get(name, {}),
            )
        )

    write_reports(results, args.results_dir)
    failures = [
        r
        for r in results
        if r.status not in {"match", "skipped"} and not r.within_threshold
    ]
    print(f"Wrote {args.results_dir / 'latest-summary.md'}")
    return 1 if failures else 0


def ensure_project(
    name: str, repo: str, revision: str, projects_dir: Path, update: bool
) -> Path:
    project_dir = projects_dir / name
    if not project_dir.exists():
        run(
            ["git", "clone", "--filter=blob:none", "--no-checkout", repo, str(project_dir)],
            cwd=projects_dir,
        )
        update = True

    current = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=project_dir,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
    ).stdout.strip()
    if current != revision:
        if not update:
            raise RuntimeError(
                f"{name} is at {current or 'no revision'}, expected {revision}; "
                "rerun with --update-projects"
            )
        dirty = subprocess.run(
            ["git", "status", "--porcelain"],
            cwd=project_dir,
            text=True,
            stdout=subprocess.PIPE,
            check=True,
        ).stdout.strip()
        if dirty:
            raise RuntimeError(f"refusing to replace modified conformance project {name}")
        run(["git", "fetch", "--depth", "1", "origin", revision], cwd=project_dir)
        run(["git", "checkout", "--detach", revision], cwd=project_dir)
    return project_dir


def run_project(
    name: str,
    repo: str,
    project_dir: Path,
    args: argparse.Namespace,
    thresholds: dict[str, int],
) -> ProjectResult:
    fsh_dir = project_dir / "input" / "fsh"
    fsh_files = sorted(fsh_dir.rglob("*.fsh")) if fsh_dir.exists() else []
    if args.limit_files:
        fsh_files = fsh_files[: args.limit_files]

    if not fsh_files:
        return ProjectResult(
            name=name,
            repo=repo,
            path=str(project_dir),
            fsh_files=0,
            sushi=ToolResult(ok=True, duration_seconds=0.0),
            rh_fsh=ToolResult(ok=True, duration_seconds=0.0),
            skipped_reason="no input/fsh/*.fsh files found",
            thresholds=thresholds,
        )

    with tempfile.TemporaryDirectory(prefix=f"rh-fsh-{name}-") as tmp:
        tmp_dir = Path(tmp)
        sushi_project = tmp_dir / "sushi-project"
        rh_out = tmp_dir / "rh-generated"
        shutil.copytree(project_dir, sushi_project, ignore=ignore_generated)
        if args.limit_files:
            replace_fsh_files(sushi_project, fsh_files, project_dir)

        sushi = timed_run(
            args.sushi_bin.split() + [str(sushi_project)],
            cwd=sushi_project,
            timeout_seconds=args.timeout_seconds,
        )
        rh = ToolResult(ok=False, duration_seconds=0.0, error="not run")
        reference: dict[str, Any] = {}
        actual: dict[str, Any] = {}
        missing: list[str] = []
        extra: list[str] = []
        mismatched: list[dict[str, Any]] = []
        category_summary: dict[str, int] = {}
        artifacts_path: str | None = None

        if sushi.ok:
            reference_raw = load_resources(sushi_project / "fsh-generated" / "resources")
            reference = normalize_resources(reference_raw)
            rh, actual_raw = run_rh_fsh_json(args.rh_cli, fsh_files, args.timeout_seconds)
            if rh.ok:
                actual = normalize_resources(actual_raw)
                missing, extra, mismatched = compare_resources(reference, actual)
                category_summary = categorize_gaps(missing, extra, mismatched)
                if args.artifacts_dir:
                    project_artifacts = args.artifacts_dir / name
                    write_comparison_artifacts(
                        project_artifacts,
                        name,
                        reference_raw,
                        actual_raw,
                    )
                    artifacts_path = str(project_artifacts)

        return ProjectResult(
            name=name,
            repo=repo,
            path=str(project_dir),
            fsh_files=len(fsh_files),
            sushi=sushi,
            rh_fsh=rh,
            reference_resources=len(reference),
            actual_resources=len(actual),
            missing=missing,
            extra=extra,
            mismatched=mismatched,
            category_summary=category_summary,
            thresholds=thresholds,
            artifacts_path=artifacts_path,
        )


def load_thresholds(path: Path | None) -> dict[str, dict[str, int]]:
    if path is None:
        return {name: values.copy() for name, values in DEFAULT_THRESHOLDS.items()}
    raw = json.loads(path.read_text())
    thresholds: dict[str, dict[str, int]] = {}
    for project, values in raw.items():
        thresholds[project] = {
            "missing": int(values.get("missing", 0)),
            "extra": int(values.get("extra", 0)),
            "mismatch": int(values.get("mismatch", 0)),
        }
    return thresholds


def ignore_generated(_dir: str, names: list[str]) -> set[str]:
    return {
        name
        for name in names
        if name in {"fsh-generated", "output", "temp", "template", "node_modules"}
    }


def replace_fsh_files(sushi_project: Path, selected_files: list[Path], original_project: Path) -> None:
    fsh_dir = sushi_project / "input" / "fsh"
    if fsh_dir.exists():
        shutil.rmtree(fsh_dir)
    fsh_dir.mkdir(parents=True, exist_ok=True)
    for original in selected_files:
        relative = original.relative_to(original_project / "input" / "fsh")
        target = fsh_dir / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(original, target)


def timed_run(command: list[str], cwd: Path, timeout_seconds: int) -> ToolResult:
    start = datetime.now(timezone.utc)
    try:
        proc = subprocess.run(
            command,
            cwd=cwd,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout_seconds,
        )
        end = datetime.now(timezone.utc)
        return ToolResult(
            ok=proc.returncode == 0,
            duration_seconds=(end - start).total_seconds(),
            stdout=tail(proc.stdout),
            stderr=tail(proc.stderr),
            error=None if proc.returncode == 0 else f"exit {proc.returncode}",
        )
    except FileNotFoundError as exc:
        end = datetime.now(timezone.utc)
        return ToolResult(False, (end - start).total_seconds(), error=str(exc))
    except subprocess.TimeoutExpired as exc:
        end = datetime.now(timezone.utc)
        return ToolResult(
            False,
            (end - start).total_seconds(),
            stdout=tail(exc.stdout or ""),
            stderr=tail(exc.stderr or ""),
            error=f"timeout after {timeout_seconds}s",
        )


def run_rh_fsh_json(
    rh_cli: Path, fsh_files: list[Path], timeout_seconds: int
) -> tuple[ToolResult, dict[str, Any]]:
    start = datetime.now(timezone.utc)
    command = [str(rh_cli), "--format", "json", "fsh", "compile", *map(str, fsh_files)]
    try:
        proc = subprocess.run(
            command,
            cwd=WORKSPACE_ROOT,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout_seconds,
        )
    except FileNotFoundError as exc:
        end = datetime.now(timezone.utc)
        return ToolResult(False, (end - start).total_seconds(), error=str(exc)), {}
    except subprocess.TimeoutExpired as exc:
        end = datetime.now(timezone.utc)
        return (
            ToolResult(
                False,
                (end - start).total_seconds(),
                stdout=tail(exc.stdout or ""),
                stderr=tail(exc.stderr or ""),
                error=f"timeout after {timeout_seconds}s",
            ),
            {},
        )

    end = datetime.now(timezone.utc)
    result = ToolResult(
        ok=proc.returncode == 0,
        duration_seconds=(end - start).total_seconds(),
        stdout=tail(proc.stdout),
        stderr=tail(proc.stderr),
        error=None if proc.returncode == 0 else f"exit {proc.returncode}",
    )
    if not result.ok:
        return result, {}
    try:
        envelope = json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        result.ok = False
        result.error = f"invalid rh JSON output: {exc}"
        return result, {}

    payload = envelope.get("result", [])
    if isinstance(payload, dict):
        payload = payload.get("resources", [])

    resources: dict[str, Any] = {}
    for value in payload:
        key = resource_key(value)
        if key:
            resources[key] = value
    return result, resources


def run(command: list[str], cwd: Path) -> None:
    subprocess.run(command, cwd=cwd, check=True)


def load_resources(directory: Path) -> dict[str, Any]:
    resources: dict[str, Any] = {}
    if not directory.exists():
        return resources
    for path in sorted(directory.rglob("*.json")):
        if path.name in IGNORED_JSON_NAMES or path.name.endswith(IGNORED_JSON_SUFFIXES):
            continue
        try:
            value = json.loads(path.read_text())
        except json.JSONDecodeError:
            continue
        key = resource_key(value)
        if key:
            resources[key] = value
    return resources


def normalize_resources(resources: dict[str, Any]) -> dict[str, Any]:
    return {key: normalize_resource(value) for key, value in resources.items()}


def artifact_filename(resource: str) -> str:
    return f"{quote(resource, safe='-._~')}.json"


def serialize_resource(value: Any) -> str:
    return json.dumps(value, indent=2, sort_keys=True) + "\n"


def write_resource_tree(directory: Path, resources: dict[str, Any]) -> None:
    directory.mkdir(parents=True, exist_ok=True)
    for resource, value in sorted(resources.items()):
        path = directory / artifact_filename(resource)
        path.write_text(serialize_resource(value))


def write_comparison_artifacts(
    directory: Path,
    project: str,
    sushi: dict[str, Any],
    rh_fsh: dict[str, Any],
) -> None:
    if directory.exists():
        shutil.rmtree(directory)

    normalized_sushi = normalize_resources(sushi)
    normalized_rh_fsh = normalize_resources(rh_fsh)
    write_resource_tree(directory / "sushi", sushi)
    write_resource_tree(directory / "rh-fsh", rh_fsh)
    write_resource_tree(directory / "normalized" / "sushi", normalized_sushi)
    write_resource_tree(directory / "normalized" / "rh-fsh", normalized_rh_fsh)

    diffs_dir = directory / "diffs"
    diffs_dir.mkdir(parents=True, exist_ok=True)
    resources = []
    for resource in sorted(set(sushi) | set(rh_fsh)):
        filename = artifact_filename(resource)
        sushi_value = sushi.get(resource)
        rh_fsh_value = rh_fsh.get(resource)
        raw_equal = resource in sushi and resource in rh_fsh and sushi_value == rh_fsh_value
        sushi_text = serialize_resource(sushi_value) if resource in sushi else ""
        rh_fsh_text = serialize_resource(rh_fsh_value) if resource in rh_fsh else ""
        serialized_equal = (
            resource in sushi and resource in rh_fsh and sushi_text == rh_fsh_text
        )
        normalized_equal = (
            resource in normalized_sushi
            and resource in normalized_rh_fsh
            and normalized_sushi[resource] == normalized_rh_fsh[resource]
        )
        if resource not in sushi:
            status = "rh-fsh-only"
        elif resource not in rh_fsh:
            status = "sushi-only"
        elif normalized_equal:
            status = "normalized-match"
        else:
            status = "different"

        if not serialized_equal:
            diff = difflib.unified_diff(
                sushi_text.splitlines(keepends=True),
                rh_fsh_text.splitlines(keepends=True),
                fromfile=f"sushi/{filename}",
                tofile=f"rh-fsh/{filename}",
            )
            (diffs_dir / f"{filename[:-5]}.diff").write_text("".join(diff))

        entry = {
            "resource": resource,
            "filename": filename,
            "status": status,
            "raw_equal": raw_equal,
            "serialized_equal": serialized_equal,
            "normalized_equal": normalized_equal,
        }
        if status == "different":
            entry.update(
                annotate_mismatch(
                    first_difference(resource, normalized_sushi[resource], normalized_rh_fsh[resource])
                )
            )
        resources.append(entry)

    manifest = {
        "project": project,
        "normalization_ignored_keys_recursively": sorted(NORMALIZATION_IGNORED_KEYS),
        "sushi_resources": len(sushi),
        "rh_fsh_resources": len(rh_fsh),
        "resources": resources,
    }
    (directory / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    (directory / "README.md").write_text(
        "# SUSHI and rh-fsh comparison artifacts\n\n"
        "Raw generated resources use identical deterministic filenames in `sushi/` and "
        "`rh-fsh/`. The `normalized/` trees apply the same recursive key filtering as "
        "the conformance summary.\n\n"
        "```bash\n"
        "diff -ru sushi rh-fsh\n"
        "diff -ru normalized/sushi normalized/rh-fsh\n"
        "```\n"
    )


def resource_key(value: Any) -> str | None:
    if not isinstance(value, dict):
        return None
    resource_type = value.get("resourceType")
    resource_id = value.get("id")
    if isinstance(resource_type, str) and isinstance(resource_id, str):
        return f"{resource_type}/{resource_id}"
    return None


def normalize_resource(value: Any) -> Any:
    if isinstance(value, dict):
        normalized = {
            key: normalize_resource(child)
            for key, child in value.items()
            if key not in NORMALIZATION_IGNORED_KEYS
        }
        return dict(sorted(normalized.items()))
    if isinstance(value, list):
        return [normalize_resource(child) for child in value]
    return value


def compare_resources(
    reference: dict[str, Any], actual: dict[str, Any]
) -> tuple[list[str], list[str], list[dict[str, Any]]]:
    missing = sorted(set(reference) - set(actual))
    extra = sorted(set(actual) - set(reference))
    mismatched = []
    for key in sorted(set(reference) & set(actual)):
        if reference[key] != actual[key]:
            mismatched.append(annotate_mismatch(first_difference(key, reference[key], actual[key])))
    return missing, extra, mismatched


def categorize_gaps(
    missing: list[str], extra: list[str], mismatched: list[dict[str, Any]]
) -> dict[str, int]:
    summary = {category: 0 for category in GAP_CATEGORIES}
    paired_missing, paired_extra = find_resource_identity_pairs(missing, extra)

    for item in missing:
        if item in paired_missing:
            summary["resource_identity"] += 1
        elif item.startswith("ImplementationGuide/"):
            summary["ig_generation"] += 1
        else:
            summary["other"] += 1

    for item in extra:
        if item in paired_extra:
            summary["resource_identity"] += 1
        elif item.startswith("ImplementationGuide/"):
            summary["ig_generation"] += 1
        else:
            summary["other"] += 1

    for item in mismatched:
        summary[classify_mismatch(item)] += 1

    return {key: value for key, value in summary.items() if value}


def find_resource_identity_pairs(
    missing: list[str], extra: list[str]
) -> tuple[set[str], set[str]]:
    missing_by_id: dict[str, list[str]] = {}
    extra_by_id: dict[str, list[str]] = {}
    for item in missing:
        missing_by_id.setdefault(resource_id(item), []).append(item)
    for item in extra:
        extra_by_id.setdefault(resource_id(item), []).append(item)
    shared_ids = set(missing_by_id) & set(extra_by_id)
    return (
        {item for resource_id in shared_ids for item in missing_by_id[resource_id]},
        {item for resource_id in shared_ids for item in extra_by_id[resource_id]},
    )


def resource_id(resource_key_value: str) -> str:
    return resource_key_value.split("/", 1)[1] if "/" in resource_key_value else resource_key_value


def classify_mismatch(item: dict[str, Any]) -> str:
    resource = item.get("resource", "")
    path = item.get("path", "")
    sushi = item.get("sushi")
    rh_fsh = item.get("rh_fsh")

    if resource.startswith("ImplementationGuide/"):
        return "ig_generation"
    if resource.startswith("StructureDefinition/") or path.startswith("$.differential") or path in {
        "$.baseDefinition",
        "$.context",
    }:
        return "structure_definition"
    if resource.startswith("CodeSystem/") or path.endswith(".count"):
        return "terminology"
    if path_matches_metadata(path):
        return "metadata"
    if type(sushi) is not type(rh_fsh) or path_matches_json_shape(path):
        return "json_shape"
    return "other"


def annotate_mismatch(item: dict[str, Any]) -> dict[str, Any]:
    category = classify_mismatch(item)
    annotated = {**item, "category": category}
    if category == "json_shape":
        annotated["shape_family"] = json_shape_family(item["path"])
    return annotated


def json_shape_family(path: str) -> str:
    if ".extension" in path:
        return "extension"
    if "._" in path:
        return "primitive_shadow"
    if ".contained" in path:
        return "contained_resource"
    if ".entry" in path:
        return "bundle_entry"
    if ".parameter" in path:
        return "parameters_part"
    if any(token in path for token in (".adjudication", ".supportingInfo", ".component", ".item")):
        return "repeating_backbone"
    if any(token in path for token in (".coding", ".code", ".reference", ".value", ".dose")):
        return "typed_value"
    return "other_json_shape"


def path_matches_metadata(path: str) -> bool:
    metadata_suffixes = (
        ".url",
        ".title",
        ".description",
        ".name",
        ".status",
        ".version",
        ".publisher",
        ".derivedFrom[0]",
    )
    return path == "$.url" or any(path.endswith(suffix) for suffix in metadata_suffixes)


def path_matches_json_shape(path: str) -> bool:
    shape_tokens = (
        ".extension",
        "._",
        ".contained",
        ".entry",
        ".parameter",
        ".coding",
        ".code",
        ".value",
        ".reference",
        ".dose",
        ".doseAndRate",
        ".supportedProfile",
        ".targetProfile",
    )
    return any(token in path for token in shape_tokens)


def first_difference(key: str, reference: Any, actual: Any) -> dict[str, Any]:
    path, expected, observed = diff_value(reference, actual, "$")
    return {
        "resource": key,
        "path": path,
        "sushi": expected,
        "rh_fsh": observed,
    }


def diff_value(left: Any, right: Any, path: str) -> tuple[str, Any, Any]:
    if type(left) is not type(right):
        return path, left, right
    if isinstance(left, dict):
        for key in sorted(set(left) | set(right)):
            if key not in left:
                return f"{path}.{key}", None, right[key]
            if key not in right:
                return f"{path}.{key}", left[key], None
            child = diff_value(left[key], right[key], f"{path}.{key}")
            if child[1] != child[2]:
                return child
        return path, left, right
    if isinstance(left, list):
        for index, (l_item, r_item) in enumerate(zip(left, right)):
            child = diff_value(l_item, r_item, f"{path}[{index}]")
            if child[1] != child[2]:
                return child
        if len(left) != len(right):
            return f"{path}.length", len(left), len(right)
        return path, left, right
    return path, left, right


def write_reports(results: list[ProjectResult], results_dir: Path) -> None:
    data = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "projects": [project_to_json(result) for result in results],
    }
    (results_dir / "latest-summary.json").write_text(json.dumps(data, indent=2) + "\n")
    (results_dir / "latest-summary.md").write_text(render_markdown(data) + "\n")


def project_to_json(result: ProjectResult) -> dict[str, Any]:
    return {
        "name": result.name,
        "repo": result.repo,
        "path": result.path,
        "status": result.status,
        "fsh_files": result.fsh_files,
        "reference_resources": result.reference_resources,
        "actual_resources": result.actual_resources,
        "skipped_reason": result.skipped_reason,
        "missing_count": len(result.missing),
        "extra_count": len(result.extra),
        "mismatch_count": len(result.mismatched),
        "thresholds": result.thresholds,
        "within_threshold": result.within_threshold,
        "category_summary": result.category_summary,
        "artifacts_path": result.artifacts_path,
        "missing": result.missing[:100],
        "extra": result.extra[:100],
        "mismatched": result.mismatched[:100],
        "sushi": result.sushi.__dict__,
        "rh_fsh": result.rh_fsh.__dict__,
    }


def render_markdown(data: dict[str, Any]) -> str:
    lines = [
        "# rh-fsh SUSHI Comparison",
        "",
        f"Generated: `{data['generated_at']}`",
        "",
        "| Project | Status | Threshold | FSH files | SUSHI resources | rh-fsh resources | Missing | Extra | Mismatch |",
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for project in data["projects"]:
        lines.append(
            "| {name} | {status} | {threshold} | {fsh_files} | {reference_resources} | {actual_resources} | {missing_count} | {extra_count} | {mismatch_count} |".format(
                threshold="pass" if project["within_threshold"] else "fail",
                **project,
            )
        )
    lines.append("")
    for project in data["projects"]:
        lines.append(f"## {project['name']}")
        if project["status"] == "skipped":
            lines.append("")
            lines.append(f"Skipped: {project['skipped_reason']}")
            lines.append("")
            continue
        if project["status"] in {"rh-fsh-failed", "sushi-failed"}:
            lines.append("")
            failed_tool = "rh_fsh" if project["status"] == "rh-fsh-failed" else "sushi"
            lines.append(f"`{failed_tool}` failed before comparison:")
            lines.append("")
            lines.append("```text")
            lines.append(project[failed_tool].get("stderr") or project[failed_tool].get("stdout") or project[failed_tool].get("error") or "")
            lines.append("```")
            lines.append("")
            continue
        if project["thresholds"]:
            lines.append("")
            lines.append(
                "Threshold: "
                + ("pass" if project["within_threshold"] else "fail")
                + " "
                + json.dumps(project["thresholds"], sort_keys=True)
            )
        if project["category_summary"]:
            lines.append("")
            lines.append("Gap categories:")
            for category, count in sorted(project["category_summary"].items()):
                lines.append(f"- `{category}`: {count}")
        if project["missing"]:
            lines.append("")
            lines.append("Missing resources:")
            for item in project["missing"][:20]:
                lines.append(f"- `{item}`")
        if project["extra"]:
            lines.append("")
            lines.append("Extra resources:")
            for item in project["extra"][:20]:
                lines.append(f"- `{item}`")
        if project["mismatched"]:
            lines.append("")
            lines.append("First mismatches:")
            for item in project["mismatched"][:20]:
                lines.append(f"- `{item['resource']}` at `{item['path']}`")
    return "\n".join(lines)


def tail(text: str, max_chars: int = 8000) -> str:
    return text[-max_chars:] if len(text) > max_chars else text


if __name__ == "__main__":
    sys.exit(main())
