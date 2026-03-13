use anyhow::Result;
use std::fs;

pub fn run_transcript(path: &str) -> Result<()> {
    let data = fs::read_to_string(path)?;
    let transcript: serde_json::Value = serde_json::from_str(&data)?;
    // TODO: integrate with runtime shim; placeholder validates shape
    if !transcript.get("steps").is_some() {
        anyhow::bail!("invalid transcript: missing steps");
    }
    Ok(())
}
