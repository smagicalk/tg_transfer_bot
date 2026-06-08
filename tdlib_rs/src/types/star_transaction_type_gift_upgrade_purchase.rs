#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a purchase of an upgrade of a gift owned by another user or channel; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftUpgradePurchase {
    /// Owner of the upgraded gift
    pub owner_id: crate::enums::MessageSender,
    /// The gift
    pub gift: crate::types::Gift,
}
