#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiChatTheme {
    /// Describes a chat theme based on an emoji
    #[serde(rename(serialize = "emojiChatTheme", deserialize = "emojiChatTheme"))]
    EmojiChatTheme(crate::types::EmojiChatTheme),
}
