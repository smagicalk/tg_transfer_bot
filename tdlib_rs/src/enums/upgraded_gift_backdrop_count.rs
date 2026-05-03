#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftBackdropCount {
    /// Describes a backdrop of an upgraded gift
    #[serde(rename(
        serialize = "upgradedGiftBackdropCount",
        deserialize = "upgradedGiftBackdropCount"
    ))]
    UpgradedGiftBackdropCount(crate::types::UpgradedGiftBackdropCount),
}
