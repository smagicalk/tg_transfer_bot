#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewPreCheckoutQuery {
    /// Unique query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the user who sent the query
    pub sender_user_id: i64,
    /// Currency for the product price
    pub currency: String,
    /// Total price for the product, in the smallest units of the currency
    pub total_amount: i64,
    /// Invoice payload
    pub invoice_payload: String,
    /// Identifier of a shipping option chosen by the user; may be empty if not applicable
    pub shipping_option_id: String,
    /// Information about the order; may be null
    pub order_info: Option<crate::types::OrderInfo>,
}
