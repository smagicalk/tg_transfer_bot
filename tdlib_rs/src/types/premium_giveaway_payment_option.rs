#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an option for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members. Use telegramPaymentPurposePremiumGiftCodes or telegramPaymentPurposePremiumGiveaway for out-of-store payments
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumGiveawayPaymentOption {
    /// ISO 4217 currency code for Telegram Premium gift code payment
    pub currency: String,
    /// The amount to pay, in the smallest units of the currency
    pub amount: i64,
    /// Number of users which will be able to activate the gift codes
    pub winner_count: i32,
    /// Number of months the Telegram Premium subscription will be active
    pub month_count: i32,
    /// Identifier of the store product associated with the option; may be empty if none
    pub store_product_id: String,
    /// Number of times the store product must be paid
    pub store_product_quantity: i32,
}
