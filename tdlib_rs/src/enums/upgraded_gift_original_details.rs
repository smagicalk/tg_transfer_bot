#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftOriginalDetails {
    /// Describes the original details about the gift
    #[serde(rename(
        serialize = "upgradedGiftOriginalDetails",
        deserialize = "upgradedGiftOriginalDetails"
    ))]
    UpgradedGiftOriginalDetails(crate::types::UpgradedGiftOriginalDetails),
}
