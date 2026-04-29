#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about an effect added to a message
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageEffect {
    /// Unique identifier of the effect
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Static icon for the effect in WEBP format; may be null if none
    pub static_icon: Option<crate::types::Sticker>,
    /// Emoji corresponding to the effect that can be used if static icon isn't available
    pub emoji: String,
    /// True, if Telegram Premium subscription is required to use the effect
    pub is_premium: bool,
    /// Type of the effect
    pub r#type: crate::enums::MessageEffectType,
}
