#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftSymbolCount {
    /// Describes a symbol shown on the pattern of an upgraded gift
    #[serde(rename(
        serialize = "upgradedGiftSymbolCount",
        deserialize = "upgradedGiftSymbolCount"
    ))]
    UpgradedGiftSymbolCount(crate::types::UpgradedGiftSymbolCount),
}
