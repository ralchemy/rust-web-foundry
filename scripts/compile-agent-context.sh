#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
routes="$repo_root/docs/agents/context-routes.tsv"
output="$repo_root/.scratch/context-pack.md"
goal=""
max_bytes=${AGENT_CONTEXT_MAX_BYTES:-60000}
declare -a paths=()
declare -a actions=()

usage() {
  cat <<'EOF'
usage: scripts/compile-agent-context.sh [options]

  --goal TEXT       task goal recorded in the pack
  --path PATH       planned touched path; repeat as needed
  --action KEY      action key from docs/agents/context-routes.tsv; repeat as needed
  --output PATH     output path (default .scratch/context-pack.md)
  --max-bytes N     hard UTF-8 byte ceiling (default 60000)
  --list-actions    print known action keys and descriptions
EOF
}

list_actions() {
  awk -F'|' '!/^#/ && NF >= 3 { printf "%-30s %s\n", $1, $2 }' "$routes"
}

while (($#)); do
  case "$1" in
    --goal) goal=${2:?missing goal}; shift 2 ;;
    --path) paths+=("${2:?missing path}"); shift 2 ;;
    --action) actions+=("${2:?missing action}"); shift 2 ;;
    --output) output=${2:?missing output}; shift 2 ;;
    --max-bytes) max_bytes=${2:?missing max bytes}; shift 2 ;;
    --list-actions) list_actions; exit 0 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
  esac
done

[[ -f "$routes" ]] || { echo "missing route manifest: $routes" >&2; exit 1; }
[[ "$max_bytes" =~ ^[0-9]+$ ]] || { echo "max bytes must be numeric" >&2; exit 2; }
((${#paths[@]} > 0)) || { echo "at least one --path is required" >&2; exit 2; }

normalize_path() {
  local p=${1#./}
  [[ "$p" != /* ]] || { echo "paths must be repository-relative: $1" >&2; exit 2; }
  [[ "$p" != *".."* ]] || { echo "paths must not contain '..': $1" >&2; exit 2; }
  printf '%s' "$p"
}

standing_for_path() {
  local p=$1
  case "$p" in
    app/*|app) printf '%s\n' app/AGENTS.md ;;
    crates/domain/*|crates/domain) printf '%s\n' crates/domain/AGENTS.md ;;
    crates/application/*|crates/application) printf '%s\n' crates/application/AGENTS.md ;;
    crates/http/*|crates/http) printf '%s\n' crates/http/AGENTS.md ;;
    crates/infrastructure/*|crates/infrastructure) printf '%s\n' crates/infrastructure/AGENTS.md ;;
  esac
}

slugify_heading() {
  printf '%s' "$1" \
    | tr '[:upper:]' '[:lower:]' \
    | sed -E 's/[`*_~]//g; s/<[^>]+>//g; s/[^[:alnum:] _-]//g; s/[[:space:]]+/-/g; s/-+/-/g; s/^-//; s/-$//'
}

emit_source() {
  local source=$1
  local relative=${source%%#*}
  local anchor=""
  [[ "$source" == *"#"* ]] && anchor=${source#*#}
  local file="$repo_root/$relative"
  [[ -f "$file" ]] || { echo "missing context source: $source" >&2; exit 1; }

  printf '\n## Source: `%s`\n\n' "$source"
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
  ((matches == 1)) || { echo "$source resolved $matches headings" >&2; exit 1; }
  [[ -n "$end" ]] || end=$line_no
  sed -n "${start},${end}p" "$file"
}

mkdir -p "$(dirname "$output")"
tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT
sources=$(mktemp)
trap 'rm -f "$tmp" "$sources"' EXIT

printf '%s\n' AGENTS.md > "$sources"
for raw in "${paths[@]}"; do
  p=$(normalize_path "$raw")
  standing_for_path "$p" >> "$sources"
done

for action in "${actions[@]}"; do
  matches=$(awk -F'|' -v key="$action" '!/^#/ && $1 == key { print $3 }' "$routes")
  [[ -n "$matches" ]] || { echo "unknown action key: $action" >&2; exit 2; }
  printf '%s\n' "$matches" >> "$sources"
done

sort -u "$sources" -o "$sources"

{
  echo '# Compiled Agent Context Pack'
  echo
  printf -- '- goal: %s\n' "${goal:-unspecified}"
  printf -- '- generated_at_commit: %s\n' "$(git -C "$repo_root" rev-parse HEAD 2>/dev/null || echo working-tree)"
  printf -- '- max_bytes: %s\n' "$max_bytes"
  echo '- planned_paths:'
  for raw in "${paths[@]}"; do printf '  - `%s`\n' "$(normalize_path "$raw")"; done
  echo '- actions:'
  if ((${#actions[@]} == 0)); then echo '  - none'; else for action in "${actions[@]}"; do printf '  - `%s`\n' "$action"; done; fi
  echo '- context_set:'
  while IFS= read -r source; do
    relative=${source%%#*}
    sha=$(git -C "$repo_root" hash-object "$relative")
    printf '  - `%s@%s`\n' "$source" "$sha"
  done < "$sources"
  echo
  echo '> This file is a generated read view. Source files remain authoritative; do not edit this pack.'
  while IFS= read -r source; do emit_source "$source"; done < "$sources"
} > "$tmp"

bytes=$(wc -c < "$tmp" | tr -d '[:space:]')
if ((bytes > max_bytes)); then
  echo "compiled context is $bytes bytes, above $max_bytes; narrow paths/actions or split the task" >&2
  exit 1
fi
printf '\n<!-- compiled_bytes: %s -->\n' "$bytes" >> "$tmp"
mv "$tmp" "$output"
trap - EXIT
rm -f "$sources"
printf 'compiled agent context: %s bytes -> %s\n' "$bytes" "${output#"$repo_root/"}"
