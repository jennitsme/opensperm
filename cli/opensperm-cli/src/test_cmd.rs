use anyhow::Result;
use opensperm_runtime::{registry_config::load_registry, sandbox::SandboxManager, egress::EgressPolicy, limits::ResourceLimits, sandbox::SandboxConfig, ipc::IpcMessage};
use std::fs;

#[derive(serde::Deserialize)]
struct TranscriptStep {
    request: serde_json::Value,
    response: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct Transcript {
    name: String,
    steps: Vec<TranscriptStep>,
}

pub fn run_transcript(path: &str) -> Result<()> {
    let data = fs::read_to_string(path)?;
    let transcript: Transcript = serde_json::from_str(&data)?;
    if transcript.steps.is_empty() {
        anyhow::bail!("no steps");
    }

    // Optional registry path (env OPENSPERM_REGISTRY) for replay via sandbox
    let reg_path = std::env::var("OPENSPERM_REGISTRY").unwrap_or_else(|_| "examples/registry.yml".into());
    let registry = load_registry(&reg_path)?;
    let sandbox = SandboxManager::with_config(SandboxConfig {
        timeout_ms: 5000,
        egress_policy: EgressPolicy { allow: vec![] },
        limits: ResourceLimits::new(Some(1000), Some(128 * 1024 * 1024)),
    })
    .with_registry(registry);

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        for step in transcript.steps.iter() {
            let tool = step.request.get("tool").and_then(|t| t.as_str()).ok_or_else(|| anyhow::anyhow!("missing tool"))?;
            let input = step.request.get("input").cloned().unwrap_or_else(|| serde_json::json!({}));
            let call = opensperm_runtime::ipc::ToolCall {
                tool: tool.to_string(),
                input,
                context: opensperm_runtime::ipc::ToolCallContext {
                    agent_id: "test-agent".into(),
                    trace_id: "test-trace".into(),
                    span_id: "test-span".into(),
                    policy_scopes: vec![],
                },
            };
            let resp = sandbox.invoke(call, opensperm_runtime::observability::TraceCtx { trace_id: "trace".into(), span_id: "span".into() }).await?;
            if let IpcMessage::ToolCallResponse { output, .. } = resp {
                let expected = step.response.get("output").cloned().unwrap_or_else(|| serde_json::json!({}));
                if output.unwrap_or_default() != expected {
                    anyhow::bail!("output mismatch for tool {tool}");
                }
            } else {
                anyhow::bail!("unexpected IPC message");
            }
        }
        Ok::<(), anyhow::Error>(())
    })?;

    println!("validated transcript: {} ({} steps) via sandbox", transcript.name, transcript.steps.len());
    Ok(())
}
