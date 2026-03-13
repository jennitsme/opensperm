# opensperm

Opensperm is a private agent runtime: dedicated pods, isolated execution, local models, and secure skills.

## Core Modules
- **Agent Pods**: Dedicated compute per agent. No shared infra. Isolated processes.
- **Agent Runtime**: Secure execution for models, tools, and workflows. Sandbox with egress allowlists, CPU/mem limits, and policy checks.
- **Agent Models**: Run local models privately. Full control; no mandatory external APIs.

## Private-by-design Actions
- **Private Skills**: Custom capabilities run entirely inside your agent’s sandbox.
- **Private Access**: Connect via secure tunnel to your agent environment.
- **Private Payment**: Process payments privately without public exposure.
- **Private Memory**: Secure storage for knowledge and state.
- **Private Backup**: Protected backups and restore flows.
- **App Manager**: Manage agent apps/tools from one place.

## Differentiators
- Secure sandboxing: per-tool egress allowlist, rlimits, signed skills.
- Policy & approvals: scopes, RBAC/ABAC, approval file hook (extensible to prompts/webhooks).
- Observability: structured IPC with error codes; traces/logs per tool call.
- Typed SDKs (TS/Rust) + schemas for manifests and IPC.
- Local/offline mode with golden transcripts and contract tests (in progress).

## Current Status
- Rust runtime skeleton (sandbox, policy, registry, egress, limits).
- CLI: init/run/test/package/policy-check; run-config + registry loader.
- Schemas: skill manifest, IPC + error codes.
- SDKs: TS/Rust stubs; signing helpers; approvals via file/prompt/webhook.
- Streaming IPC active; contract runner executes transcripts via sandbox.
- Demo skills: echo, uppercase, reverse, http_mock.

## Quickstart (local demo)
```bash
git clone https://github.com/jennitsme/opensperm.git
cd opensperm
cargo build

# Approvals (choose one):
# export OPENSPERM_APPROVE_ALL=1
# export OPENSPERM_APPROVAL_FILE=approvals.txt   # comma-separated scopes
# export OPENSPERM_APPROVAL_PROMPT=1             # interactive prompt
# export OPENSPERM_APPROVAL_WEBHOOK=https://example.com/approve

# Optional logging
# export OPENSPERM_LOG_FILE=run.log

# Run demo plan with sample registry/policy
opensperm run --plan examples/plan.json --policy examples/policy.json --run examples/run.yml

# Validate demo transcript (executes via sandbox+registry)
opensperm test --transcript examples/transcript.json
```

## License
Apache-2.0
