#!/usr/bin/env bash
set -euo pipefail

source_root=crates/infrastructure/src

forbidden=$(
  grep -RInE \
    'sqlx::(query|query_as|query_scalar)[[:space:]]*\(|MySqlRow|sqlx::Row|Row::(get|try_get)|\.try_get[[:space:]]*\(|SELECT[[:space:]]+\*' \
    "$source_root" \
    --include='*.rs' || true
)

if [[ -n "$forbidden" ]]; then
  echo "Infrastructure production code must use checked SQLx macros, explicit columns, and typed private rows:" >&2
  echo "$forbidden" >&2
  exit 1
fi

if grep -RInE '#\[derive\([^]]*sqlx::FromRow' "$source_root" --include='*.rs'; then
  echo "query_as! row types must not derive sqlx::FromRow merely to preserve an unchecked path" >&2
  exit 1
fi
