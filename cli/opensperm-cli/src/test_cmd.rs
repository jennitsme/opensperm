use anyhow::Result;
use std::fs;

pub fn run_transcript(path: &str) -> Result<()> {
    let data = fs::read_to_string(path)?;
    let transcript: serde_json::Value = serde_json::from_str(&data)?;
    if transcript.get("steps").is_none() {
        anyhow::bail!("invalid transcript: missing steps");
    }
    // TODO: invoke runtime shim (TS/Rust) when available.
    Ok(())
}
