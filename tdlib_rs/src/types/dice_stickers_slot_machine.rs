#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Animated stickers to be combined into a slot machine
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiceStickersSlotMachine {
    /// The animated sticker with the slot machine background. The background animation must start playing after all reel animations finish
    pub background: crate::types::Sticker,
    /// The animated sticker with the lever animation. The lever animation must play once in the initial dice state
    pub lever: crate::types::Sticker,
    /// The animated sticker with the left reel
    pub left_reel: crate::types::Sticker,
    /// The animated sticker with the center reel
    pub center_reel: crate::types::Sticker,
    /// The animated sticker with the right reel
    pub right_reel: crate::types::Sticker,
}
