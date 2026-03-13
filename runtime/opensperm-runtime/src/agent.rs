use crate::{ipc::ToolCall, observability::TraceCtx, planner::Plan, policy::PolicyEngine, sandbox::SandboxManager};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: String,
    pub policy_scopes: Vec<String>,
    pub budget_ms: u64,
}

pub struct AgentRuntime {
    sandbox: SandboxManager,
    policy: PolicyEngine,
}

impl AgentRuntime {
    pub fn new(sandbox: SandboxManager, policy: PolicyEngine) -> Self {
        Self { sandbox, policy }
    }

    pub async fn execute_plan(&self, plan: Plan, trace: TraceCtx) -> Result<(), AgentError> {
        for step in plan.steps {
            self.policy.check(&step, &trace)?;
            let tool_call = ToolCall::from_step(&step);
            let _resp = self.sandbox.invoke(tool_call, trace.clone()).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("policy violation: {0}")]
    Policy(String),
    #[error("sandbox error: {0}")]
    Sandbox(String),
}
