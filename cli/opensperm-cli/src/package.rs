use anyhow::Result;
use opensperm_runtime::signer::sign_manifest;
use std::fs;

pub fn sign(manifest_path: &str, secret_key_b64: &str) -> Result<()> {
    let bytes = fs::read(manifest_path)?;
    let signing = sign_manifest(&bytes, secret_key_b64)?;
    let mut manifest: serde_json::Value = serde_json::from_slice(&bytes)?;
    manifest["signing"] = serde_json::to_value(signing)?;
    let out = serde_json::to_vec_pretty(&manifest)?;
    fs::write(manifest_path, out)?;
    Ok(())
}
