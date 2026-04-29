#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a gift available for resale
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiftForResale {
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// Unique identifier of the received gift for the current user; only for the gifts owned by the current user
    pub received_gift_id: String,
}
