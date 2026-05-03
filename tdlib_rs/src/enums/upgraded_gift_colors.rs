#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftColors {
    /// Contains information about color scheme for user's name, background of empty chat photo, replies to messages and link previews
    #[serde(rename(serialize = "upgradedGiftColors", deserialize = "upgradedGiftColors"))]
    UpgradedGiftColors(crate::types::UpgradedGiftColors),
}
