#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a sending of a paid reaction to a message in a channel chat by the current user; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeChannelPaidReactionSend {
    /// Identifier of the channel chat
    pub chat_id: i64,
    /// Identifier of the reacted message; may be 0 or an identifier of a deleted message
    pub message_id: i64,
}
