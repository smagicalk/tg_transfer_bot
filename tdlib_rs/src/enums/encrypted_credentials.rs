#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EncryptedCredentials {
    /// Contains encrypted Telegram Passport data credentials
    #[serde(rename(
        serialize = "encryptedCredentials",
        deserialize = "encryptedCredentials"
    ))]
    EncryptedCredentials(crate::types::EncryptedCredentials),
}
