#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a receiving of a paid reaction to a message by the channel chat; relevant for channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeChannelPaidReactionReceive {
    /// Identifier of the user who added the paid reaction
    pub user_id: i64,
    /// Identifier of the reacted message; may be 0 or an identifier of a deleted message
    pub message_id: i64,
}
