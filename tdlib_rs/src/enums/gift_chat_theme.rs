#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftChatTheme {
    /// Describes a chat theme based on an upgraded gift
    #[serde(rename(serialize = "giftChatTheme", deserialize = "giftChatTheme"))]
    GiftChatTheme(crate::types::GiftChatTheme),
}
