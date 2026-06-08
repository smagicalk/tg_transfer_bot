#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains result of gift upgrading
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradeGiftResult {
    /// The upgraded gift
    pub gift: crate::types::UpgradedGift,
    /// Unique identifier of the received gift for the current user
    pub received_gift_id: String,
    /// True, if the gift is displayed on the user's or the channel's profile page
    pub is_saved: bool,
    /// True, if the gift can be transferred to another owner
    pub can_be_transferred: bool,
    /// Number of Telegram Stars that must be paid to transfer the upgraded gift
    pub transfer_star_count: i64,
    /// Number of Telegram Stars that must be paid to drop original details of the upgraded gift; 0 if not available
    pub drop_original_details_star_count: i64,
    /// Point in time (Unix timestamp) when the gift can be transferred to another owner; can be in the past; 0 if the gift can be transferred immediately or transfer isn't possible
    pub next_transfer_date: i32,
    /// Point in time (Unix timestamp) when the gift can be resold to another user; can be in the past; 0 if the gift can't be resold; only for the receiver of the gift
    pub next_resale_date: i32,
    /// Point in time (Unix timestamp) when the gift can be transferred to the TON blockchain as an NFT; can be in the past
    pub export_date: i32,
}
