#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftValueInfo {
    /// Contains information about value of an upgraded gift
    #[serde(rename(serialize = "upgradedGiftValueInfo", deserialize = "upgradedGiftValueInfo"))]
    UpgradedGiftValueInfo(crate::types::UpgradedGiftValueInfo),
}
