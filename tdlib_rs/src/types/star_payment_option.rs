#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an option for buying Telegram Stars. Use telegramPaymentPurposeStars for out-of-store payments
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarPaymentOption {
    /// ISO 4217 currency code for the payment
    pub currency: String,
    /// The amount to pay, in the smallest units of the currency
    pub amount: i64,
    /// Number of Telegram Stars that will be purchased
    pub star_count: i64,
    /// Identifier of the store product associated with the option; may be empty if none
    pub store_product_id: String,
    /// True, if the option must be shown only in the full list of payment options
    pub is_additional: bool,
}
