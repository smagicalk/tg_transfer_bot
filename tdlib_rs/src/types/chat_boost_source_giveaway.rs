#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat created a giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostSourceGiveaway {
    /// Identifier of a user who won in the giveaway; 0 if none
    pub user_id: i64,
    /// The created Telegram Premium gift code if it was used by the user or can be claimed by the current user; an empty string otherwise; for Telegram Premium giveways only
    pub gift_code: String,
    /// Number of Telegram Stars distributed among winners of the giveaway
    pub star_count: i64,
    /// Identifier of the corresponding giveaway message; can be an identifier of a deleted message
    pub giveaway_message_id: i64,
    /// True, if the winner for the corresponding giveaway prize wasn't chosen, because there were not enough participants
    pub is_unclaimed: bool,
}
