#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a purchase of an upgraded gift for some user or channel
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeUpgradedGiftPurchase {
    /// Identifier of the user who sold the gift
    pub user_id: i64,
    /// The gift
    pub gift: crate::types::UpgradedGift,
}
