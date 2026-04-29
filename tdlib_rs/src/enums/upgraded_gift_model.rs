#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftModel {
    /// Describes a model of an upgraded gift
    #[serde(rename(serialize = "upgradedGiftModel", deserialize = "upgradedGiftModel"))]
    UpgradedGiftModel(crate::types::UpgradedGiftModel),
}
