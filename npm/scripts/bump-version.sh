#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:?Usage: bump-version.sh <version>}"

echo "Bumping all npm packages to version ${VERSION}..."

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
NPM_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

jq --arg v "$VERSION" '.version = $v | .optionalDependencies |= with_entries(.value = $v)' \
  "${NPM_DIR}/letta-mcp-server/package.json" > tmp.json && mv tmp.json "${NPM_DIR}/letta-mcp-server/package.json"

for dir in "${NPM_DIR}"/platforms/*/; do
  jq --arg v "$VERSION" '.version = $v' "${dir}package.json" > tmp.json && mv tmp.json "${dir}package.json"
done

echo "All packages set to ${VERSION}:"
echo ""
echo "  letta-mcp-server: $(jq -r .version "${NPM_DIR}/letta-mcp-server/package.json")"
for dir in "${NPM_DIR}"/platforms/*/; do
  name=$(jq -r .name "${dir}package.json")
  ver=$(jq -r .version "${dir}package.json")
  echo "  ${name}: ${ver}"
done
