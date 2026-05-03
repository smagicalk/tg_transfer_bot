#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Telegram Stars were gifted to a user
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGiftedStars {
    /// The identifier of a user who gifted Telegram Stars; 0 if the gift was anonymous or is outgoing
    pub gifter_user_id: i64,
    /// The identifier of a user who received Telegram Stars; 0 if the gift is incoming
    pub receiver_user_id: i64,
    /// Currency for the paid amount
    pub currency: String,
    /// The paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Cryptocurrency used to pay for the gift; may be empty if none
    pub cryptocurrency: String,
    /// The paid amount, in the smallest units of the cryptocurrency; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub cryptocurrency_amount: i64,
    /// Number of Telegram Stars that were gifted
    pub star_count: i64,
    /// Identifier of the transaction for Telegram Stars purchase; for receiver only
    pub transaction_id: String,
    /// A sticker to be shown in the message; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}
