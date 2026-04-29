#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradeGiftResult {
    /// Contains result of gift upgrading
    #[serde(rename(serialize = "upgradeGiftResult", deserialize = "upgradeGiftResult"))]
    UpgradeGiftResult(crate::types::UpgradeGiftResult),
}
