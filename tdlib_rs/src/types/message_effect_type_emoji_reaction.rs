#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An effect from an emoji reaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageEffectTypeEmojiReaction {
    /// Select animation for the effect in TGS format
    pub select_animation: crate::types::Sticker,
    /// Effect animation for the effect in TGS format
    pub effect_animation: crate::types::Sticker,
}
