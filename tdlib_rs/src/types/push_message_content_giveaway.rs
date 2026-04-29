#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentGiveaway {
    /// Number of users which will receive giveaway prizes; 0 for pinned message
    pub winner_count: i32,
    /// Prize of the giveaway; may be null for pinned message
    pub prize: Option<crate::enums::GiveawayPrize>,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
