use crate::{observability::TraceCtx, planner::PlanStep};
use thiserror::Error;

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn check(&self, step: &PlanStep, _trace: &TraceCtx) -> Result<(), PolicyError> {
        // TODO: enforce RBAC/ABAC, scopes, egress, secrets, etc.
        let _ = step;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum PolicyError {
    #[error("denied: {0}")]
    Denied(String),
}
