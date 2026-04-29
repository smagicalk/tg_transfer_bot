#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGift {
    /// Describes an upgraded gift that can be transferred to another owner or transferred to the TON blockchain as an NFT
    #[serde(rename(serialize = "upgradedGift", deserialize = "upgradedGift"))]
    UpgradedGift(crate::types::UpgradedGift),
}
