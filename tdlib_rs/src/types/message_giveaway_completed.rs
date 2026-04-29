#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A giveaway without public winners has been completed for the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageGiveawayCompleted {
    /// Identifier of the message with the giveaway; may be 0 or an identifier of a deleted message
    pub giveaway_message_id: i64,
    /// Number of winners in the giveaway
    pub winner_count: i32,
    /// True, if the giveaway is a Telegram Star giveaway
    pub is_star_giveaway: bool,
    /// Number of undistributed prizes; for Telegram Premium giveaways only
    pub unclaimed_prize_count: i32,
}
