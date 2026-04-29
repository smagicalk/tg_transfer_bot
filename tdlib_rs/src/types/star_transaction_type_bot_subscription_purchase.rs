#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a purchase of a subscription from a bot or a business account by the current user; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBotSubscriptionPurchase {
    /// Identifier of the bot or the business account user who created the subscription link
    pub user_id: i64,
    /// The number of seconds between consecutive Telegram Star debitings
    pub subscription_period: i32,
    /// Information about the bought subscription
    pub product_info: crate::types::ProductInfo,
}
