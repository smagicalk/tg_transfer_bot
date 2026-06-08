#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageEffectType {
    /// An effect from an emoji reaction
    #[serde(rename(
        serialize = "messageEffectTypeEmojiReaction",
        deserialize = "messageEffectTypeEmojiReaction"
    ))]
    EmojiReaction(crate::types::MessageEffectTypeEmojiReaction),
    /// An effect from a premium sticker
    #[serde(rename(
        serialize = "messageEffectTypePremiumSticker",
        deserialize = "messageEffectTypePremiumSticker"
    ))]
    PremiumSticker(crate::types::MessageEffectTypePremiumSticker),
}
