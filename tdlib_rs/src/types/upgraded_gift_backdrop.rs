#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a backdrop of an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftBackdrop {
    /// Unique identifier of the backdrop
    pub id: i32,
    /// Name of the backdrop
    pub name: String,
    /// Colors of the backdrop
    pub colors: crate::types::UpgradedGiftBackdropColors,
    /// The rarity of the backdrop
    pub rarity: crate::enums::UpgradedGiftAttributeRarity,
}
