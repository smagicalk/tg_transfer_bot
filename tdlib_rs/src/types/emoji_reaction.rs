#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about an emoji reaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmojiReaction {
    /// Text representation of the reaction
    pub emoji: String,
    /// Reaction title
    pub title: String,
    /// True, if the reaction can be added to new messages and enabled in chats
    pub is_active: bool,
    /// Static icon for the reaction
    pub static_icon: crate::types::Sticker,
    /// Appear animation for the reaction
    pub appear_animation: crate::types::Sticker,
    /// Select animation for the reaction
    pub select_animation: crate::types::Sticker,
    /// Activate animation for the reaction
    pub activate_animation: crate::types::Sticker,
    /// Effect animation for the reaction
    pub effect_animation: crate::types::Sticker,
    /// Around animation for the reaction; may be null
    pub around_animation: Option<crate::types::Sticker>,
    /// Center animation for the reaction; may be null
    pub center_animation: Option<crate::types::Sticker>,
}
