#!/usr/bin/env bash
set -euo pipefail
input=$(cat -)
url=$(python - <<'PY'
import sys, json
try:
    data=json.load(sys.stdin)
    print(data.get("url",""))
except Exception:
    print("")
PY
)
if [ -z "$url" ]; then
  echo "{}"
  exit 0
fi
curl -sL "$url"
