use anyhow::Result;
use std::fs;

#[derive(serde::Deserialize)]
struct TranscriptStep {
    request: serde_json::Value,
    response: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct Transcript {
    name: String,
    steps: Vec<TranscriptStep>,
}

pub fn run_transcript(path: &str) -> Result<()> {
    let data = fs::read_to_string(path)?;
    let transcript: Transcript = serde_json::from_str(&data)?;
    if transcript.steps.is_empty() {
        anyhow::bail!("no steps");
    }
    for step in transcript.steps.iter() {
        if step.request.get("tool").is_none() {
            anyhow::bail!("step missing tool");
        }
        if step.response.get("output").is_none() {
            anyhow::bail!("step missing output");
        }
    }
    println!("validated transcript: {} ({} steps)", transcript.name, transcript.steps.len());
    Ok(())
}
