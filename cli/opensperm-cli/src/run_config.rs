use anyhow::Result;
use opensperm_runtime::{egress::EgressPolicy, limits::ResourceLimits, sandbox::SandboxConfig, tool_registry::ToolRegistry};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct RunConfig {
    pub timeout_ms: Option<u64>,
    pub egress_allow: Option<Vec<String>>,
    pub cpu_time_ms: Option<u64>,
    pub memory_bytes: Option<u64>,
    pub registry: Option<String>,
}

pub fn load(path: &str) -> Result<(SandboxConfig, Option<ToolRegistry>)> {
    let data = fs::read_to_string(path)?;
    let cfg: RunConfig = serde_yaml::from_str(&data)?;

    let sandbox = SandboxConfig {
        timeout_ms: cfg.timeout_ms.unwrap_or(10_000),
        egress_policy: EgressPolicy { allow: cfg.egress_allow.unwrap_or_default() },
        limits: ResourceLimits::new(cfg.cpu_time_ms, cfg.memory_bytes),
    };

    let registry = if let Some(path) = cfg.registry {
        Some(opensperm_runtime::registry_config::load_registry(&path)?)
    } else {
        None
    };

    Ok((sandbox, registry))
}
