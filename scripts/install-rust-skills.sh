#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
lock_file="$repo_root/.agents/rust-skills.lock"
install_dir=${RUST_SKILLS_DIR:-"$repo_root/.scratch/rust-skills"}

fail() {
  echo "rust-skills: $*" >&2
  exit 1
}

lock_value() {
  local key=$1 value count
  count=$(grep -Ec "^${key}=[^[:space:]]+$" "$lock_file" || true)
  ((count == 1)) || fail "lock file must contain exactly one ${key}=... entry"
  value=$(sed -nE "s/^${key}=([^[:space:]]+)$/\1/p" "$lock_file")
  printf '%s' "$value"
}

[[ -f "$lock_file" ]] || fail "missing lock file: .agents/rust-skills.lock"
repository=$(lock_value repository)
commit=$(lock_value commit)
version=$(lock_value version)
license=$(lock_value license)
[[ "$commit" =~ ^[0-9a-f]{40}$ ]] || fail "commit must be a full lowercase SHA"

metadata() {
  printf 'rust_skills: repository=%s commit=%s version=%s license=%s path=%s\n' \
    "$repository" "$commit" "$version" "$license" "${install_dir#"$repo_root/"}"
}

check_install() {
  [[ -f "$install_dir/SKILL.md" && -d "$install_dir/rules" && -d "$install_dir/.git" ]] \
    || fail "pinned checkout is missing; run scripts/install-rust-skills.sh"
  actual=$(git -C "$install_dir" rev-parse HEAD 2>/dev/null) \
    || fail "installed checkout has no readable HEAD"
  [[ "$actual" == "$commit" ]] \
    || fail "installed commit is $actual, expected $commit; reinstall it"
  metadata
}

case ${1:-install} in
  --metadata)
    metadata
    exit 0
    ;;
  --check)
    check_install
    exit 0
    ;;
  install)
    ;;
  *)
    echo "usage: scripts/install-rust-skills.sh [install|--check|--metadata]" >&2
    exit 2
    ;;
esac

if [[ -d "$install_dir/.git" ]] && \
   [[ "$(git -C "$install_dir" rev-parse HEAD 2>/dev/null || true)" == "$commit" ]]; then
  check_install
  exit 0
fi

command -v git >/dev/null 2>&1 || fail "git is required"
mkdir -p "$(dirname "$install_dir")"
tmp=$(mktemp -d "${TMPDIR:-/tmp}/rust-skills.XXXXXX")
cleanup() {
  [[ -z "${tmp:-}" ]] || rm -rf "$tmp"
}
trap cleanup EXIT

git init -q "$tmp"
git -C "$tmp" remote add origin "$repository"
git -C "$tmp" fetch -q --depth 1 origin "$commit"
git -C "$tmp" checkout -q --detach FETCH_HEAD
actual=$(git -C "$tmp" rev-parse HEAD)
[[ "$actual" == "$commit" ]] || fail "fetched $actual, expected $commit"

rm -rf "$install_dir"
mv "$tmp" "$install_dir"
tmp=""
check_install
