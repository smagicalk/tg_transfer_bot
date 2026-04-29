#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Crafting was successful
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CraftGiftResultSuccess {
    /// The created gift
    pub gift: crate::types::UpgradedGift,
    /// Unique identifier of the received gift for the current user
    pub received_gift_id: String,
}
