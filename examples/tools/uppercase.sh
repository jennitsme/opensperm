#!/usr/bin/env bash
set -euo pipefail
input=$(cat -)
text=$(python - <<'PY'
import sys, json
try:
    data=json.load(sys.stdin)
    print(data.get("text",""))
except Exception:
    print("")
PY
)
printf '%s' "${text^^}"
