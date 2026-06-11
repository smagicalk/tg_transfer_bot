#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A payment has been received by the bot or the business account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePaymentSuccessfulBot {
    /// Currency for price of the product
    pub currency: String,
    /// Total price for the product, in the smallest units of the currency
    pub total_amount: i64,
    /// Point in time (Unix timestamp) when the subscription will expire; 0 if unknown or the payment isn't recurring
    pub subscription_until_date: i32,
    /// True, if this is a recurring payment
    pub is_recurring: bool,
    /// True, if this is the first recurring payment
    pub is_first_recurring: bool,
    /// Invoice payload
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user; may be empty if not applicable; for bots only
    pub shipping_option_id: String,
    /// Information about the order; may be null; for bots only
    pub order_info: Option<crate::types::OrderInfo>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}
