use anyhow::Result;
use std::fs;

pub fn run_transcript(path: &str) -> Result<()> {
    let data = fs::read_to_string(path)?;
    let transcript: serde_json::Value = serde_json::from_str(&data)?;
    if transcript.get("steps").is_none() {
        anyhow::bail!("invalid transcript: missing steps");
    }
    // TODO: call TS/Rust shim for actual execution.
    println!("validated transcript structure: {}", path);
    Ok(())
}
