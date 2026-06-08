#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a purchase of a subscription to a channel chat by the current user; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeChannelSubscriptionPurchase {
    /// Identifier of the channel chat that created the subscription
    pub chat_id: i64,
    /// The number of seconds between consecutive Telegram Star debitings
    pub subscription_period: i32,
}
