# Runtime (Rust)
Planned layout:
- `opensperm-runtime`: core crate (agent loop, planner/executor, tool IPC, sandbox manager, policy engine, memory providers).
- `opensperm-runtime-ffi`: bindings for SDKs (if needed).

Next steps:
- Define IPC schema and manifest types.
- Implement process sandbox abstraction (per-OS) with resource limits + egress policy.
- Add tracing/logging middleware.
