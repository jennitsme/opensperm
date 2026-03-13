# Architecture

## Goals
- Secure, typed, observable agent runtime with strong sandboxing and policy controls.
- Local-first dev with deterministic fixtures and golden transcripts.
- Extensible via typed SDKs (Rust + TS) and signed skill bundles.

## Components
- **Runtime (Rust)**
  - Agent loop: planner + executor; capability graph; budgeted plans (cost/latency).
  - Tool bridge: IPC protocol for tool calls (structured payloads, streaming tokens).
  - Sandbox manager: per-agent processes, resource quotas, egress allowlists, seccomp profile (Linux), macOS sandboxing strategy TBD.
  - Policy engine: RBAC/ABAC; approval hooks; data-scope enforcement.
  - Memory: pluggable providers (local KV, vector stores) with retention and redaction policies.
- **SDKs**
  - **TypeScript**: define tools/skills with zod-style schemas; middleware (auth/log/policy); contract test harness; packaging + signing.
  - **Rust**: derive macros for tool contracts; middleware; test harness; signer/validator for bundles.
- **CLI**
  - Commands: init skill/app, run agent locally, replay transcript, contract tests, package/sign, policy check.
- **Observability**
  - Structured logs; OpenTelemetry traces; per-tool call metrics; replay from transcripts.
- **Testing**
  - Deterministic fixtures, golden transcripts, load tests for latency/concurrency.

## Process model
- Runtime spawns agent workers as separate processes.
- Tool calls are IPC over a framed protocol (e.g., JSON or Cap’n Proto); streaming supported.
- Policy checks run before tool invocation and on outputs (redaction hooks).
- Observability middleware emits traces/logs for each call with correlation IDs.

## Security posture
- Signed skill bundles; verify signature + manifest hash before load.
- Resource limits: CPU/mem/timeouts per call; network egress allowlists per agent.
- Approval hooks: human-in-the-loop for sensitive scopes.
- Audit logs: immutable append-only log for privileged actions.

## Early milestones
- Define manifest + IPC schema.
- Implement runtime skeleton with sandbox manager stub.
- TS SDK minimal tool definition + contract tests.
- CLI: init skill, run skill in local sandbox, policy check stub.
- Golden transcript runner with deterministic shims.
