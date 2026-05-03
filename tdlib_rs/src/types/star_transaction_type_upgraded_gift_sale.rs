#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a sale of an upgraded gift; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeUpgradedGiftSale {
    /// Identifier of the user who bought the gift
    pub user_id: i64,
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// The number of Telegram Stars received by the Telegram for each 1000 Telegram Stars received by the seller of the gift
    pub commission_per_mille: i32,
    /// The Telegram Star amount that was received by Telegram; can be negative for refunds
    pub commission_star_amount: crate::types::StarAmount,
    /// True, if the gift was sold through a purchase offer
    pub via_offer: bool,
}
