#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a backdrop of an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftBackdropCount {
    /// The backdrop
    pub backdrop: crate::types::UpgradedGiftBackdrop,
    /// Total number of gifts with the symbol
    pub total_count: i32,
}
