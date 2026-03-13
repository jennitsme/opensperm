use serde::Serialize;
use std::process::Stdio;
use tokio::process::Command;
use thiserror::Error;

#[derive(Debug, Clone, Serialize)]
pub struct DockerSpec {
    pub image: String,
    pub network_disabled: bool,
    pub env: Vec<(String, String)>,
}

#[derive(Debug, Error)]
pub enum DockerError {
    #[error("docker failed: {0}")]
    Failed(String),
}

pub async fn run_docker(spec: &DockerSpec, stdin_payload: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), DockerError> {
    let mut cmd = Command::new("docker");
    cmd.arg("run").arg("--rm");
    if spec.network_disabled {
        cmd.arg("--network").arg("none");
    }
    for (k, v) in spec.env.iter() {
        cmd.arg("-e").arg(format!("{}={}", k, v));
    }
    cmd.arg(&spec.image);
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| DockerError::Failed(e.to_string()))?;
    if let Some(mut sin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        sin.write_all(&stdin_payload).await.map_err(|e| DockerError::Failed(e.to_string()))?;
    }

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    use tokio::io::AsyncReadExt;
    let mut out = Vec::new();
    let mut err = Vec::new();
    if let Some(mut so) = stdout.map(tokio::io::BufReader::new) {
        so.read_to_end(&mut out).await.map_err(|e| DockerError::Failed(e.to_string()))?;
    }
    if let Some(mut se) = stderr.map(tokio::io::BufReader::new) {
        se.read_to_end(&mut err).await.map_err(|e| DockerError::Failed(e.to_string()))?;
    }
    let status = child.wait().await.map_err(|e| DockerError::Failed(e.to_string()))?;
    if !status.success() {
        return Err(DockerError::Failed(format!("exit {:?}, stderr {}", status.code(), String::from_utf8_lossy(&err))));
    }
    Ok((out, err))
}
