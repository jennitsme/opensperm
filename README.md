# opensperm

Agentic runtime inspired by OpenClaw, focused on secure sandboxing, typed skill contracts, and deep observability. Built for local-first development with a clear path to production.

## Differentiators
- **Secure-by-default agents**: per-agent process isolation, CPU/mem quotas, egress allowlists, signed skill bundles.
- **Typed Skill SDKs**: Rust + TypeScript SDKs with strict input/output schemas, contract tests, and generated typings.
- **Policy layer**: declarative RBAC/ABAC for tools and data scopes; human-in-the-loop approval hooks.
- **Observability baked in**: structured logs + OpenTelemetry traces for every tool call; replay and deterministic test harnesses.
- **Offline/dev mode**: deterministic fixtures, golden transcripts, and no-internet sandbox for safe iteration.
- **Multi-agent orchestration**: planner/executor with capability graph; cost/latency budgets per task.

## High-level architecture
See [ARCHITECTURE.md](ARCHITECTURE.md) for details.
- **Runtime (Rust)**: planner/executor loop, sandbox manager, policy engine, tool IPC bridge, memory providers.
- **SDKs (TS + Rust)**: define skills/tools with typed contracts, middleware (auth/logging/policy), contract tests, packaging/signing.
- **CLI**: init skills, run agents locally, test/replay transcripts, package/sign/publish skill bundles.
- **Observability**: OpenTelemetry exporters, structured logs, replay UI (later), golden transcript runner.

## Roadmap (MVP)
1) **Spec & contracts**
   - Skill manifest schema (YAML/JSON) with typed IO, capabilities, policy scopes, and signing.
   - IPC protocol between runtime and skills (tool calls, streaming tokens, traces).
2) **Runtime core** (Rust)
   - Agent loop (planner/executor), tool abstraction, capability graph, policy enforcement, sandbox manager.
   - Memory providers (local kv + pluggable vector store interface) with retention/redaction policies.
3) **SDKs**
   - TS + Rust SDKs for skills; contract test harness; typed clients; middleware hooks.
4) **CLI**
   - `opensperm init skill`, `opensperm run`, `opensperm test`, `opensperm package --sign`, `opensperm policy check`.
5) **Observability**
   - Structured logs, traces; local replay of transcripts; deterministic fixtures for tests.

## Contribution / dev
- Target platform: local macOS/Linux.
- Primary language: Rust for runtime/CLI; TS bindings for SDK.
- Public repo; open to contributors. CI planned: lint + fmt + tests + contract tests.

## Status
Scaffolding in progress. See `/docs` and `/policies` for specs as they land.
