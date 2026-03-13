use thiserror::Error;

#[derive(Clone, Default)]
pub struct ResourceLimits {
    pub cpu_time_ms: Option<u64>,
    pub memory_bytes: Option<u64>,
}

impl ResourceLimits {
    pub fn new(cpu_time_ms: Option<u64>, memory_bytes: Option<u64>) -> Self {
        Self { cpu_time_ms, memory_bytes }
    }
}

#[derive(Debug, Error)]
pub enum LimitError {
    #[error("failed to apply limits: {0}")]
    Apply(String),
}

#[cfg(target_os = "macos")]
pub fn apply_limits(_child: &mut tokio::process::Child, _limits: &ResourceLimits) -> Result<(), LimitError> {
    // TODO: macOS sandbox/seatbelt profile integration
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn apply_limits(_child: &mut tokio::process::Child, _limits: &ResourceLimits) -> Result<(), LimitError> {
    // TODO: cgroups/rlimits
    Ok(())
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn apply_limits(_child: &mut tokio::process::Child, _limits: &ResourceLimits) -> Result<(), LimitError> {
    Ok(())
}
