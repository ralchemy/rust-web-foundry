#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
routes="$repo_root/docs/agents/context-routes.tsv"
output=""
output_set=0
goal=""
max_bytes=${AGENT_CONTEXT_MAX_BYTES:-48000}
extend_from=""
verify_pack=""
base_ref=""
declare -a paths=()
declare -a actions=()

usage() {
  cat <<'EOF'
usage: scripts/compile-agent-context.sh [options]

  --goal TEXT         task goal recorded in the pack
  --path PATH         planned or final touched path; repeat as needed
  --action KEY        action key from docs/agents/context-routes.tsv; repeat as needed
  --output PATH       explicit immutable output path
  --max-bytes N       hard UTF-8 byte ceiling (default 48000)
  --extend-from PACK  inherit paths/actions from an earlier pack
  --verify-pack PACK  verify identity, freshness, and declared coverage
  --base REF          with --verify-pack, add changed and untracked paths since REF
  --list-actions [ERE]  print keys; with ERE, print matching key/layer details
EOF
}

fail() {
  echo "agent context: $*" >&2
  exit 1
}

list_actions() {
  local filter=${1:-}
  awk -F'|' -v filter="$filter" '
    BEGIN { filter = tolower(filter) }
    !/^#/ && NF >= 3 {
      key = tolower($1)
      if (filter == "") {
        print $1
        next
      }
      layer = ""
      if (key ~ /^host-/) layer = "app"
      else if (key ~ /^ports-/) layer = "application"
      else if (key ~ /^infra-/) layer = "infrastructure"
      if (key ~ filter || layer ~ filter) printf "%-30s %s\n", $1, $2
    }
  ' "$routes"
}

resolve_file() {
  if [[ "$1" == /* ]]; then printf '%s' "$1"; else printf '%s/%s' "$repo_root" "${1#./}"; fi
}

normalize_path() {
  local p=${1#./}
  [[ "$p" != /* ]] || fail "paths must be repository-relative: $1"
  [[ "$p" != *".."* ]] || fail "paths must not contain '..': $1"
  printf '%s' "$p"
}

read_pack_section() {
  local pack=$1 section=$2
  awk -v marker="- $section:" '
    $0 == marker { inside = 1; next }
    inside && ($0 == "" || $0 ~ /^- /) { exit }
    inside && $0 ~ /^  - `/ {
      line = $0
      sub(/^  - `/, "", line)
      sub(/`[[:space:]]*$/, "", line)
      print line
    }
  ' "$pack"
}

pack_identity() {
  sed -E \
    -e 's/^- pack_id: [0-9a-f]+$/- pack_id: __PACK_ID__/' \
    -e '/^<!-- compiled_bytes: [0-9]+ -->$/d' \
    "$1" | git -C "$repo_root" hash-object --stdin
}

recorded_pack_id() {
  sed -nE 's/^- pack_id: ([0-9a-f]+)$/\1/p' "$1"
}

verify_identity() {
  local pack=$1 recorded computed count
  count=$(grep -Ec '^- pack_id: [0-9a-f]+$' "$pack" || true)
  ((count == 1)) || fail "pack must contain exactly one pack_id: $pack"
  recorded=$(recorded_pack_id "$pack")
  computed=$(pack_identity "$pack")
  [[ "$recorded" == "$computed" ]] || fail "pack identity mismatch: $pack"
}

path_is_covered() {
  local requested=$1 planned
  while IFS= read -r planned; do
    [[ -n "$planned" ]] || continue
    if [[ "$requested" == "$planned" || "$requested" == "$planned/"* ]]; then return 0; fi
  done < "$work_dir/pack-paths"
  return 1
}

verify_context_pack() {
  local pack=$1 entry source relative expected current requested action
  local recorded_bytes actual_bytes path_count=0 action_count=0 source_count=0

  [[ -f "$pack" ]] || fail "missing pack: $pack"
  verify_identity "$pack"

  recorded_bytes=$(sed -nE 's/^<!-- compiled_bytes: ([0-9]+) -->$/\1/p' "$pack")
  [[ "$recorded_bytes" =~ ^[0-9]+$ ]] || fail "pack has no valid compiled_bytes: $pack"
  actual_bytes=$(sed -E '/^<!-- compiled_bytes: [0-9]+ -->$/d' "$pack" | wc -c | tr -d '[:space:]')
  [[ "$recorded_bytes" == "$actual_bytes" ]] || fail "compiled byte count mismatch: $pack"

  read_pack_section "$pack" planned_paths | sort -u > "$work_dir/pack-paths"
  read_pack_section "$pack" actions | grep -Fxv none | sort -u > "$work_dir/pack-actions" || true
  read_pack_section "$pack" context_set > "$work_dir/pack-context"
  [[ -s "$work_dir/pack-paths" ]] || fail "pack contains no planned paths: $pack"
  [[ -s "$work_dir/pack-context" ]] || fail "pack contains no Context Set: $pack"

  : > "$work_dir/requested-paths"
  for requested in "${paths[@]}"; do normalize_path "$requested" >> "$work_dir/requested-paths"; echo >> "$work_dir/requested-paths"; done
  if [[ -n "$base_ref" ]]; then
    git -C "$repo_root" rev-parse --verify "$base_ref^{commit}" >/dev/null 2>&1 \
      || fail "unknown base ref: $base_ref"
    git -C "$repo_root" diff --name-only --diff-filter=ACDMRTUXB "$base_ref" -- >> "$work_dir/requested-paths"
    git -C "$repo_root" ls-files --others --exclude-standard >> "$work_dir/requested-paths"
  fi
  sort -u "$work_dir/requested-paths" -o "$work_dir/requested-paths"

  while IFS= read -r requested; do
    [[ -n "$requested" ]] || continue
    path_is_covered "$requested" || fail "pack does not cover path: $requested"
    ((path_count += 1))
  done < "$work_dir/requested-paths"

  : > "$work_dir/requested-actions"
  for action in "${actions[@]}"; do
    awk -F'|' -v key="$action" '!/^#/ && $1 == key { found = 1 } END { exit !found }' "$routes" \
      || fail "unknown action key: $action"
    printf '%s\n' "$action" >> "$work_dir/requested-actions"
  done
  sort -u "$work_dir/requested-actions" -o "$work_dir/requested-actions"
  while IFS= read -r action; do
    [[ -n "$action" ]] || continue
    grep -Fxq "$action" "$work_dir/pack-actions" || fail "pack does not cover action: $action"
    ((action_count += 1))
  done < "$work_dir/requested-actions"

  while IFS= read -r entry; do
    [[ "$entry" == *@* ]] || fail "invalid Context Set entry: $entry"
    expected=${entry##*@}
    source=${entry%@*}
    relative=${source%%#*}
    [[ -f "$repo_root/$relative" ]] || fail "missing context source: $source"
    current=$(git -C "$repo_root" hash-object "$relative")
    [[ "$current" == "$expected" ]] || fail "stale context source: $source"
    ((source_count += 1))
  done < "$work_dir/pack-context"

  if ((path_count == 0 && action_count == 0)); then
    printf 'context_integrity: sources=%d pack_id=%s coverage=not-checked\n' \
      "$source_count" "$(recorded_pack_id "$pack")"
  else
    printf 'context_coverage: 100%% paths=%d/%d actions=%d/%d sources=%d pack_id=%s\n' \
      "$path_count" "$path_count" "$action_count" "$action_count" "$source_count" "$(recorded_pack_id "$pack")"
  fi
}

slugify_heading() {
  printf '%s' "$1" \
    | tr '[:upper:]' '[:lower:]' \
    | sed -E 's/[`*_~]//g; s/<[^>]+>//g; s/[^[:alnum:] _-]//g; s/[[:space:]]+/-/g; s/-+/-/g; s/^-//; s/-$//'
}

standing_for_path() {
  local p=$1 dir
  if [[ -d "$repo_root/$p" ]]; then dir=$p; else dir=$(dirname "$p"); fi
  while [[ "$dir" != "." && "$dir" != "/" ]]; do
    if [[ -f "$repo_root/$dir/AGENTS.md" ]]; then
      printf '%s\n' "$dir/AGENTS.md"
      return
    fi
    dir=$(dirname "$dir")
  done
}

emit_source() {
  local source=$1
  local relative=${source%%#*}
  local anchor=""
  [[ "$source" == *"#"* ]] && anchor=${source#*#}
  local file="$repo_root/$relative"
  [[ -f "$file" ]] || fail "missing context source: $source"

  printf "\n## Source: \`%s\`\n\n" "$source"
  if [[ -z "$anchor" ]]; then
    cat "$file"
    return
  fi

  local start="" end="" level="" matches=0 line_no=0 line hashes heading slug current_level
  while IFS= read -r line || [[ -n "$line" ]]; do
    ((line_no += 1))
    if [[ "$line" =~ ^(#{1,6})[[:space:]]+(.+)$ ]]; then
      hashes=${BASH_REMATCH[1]}
      heading=${BASH_REMATCH[2]}
      current_level=${#hashes}
      slug=$(slugify_heading "$heading")
      if [[ "$slug" == "$anchor" ]]; then
        ((matches += 1))
        if ((matches == 1)); then start=$line_no; level=$current_level; fi
      elif [[ -n "$start" && -z "$end" && $current_level -le $level ]]; then
        end=$((line_no - 1))
      fi
    fi
  done < "$file"
  ((matches == 1)) || fail "$source resolved $matches headings"
  [[ -n "$end" ]] || end=$line_no
  sed -n "${start},${end}p" "$file"
}

while (($#)); do
  case "$1" in
    --goal) goal=${2:?missing goal}; shift 2 ;;
    --path) paths+=("${2:?missing path}"); shift 2 ;;
    --action) actions+=("${2:?missing action}"); shift 2 ;;
    --output) output=${2:?missing output}; output_set=1; shift 2 ;;
    --max-bytes) max_bytes=${2:?missing max bytes}; shift 2 ;;
    --extend-from) extend_from=${2:?missing pack}; shift 2 ;;
    --verify-pack) verify_pack=${2:?missing pack}; shift 2 ;;
    --base) base_ref=${2:?missing ref}; shift 2 ;;
    --list-actions)
      if (($# > 1)) && [[ "$2" != --* ]]; then list_actions "$2"; else list_actions; fi
      exit 0
      ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
  esac
done

[[ -f "$routes" ]] || fail "missing route manifest: $routes"
[[ "$max_bytes" =~ ^[0-9]+$ ]] || { echo "max bytes must be numeric" >&2; exit 2; }
work_dir=$(mktemp -d)
trap 'rm -rf "$work_dir"' EXIT

if [[ -n "$verify_pack" ]]; then
  [[ -z "$extend_from" && $output_set -eq 0 && -z "$goal" ]] \
    || { echo "--verify-pack cannot be combined with compile options" >&2; exit 2; }
  verify_context_pack "$(resolve_file "$verify_pack")"
  exit 0
fi
[[ -z "$base_ref" ]] || { echo "--base requires --verify-pack" >&2; exit 2; }

extend_id=""
if [[ -n "$extend_from" ]]; then
  extend_from=$(resolve_file "$extend_from")
  [[ -f "$extend_from" ]] || fail "missing extension pack: $extend_from"
  if grep -Eq '^- pack_id: [0-9a-f]+$' "$extend_from"; then
    verify_identity "$extend_from"
    extend_id=$(recorded_pack_id "$extend_from")
  else
    extend_id=$(git -C "$repo_root" hash-object "$extend_from")
  fi
  while IFS= read -r value; do paths+=("$value"); done < <(read_pack_section "$extend_from" planned_paths)
  while IFS= read -r value; do [[ "$value" == none ]] || actions+=("$value"); done < <(read_pack_section "$extend_from" actions)
fi

: > "$work_dir/paths"
for raw in "${paths[@]}"; do normalize_path "$raw" >> "$work_dir/paths"; echo >> "$work_dir/paths"; done
sort -u "$work_dir/paths" -o "$work_dir/paths"
paths=()
while IFS= read -r value; do [[ -n "$value" ]] && paths+=("$value"); done < "$work_dir/paths"
((${#paths[@]} > 0)) || { echo "at least one --path or --extend-from is required" >&2; exit 2; }

: > "$work_dir/actions"
for action in "${actions[@]}"; do printf '%s\n' "$action" >> "$work_dir/actions"; done
sort -u "$work_dir/actions" -o "$work_dir/actions"
actions=()
while IFS= read -r value; do [[ -n "$value" ]] && actions+=("$value"); done < "$work_dir/actions"

sources="$work_dir/sources"
printf '%s\n' AGENTS.md > "$sources"
for path in "${paths[@]}"; do standing_for_path "$path" >> "$sources"; done
for action in "${actions[@]}"; do
  matches=$(awk -F'|' -v key="$action" '!/^#/ && $1 == key { print $3 }' "$routes")
  [[ -n "$matches" ]] || { echo "unknown action key: $action" >&2; exit 2; }
  printf '%s\n' "$matches" >> "$sources"
done
sort -u "$sources" -o "$sources"
all_sources="$work_dir/all-sources"
whole_sources="$work_dir/whole-sources"
mv "$sources" "$all_sources"
awk 'index($0, "#") == 0' "$all_sources" > "$whole_sources"
: > "$sources"
while IFS= read -r source; do
  relative=${source%%#*}
  if [[ "$source" == *"#"* ]] && grep -Fxq "$relative" "$whole_sources"; then continue; fi
  printf '%s\n' "$source" >> "$sources"
done < "$all_sources"

tmp="$work_dir/pack-with-placeholder"
resolved="$work_dir/pack"
generated_at_commit=$(git -C "$repo_root" rev-parse --verify 'HEAD^{commit}' 2>/dev/null || echo working-tree)
{
  echo '# Compiled Agent Context Pack'
  echo
  printf -- '- pack_id: __PACK_ID__\n'
  printf -- '- goal: %s\n' "${goal:-unspecified}"
  printf -- '- generated_at_commit: %s\n' "$generated_at_commit"
  [[ -z "$extend_id" ]] || printf -- '- extends_pack: %s\n' "$extend_id"
  printf -- '- max_bytes: %s\n' "$max_bytes"
  echo '- planned_paths:'
  for path in "${paths[@]}"; do printf "  - \`%s\`\n" "$path"; done
  echo '- actions:'
  if ((${#actions[@]} == 0)); then echo '  - none'; else for action in "${actions[@]}"; do printf "  - \`%s\`\n" "$action"; done; fi
  echo '- context_set:'
  while IFS= read -r source; do
    relative=${source%%#*}
    sha=$(git -C "$repo_root" hash-object "$relative")
    printf "  - \`%s@%s\`\n" "$source" "$sha"
  done < "$sources"
  echo
  echo '> This file is a generated read view. Source files remain authoritative; do not edit this pack.'
  while IFS= read -r source; do emit_source "$source"; done < "$sources"
} > "$tmp"
printf '\n' >> "$tmp"

pack_id=$(pack_identity "$tmp")
sed "s/^- pack_id: __PACK_ID__$/- pack_id: $pack_id/" "$tmp" > "$resolved"
bytes=$(wc -c < "$resolved" | tr -d '[:space:]')
if ((bytes > max_bytes)); then
  fail "compiled context is $bytes bytes, above $max_bytes; narrow paths/actions or split the task"
fi
printf '<!-- compiled_bytes: %s -->\n' "$bytes" >> "$resolved"

if ((output_set == 0)); then output="$repo_root/.scratch/context-packs/$pack_id.md"; else output=$(resolve_file "$output"); fi
mkdir -p "$(dirname "$output")"
if [[ -e "$output" ]]; then
  cmp -s "$resolved" "$output" || fail "refusing to overwrite immutable pack: $output"
else
  mv "$resolved" "$output"
fi

display=$output
[[ "$display" == "$repo_root/"* ]] && display=${display#"$repo_root/"}
printf 'compiled agent context: pack_id=%s bytes=%s -> %s\n' "$pack_id" "$bytes" "$display"
