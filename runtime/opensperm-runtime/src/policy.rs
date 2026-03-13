use crate::{observability::TraceCtx, planner::PlanStep};
use thiserror::Error;

#[derive(Clone, Default)]
pub struct PolicyEngine {
    allowed_scopes: Vec<String>,
    approvals_required: Vec<String>,
    approvals: crate::approvals::ApprovalState,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self { allowed_scopes: vec![], approvals_required: vec![], approvals: crate::approvals::ApprovalState::default() }
    }

    pub fn with_scopes(scopes: Vec<String>) -> Self {
        Self { allowed_scopes: scopes, approvals_required: vec![], approvals: crate::approvals::ApprovalState::default() }
    }

    pub fn with_approvals(scopes: Vec<String>) -> Self {
        Self { allowed_scopes: vec![], approvals_required: scopes, approvals: crate::approvals::ApprovalState::default() }
    }

    pub async fn check(&self, step: &PlanStep, _trace: &TraceCtx) -> Result<(), PolicyError> {
        if step.tool.is_empty() {
            return Err(PolicyError::Denied("tool missing".into()));
        }
        for scope in &step.policy_scopes {
            if !self.allowed_scopes.is_empty() && !self.allowed_scopes.contains(scope) {
                return Err(PolicyError::Denied(format!("scope {scope} not allowed")));
            }
            if self.approvals_required.contains(scope) {
                let ok = self.approvals.request(scope).await;
                if !ok {
                    return Err(PolicyError::Denied(format!("approval denied for scope {scope}")));
                }
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
