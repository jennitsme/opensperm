use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallContext {
    pub agent_id: String,
    pub trace_id: String,
    pub span_id: String,
    pub policy_scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IpcMessage {
    #[serde(rename = "tool_call_request")]
    ToolCallRequest {
        id: String,
        tool: String,
        input: serde_json::Value,
        context: ToolCallContext,
    },
    #[serde(rename = "tool_call_response")]
    ToolCallResponse {
        id: String,
        status: ToolCallStatus,
        output: Option<serde_json::Value>,
        error: Option<IpcError>,
    },
    #[serde(rename = "stream_token")]
    StreamToken { id: String, delta: String, done: bool },
    #[serde(rename = "trace")]
    TraceEvent { level: TraceLevel, message: String, metadata: serde_json::Value },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolCallStatus {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcError {
    pub message: String,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub struct ToolCall {
    pub tool: String,
    pub input: serde_json::Value,
    pub context: ToolCallContext,
}

impl ToolCall {
    pub fn from_step(step: &crate::planner::PlanStep) -> Self {
        Self {
            tool: step.tool.clone(),
            input: step.input.clone(),
            context: ToolCallContext {
                agent_id: step.agent_id.clone(),
                trace_id: step.trace_id.clone(),
                span_id: step.span_id.clone(),
                policy_scopes: step.policy_scopes.clone(),
            },
        }
    }
}
