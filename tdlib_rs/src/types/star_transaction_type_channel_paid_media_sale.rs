#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a sale of paid media by the channel chat; relevant for channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeChannelPaidMediaSale {
    /// Identifier of the user who bought the media
    pub user_id: i64,
    /// Identifier of the corresponding message with paid media; may be 0 or an identifier of a deleted message
    pub message_id: i64,
    /// The bought media
    pub media: Vec<crate::enums::PaidMedia>,
}
