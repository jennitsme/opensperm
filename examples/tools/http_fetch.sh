#!/usr/bin/env bash
set -euo pipefail
python - <<'PY'
import json, sys, urllib.request

def main():
    try:
        data = json.load(sys.stdin)
    except Exception:
        data = {}
    url = data.get("url", "")
    if not url:
        print(json.dumps({"url": url, "status": 400, "body": ""}))
        return
    try:
        with urllib.request.urlopen(url, timeout=5) as resp:
            body = resp.read(200).decode("utf-8", "ignore")
            print(json.dumps({"url": url, "status": resp.getcode(), "body": body}))
    except Exception as e:
        print(json.dumps({"url": url, "status": 500, "body": str(e)}))

if __name__ == "__main__":
    main()
PY
