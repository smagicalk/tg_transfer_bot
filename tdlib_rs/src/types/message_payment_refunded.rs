#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A payment has been refunded
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessagePaymentRefunded {
    /// Identifier of the previous owner of the Telegram Stars that refunds them
    pub owner_id: crate::enums::MessageSender,
    /// Currency for the price of the product
    pub currency: String,
    /// Total price for the product, in the smallest units of the currency
    pub total_amount: i64,
    /// Invoice payload; only for bots
    pub invoice_payload: String,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}
