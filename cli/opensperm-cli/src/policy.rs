use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PolicyConfig {
    pub allowed_scopes: Option<Vec<String>>,
}

pub fn load(path: &str) -> anyhow::Result<PolicyConfig> {
    let data = std::fs::read_to_string(path)?;
    let cfg: PolicyConfig = serde_json::from_str(&data)?;
    Ok(cfg)
}
