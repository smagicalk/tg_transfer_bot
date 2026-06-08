#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a sale of a subscription by the bot; relevant for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBotSubscriptionSale {
    /// Identifier of the user who bought the subscription
    pub user_id: i64,
    /// The number of seconds between consecutive Telegram Star debitings
    pub subscription_period: i32,
    /// Information about the bought subscription
    pub product_info: crate::types::ProductInfo,
    /// Invoice payload
    pub invoice_payload: String,
    /// Information about the affiliate which received commission from the transaction; may be null if none
    pub affiliate: Option<crate::types::AffiliateInfo>,
}
