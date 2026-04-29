#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user buying Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorePaymentPurposeStars {
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Number of bought Telegram Stars
    pub star_count: i64,
    /// Identifier of the chat that is supposed to receive the Telegram Stars; pass 0 if none
    pub chat_id: i64,
}
