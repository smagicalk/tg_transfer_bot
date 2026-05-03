#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DiceStickers {
    /// A regular animated sticker
    #[serde(rename(serialize = "diceStickersRegular", deserialize = "diceStickersRegular"))]
    Regular(crate::types::DiceStickersRegular),
    /// Animated stickers to be combined into a slot machine
    #[serde(rename(
        serialize = "diceStickersSlotMachine",
        deserialize = "diceStickersSlotMachine"
    ))]
    SlotMachine(crate::types::DiceStickersSlotMachine),
}
