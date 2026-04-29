#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes an option for buying Telegram Premium to a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumPaymentOption {
    /// ISO 4217 currency code for Telegram Premium subscription payment
    pub currency: String,
    /// The amount to pay, in the smallest units of the currency
    pub amount: i64,
    /// The discount associated with this option, as a percentage
    pub discount_percentage: i32,
    /// Number of months the Telegram Premium subscription will be active. Use getPremiumInfoSticker to get the sticker to be used as representation of the Telegram Premium subscription
    pub month_count: i32,
    /// Identifier of the store product associated with the option
    pub store_product_id: String,
    /// An internal link to be opened for buying Telegram Premium to the user if store payment isn't possible; may be null if direct payment isn't available
    pub payment_link: Option<crate::enums::InternalLinkType>,
}
