#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftSymbol {
    /// Describes a symbol shown on the pattern of an upgraded gift
    #[serde(rename(serialize = "upgradedGiftSymbol", deserialize = "upgradedGiftSymbol"))]
    UpgradedGiftSymbol(crate::types::UpgradedGiftSymbol),
}
