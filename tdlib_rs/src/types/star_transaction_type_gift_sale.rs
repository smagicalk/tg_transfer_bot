#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a sale of a received gift; relevant for regular users and channel chats only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftSale {
    /// Identifier of the user who sent the gift
    pub user_id: i64,
    /// The gift
    pub gift: crate::types::Gift,
}
