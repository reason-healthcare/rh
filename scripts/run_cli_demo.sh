#!/usr/bin/env bash
set -euo pipefail

export RUST_LOG=info

function h() { echo "$ $*"; }

pushd "$(git rev-parse --show-toplevel)" >/dev/null

# Always use the typed REPL script when expect is available
if command -v expect >/dev/null 2>&1; then
  EXPECT_CHAR_DELAY_MS=${EXPECT_CHAR_DELAY_MS:-80} \
  EXPECT_LINE_PAUSE_MS=${EXPECT_LINE_PAUSE_MS:-700} \
  EXPECT_AFTER_CMD_MS=${EXPECT_AFTER_CMD_MS:-1000} \
  expect scripts/repl_demo.expect
else
  echo "(interactive REPL demo skipped: 'expect' not found)"
fi

echo
h asciinema rec --overwrite --capture-input --idle-time-limit 2 --window-size 80x24 --title "RH CLI Demo" -c "bash scripts/run_cli_demo.sh" assets/cli-demo.cast

popd >/dev/null
