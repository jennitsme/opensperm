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

#[cfg(unix)]
pub fn apply_limits(cmd: &mut tokio::process::Command, limits: &ResourceLimits) -> Result<(), LimitError> {
    use std::os::unix::process::CommandExt;
    use libc::{rlimit, setrlimit, RLIMIT_AS, RLIMIT_CPU};

    let cpu_ms = limits.cpu_time_ms;
    let mem_bytes = limits.memory_bytes;

    if cpu_ms.is_none() && mem_bytes.is_none() {
        return Ok(());
    }

    unsafe {
        cmd.pre_exec(move || {
            if let Some(ms) = cpu_ms {
                let secs = (ms + 999) / 1000; // ceil to seconds
                let lim = rlimit { rlim_cur: secs as u64, rlim_max: secs as u64 };
                let rc = setrlimit(RLIMIT_CPU, &lim);
                if rc != 0 { return Err(std::io::Error::last_os_error()); }
            }
            if let Some(bytes) = mem_bytes {
                let lim = rlimit { rlim_cur: bytes as u64, rlim_max: bytes as u64 };
                let rc = setrlimit(RLIMIT_AS, &lim);
                if rc != 0 { return Err(std::io::Error::last_os_error()); }
            }
            Ok(())
        });
    }
    Ok(())
}

#[cfg(not(unix))]
pub fn apply_limits(_cmd: &mut tokio::process::Command, _limits: &ResourceLimits) -> Result<(), LimitError> {
    Ok(())
}
