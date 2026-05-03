#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about an invoice payment form
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PaymentForm {
    /// The payment form identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Type of the payment form
    pub r#type: crate::enums::PaymentFormType,
    /// User identifier of the seller bot
    pub seller_bot_user_id: i64,
    /// Information about the product
    pub product_info: crate::types::ProductInfo,
}
