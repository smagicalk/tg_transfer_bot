#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A giveaway with public winners has been completed for the chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGiveawayWinners {
    /// Identifier of the supergroup or channel chat, which was automatically boosted by the winners of the giveaway
    pub boosted_chat_id: i64,
    /// Identifier of the message with the giveaway in the boosted chat
    pub giveaway_message_id: i64,
    /// Number of other chats that participated in the giveaway
    pub additional_chat_count: i32,
    /// Point in time (Unix timestamp) when the winners were selected. May be bigger than winners selection date specified in parameters of the giveaway
    pub actual_winners_selection_date: i32,
    /// True, if only new members of the chats were eligible for the giveaway
    pub only_new_members: bool,
    /// True, if the giveaway was canceled and was fully refunded
    pub was_refunded: bool,
    /// Prize of the giveaway
    pub prize: crate::enums::GiveawayPrize,
    /// Additional description of the giveaway prize
    pub prize_description: String,
    /// Total number of winners in the giveaway
    pub winner_count: i32,
    /// Up to 100 user identifiers of the winners of the giveaway
    pub winner_user_ids: Vec<i64>,
    /// Number of undistributed prizes; for Telegram Premium giveaways only
    pub unclaimed_prize_count: i32,
}
