#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EncryptedPassportElement {
    /// Contains information about an encrypted Telegram Passport element; for bots only
    #[serde(rename(
        serialize = "encryptedPassportElement",
        deserialize = "encryptedPassportElement"
    ))]
    EncryptedPassportElement(crate::types::EncryptedPassportElement),
}
