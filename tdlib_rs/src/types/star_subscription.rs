#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about subscription to a channel chat, a bot, or a business account that was paid in Telegram Stars
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarSubscription {
    /// Unique identifier of the subscription
    pub id: String,
    /// Identifier of the chat that is subscribed
    pub chat_id: i64,
    /// Point in time (Unix timestamp) when the subscription will expire or expired
    pub expiration_date: i32,
    /// True, if the subscription was canceled
    pub is_canceled: bool,
    /// True, if the subscription expires soon and there are no enough Telegram Stars on the user's balance to extend it
    pub is_expiring: bool,
    /// The subscription plan
    pub pricing: crate::types::StarSubscriptionPricing,
    /// Type of the subscription
    pub r#type: crate::enums::StarSubscriptionType,
}
