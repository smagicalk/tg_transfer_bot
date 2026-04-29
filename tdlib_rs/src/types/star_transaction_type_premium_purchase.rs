#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a purchase of Telegram Premium subscription; relevant for regular users and bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePremiumPurchase {
    /// Identifier of the user who received the Telegram Premium subscription
    pub user_id: i64,
    /// Number of months the Telegram Premium subscription will be active
    pub month_count: i32,
    /// A sticker to be shown in the transaction information; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}
