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

for legacy in \
  docs/agents \
  scripts/compile-agent-context.sh \
  scripts/load-review-reports.sh
do
  [[ ! -e "$repo_root/$legacy" ]] || fail "legacy generated-context artifact remains: $legacy"
done

skill="$repo_root/.agents/skills/review-rust-web/SKILL.md"
openai_policy="$repo_root/.agents/skills/review-rust-web/agents/openai.yaml"
[[ -f "$skill" ]] || fail "missing generated review Skill"
[[ -f "$openai_policy" ]] || fail "missing OpenAI review Skill invocation policy"

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
  || fail "review Skill must remain explicit-only for compatible hosts"
grep -Fq 'allow_implicit_invocation: false' "$openai_policy" \
  || fail "OpenAI review Skill policy must disable implicit invocation"
grep -Fq '.scratch/rust-skills/rules/' "$skill" \
  || fail "review Skill must select pinned Rust rules progressively"
grep -Fq '.agents/rust-skills-overrides.md' "$skill" \
  || fail "review Skill must apply project overrides"

metadata=$(bash "$repo_root/scripts/install-rust-skills.sh" --metadata)
repository=$(sed -nE 's/^repository=([^[:space:]]+)$/\1/p' "$repo_root/.agents/rust-skills.lock")
commit=$(sed -nE 's/^commit=([0-9a-f]{40})$/\1/p' "$repo_root/.agents/rust-skills.lock")
[[ -n "$repository" && -n "$commit" ]] || fail "invalid rust-skills lock metadata"
grep -Fq "repository=$repository" <<<"$metadata" || fail "installer repository does not match lock"
grep -Fq "commit=$commit" <<<"$metadata" || fail "installer commit does not match lock"

if grep -REiq \
  'compiled Context Pack|context-routes|routed-context-budget|nearest-local rules|add-endpoint.*review-pr' \
  "$repo_root/AGENTS.md" "$repo_root/docs/guide"; then
  fail "legacy agent orchestration language remains in generated guidance"
fi

version=$(sed -nE 's/^version=([^[:space:]]+)$/\1/p' "$repo_root/.agents/rust-skills.lock")
printf 'project_contract: code-first rust-skills=%s agents_bytes=%s\n' "$version" "$bytes"
