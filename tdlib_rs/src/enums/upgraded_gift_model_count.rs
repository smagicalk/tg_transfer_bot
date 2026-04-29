#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftModelCount {
    /// Describes a model of an upgraded gift with the number of gifts found
    #[serde(rename(serialize = "upgradedGiftModelCount", deserialize = "upgradedGiftModelCount"))]
    UpgradedGiftModelCount(crate::types::UpgradedGiftModelCount),
}
