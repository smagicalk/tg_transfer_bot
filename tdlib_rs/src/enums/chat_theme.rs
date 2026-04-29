#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatTheme {
    /// A chat theme based on an emoji
    #[serde(rename(serialize = "chatThemeEmoji", deserialize = "chatThemeEmoji"))]
    Emoji(crate::types::ChatThemeEmoji),
    /// A chat theme based on an upgraded gift
    #[serde(rename(serialize = "chatThemeGift", deserialize = "chatThemeGift"))]
    Gift(crate::types::ChatThemeGift),
}
