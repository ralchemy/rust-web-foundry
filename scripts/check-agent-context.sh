#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
manifest=${1:-"$repo_root/docs/agents/routed-context-budgets.tsv"}
work_dir=$(mktemp -d)
trap 'rm -rf "$work_dir"' EXIT

fail() {
  echo "agent context check: $*" >&2
  exit 1
}

slugify_heading() {
  local heading=$1
  printf '%s' "$heading" \
    | tr '[:upper:]' '[:lower:]' \
    | sed -E 's/[`*_~]//g; s/<[^>]+>//g; s/[^[:alnum:] _-]//g; s/[[:space:]]+/-/g; s/-+/-/g; s/^-//; s/-$//'
}

section_bytes() {
  local relative=$1
  local anchor=$2
  local file="$repo_root/$relative"
  [[ -f "$file" ]] || fail "missing routed owner: $relative"

  local start=
  local end=
  local level=
  local matches=0
  local line_no=0
  local line hashes heading slug current_level

  while IFS= read -r line || [[ -n "$line" ]]; do
    ((line_no += 1))
    if [[ "$line" =~ ^(#{1,6})[[:space:]]+(.+)$ ]]; then
      hashes=${BASH_REMATCH[1]}
      heading=${BASH_REMATCH[2]}
      current_level=${#hashes}
      slug=$(slugify_heading "$heading")

      if [[ "$slug" == "$anchor" ]]; then
        ((matches += 1))
        if (( matches == 1 )); then
          start=$line_no
          level=$current_level
        fi
      elif [[ -n "$start" && -z "$end" && $current_level -le $level ]]; then
        end=$((line_no - 1))
      fi
    fi
  done < "$file"

  (( matches == 1 )) || fail "$relative#$anchor resolved $matches headings"
  [[ -n "$end" ]] || end=$line_no
  sed -n "${start},${end}p" "$file" | wc -c | tr -d '[:space:]'
}

source_bytes() {
  local source=$1
  if [[ "$source" == *"#"* ]]; then
    section_bytes "${source%%#*}" "${source#*#}"
  else
    local file="$repo_root/$source"
    [[ -f "$file" ]] || fail "missing routed source: $source"
    wc -c < "$file" | tr -d '[:space:]'
  fi
}

check_skill_protocol() {
  local skill found=0
  while IFS= read -r -d '' skill; do
    found=1
    grep -Fq 'Context Pointers' "$skill" \
      || fail "${skill#"$repo_root/"} does not route through Context Pointers"
    grep -Fq 'Context Set' "$skill" \
      || fail "${skill#"$repo_root/"} does not maintain a Context Set"
  done < <(find "$repo_root/.agents/skills" -type f -name SKILL.md -print0)
  (( found == 1 )) || fail "no repository Skills found"
}

check_large_owner_anchors() {
  local briefs=(
    "$repo_root/AGENTS.md"
    "$repo_root/app/AGENTS.md"
    "$repo_root/crates/application/AGENTS.md"
    "$repo_root/crates/domain/AGENTS.md"
    "$repo_root/crates/http/AGENTS.md"
    "$repo_root/crates/infrastructure/AGENTS.md"
  )
  local skill owner relative needle bytes
  while IFS= read -r -d '' skill; do
    briefs[${#briefs[@]}]=$skill
  done < <(find "$repo_root/.agents/skills" -type f -name SKILL.md -print0)

  while IFS= read -r -d '' owner; do
    bytes=$(wc -c < "$owner" | tr -d '[:space:]')
    (( bytes >= 7500 )) || continue
    relative=${owner#"$repo_root/"}
    needle="\`$relative\`"
    if grep -nF "$needle" "${briefs[@]}"; then
      fail "$relative is $bytes bytes and must be routed through an anchor"
    fi
  done < <(find "$repo_root/docs/agents" "$repo_root/docs/guide" -type f -name '*.md' -print0)
}

check_budgets() {
  [[ -f "$manifest" ]] || fail "missing budget manifest: ${manifest#"$repo_root/"}"

  local scenario max_bytes source line_no=0 bytes current
  while IFS='|' read -r scenario max_bytes source || [[ -n "${scenario}${max_bytes}${source}" ]]; do
    ((line_no += 1))
    [[ -z "$scenario" || "$scenario" == \#* ]] && continue
    [[ "$scenario" =~ ^[A-Za-z0-9_-]+$ ]] || fail "invalid scenario name on row $line_no"
    [[ -n "$max_bytes" && -n "$source" ]] || fail "invalid manifest row $line_no"
    [[ "$max_bytes" =~ ^[0-9]+$ ]] || fail "non-numeric budget on row $line_no"

    if [[ ! -f "$work_dir/$scenario.limit" ]]; then
      printf '%s\n' "$max_bytes" > "$work_dir/$scenario.limit"
      printf '0\n' > "$work_dir/$scenario.total"
      : > "$work_dir/$scenario.sources"
      printf '%s\n' "$scenario" >> "$work_dir/scenarios"
    elif [[ "$(cat "$work_dir/$scenario.limit")" != "$max_bytes" ]]; then
      fail "scenario $scenario has inconsistent ceilings"
    fi

    if ! grep -Fxq "$source" "$work_dir/$scenario.sources"; then
      printf '%s\n' "$source" >> "$work_dir/$scenario.sources"
      bytes=$(source_bytes "$source")
      current=$(cat "$work_dir/$scenario.total")
      printf '%s\n' "$((current + bytes))" > "$work_dir/$scenario.total"
    fi
  done < "$manifest"

  [[ -s "$work_dir/scenarios" ]] || fail "budget manifest contains no scenarios"

  local failed=0 limit total
  while IFS= read -r scenario; do
    limit=$(cat "$work_dir/$scenario.limit")
    total=$(cat "$work_dir/$scenario.total")
    printf 'routed context %-24s %6d / %6d bytes\n' "$scenario" "$total" "$limit"
    if (( total > limit )); then
      failed=1
    fi
  done < "$work_dir/scenarios"
  (( failed == 0 )) || fail "one or more routed-context budgets were exceeded"
}

check_skill_protocol
check_large_owner_anchors
check_budgets
