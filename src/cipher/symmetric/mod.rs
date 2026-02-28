mod metadata;
mod v1;

use base64::{Engine, engine::general_purpose::STANDARD};
use chacha20poly1305::XChaCha20Poly1305;
use chacha20poly1305::aead::{KeyInit, OsRng};
use std::any::Any;
use zeroize::Zeroizing;

use crate::{
    EnvMap,
    cipher::{Cipher, CipherKind, EncryptedContent},
    error::Result,
};

use metadata::VersionedMetadata;

#[derive(Clone)]
pub struct SYMMETRIC {
    key: Zeroizing<String>,
    metadata: VersionedMetadata,
}

impl SYMMETRIC {
    pub fn new(key: Zeroizing<String>) -> Self {
        SYMMETRIC {
            key,
            metadata: VersionedMetadata::default(),
        }
    }

    pub fn generate_key() -> Zeroizing<String> {
        Zeroizing::new(STANDARD.encode(XChaCha20Poly1305::generate_key(&mut OsRng)))
    }

    pub fn set_key(&mut self, key: Zeroizing<String>) {
        self.key = key;
    }
}

impl Cipher for SYMMETRIC {
    fn kind(&self) -> CipherKind {
        CipherKind::SYMMETRIC
    }

    fn encrypt(&mut self, envs: &EnvMap) -> Result<EncryptedContent> {
        let data = envs.as_bytes()?;

        let (encrypted, metadata) = v1::encrypt(&self.key, &data)?;
        self.metadata = metadata.into();

        Ok(EncryptedContent::Bytes(encrypted))
    }

    fn decrypt(&self, encrypted_data: &EncryptedContent) -> Result<EnvMap> {
        let raw_data = encrypted_data.as_bytes()?;
        let decrypted = match &self.metadata {
            VersionedMetadata::V1(metadata) => v1::decrypt(&self.key, metadata, &raw_data)?,
        };
        Ok(decrypted.into())
    }

    fn export_metadata(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self.metadata.clone()).ok()
    }

    fn import_metadata(&mut self, data: serde_json::Value) -> Result<()> {
        self.metadata = serde_json::from_value(data)?;

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
