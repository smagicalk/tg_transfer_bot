#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftChatThemes {
    /// Contains a list of chat themes based on upgraded gifts
    #[serde(rename(serialize = "giftChatThemes", deserialize = "giftChatThemes"))]
    GiftChatThemes(crate::types::GiftChatThemes),
}
