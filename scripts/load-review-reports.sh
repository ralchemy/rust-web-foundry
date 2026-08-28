#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo 'usage: scripts/load-review-reports.sh <batch-id> <snapshot-id> <standards-report> <spec-report>' >&2
}

fail() {
  echo "review reports: $*" >&2
  exit 1
}

[[ $# -eq 4 ]] || { usage; exit 2; }
batch_id=$1
snapshot_id=$2
standards_report=$3
spec_report=$4

[[ "$batch_id" =~ ^[A-Za-z0-9._:-]+$ ]] || fail 'invalid batch id'
[[ "$snapshot_id" =~ ^[A-Za-z0-9._:-]+$ ]] || fail 'invalid snapshot id'

for report in "$standards_report" "$spec_report"; do
  [[ -f "$report" && -r "$report" ]] || fail "missing or unreadable report: $report"
done
[[ ! "$standards_report" -ef "$spec_report" ]] || fail 'Standards and Spec reports must be different files'

validate_report() {
  local axis=$1 report=$2 actual_batch actual_snapshot
  actual_batch=$(sed -n '1 { s/[[:space:]]*$//; p; }' "$report")
  actual_snapshot=$(sed -n '2 { s/[[:space:]]*$//; p; }' "$report")
  [[ "$actual_batch" == "batchId=$batch_id" ]] || fail "$axis report has the wrong batchId"
  [[ "$actual_snapshot" == "snapshotId=$snapshot_id" ]] || fail "$axis report has the wrong snapshotId"
  awk '
    /^## Review[[:space:]]*$/ { review = 1; next }
    review && $0 !~ /^[[:space:]]*$/ { body = 1; exit }
    END { exit !(review && body) }
  ' "$report" || fail "$axis report has no Review body"
}

validate_report Standards "$standards_report"
validate_report Spec "$spec_report"

printf '%s\n' '=== Standards report ==='
cat "$standards_report"
printf '\n%s\n' '=== Spec report ==='
cat "$spec_report"
printf '\nreports_loaded: 2/2 batchId=%s snapshotId=%s\n' "$batch_id" "$snapshot_id"
