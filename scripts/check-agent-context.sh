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

standing_for_path() {
  local p=${1#./} dir
  printf '%s\n' AGENTS.md
  if [[ -d "$repo_root/$p" ]]; then dir=$p; else dir=$(dirname "$p"); fi
  while [[ "$dir" != "." && "$dir" != "/" ]]; do
    if [[ -f "$repo_root/$dir/AGENTS.md" ]]; then
      printf '%s\n' "$dir/AGENTS.md"
      return
    fi
    dir=$(dirname "$dir")
  done
}

route_source() {
  local action=$1
  awk -F'|' -v key="$action" '!/^#/ && $1 == key { print $3 }' "$routes"
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
  while IFS= read -r -d '' brief; do
    if grep -Fq '→ read' "$brief"; then
      fail "${brief#"$repo_root/"} contains conditional routing; routes belong in docs/agents/context-routes.tsv"
    fi
  done < <(find "$repo_root" -name AGENTS.md -type f -print0)

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

add_budget_source() {
  local scenario=$1 source=$2 bytes current
  if ! grep -Fxq "$source" "$work_dir/$scenario.sources"; then
    printf '%s\n' "$source" >> "$work_dir/$scenario.sources"
    bytes=$(source_bytes "$source")
    current=$(cat "$work_dir/$scenario.total")
    printf '%s\n' "$((current + bytes))" > "$work_dir/$scenario.total"
  fi
}

check_budgets() {
  [[ -f "$manifest" ]] || fail "missing budget manifest: ${manifest#"$repo_root/"}"

  local scenario max_bytes kind value line_no=0 source
  while IFS='|' read -r scenario max_bytes kind value || [[ -n "${scenario}${max_bytes}${kind}${value}" ]]; do
    ((line_no += 1))
    [[ -z "$scenario" || "$scenario" == \#* ]] && continue
    [[ "$scenario" =~ ^[A-Za-z0-9_-]+$ ]] || fail "invalid scenario name on row $line_no"
    [[ "$max_bytes" =~ ^[0-9]+$ ]] || fail "non-numeric budget on row $line_no"
    [[ "$kind" == path || "$kind" == action ]] || fail "invalid budget kind on row $line_no: $kind"
    [[ -n "$value" ]] || fail "missing budget value on row $line_no"

    if [[ ! -f "$work_dir/$scenario.limit" ]]; then
      printf '%s\n' "$max_bytes" > "$work_dir/$scenario.limit"
      printf '0\n' > "$work_dir/$scenario.total"
      : > "$work_dir/$scenario.sources"
      printf '%s\n' "$scenario" >> "$work_dir/scenarios"
    elif [[ "$(cat "$work_dir/$scenario.limit")" != "$max_bytes" ]]; then
      fail "scenario $scenario has inconsistent ceilings"
    fi

    if [[ "$kind" == path ]]; then
      while IFS= read -r source; do add_budget_source "$scenario" "$source"; done < <(standing_for_path "$value")
    else
      source=$(route_source "$value")
      [[ -n "$source" ]] || fail "budget scenario $scenario uses unknown action: $value"
      add_budget_source "$scenario" "$source"
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
