#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a deposit of Telegram Stars from a giveaway; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiveawayDeposit {
    /// Identifier of a supergroup or a channel chat that created the giveaway
    pub chat_id: i64,
    /// Identifier of the message with the giveaway; may be 0 or an identifier of a deleted message
    pub giveaway_message_id: i64,
}
