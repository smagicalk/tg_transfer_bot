#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A Telegram Stars were received by the current user from a giveaway
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGiveawayPrizeStars {
    /// Number of Telegram Stars that were received
    pub star_count: i64,
    /// Identifier of the transaction for Telegram Stars credit
    pub transaction_id: String,
    /// Identifier of the supergroup or channel chat, which was automatically boosted by the winners of the giveaway
    pub boosted_chat_id: i64,
    /// Identifier of the message with the giveaway in the boosted chat; may be 0 or an identifier of a deleted message
    pub giveaway_message_id: i64,
    /// True, if the corresponding winner wasn't chosen and the Telegram Stars were received by the owner of the boosted chat
    pub is_unclaimed: bool,
    /// A sticker to be shown in the message; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}
