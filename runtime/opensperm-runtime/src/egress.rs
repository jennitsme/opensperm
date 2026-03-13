use thiserror::Error;

#[derive(Clone, Default)]
pub struct EgressPolicy {
    pub allow: Vec<String>,
}

impl EgressPolicy {
    pub fn permits(&self, target: &str) -> bool {
        if self.allow.is_empty() {
            return false;
        }
        self.allow.iter().any(|a| target.starts_with(a))
    }
}

#[derive(Debug, Error)]
pub enum EgressError {
    #[error("egress denied: {0}")]
    Denied(String),
}
