use crate::{ipc::{IpcMessage, ToolCall}, observability::TraceCtx};
use thiserror::Error;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

#[derive(Clone)]
pub struct SandboxConfig {
    pub timeout_ms: u64,
    pub egress_allow: Vec<String>,
}

pub struct SandboxManager {
    config: SandboxConfig,
}

impl SandboxManager {
    pub fn new() -> Self {
        Self {
            config: SandboxConfig { timeout_ms: 10_000, egress_allow: vec![] },
        }
    }

    pub fn with_config(config: SandboxConfig) -> Self {
        Self { config }
    }

    pub async fn invoke(&self, _call: ToolCall, trace: TraceCtx) -> Result<IpcMessage, SandboxError> {
        // TODO: per-tool command mapping; stub executes /bin/true
        let span = trace.enter_span("sandbox_invoke");
        let fut = async {
            let status = Command::new("/bin/true").status().await.map_err(|e| SandboxError::Process(e.to_string()))?;
            if !status.success() {
                return Err(SandboxError::Process(format!("exit code {:?}", status.code())));
            }
            Ok(IpcMessage::ToolCallResponse {
                id: "stub".into(),
                status: crate::ipc::ToolCallStatus::Ok,
                output: Some(serde_json::json!({"ok": true})),
                error: None,
                trace: None,
            })
        };
        let res = timeout(Duration::from_millis(self.config.timeout_ms), fut)
            .await
            .map_err(|_| SandboxError::Timeout)??;
        drop(span);
        Ok(res)
    }
}

#[derive(Debug, Error)]
pub enum SandboxError {
    #[error("process failed: {0}")]
    Process(String),
    #[error("timeout")]
    Timeout,
}
