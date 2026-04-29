#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes an option for gifting Telegram Premium to a user. Use telegramPaymentPurposePremiumGift for out-of-store payments or payments in Telegram Stars
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PremiumGiftPaymentOption {
    /// ISO 4217 currency code for the payment
    pub currency: String,
    /// The amount to pay, in the smallest units of the currency
    pub amount: i64,
    /// The alternative Telegram Star amount to pay; 0 if payment in Telegram Stars is not possible
    pub star_count: i64,
    /// The discount associated with this option, as a percentage
    pub discount_percentage: i32,
    /// Number of months the Telegram Premium subscription will be active
    pub month_count: i32,
    /// Identifier of the store product associated with the option
    pub store_product_id: String,
    /// A sticker to be shown along with the option; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}
