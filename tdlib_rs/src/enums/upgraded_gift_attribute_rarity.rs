#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftAttributeRarity {
    /// The rarity is represented as the numeric frequence of the model
    #[serde(rename(serialize = "upgradedGiftAttributeRarityPerMille", deserialize = "upgradedGiftAttributeRarityPerMille"))]
    PerMille(crate::types::UpgradedGiftAttributeRarityPerMille),
    /// The attribute is uncommon
    #[serde(rename(serialize = "upgradedGiftAttributeRarityUncommon", deserialize = "upgradedGiftAttributeRarityUncommon"))]
    Uncommon,
    /// The attribute is rare
    #[serde(rename(serialize = "upgradedGiftAttributeRarityRare", deserialize = "upgradedGiftAttributeRarityRare"))]
    Rare,
    /// The attribute is epic
    #[serde(rename(serialize = "upgradedGiftAttributeRarityEpic", deserialize = "upgradedGiftAttributeRarityEpic"))]
    Epic,
    /// The attribute is legendary
    #[serde(rename(serialize = "upgradedGiftAttributeRarityLegendary", deserialize = "upgradedGiftAttributeRarityLegendary"))]
    Legendary,
}
