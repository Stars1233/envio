use base64::{Engine, engine::general_purpose::STANDARD};
use chacha20poly1305::{
    Key, XChaCha20Poly1305,
    aead::stream::{DecryptorBE32, EncryptorBE32},
    aead::{AeadCore, KeyInit, OsRng},
};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

pub const CHUNK_SIZE: usize = 1024;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct MetadataV1 {
    pub nonce: String,
}

pub fn encrypt(key: &str, data: &[u8]) -> Result<(Vec<u8>, MetadataV1)> {
    let key_bytes = STANDARD
        .decode(key)
        .map_err(|e| Error::Cipher(e.to_string()))?;

    if key_bytes.len() != 32 {
        return Err(Error::Cipher(
            "Symmetric key must be exactly 32 bytes (base64 encoded)".to_string(),
        ));
    }

    let nonce_bytes = &XChaCha20Poly1305::generate_nonce(&mut OsRng)[0..19];
    let mut encryptor = EncryptorBE32::<XChaCha20Poly1305>::from_aead(
        XChaCha20Poly1305::new(Key::from_slice(&key_bytes)),
        nonce_bytes.into(),
    );

    let mut encrypted_buffer = Vec::new();
    let mut offset = 0;

    while offset + CHUNK_SIZE < data.len() {
        let end = usize::min(offset + CHUNK_SIZE, data.len());
        let chunk = &data[offset..end];

        encrypted_buffer.extend(
            encryptor
                .encrypt_next(chunk)
                .map_err(|e| Error::Cipher(e.to_string()))?,
        );

        offset = end;
    }

    let last_chunk = &data[offset..];
    encrypted_buffer.extend(
        encryptor
            .encrypt_last(last_chunk)
            .map_err(|e| Error::Cipher(e.to_string()))?,
    );

    let metadata = MetadataV1 {
        nonce: STANDARD.encode(nonce_bytes),
    };

    Ok((encrypted_buffer, metadata))
}

pub fn decrypt(key: &str, metadata: &MetadataV1, encrypted_data: &[u8]) -> Result<Vec<u8>> {
    let key_bytes = STANDARD
        .decode(key)
        .map_err(|e| Error::Cipher(e.to_string()))?;

    if key_bytes.len() != 32 {
        return Err(Error::Cipher(
            "Symmetric key must be exactly 32 bytes (base64 encoded)".to_string(),
        ));
    }

    let nonce_bytes = STANDARD
        .decode(&metadata.nonce)
        .map_err(|e| Error::Cipher(e.to_string()))?;

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key_bytes));

    let mut decryptor =
        DecryptorBE32::<XChaCha20Poly1305>::from_aead(cipher, nonce_bytes.as_slice().into());

    let mut decrypted_buffer = Vec::new();
    let mut offset = 0;

    const BUFFER_LEN: usize = CHUNK_SIZE + 16;
    while offset + BUFFER_LEN < encrypted_data.len() {
        let end = usize::min(offset + BUFFER_LEN, encrypted_data.len());
        let chunk = &encrypted_data[offset..end];

        decrypted_buffer.extend(
            decryptor
                .decrypt_next(chunk)
                .map_err(|e| Error::Cipher(e.to_string()))?,
        );

        offset = end;
    }

    let last_chunk = &encrypted_data[offset..];
    decrypted_buffer.extend(
        decryptor
            .decrypt_last(last_chunk)
            .map_err(|e| Error::Cipher(e.to_string()))?,
    );

    Ok(decrypted_buffer)
}
