#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A payment has been sent to a bot or a business account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePaymentSuccessful {
    /// Identifier of the chat, containing the corresponding invoice message
    pub invoice_chat_id: i64,
    /// Identifier of the message with the corresponding invoice; may be 0 or an identifier of a deleted message
    pub invoice_message_id: i64,
    /// Currency for the price of the product
    pub currency: String,
    /// Total price for the product, in the smallest units of the currency
    pub total_amount: i64,
    /// Point in time (Unix timestamp) when the subscription will expire; 0 if unknown or the payment isn't recurring
    pub subscription_until_date: i32,
    /// True, if this is a recurring payment
    pub is_recurring: bool,
    /// True, if this is the first recurring payment
    pub is_first_recurring: bool,
    /// Name of the invoice; may be empty if unknown
    pub invoice_name: String,
}
