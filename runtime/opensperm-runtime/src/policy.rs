use crate::{observability::TraceCtx, planner::PlanStep};
use thiserror::Error;

#[derive(Clone, Default)]
pub struct PolicyEngine {
    allowed_scopes: Vec<String>,
    approvals_required: Vec<String>,
}

pub struct ApprovalHook;

impl ApprovalHook {
    pub fn request(_scope: &str) -> bool {
        // TODO: integrate real approval workflow; stub returns true
        true
    }
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self { allowed_scopes: vec![], approvals_required: vec![] }
    }

    pub fn with_scopes(scopes: Vec<String>) -> Self {
        Self { allowed_scopes: scopes, approvals_required: vec![] }
    }

    pub fn with_approvals(scopes: Vec<String>) -> Self {
        Self { allowed_scopes: vec![], approvals_required: scopes }
    }

    pub fn check(&self, step: &PlanStep, _trace: &TraceCtx) -> Result<(), PolicyError> {
        if step.tool.is_empty() {
            return Err(PolicyError::Denied("tool missing".into()));
        }
        for scope in &step.policy_scopes {
            if !self.allowed_scopes.is_empty() && !self.allowed_scopes.contains(scope) {
                return Err(PolicyError::Denied(format!("scope {scope} not allowed")));
            }
            if self.approvals_required.contains(scope) && !ApprovalHook::request(scope) {
                return Err(PolicyError::Denied(format!("approval denied for scope {scope}")));
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
