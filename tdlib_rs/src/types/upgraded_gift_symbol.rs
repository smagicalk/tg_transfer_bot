#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a symbol shown on the pattern of an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftSymbol {
    /// Name of the symbol
    pub name: String,
    /// The sticker representing the symbol
    pub sticker: crate::types::Sticker,
    /// The rarity of the symbol
    pub rarity: crate::enums::UpgradedGiftAttributeRarity,
}
