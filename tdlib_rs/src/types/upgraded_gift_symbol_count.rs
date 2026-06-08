#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a symbol shown on the pattern of an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftSymbolCount {
    /// The symbol
    pub symbol: crate::types::UpgradedGiftSymbol,
    /// Total number of gifts with the symbol
    pub total_count: i32,
}
