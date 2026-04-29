#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a drop of original details of an upgraded gift; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftOriginalDetailsDrop {
    /// Identifier of the user or the channel that owns the gift
    pub owner_id: crate::enums::MessageSender,
    /// The gift
    pub gift: crate::types::UpgradedGift,
}
