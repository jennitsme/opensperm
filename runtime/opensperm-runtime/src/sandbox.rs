use crate::{ipc::{IpcMessage, ToolCall, ToolCallStatus}, observability::TraceCtx};
use std::process::Stdio;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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

    pub async fn invoke(&self, call: ToolCall, trace: TraceCtx) -> Result<IpcMessage, SandboxError> {
        let _span = trace.enter_span("sandbox_invoke");

        // TODO: Map tool -> executable/args via registry. Stub: echo JSON input.
        let payload = serde_json::to_vec(&call.input).map_err(|e| SandboxError::Process(e.to_string()))?;
        let mut cmd = Command::new("/usr/bin/env");
        cmd.arg("cat")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| SandboxError::Process(e.to_string()))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(&payload).await.map_err(|e| SandboxError::Process(e.to_string()))?;
        }

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let fut = async move {
            let mut out = Vec::new();
            if let Some(mut so) = stdout.map(tokio::io::BufReader::new) {
                so.read_to_end(&mut out).await.map_err(|e| SandboxError::Process(e.to_string()))?;
            }
            let mut err = Vec::new();
            if let Some(mut se) = stderr.map(tokio::io::BufReader::new) {
                se.read_to_end(&mut err).await.map_err(|e| SandboxError::Process(e.to_string()))?;
            }
            let status = child.wait().await.map_err(|e| SandboxError::Process(e.to_string()))?;
            if !status.success() {
                return Err(SandboxError::Process(format!("exit code {:?}, stderr {}", status.code(), String::from_utf8_lossy(&err))));
            }
            let output_json: serde_json::Value = serde_json::from_slice(&out).unwrap_or(serde_json::json!({"raw": String::from_utf8_lossy(&out)}));
            Ok(IpcMessage::ToolCallResponse {
                id: call.context.span_id.clone(),
                status: ToolCallStatus::Ok,
                output: Some(output_json),
                error: None,
                trace: None,
            })
        };

        let res = timeout(Duration::from_millis(self.config.timeout_ms), fut)
            .await
            .map_err(|_| SandboxError::Timeout)??;
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
