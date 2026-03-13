# opensperm

Spawn. Inject. Deploy. Ghost. — Opensperm is a private agent runtime: dedicated pods, isolated execution, local models, and secure skills.

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
- SDKs: TS/Rust stubs; signing helpers; approvals via file.
- TODO (next): streaming IPC, contract runner via shim, approval channel beyond file, demo skills/policies.

## Quickstart (local)
```bash
# Run plan with policy and run configs
auth your gh; git clone https://github.com/jennitsme/opensperm.git
cd opensperm
cargo build
# example
# opensperm run --plan plan.json --policy policy.json --run run.yml
```

## License
Apache-2.0
