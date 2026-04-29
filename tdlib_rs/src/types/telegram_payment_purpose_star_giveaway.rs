#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user creating a Telegram Star giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TelegramPaymentPurposeStarGiveaway {
    /// Giveaway parameters
    pub parameters: crate::types::GiveawayParameters,
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// The number of users to receive Telegram Stars
    pub winner_count: i32,
    /// The number of Telegram Stars to be distributed through the giveaway
    pub star_count: i64,
}
