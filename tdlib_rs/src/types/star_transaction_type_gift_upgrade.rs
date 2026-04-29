#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is an upgrade of a gift; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftUpgrade {
    /// Identifier of the user who initially sent the gift
    pub user_id: i64,
    /// The upgraded gift
    pub gift: crate::types::UpgradedGift,
}
