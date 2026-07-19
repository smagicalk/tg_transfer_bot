#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Telegram Passport data has been received; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePassportDataReceived {
    /// List of received Telegram Passport elements
    pub elements: Vec<crate::types::EncryptedPassportElement>,
    /// Encrypted data credentials
    pub credentials: crate::types::EncryptedCredentials,
}
