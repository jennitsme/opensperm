use crate::signing::{hash_manifest, Signer, Verifier};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct ManifestSigning {
    pub algorithm: String,
    pub signature: String,
    pub publicKey: String,
    pub manifestHash: Option<String>,
}

#[derive(Debug, Error)]
pub enum ManifestVerifyError {
    #[error("unsupported algorithm: {0}")]
    UnsupportedAlg(String),
    #[error("signature invalid")]
    Invalid,
    #[error("hash mismatch")]
    HashMismatch,
    #[error("decode error")]
    Decode,
}

pub fn sign_manifest(manifest_bytes: &[u8], secret_key_b64: &str) -> Result<ManifestSigning, ManifestVerifyError> {
    let key_bytes = base64::decode(secret_key_b64).map_err(|_| ManifestVerifyError::Decode)?;
    let signer = Signer::from_bytes(&key_bytes).map_err(|_| ManifestVerifyError::Invalid)?;
    let sig = signer.sign(manifest_bytes);
    let hash = hash_manifest(manifest_bytes);
    Ok(ManifestSigning {
        algorithm: "ed25519".into(),
        signature: base64::encode(sig),
        publicKey: base64::encode(signer.public_key()),
        manifestHash: Some(base64::encode(hash)),
    })
}

pub fn verify_manifest(manifest_bytes: &[u8], signing: &ManifestSigning) -> Result<(), ManifestVerifyError> {
    if signing.algorithm != "ed25519" {
        return Err(ManifestVerifyError::UnsupportedAlg(signing.algorithm.clone()));
    }
    let public = base64::decode(&signing.publicKey).map_err(|_| ManifestVerifyError::Decode)?;
    let sig = base64::decode(&signing.signature).map_err(|_| ManifestVerifyError::Decode)?;
    let expected_hash = signing.manifestHash.as_ref().map(|h| base64::decode(h).map_err(|_| ManifestVerifyError::Decode)).transpose()?;
    if let Some(eh) = expected_hash {
        let actual = hash_manifest(manifest_bytes);
        if eh != actual {
            return Err(ManifestVerifyError::HashMismatch);
        }
    }
    let verifier = Verifier::from_bytes(&public).map_err(|_| ManifestVerifyError::Invalid)?;
    verifier.verify(manifest_bytes, &sig).map_err(|_| ManifestVerifyError::Invalid)
}
