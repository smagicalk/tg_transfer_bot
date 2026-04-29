#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftBackdrop {
    /// Describes a backdrop of an upgraded gift
    #[serde(rename(serialize = "upgradedGiftBackdrop", deserialize = "upgradedGiftBackdrop"))]
    UpgradedGiftBackdrop(crate::types::UpgradedGiftBackdrop),
}
