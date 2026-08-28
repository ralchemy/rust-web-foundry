#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
manifest=${1:-"$repo_root/docs/agents/routed-context-budgets.tsv"}
routes="$repo_root/docs/agents/context-routes.tsv"
compiler="$repo_root/scripts/compile-agent-context.sh"
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

  local start= end= level= matches=0 line_no=0
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
        if ((matches == 1)); then
          start=$line_no
          level=$current_level
        fi
      elif [[ -n "$start" && -z "$end" && $current_level -le $level ]]; then
        end=$((line_no - 1))
      fi
    fi
  done < "$file"

  ((matches == 1)) || fail "$relative#$anchor resolved $matches headings"
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

check_route_manifest() {
  [[ -f "$routes" ]] || fail "missing context route manifest"
  [[ -f "$compiler" ]] || fail "missing context compiler"

  local action when source line_no=0
  : > "$work_dir/actions"
  while IFS='|' read -r action when source || [[ -n "${action}${when}${source}" ]]; do
    ((line_no += 1))
    [[ -z "$action" || "$action" == \#* ]] && continue
    [[ "$action" =~ ^[a-z0-9][a-z0-9-]*$ ]] || fail "invalid action key on route row $line_no: $action"
    [[ -n "$when" && -n "$source" ]] || fail "invalid route row $line_no"
    if grep -Fxq "$action" "$work_dir/actions"; then
      fail "duplicate action key: $action"
    fi
    printf '%s\n' "$action" >> "$work_dir/actions"
    source_bytes "$source" >/dev/null
  done < "$routes"
  [[ -s "$work_dir/actions" ]] || fail "context route manifest contains no actions"
}

check_skill_protocol() {
  local skill found=0
  while IFS= read -r -d '' skill; do
    found=1
    grep -Fq 'Context Pack' "$skill" \
      || fail "${skill#"$repo_root/"} does not consume the Context Pack protocol"
    if grep -Eq 'docs/(guide|agents/domain\.md)' "$skill"; then
      fail "${skill#"$repo_root/"} directly names a conditional owner; use action keys and the Context Pack"
    fi
    if grep -Fq 'Context Pointers' "$skill"; then
      fail "${skill#"$repo_root/"} still references the retired Context Pointer protocol"
    fi
  done < <(find "$repo_root/.agents/skills" -type f -name SKILL.md -print0)
  ((found == 1)) || fail "no repository Skills found"
}

check_standing_briefs() {
  local brief
  for brief in \
    "$repo_root/AGENTS.md" \
    "$repo_root/app/AGENTS.md" \
    "$repo_root/crates/application/AGENTS.md" \
    "$repo_root/crates/domain/AGENTS.md" \
    "$repo_root/crates/http/AGENTS.md" \
    "$repo_root/crates/infrastructure/AGENTS.md"; do
    [[ -f "$brief" ]] || fail "missing standing brief: ${brief#"$repo_root/"}"
    if grep -Fq '→ read' "$brief"; then
      fail "${brief#"$repo_root/"} contains conditional routing; routes belong in docs/agents/context-routes.tsv"
    fi
  done

  local root_bytes
  root_bytes=$(wc -c < "$repo_root/AGENTS.md" | tr -d '[:space:]')
  ((root_bytes <= 6000)) || fail "root AGENTS.md is $root_bytes bytes; keep standing governance at or below 6000"
}

check_large_owner_anchors() {
  local owner relative bytes source
  while IFS= read -r -d '' owner; do
    bytes=$(wc -c < "$owner" | tr -d '[:space:]')
    ((bytes >= 7500)) || continue
    relative=${owner#"$repo_root/"}
    while IFS='|' read -r _ _ source || [[ -n "$source" ]]; do
      [[ -z "$source" || "$source" == \#* ]] && continue
      if [[ "${source%%#*}" == "$relative" && "$source" != *"#"* ]]; then
        fail "$relative is $bytes bytes and context routes must reference it through an anchor"
      fi
    done < "$routes"
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
    if ((total > limit)); then failed=1; fi
  done < "$work_dir/scenarios"
  ((failed == 0)) || fail "one or more routed-context budgets were exceeded"
}

check_route_manifest
check_skill_protocol
check_standing_briefs
check_large_owner_anchors
check_budgets
