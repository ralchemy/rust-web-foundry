#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
manifest=${1:-"$repo_root/docs/agents/routed-context-budgets.tsv"}
routes="$repo_root/docs/agents/context-routes.tsv"
compiler="$repo_root/scripts/compile-agent-context.sh"
report_loader="$repo_root/scripts/load-review-reports.sh"
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

  local start="" end="" level="" matches=0 line_no=0
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
  [[ -f "$report_loader" ]] || fail "missing review report loader"

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
  ((root_bytes <= 4000)) || fail "root AGENTS.md is $root_bytes bytes; keep standing governance at or below 4000"
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

check_compiler_protocol() {
  local fixture="$work_dir/compiler-fixture"
  local unborn_pack="$work_dir/unborn-pack.md" base_pack="$work_dir/base-pack.md"
  local same_pack="$work_dir/same-pack.md"
  local extended_pack="$work_dir/extended-pack.md" deduped_pack="$work_dir/deduped-pack.md"
  local integrity_output="$work_dir/integrity.out"
  local tampered_pack="$work_dir/tampered-pack.md"
  local base_ref base_id same_id

  mkdir -p "$fixture/scripts" "$fixture/docs/agents" "$fixture/docs/guide"
  cp "$compiler" "$fixture/scripts/compile-agent-context.sh"
  printf '# Fixture rules\n' > "$fixture/AGENTS.md"
  printf '%s\n' \
    'governance|Changing governance|docs/guide/development.md#governance-and-documentation-changes' \
    'ports-adapters|Changing an Application Port|docs/guide/testing.md' \
    'infra-query|Changing an Infrastructure query|docs/guide/testing.md' \
    'testing-public-path|Changing a public check|docs/guide/testing.md' \
    'testing-sqlx-offline|Changing SQLx metadata|docs/guide/testing.md#sqlx-offline-is-a-compile-boundary' \
    > "$fixture/docs/agents/context-routes.tsv"
  printf '# Development\n\n## Governance and documentation changes\n\nFixture governance.\n' \
    > "$fixture/docs/guide/development.md"
  printf '# Testing\n\nFixture testing.\n\n## SQLx offline is a compile boundary\n\nFixture SQLx.\n' \
    > "$fixture/docs/guide/testing.md"
  printf 'tracked baseline\n' > "$fixture/tracked.txt"
  printf 'unstaged baseline\n' > "$fixture/unstaged.txt"

  git -C "$fixture" init -q
  bash "$fixture/scripts/compile-agent-context.sh" \
    --goal unborn --path tracked.txt --action governance --output "$unborn_pack" >/dev/null
  grep -Fqx -- '- generated_at_commit: working-tree' "$unborn_pack" \
    || fail "context compiler emitted an invalid unborn-HEAD marker"
  bash "$fixture/scripts/compile-agent-context.sh" --list-actions '^testing-' \
    | grep -Fq 'testing-sqlx-offline' \
    || fail "context compiler action filtering omitted a matching action"
  if bash "$fixture/scripts/compile-agent-context.sh" --list-actions | grep -Fq 'Changing '; then
    fail "unfiltered action listing emitted descriptions"
  fi
  if bash "$fixture/scripts/compile-agent-context.sh" --list-actions '^testing-' | grep -Fq governance; then
    fail "context compiler action filtering retained a nonmatching action"
  fi
  bash "$fixture/scripts/compile-agent-context.sh" --list-actions '^application$' \
    | grep -Fq 'ports-adapters' \
    || fail "context compiler did not map the Application layer alias"
  bash "$fixture/scripts/compile-agent-context.sh" --list-actions '^infrastructure$' \
    | grep -Fq 'infra-query' \
    || fail "context compiler did not map the Infrastructure layer alias"
  git -C "$fixture" add .
  git -C "$fixture" -c user.name=fixture -c user.email=fixture@example.invalid \
    -c commit.gpgSign=false -c core.hooksPath=/dev/null commit -qm baseline
  base_ref=$(git -C "$fixture" rev-parse HEAD)

  bash "$fixture/scripts/compile-agent-context.sh" \
    --goal fixture --path tracked.txt --action governance --output "$base_pack" >/dev/null
  bash "$fixture/scripts/compile-agent-context.sh" \
    --goal fixture --path tracked.txt --action governance --output "$base_pack" >/dev/null
  bash "$fixture/scripts/compile-agent-context.sh" \
    --goal fixture --path tracked.txt --action governance --output "$same_pack" >/dev/null
  base_id=$(sed -nE 's/^- pack_id: ([0-9a-f]+)$/\1/p' "$base_pack")
  same_id=$(sed -nE 's/^- pack_id: ([0-9a-f]+)$/\1/p' "$same_pack")
  [[ -n "$base_id" && "$base_id" == "$same_id" ]] || fail "context compiler does not produce stable pack IDs"

  if bash "$fixture/scripts/compile-agent-context.sh" \
    --goal changed --path tracked.txt --action governance --output "$base_pack" >/dev/null 2>&1; then
    fail "context compiler overwrote an immutable pack"
  fi

  printf 'committed change\n' > "$fixture/committed.txt"
  git -C "$fixture" add committed.txt
  git -C "$fixture" -c user.name=fixture -c user.email=fixture@example.invalid \
    -c commit.gpgSign=false -c core.hooksPath=/dev/null commit -qm committed-change
  printf 'staged change\n' >> "$fixture/tracked.txt"
  git -C "$fixture" add tracked.txt
  printf 'unstaged change\n' >> "$fixture/unstaged.txt"
  printf 'untracked change\n' > "$fixture/untracked.txt"

  bash "$fixture/scripts/compile-agent-context.sh" \
    --goal extended --extend-from "$base_pack" \
    --path committed.txt --path unstaged.txt --path untracked.txt \
    --action testing-public-path --output "$extended_pack" >/dev/null
  bash "$fixture/scripts/compile-agent-context.sh" \
    --verify-pack "$extended_pack" --base "$base_ref" \
    --action governance --action testing-public-path >/dev/null
  bash "$fixture/scripts/compile-agent-context.sh" --verify-pack "$extended_pack" > "$integrity_output"
  grep -Fq 'coverage=not-checked' "$integrity_output" \
    || fail "context compiler reported empty requested scope as coverage"

  bash "$fixture/scripts/compile-agent-context.sh" \
    --goal deduped --path tracked.txt \
    --action testing-public-path --action testing-sqlx-offline \
    --output "$deduped_pack" >/dev/null
  [[ "$(grep -Fc "## Source: \`docs/guide/testing.md\`" "$deduped_pack")" -eq 1 ]] \
    || fail "context compiler did not emit one whole-file testing source"
  if grep -Fq 'docs/guide/testing.md#sqlx-offline-is-a-compile-boundary@' "$deduped_pack"; then
    fail "context compiler retained an anchor subsumed by a whole-file source"
  fi

  if bash "$fixture/scripts/compile-agent-context.sh" \
    --verify-pack "$base_pack" --base "$base_ref" --action governance >/dev/null 2>&1; then
    fail "context verification accepted missing changed paths"
  fi
  if bash "$fixture/scripts/compile-agent-context.sh" \
    --verify-pack "$base_pack" --action testing-public-path >/dev/null 2>&1; then
    fail "context verification accepted a missing action"
  fi

  cp "$extended_pack" "$tampered_pack"
  printf 'tampered\n' >> "$tampered_pack"
  if bash "$fixture/scripts/compile-agent-context.sh" --verify-pack "$tampered_pack" >/dev/null 2>&1; then
    fail "context verification accepted a tampered pack"
  fi

  printf 'stale\n' >> "$fixture/AGENTS.md"
  if bash "$fixture/scripts/compile-agent-context.sh" --verify-pack "$extended_pack" >/dev/null 2>&1; then
    fail "context verification accepted a stale source"
  fi
}

check_review_loader() {
  local standards="$work_dir/standards.md" spec="$work_dir/spec.md"
  local mismatch="$work_dir/mismatch.md" empty="$work_dir/empty.md" output="$work_dir/reports.out"

  printf 'batchId=batch-1  \nsnapshotId=snapshot-1\n\n## Review\n- Standards finding.\n' > "$standards"
  printf 'batchId=batch-1\nsnapshotId=snapshot-1  \n\n## Review\n- Spec result.\n' > "$spec"
  bash "$report_loader" batch-1 snapshot-1 "$standards" "$spec" > "$output"
  grep -Fq -- '- Standards finding.' "$output" || fail "review loader omitted the Standards report"
  grep -Fq -- '- Spec result.' "$output" || fail "review loader omitted the Spec report"
  [[ "$(grep -Fc 'reports_loaded: 2/2' "$output")" -eq 1 ]] || fail "review loader emitted an ambiguous success marker"
  tail -n 1 "$output" | grep -Fqx 'reports_loaded: 2/2 batchId=batch-1 snapshotId=snapshot-1' \
    || fail "review loader did not finish with the 2/2 marker"

  if bash "$report_loader" batch-1 snapshot-1 "$standards" "$work_dir/missing.md" >/dev/null 2>&1; then
    fail "review loader accepted a missing report"
  fi
  if bash "$report_loader" batch-1 snapshot-1 "$standards" "$standards" >/dev/null 2>&1; then
    fail "review loader accepted the same report twice"
  fi

  printf 'batchId=batch-1\nsnapshotId=snapshot-2\n\n## Review\n- Mismatch.\n' > "$mismatch"
  if bash "$report_loader" batch-1 snapshot-1 "$standards" "$mismatch" >/dev/null 2>&1; then
    fail "review loader accepted mismatched markers"
  fi

  printf 'batchId=batch-1\nsnapshotId=snapshot-1\n\n## Review\n\n' > "$empty"
  if bash "$report_loader" batch-1 snapshot-1 "$standards" "$empty" >/dev/null 2>&1; then
    fail "review loader accepted an empty Review body"
  fi
}

add_budget_source() {
  local scenario=$1 source=$2
  if ! grep -Fxq "$source" "$work_dir/$scenario.sources"; then
    printf '%s\n' "$source" >> "$work_dir/$scenario.sources"
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

  local failed=0 limit total source relative scenario_sources whole_sources
  while IFS= read -r scenario; do
    limit=$(cat "$work_dir/$scenario.limit")
    total=0
    scenario_sources="$work_dir/$scenario.sources"
    whole_sources="$work_dir/$scenario.whole-sources"
    awk 'index($0, "#") == 0' "$scenario_sources" > "$whole_sources"
    while IFS= read -r source; do
      relative=${source%%#*}
      if [[ "$source" == *"#"* ]] && grep -Fxq "$relative" "$whole_sources"; then
        continue
      fi
      total=$((total + $(source_bytes "$source")))
    done < "$scenario_sources"
    printf 'routed context %-24s %6d / %6d bytes\n' "$scenario" "$total" "$limit"
    if ((total > limit)); then failed=1; fi
  done < "$work_dir/scenarios"
  ((failed == 0)) || fail "one or more routed-context budgets were exceeded"
}

check_route_manifest
check_skill_protocol
check_standing_briefs
check_large_owner_anchors
check_compiler_protocol
check_review_loader
check_budgets
