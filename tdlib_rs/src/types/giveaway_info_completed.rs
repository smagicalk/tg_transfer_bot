#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a completed giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayInfoCompleted {
    /// Point in time (Unix timestamp) when the giveaway was created
    pub creation_date: i32,
    /// Point in time (Unix timestamp) when the winners were selected. May be bigger than winners selection date specified in parameters of the giveaway
    pub actual_winners_selection_date: i32,
    /// True, if the giveaway was canceled and was fully refunded
    pub was_refunded: bool,
    /// True, if the current user is a winner of the giveaway
    pub is_winner: bool,
    /// Number of winners in the giveaway
    pub winner_count: i32,
    /// Number of winners, which activated their gift codes; for Telegram Premium giveaways only
    pub activation_count: i32,
    /// Telegram Premium gift code that was received by the current user; empty if the user isn't a winner in the giveaway or the giveaway isn't a Telegram Premium giveaway
    pub gift_code: String,
    /// The Telegram Star amount won by the current user; 0 if the user isn't a winner in the giveaway or the giveaway isn't a Telegram Star giveaway
    pub won_star_count: i64,
}
