#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user creating a Telegram Premium giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorePaymentPurposePremiumGiveaway {
    /// Giveaway parameters
    pub parameters: crate::types::GiveawayParameters,
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
}
