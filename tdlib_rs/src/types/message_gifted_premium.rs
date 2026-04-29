#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Telegram Premium was gifted to a user
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGiftedPremium {
    /// The identifier of a user who gifted Telegram Premium; 0 if the gift was anonymous or is outgoing
    pub gifter_user_id: i64,
    /// The identifier of a user who received Telegram Premium; 0 if the gift is incoming
    pub receiver_user_id: i64,
    /// Message added to the gifted Telegram Premium by the sender
    pub text: crate::types::FormattedText,
    /// Currency for the paid amount
    pub currency: String,
    /// The paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Cryptocurrency used to pay for the gift; may be empty if none
    pub cryptocurrency: String,
    /// The paid amount, in the smallest units of the cryptocurrency; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub cryptocurrency_amount: i64,
    /// Number of months the Telegram Premium subscription will be active after code activation; 0 if the number of months isn't integer
    pub month_count: i32,
    /// Number of days the Telegram Premium subscription will be active
    pub day_count: i32,
    /// A sticker to be shown in the message; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}
