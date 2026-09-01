#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

fail() {
  echo "project contract: $*" >&2
  exit 1
}

[[ -f "$repo_root/AGENTS.md" ]] || fail "missing root AGENTS.md"
bytes=$(wc -c < "$repo_root/AGENTS.md" | tr -d '[:space:]')
((bytes <= 2000)) || fail "AGENTS.md is $bytes bytes; keep the standing contract below 2000"

nested=$(find "$repo_root/app" "$repo_root/crates" -type f -name AGENTS.md -print)
[[ -z "$nested" ]] \
  || fail "generated projects use one root contract; remove nested AGENTS.md files: $nested"

for legacy in \
  docs/agents \
  scripts/compile-agent-context.sh \
  scripts/load-review-reports.sh
do
  [[ ! -e "$repo_root/$legacy" ]] || fail "legacy generated-context artifact remains: $legacy"
done

skills=$(find "$repo_root/.agents/skills" -type f -name SKILL.md -print | sort)
skill_count=$(printf '%s\n' "$skills" | sed '/^$/d' | wc -l | tr -d '[:space:]')
((skill_count == 1)) || fail "expected exactly one generated review Skill, found $skill_count"
skill=$skills
[[ "$skill" == "$repo_root/.agents/skills/review-rust-web/SKILL.md" ]] \
  || fail "unexpected generated Skill: ${skill#"$repo_root/"}"

for required in \
  .agents/rust-skills.lock \
  .agents/rust-skills-overrides.md \
  docs/guide/stack.md \
  docs/guide/reviewing.md \
  scripts/install-rust-skills.sh
do
  [[ -f "$repo_root/$required" ]] || fail "missing $required"
done

grep -Fqx 'disable-model-invocation: true' "$skill" \
  || fail "review Skill must be explicit-only"
grep -Fq '.scratch/rust-skills/SKILL.md' "$skill" \
  || fail "review Skill must consume the pinned local rust-skills index"
grep -Fq '.agents/rust-skills-overrides.md' "$skill" \
  || fail "review Skill must apply project overrides"

metadata=$(bash "$repo_root/scripts/install-rust-skills.sh" --metadata)
repository=$(sed -nE 's/^repository=([^[:space:]]+)$/\1/p' "$repo_root/.agents/rust-skills.lock")
commit=$(sed -nE 's/^commit=([0-9a-f]{40})$/\1/p' "$repo_root/.agents/rust-skills.lock")
[[ -n "$repository" && -n "$commit" ]] || fail "invalid rust-skills lock metadata"
grep -Fq "repository=$repository" <<<"$metadata" || fail "installer repository does not match lock"
grep -Fq "commit=$commit" <<<"$metadata" || fail "installer commit does not match lock"

for file in \
  "$repo_root/AGENTS.md" \
  "$repo_root/docs/guide/README.md" \
  "$repo_root/docs/guide/development.md"
do
  if grep -Eiq 'compiled context pack|context-routes|routed-context-budget' "$file"; then
    fail "legacy implementation orchestration remains in ${file#"$repo_root/"}"
  fi
done

version=$(sed -nE 's/^version=([^[:space:]]+)$/\1/p' "$repo_root/.agents/rust-skills.lock")
printf 'project_contract: code-first rust-skills=%s agents_bytes=%s\n' "$version" "$bytes"
