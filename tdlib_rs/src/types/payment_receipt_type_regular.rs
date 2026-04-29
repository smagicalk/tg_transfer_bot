#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The payment was done using a third-party payment provider
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentReceiptTypeRegular {
    /// User identifier of the payment provider bot
    pub payment_provider_user_id: i64,
    /// Information about the invoice
    pub invoice: crate::types::Invoice,
    /// Order information; may be null
    pub order_info: Option<crate::types::OrderInfo>,
    /// Chosen shipping option; may be null
    pub shipping_option: Option<crate::types::ShippingOption>,
    /// Title of the saved credentials chosen by the buyer
    pub credentials_title: String,
    /// The amount of tip chosen by the buyer in the smallest units of the currency
    pub tip_amount: i64,
}
