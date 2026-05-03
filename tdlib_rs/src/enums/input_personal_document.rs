#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPersonalDocument {
    /// A personal document to be saved to Telegram Passport
    #[serde(rename(
        serialize = "inputPersonalDocument",
        deserialize = "inputPersonalDocument"
    ))]
    InputPersonalDocument(crate::types::InputPersonalDocument),
}
