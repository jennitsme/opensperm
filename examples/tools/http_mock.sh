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
# Mock response
cat <<JSON
{"url": "${url}", "status": 200, "body": "mocked"}
JSON
