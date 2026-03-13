use crate::{
    egress::{EgressError, EgressPolicy},
    ipc::{
        IpcError, IpcMessage, ToolCall, ToolCallStatus, ERR_EGRESS_DENIED, ERR_SANDBOX_FAILED, ERR_TIMEOUT, ERR_UNKNOWN_TOOL,
    },
    limits::{apply_limits, LimitError, ResourceLimits},
    observability::TraceCtx,
    tool_registry::ToolRegistry,
};
use std::process::Stdio;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::time::{timeout, Duration};

#[derive(Clone)]
pub struct SandboxConfig {
    pub timeout_ms: u64,
    pub egress_policy: EgressPolicy,
    pub limits: ResourceLimits,
}

pub struct SandboxManager {
    pub config: SandboxConfig,
    registry: ToolRegistry,
}

impl SandboxManager {
    pub fn new() -> Self {
        Self {
            config: SandboxConfig {
                timeout_ms: 10_000,
                egress_policy: EgressPolicy::default(),
                limits: ResourceLimits::default(),
            },
            registry: ToolRegistry::new(),
        }
    }

    pub fn with_config(config: SandboxConfig) -> Self {
        Self { config, registry: ToolRegistry::new() }
    }

    pub fn config(&self) -> SandboxConfig {
        self.config.clone()
    }

    pub fn with_registry(mut self, registry: ToolRegistry) -> Self {
        self.registry = registry;
        self
    }

    /// Invoke a tool, emitting stream tokens for stdout chunks and a final response.
    pub async fn invoke(&self, call: ToolCall, trace: TraceCtx) -> Result<Vec<IpcMessage>, SandboxError> {
        let _span = trace.enter_span("sandbox_invoke");

        let spec = self
            .registry
            .resolve(&call)
            .ok_or_else(|| SandboxError::UnknownTool(call.tool.clone()))?;

        for allowed in &spec.allow_egress {
            if !self.config.egress_policy.permits(allowed) {
                return Err(SandboxError::Egress(EgressError::Denied(allowed.clone())));
            }
        }

        let payload = serde_json::to_vec(&call.input).map_err(|e| SandboxError::Process(e.to_string()))?;
        let mut cmd = Command::new(&spec.command);
        cmd.args(&spec.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        apply_limits(&mut cmd, &self.config.limits).map_err(SandboxError::Limit)?;
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
                tracing::error!(span_id=%call.context.span_id, stderr=%String::from_utf8_lossy(&err), code=?status.code(), "sandbox process failed");
                return Err(SandboxError::Process(format!("exit code {:?}, stderr {}", status.code(), String::from_utf8_lossy(&err))));
            }
            let output_json: serde_json::Value = serde_json::from_slice(&out).unwrap_or(serde_json::json!({"raw": String::from_utf8_lossy(&out)}));

            // Streaming: emit stream_token chunks before final response
            let mut msgs: Vec<IpcMessage> = Vec::new();
            let text_out = String::from_utf8_lossy(&out).to_string();
            if !text_out.is_empty() {
                let chunk_size = 512;
                for chunk in text_out.as_bytes().chunks(chunk_size) {
                    let delta = String::from_utf8_lossy(chunk).to_string();
                    tracing::info!(span_id=%call.context.span_id, "stream_chunk" = %delta);
                    msgs.push(IpcMessage::StreamToken {
                        id: call.context.span_id.clone(),
                        delta,
                        done: false,
                    });
                }
            }

            msgs.push(IpcMessage::ToolCallResponse {
                id: call.context.span_id.clone(),
                status: ToolCallStatus::Ok,
                output: Some(output_json),
                error: None,
                trace: None,
            });
            Ok(msgs)
        };

        let res = timeout(Duration::from_millis(self.config.timeout_ms), fut)
            .await
            .map_err(|_| SandboxError::Timeout)??;
        Ok(res)
    }
}

#[derive(Debug, Error)]
pub enum SandboxError {
    #[error("unknown tool: {0}")]
    UnknownTool(String),
    #[error("process failed: {0}")]
    Process(String),
    #[error("timeout")]
    Timeout,
    #[error(transparent)]
    Egress(#[from] EgressError),
    #[error(transparent)]
    Limit(#[from] LimitError),
}

impl SandboxError {
    pub fn to_ipc_error(&self) -> IpcError {
        match self {
            SandboxError::UnknownTool(_) => IpcError { message: self.to_string(), code: Some(ERR_UNKNOWN_TOOL.into()) },
            SandboxError::Timeout => IpcError { message: self.to_string(), code: Some(ERR_TIMEOUT.into()) },
            SandboxError::Egress(_) => IpcError { message: self.to_string(), code: Some(ERR_EGRESS_DENIED.into()) },
            _ => IpcError { message: self.to_string(), code: Some(ERR_SANDBOX_FAILED.into()) },
        }
    }
}
