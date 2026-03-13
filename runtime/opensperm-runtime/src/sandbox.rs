use crate::{ipc::ToolCall, observability::TraceCtx};
use thiserror::Error;

pub struct SandboxManager;

impl SandboxManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn invoke(&self, _call: ToolCall, _trace: TraceCtx) -> Result<(), SandboxError> {
        // TODO: spawn isolated process, enforce CPU/mem/timeouts, egress allowlist, stream output.
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum SandboxError {
    #[error("process failed: {0}")]
    Process(String),
}
