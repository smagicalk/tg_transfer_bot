#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CollectibleItemInfo {
    /// Contains information about a collectible item and its last purchase
    #[serde(rename(serialize = "collectibleItemInfo", deserialize = "collectibleItemInfo"))]
    CollectibleItemInfo(crate::types::CollectibleItemInfo),
}
