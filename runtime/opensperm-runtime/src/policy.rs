use crate::{observability::TraceCtx, planner::PlanStep};
use thiserror::Error;

#[derive(Clone, Default)]
pub struct PolicyEngine {
    allowed_scopes: Vec<String>,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self { allowed_scopes: vec![] }
    }

    pub fn with_scopes(scopes: Vec<String>) -> Self {
        Self { allowed_scopes: scopes }
    }

    pub fn check(&self, step: &PlanStep, _trace: &TraceCtx) -> Result<(), PolicyError> {
        if step.tool.is_empty() {
            return Err(PolicyError::Denied("tool missing".into()));
        }
        for scope in &step.policy_scopes {
            if !self.allowed_scopes.contains(scope) {
                return Err(PolicyError::Denied(format!("scope {scope} not allowed")));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum PolicyError {
    #[error("denied: {0}")]
    Denied(String),
}
