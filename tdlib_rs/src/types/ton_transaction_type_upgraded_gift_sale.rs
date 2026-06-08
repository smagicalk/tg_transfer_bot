#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a sale of an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeUpgradedGiftSale {
    /// Identifier of the user who bought the gift
    pub user_id: i64,
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// The number of Toncoins received by the Telegram for each 1000 Toncoins received by the seller of the gift
    pub commission_per_mille: i32,
    /// The Toncoin amount that was received by the Telegram; in the smallest units of the currency
    pub commission_toncoin_amount: i64,
    /// True, if the gift was sold through a purchase offer
    pub via_offer: bool,
}
