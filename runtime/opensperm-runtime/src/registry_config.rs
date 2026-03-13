use crate::tool_registry::{ToolRegistry, ToolSpec};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct RegistryFile {
    pub tools: Vec<RegistryTool>,
}

#[derive(Debug, Deserialize)]
pub struct RegistryTool {
    pub name: String,
    pub command: String,
    pub args: Option<Vec<String>>,
    pub allow_egress: Option<Vec<String>>,
}

pub fn load_registry(path: &str) -> anyhow::Result<ToolRegistry> {
    let data = fs::read_to_string(path)?;
    let cfg: RegistryFile = serde_yaml::from_str(&data)?;
    let mut reg = ToolRegistry::new();
    for t in cfg.tools {
        reg = reg.register(
            &t.name,
            ToolSpec {
                command: t.command,
                args: t.args.unwrap_or_default(),
                allow_egress: t.allow_egress.unwrap_or_default(),
            },
        );
    }
    Ok(reg)
}
