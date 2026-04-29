#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a purchase of a product from a bot or a business account by the current user; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBotInvoicePurchase {
    /// Identifier of the bot or the business account user who created the invoice
    pub user_id: i64,
    /// Information about the bought product
    pub product_info: crate::types::ProductInfo,
}
