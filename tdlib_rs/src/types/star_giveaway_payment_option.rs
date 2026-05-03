#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an option for creating of Telegram Star giveaway. Use telegramPaymentPurposeStarGiveaway for out-of-store payments
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarGiveawayPaymentOption {
    /// ISO 4217 currency code for the payment
    pub currency: String,
    /// The amount to pay, in the smallest units of the currency
    pub amount: i64,
    /// Number of Telegram Stars that will be distributed among winners
    pub star_count: i64,
    /// Identifier of the store product associated with the option; may be empty if none
    pub store_product_id: String,
    /// Number of times the chat will be boosted for one year if the option is chosen
    pub yearly_boost_count: i32,
    /// Allowed options for the number of giveaway winners
    pub winner_options: Vec<crate::types::StarGiveawayWinnerOption>,
    /// True, if the option must be chosen by default
    pub is_default: bool,
    /// True, if the option must be shown only in the full list of payment options
    pub is_additional: bool,
}
