#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user buying Telegram Stars for other users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TelegramPaymentPurposeGiftedStars {
    /// Identifier of the user to which Telegram Stars are gifted
    pub user_id: i64,
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Number of bought Telegram Stars
    pub star_count: i64,
}
