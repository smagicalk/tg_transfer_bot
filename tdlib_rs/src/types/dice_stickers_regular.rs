#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A regular animated sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiceStickersRegular {
    /// The animated sticker with the dice animation
    pub sticker: crate::types::Sticker,
}
