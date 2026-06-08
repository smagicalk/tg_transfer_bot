#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An effect from a premium sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageEffectTypePremiumSticker {
    /// The premium sticker. The effect can be found at sticker.full_type.premium_animation
    pub sticker: crate::types::Sticker,
}
