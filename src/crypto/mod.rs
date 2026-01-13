use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Encrypted message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

/// Crypto service for encrypting/decrypting messages
pub struct CryptoService {
    cipher: Aes256Gcm,
}

impl CryptoService {
    /// Create a new crypto service with a key derived from a shared secret
    pub fn new(shared_secret: &[u8]) -> Result<Self> {
        // Derive a 256-bit key from the shared secret
        let mut hasher = Sha256::new();
        hasher.update(shared_secret);
        let key = hasher.finalize();
        
        let cipher = Aes256Gcm::new(&key);
        
        Ok(Self { cipher })
    }

    /// Encrypt a message
    pub fn encrypt(&self, plaintext: &str) -> Result<EncryptedMessage> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .context("Encryption failed")?;
        
        Ok(EncryptedMessage {
            nonce: nonce.to_vec(),
            ciphertext,
        })
    }

    /// Decrypt a message
    pub fn decrypt(&self, encrypted: &EncryptedMessage) -> Result<String> {
        let nonce = Nonce::from_slice(&encrypted.nonce);
        
        let plaintext = self
            .cipher
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .context("Decryption failed")?;
        
        String::from_utf8(plaintext).context("Invalid UTF-8")
    }
}

/// Generate a shared secret from peer IDs (simplified for demo)
pub fn derive_shared_secret(local_id: &str, peer_id: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    
    // Sort IDs to ensure both peers derive the same secret
    if local_id < peer_id {
        hasher.update(local_id.as_bytes());
        hasher.update(peer_id.as_bytes());
    } else {
        hasher.update(peer_id.as_bytes());
        hasher.update(local_id.as_bytes());
    }
    
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let secret = b"test secret key for encryption";
        let crypto = CryptoService::new(secret).unwrap();
        
        let message = "Hello, secure world!";
        let encrypted = crypto.encrypt(message).unwrap();
        
        assert_ne!(encrypted.ciphertext, message.as_bytes());
        
        let decrypted = crypto.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_derive_shared_secret() {
        let secret1 = derive_shared_secret("alice", "bob");
        let secret2 = derive_shared_secret("bob", "alice");
        
        assert_eq!(secret1, secret2);
    }
}
