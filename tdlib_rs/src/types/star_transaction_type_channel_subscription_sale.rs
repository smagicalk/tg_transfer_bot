#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a sale of a subscription by the channel chat; relevant for channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeChannelSubscriptionSale {
    /// Identifier of the user who bought the subscription
    pub user_id: i64,
    /// The number of seconds between consecutive Telegram Star debitings
    pub subscription_period: i32,
}
