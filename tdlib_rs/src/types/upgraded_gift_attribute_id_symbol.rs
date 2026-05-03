#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Identifier of a gift symbol
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftAttributeIdSymbol {
    /// Identifier of the sticker representing the symbol
    #[serde_as(as = "DisplayFromStr")]
    pub sticker_id: i64,
}
