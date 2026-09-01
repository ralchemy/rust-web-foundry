#!/usr/bin/env bash
set -euo pipefail

# Compatibility entry point for the existing `just architecture` recipe.
# The generated project is code-first; this now validates only the small
# project contract and review integration.
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
exec bash "$repo_root/scripts/check-project-contract.sh"
