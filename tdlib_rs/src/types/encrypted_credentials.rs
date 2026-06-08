#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains encrypted Telegram Passport data credentials
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EncryptedCredentials {
    /// The encrypted credentials
    pub data: String,
    /// The decrypted data hash
    pub hash: String,
    /// Secret for data decryption, encrypted with the service's public key
    pub secret: String,
}
