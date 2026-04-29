#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a transfer of an upgraded gift; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftTransfer {
    /// Identifier of the user or the channel that received the gift
    pub owner_id: crate::enums::MessageSender,
    /// The gift
    pub gift: crate::types::UpgradedGift,
}
