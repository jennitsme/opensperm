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
