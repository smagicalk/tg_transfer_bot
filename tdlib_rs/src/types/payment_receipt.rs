#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a successful payment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PaymentReceipt {
    /// Information about the product
    pub product_info: crate::types::ProductInfo,
    /// Point in time (Unix timestamp) when the payment was made
    pub date: i32,
    /// User identifier of the seller bot
    pub seller_bot_user_id: i64,
    /// Type of the payment receipt
    pub r#type: crate::enums::PaymentReceiptType,
}
