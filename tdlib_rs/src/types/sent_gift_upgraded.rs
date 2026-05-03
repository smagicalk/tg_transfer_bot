#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SentGiftUpgraded {
    /// The gift
    pub gift: crate::types::UpgradedGift,
}
