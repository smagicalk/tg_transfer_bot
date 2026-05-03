#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputIdentityDocument {
    /// An identity document to be saved to Telegram Passport
    #[serde(rename(
        serialize = "inputIdentityDocument",
        deserialize = "inputIdentityDocument"
    ))]
    InputIdentityDocument(crate::types::InputIdentityDocument),
}
