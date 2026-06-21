#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFORMANCE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
TOOLS_DIR="$CONFORMANCE_DIR/tools"

CMS_ECQM_REPO="${CMS_ECQM_REPO:-https://github.com/cqframework/ecqm-content-cms-2025.git}"
CMS_ECQM_REF="${CMS_ECQM_REF:-master}"

mkdir -p "$TOOLS_DIR"

echo "Setting up CQL reference corpora..."

"$SCRIPT_DIR/setup.sh"

if [ ! -d "$TOOLS_DIR/ecqm-content-cms-2025" ]; then
    echo "Cloning CMS eCQM content from ${CMS_ECQM_REPO} (${CMS_ECQM_REF})..."
    git clone --depth 1 --branch "$CMS_ECQM_REF" "$CMS_ECQM_REPO" "$TOOLS_DIR/ecqm-content-cms-2025"
else
    echo "✓ CMS eCQM repo already cloned"
fi

cd "$TOOLS_DIR/ecqm-content-cms-2025"
git fetch --depth 1 origin "$CMS_ECQM_REF"
git checkout FETCH_HEAD
CMS_ECQM_COMMIT="$(git rev-parse HEAD)"
echo "✓ CMS eCQM content pinned: ${CMS_ECQM_REF} (${CMS_ECQM_COMMIT})"

cat > "$CONFORMANCE_DIR/corpus-version.json" << EOF
{
  "cms_ecqm_2025": {
    "repository": "$CMS_ECQM_REPO",
    "ref": "$CMS_ECQM_REF",
    "commit": "$CMS_ECQM_COMMIT"
  }
}
EOF
echo "✓ Wrote corpus-version.json"
