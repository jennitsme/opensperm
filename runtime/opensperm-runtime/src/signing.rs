use ed25519_dalek::{Signature, SigningKey, VerifyingKey, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SIGNATURE_LENGTH};
use sha2::{Digest, Sha256};
use thiserror::Error;

pub struct Signer {
    key: SigningKey,
}

pub struct Verifier {
    key: VerifyingKey,
}

impl Signer {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SignError> {
        let key = SigningKey::from_bytes(bytes.try_into().map_err(|_| SignError::InvalidKey)?);
        Ok(Self { key })
    }

    pub fn sign(&self, payload: &[u8]) -> [u8; SIGNATURE_LENGTH] {
        self.key.sign(payload).to_bytes()
    }

    pub fn public_key(&self) -> [u8; PUBLIC_KEY_LENGTH] {
        self.key.verifying_key().to_bytes()
    }
}

impl Verifier {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SignError> {
        let key = VerifyingKey::from_bytes(bytes.try_into().map_err(|_| SignError::InvalidKey)?).map_err(|_| SignError::InvalidKey)?;
        Ok(Self { key })
    }

    pub fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), SignError> {
        let sig = Signature::from_bytes(signature.try_into().map_err(|_| SignError::InvalidSignature)?);
        self.key.verify_strict(payload, &sig).map_err(|_| SignError::VerifyFailed)
    }
}

pub fn hash_manifest(manifest_bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(manifest_bytes);
    hasher.finalize().into()
}

#[derive(Debug, Error)]
pub enum SignError {
    #[error("invalid key")]
    InvalidKey,
    #[error("invalid signature")]
    InvalidSignature,
    #[error("verify failed")]
    VerifyFailed,
}
