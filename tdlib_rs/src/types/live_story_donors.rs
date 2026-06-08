#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of users and chats that spend most money on paid messages and reactions in a live story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LiveStoryDonors {
    /// Total amount of spend Telegram Stars
    pub total_star_count: i64,
    /// List of top donors in the live story
    pub top_donors: Vec<crate::types::PaidReactor>,
}
