#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftBackdropColors {
    /// Describes colors of a backdrop of an upgraded gift
    #[serde(rename(serialize = "upgradedGiftBackdropColors", deserialize = "upgradedGiftBackdropColors"))]
    UpgradedGiftBackdropColors(crate::types::UpgradedGiftBackdropColors),
}
