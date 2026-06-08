#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a purchase of paid media from a bot or a business account by the current user; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBotPaidMediaPurchase {
    /// Identifier of the bot or the business account user who sent the paid media
    pub user_id: i64,
    /// The bought media if the transaction wasn't refunded
    pub media: Vec<crate::enums::PaidMedia>,
}
