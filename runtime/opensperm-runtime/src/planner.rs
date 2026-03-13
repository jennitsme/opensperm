use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub steps: Vec<PlanStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub agent_id: String,
    pub trace_id: String,
    pub span_id: String,
    pub tool: String,
    pub input: serde_json::Value,
    pub policy_scopes: Vec<String>,
}

impl Plan {
    pub fn single(step: PlanStep) -> Self {
        Self { steps: vec![step] }
    }
}
