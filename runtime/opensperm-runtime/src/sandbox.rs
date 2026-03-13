use crate::{
    egress::{EgressError, EgressPolicy},
    ipc::{IpcMessage, ToolCall, ToolCallStatus},
    limits::ResourceLimits,
    observability::TraceCtx,
    tool_registry::{ToolRegistry, ToolSpec},
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
    config: SandboxConfig,
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

    pub fn with_registry(mut self, registry: ToolRegistry) -> Self {
        self.registry = registry;
        self
    }

    pub async fn invoke(&self, call: ToolCall, trace: TraceCtx) -> Result<IpcMessage, SandboxError> {
        let _span = trace.enter_span("sandbox_invoke");

        let spec = self
            .registry
            .resolve(&call)
            .ok_or_else(|| SandboxError::UnknownTool(call.tool.clone()))?;

        // Egress check (simplified): ensure requested tool egress is allowed
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

        // TODO: apply resource limits per OS (rlimits/cgroups)
        if let Some(_mem) = self.config.limits.memory_bytes {
            // placeholder hook
        }

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
    #[error("unknown tool: {0}")]
    UnknownTool(String),
    #[error("process failed: {0}")]
    Process(String),
    #[error("timeout")]
    Timeout,
    #[error(transparent)]
    Egress(#[from] EgressError),
}
