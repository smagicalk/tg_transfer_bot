#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a Telegram Premium gift code
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumGiftCodeInfo {
    /// Identifier of a chat or a user who created the gift code; may be null if unknown. If null and the code is from messagePremiumGiftCode message, then creator_id from the message can be used
    pub creator_id: Option<crate::enums::MessageSender>,
    /// Point in time (Unix timestamp) when the code was created
    pub creation_date: i32,
    /// True, if the gift code was created for a giveaway
    pub is_from_giveaway: bool,
    /// Identifier of the corresponding giveaway message in the creator_id chat; may be 0 or an identifier of a deleted message
    pub giveaway_message_id: i64,
    /// Number of months the Telegram Premium subscription will be active after code activation; 0 if the number of months isn't integer
    pub month_count: i32,
    /// Number of days the Telegram Premium subscription will be active after code activation
    pub day_count: i32,
    /// Identifier of a user for which the code was created; 0 if none
    pub user_id: i64,
    /// Point in time (Unix timestamp) when the code was activated; 0 if none
    pub use_date: i32,
}
