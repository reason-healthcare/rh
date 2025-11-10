#!/usr/bin/env bash
set -euo pipefail

if ! command -v asciinema >/dev/null 2>&1; then
  echo "asciinema is not installed. Please install it first:"
  echo "  macOS (brew):   brew install asciinema"
  echo "  Ubuntu/Debian:  sudo apt-get install asciinema"
  echo "  Pipx:           pipx install asciinema"
  exit 1
fi

ROOT_DIR="$(git rev-parse --show-toplevel)"
CAST_OUT="${ROOT_DIR}/assets/cli-demo.cast"

pushd "$ROOT_DIR" >/dev/null

echo "Building CLI (for fast demo runs)..."
cargo build -p rh >/dev/null

echo "Recording demo to ${CAST_OUT}..."
# Prefer fully typed demo via expect when available
if command -v expect >/dev/null 2>&1; then
  ASCIINEMA_REC_COMMAND="expect ${ROOT_DIR}/scripts/repl_demo.expect"
else
  ASCIINEMA_REC_COMMAND="bash ${ROOT_DIR}/scripts/run_cli_demo.sh"
fi

# If not attached to a TTY, asciinema will run in headless mode and won't show per-keystroke typing.
if [ ! -t 1 ]; then
  echo "No TTY detected. Live typing won't be visible in headless mode."
  echo "Tip: run this script from a real terminal to capture as-you-type."
fi

# Normalize width/height and capture keyboard input (when TTY is available)
env COLUMNS=80 asciinema rec \
  --overwrite \
  --capture-input \
  --idle-time-limit 2 \
  --window-size 80x24 \
  --title "RH CLI Demo" \
  -c "${ASCIINEMA_REC_COMMAND}" \
  "${CAST_OUT}"

echo "Done. Play it back with: asciinema play ${CAST_OUT}"

popd >/dev/null
