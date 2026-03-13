use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub language: String,
    pub entry: String,
}

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("invalid manifest: {0}")]
    InvalidManifest(String),
}

pub type ToolHandler = fn(serde_json::Value) -> Result<serde_json::Value, SdkError>;

#[derive(Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub handler: ToolHandler,
}

#[derive(Clone)]
pub struct SkillBundle {
    pub manifest: Manifest,
    pub tools: Vec<ToolDefinition>,
}

pub fn define_skill(bundle: SkillBundle) -> Result<SkillBundle, SdkError> {
    if bundle.manifest.name.is_empty() {
        return Err(SdkError::InvalidManifest("name missing".into()));
    }
    // TODO: add full schema validation
    Ok(bundle)
}
