#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a sale of a product by the bot; relevant for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBotInvoiceSale {
    /// Identifier of the user who bought the product
    pub user_id: i64,
    /// Information about the bought product
    pub product_info: crate::types::ProductInfo,
    /// Invoice payload
    pub invoice_payload: String,
    /// Information about the affiliate which received commission from the transaction; may be null if none
    pub affiliate: Option<crate::types::AffiliateInfo>,
}
