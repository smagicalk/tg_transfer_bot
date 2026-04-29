#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a sale of paid media by the bot or a business account managed by the bot; relevant for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBotPaidMediaSale {
    /// Identifier of the user who bought the media
    pub user_id: i64,
    /// The bought media
    pub media: Vec<crate::enums::PaidMedia>,
    /// Bot-provided payload
    pub payload: String,
    /// Information about the affiliate which received commission from the transaction; may be null if none
    pub affiliate: Option<crate::types::AffiliateInfo>,
}
