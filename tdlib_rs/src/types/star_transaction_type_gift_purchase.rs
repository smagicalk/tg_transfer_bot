#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a purchase of a regular gift; relevant for regular users and bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftPurchase {
    /// Identifier of the user or the channel that received the gift
    pub owner_id: crate::enums::MessageSender,
    /// The gift
    pub gift: crate::types::Gift,
}
