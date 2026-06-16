#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
metadata_file="$(mktemp)"
help_file="$(mktemp)"
trap 'rm -f "$metadata_file" "$help_file"' EXIT

cd "$repo_root"

cargo metadata --format-version 1 --no-deps > "$metadata_file"
cargo run -q -p rh-cli -- --help > "$help_file"

python3 - "$repo_root" "$metadata_file" "$help_file" <<'PY'
import json
import re
import sys
from pathlib import Path

repo_root = Path(sys.argv[1])
metadata_path = Path(sys.argv[2])
help_path = Path(sys.argv[3])


def fail(message: str) -> None:
    print(f"docs sync check failed: {message}", file=sys.stderr)
    sys.exit(1)


def read(path: str) -> str:
    return (repo_root / path).read_text(encoding="utf-8")


def marked_block(path: str, name: str) -> str:
    text = read(path)
    start = f"<!-- docs-sync:{name}:start -->"
    end = f"<!-- docs-sync:{name}:end -->"
    try:
        body = text.split(start, 1)[1].split(end, 1)[0].strip()
    except IndexError:
        fail(f"{path} is missing marker block {name}")

    lines = body.splitlines()
    if lines and lines[0].startswith("```"):
        lines = lines[1:]
    if lines and lines[-1].startswith("```"):
        lines = lines[:-1]
    return "\n".join(line.rstrip() for line in lines).strip()


def check_architecture_deps() -> None:
    metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
    local = {pkg["name"] for pkg in metadata["packages"] if pkg.get("source") is None}
    lines = []
    for pkg in sorted((pkg for pkg in metadata["packages"] if pkg.get("source") is None), key=lambda p: p["name"]):
        deps = sorted({
            dep["name"]
            for dep in pkg["dependencies"]
            if dep.get("path") and dep["name"] in local
        })
        lines.append(f"{pkg['name']}: {', '.join(deps) if deps else '-'}")

    actual = "\n".join(lines)
    documented = marked_block(".github/ARCHITECTURE.md", "crate-deps")
    if documented != actual:
        fail(
            ".github/ARCHITECTURE.md crate dependency block is stale\n"
            f"expected:\n{actual}\n\nfound:\n{documented}"
        )


def parse_cli_help_commands() -> list[tuple[str, str]]:
    help_text = help_path.read_text(encoding="utf-8")
    try:
        command_text = help_text.split("Commands:", 1)[1].split("Options:", 1)[0]
    except IndexError:
        fail("could not parse Commands section from rh --help")

    commands = []
    for line in command_text.splitlines():
        match = re.match(r"^\s{2}(\S+)\s{2,}(.+?)\s*$", line)
        if match:
            commands.append((f"rh {match.group(1)}", match.group(2)))
    if not commands:
        fail("no commands parsed from rh --help")
    return commands


def parse_markdown_command_rows() -> list[tuple[str, str]]:
    block = marked_block("apps/rh-cli/README.md", "cli-commands")
    rows = []
    for line in block.splitlines():
        match = re.match(r"^\|\s*`([^`]+)`\s*\|\s*([^|]+?)\s*\|", line)
        if match:
            rows.append((match.group(1), match.group(2).strip()))
    if not rows:
        fail("no command rows parsed from apps/rh-cli/README.md")
    return rows


def check_cli_commands() -> None:
    actual = parse_cli_help_commands()
    documented = parse_markdown_command_rows()
    if documented != actual:
        fail(
            "apps/rh-cli/README.md command table is stale\n"
            f"expected:\n{actual}\n\nfound:\n{documented}"
        )


def parse_cli_help_options() -> set[str]:
    help_text = help_path.read_text(encoding="utf-8")
    try:
        option_text = help_text.split("Options:", 1)[1]
    except IndexError:
        fail("could not parse Options section from rh --help")

    options = set()
    for line in option_text.splitlines():
        match = re.match(r"^\s*(?:-\w,\s*)?(--[a-z-]+)", line)
        if match:
            options.add(match.group(1))
    if not options:
        fail("no global options parsed from rh --help")
    return options


def parse_markdown_options() -> set[str]:
    block = marked_block("apps/rh-cli/README.md", "cli-options")
    options = set()
    for line in block.splitlines():
        match = re.match(r"^\|\s*`(--[a-z-]+)", line)
        if match:
            options.add(match.group(1))
    if not options:
        fail("no global option rows parsed from apps/rh-cli/README.md")
    return options


def check_cli_options() -> None:
    actual = parse_cli_help_options()
    documented = parse_markdown_options()
    if documented != actual:
        fail(
            "apps/rh-cli/README.md global option table is stale\n"
            f"expected: {sorted(actual)}\nfound: {sorted(documented)}"
        )


check_architecture_deps()
check_cli_commands()
check_cli_options()
print("docs sync check passed")
PY
