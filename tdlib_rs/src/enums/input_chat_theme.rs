#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputChatTheme {
    /// A theme based on an emoji
    #[serde(rename(serialize = "inputChatThemeEmoji", deserialize = "inputChatThemeEmoji"))]
    Emoji(crate::types::InputChatThemeEmoji),
    /// A theme based on an upgraded gift
    #[serde(rename(serialize = "inputChatThemeGift", deserialize = "inputChatThemeGift"))]
    Gift(crate::types::InputChatThemeGift),
}
