#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An upgraded gift set as emoji status
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiStatusTypeUpgradedGift {
    /// Identifier of the upgraded gift
    #[serde_as(as = "DisplayFromStr")]
    pub upgraded_gift_id: i64,
    /// The title of the upgraded gift
    pub gift_title: String,
    /// Unique name of the upgraded gift that can be used with internalLinkTypeUpgradedGift
    pub gift_name: String,
    /// Custom emoji identifier of the model of the upgraded gift
    #[serde_as(as = "DisplayFromStr")]
    pub model_custom_emoji_id: i64,
    /// Custom emoji identifier of the symbol of the upgraded gift
    #[serde_as(as = "DisplayFromStr")]
    pub symbol_custom_emoji_id: i64,
    /// Colors of the backdrop of the upgraded gift
    pub backdrop_colors: crate::types::UpgradedGiftBackdropColors,
}
