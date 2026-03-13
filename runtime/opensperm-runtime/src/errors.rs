use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("policy violation: {0}")]
    Policy(String),
    #[error("sandbox: {0}")]
    Sandbox(String),
    #[error("io: {0}")]
    Io(String),
    #[error("timeout")]
    Timeout,
}
