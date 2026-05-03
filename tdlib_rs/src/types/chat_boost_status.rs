#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes current boost status of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostStatus {
    /// An HTTP URL, which can be used to boost the chat
    pub boost_url: String,
    /// Identifiers of boost slots of the current user applied to the chat
    pub applied_slot_ids: Vec<i32>,
    /// Current boost level of the chat
    pub level: i32,
    /// The number of boosts received by the chat from created Telegram Premium gift codes and giveaways; always 0 if the current user isn't an administrator in the chat
    pub gift_code_boost_count: i32,
    /// The number of boosts received by the chat
    pub boost_count: i32,
    /// The number of boosts added to reach the current level
    pub current_level_boost_count: i32,
    /// The number of boosts needed to reach the next level; 0 if the next level isn't available
    pub next_level_boost_count: i32,
    /// Approximate number of Telegram Premium subscribers joined the chat; always 0 if the current user isn't an administrator in the chat
    pub premium_member_count: i32,
    /// A percentage of Telegram Premium subscribers joined the chat; always 0 if the current user isn't an administrator in the chat
    pub premium_member_percentage: f64,
    /// The list of prepaid giveaways available for the chat; only for chat administrators
    pub prepaid_giveaways: Vec<crate::types::PrepaidGiveaway>,
}
