#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftAttributeId {
    /// Identifier of a gift model
    #[serde(rename(
        serialize = "upgradedGiftAttributeIdModel",
        deserialize = "upgradedGiftAttributeIdModel"
    ))]
    Model(crate::types::UpgradedGiftAttributeIdModel),
    /// Identifier of a gift symbol
    #[serde(rename(
        serialize = "upgradedGiftAttributeIdSymbol",
        deserialize = "upgradedGiftAttributeIdSymbol"
    ))]
    Symbol(crate::types::UpgradedGiftAttributeIdSymbol),
    /// Identifier of a gift backdrop
    #[serde(rename(
        serialize = "upgradedGiftAttributeIdBackdrop",
        deserialize = "upgradedGiftAttributeIdBackdrop"
    ))]
    Backdrop(crate::types::UpgradedGiftAttributeIdBackdrop),
}
