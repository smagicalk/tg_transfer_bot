#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Identifier of a gift model
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftAttributeIdModel {
    /// Identifier of the sticker representing the model
    #[serde_as(as = "DisplayFromStr")]
    pub sticker_id: i64,
}
