#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a model of an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftModel {
    /// Name of the model
    pub name: String,
    /// The sticker representing the upgraded gift
    pub sticker: crate::types::Sticker,
    /// The rarity of the model
    pub rarity: crate::enums::UpgradedGiftAttributeRarity,
    /// True, if the model can be obtained only through gift crafting
    pub is_crafted: bool,
}
