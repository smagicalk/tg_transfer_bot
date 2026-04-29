#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An upgraded gift was received or sent by the current user, or the current user was notified about a channel gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageUpgradedGift {
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// Sender of the gift; may be null for anonymous gifts
    pub sender_id: Option<crate::enums::MessageSender>,
    /// Receiver of the gift
    pub receiver_id: crate::enums::MessageSender,
    /// Origin of the upgraded gift
    pub origin: crate::enums::UpgradedGiftOrigin,
    /// Unique identifier of the received gift for the current user; only for the receiver of the gift
    pub received_gift_id: String,
    /// True, if the gift is displayed on the user's or the channel's profile page; only for the receiver of the gift
    pub is_saved: bool,
    /// True, if the gift can be transferred to another owner; only for the receiver of the gift
    pub can_be_transferred: bool,
    /// True, if the gift has already been transferred to another owner; only for the receiver of the gift
    pub was_transferred: bool,
    /// Number of Telegram Stars that must be paid to transfer the upgraded gift; only for the receiver of the gift
    pub transfer_star_count: i64,
    /// Number of Telegram Stars that must be paid to drop original details of the upgraded gift; 0 if not available; only for the receiver of the gift
    pub drop_original_details_star_count: i64,
    /// Point in time (Unix timestamp) when the gift can be transferred to another owner; can be in the past; 0 if the gift can be transferred immediately or transfer isn't possible; only for the receiver of the gift
    pub next_transfer_date: i32,
    /// Point in time (Unix timestamp) when the gift can be resold to another user; can be in the past; 0 if the gift can't be resold; only for the receiver of the gift
    pub next_resale_date: i32,
    /// Point in time (Unix timestamp) when the gift can be transferred to the TON blockchain as an NFT; can be in the past; 0 if NFT export isn't possible; only for the receiver of the gift
    pub export_date: i32,
    /// Point in time (Unix timestamp) when the gift can be used to craft another gift can be in the past; only for the receiver of the gift
    pub craft_date: i32,
}
