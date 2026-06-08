#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a gift received by a user or a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ReceivedGift {
    /// Unique identifier of the received gift for the current user; only for the receiver of the gift
    pub received_gift_id: String,
    /// Identifier of a user or a chat that sent the gift; may be null if unknown
    pub sender_id: Option<crate::enums::MessageSender>,
    /// Message added to the gift
    pub text: crate::types::FormattedText,
    /// Unique number of the gift among gifts upgraded from the same gift after upgrade; 0 if yet unassigned
    pub unique_gift_number: i32,
    /// True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone are able to see them
    pub is_private: bool,
    /// True, if the gift is displayed on the chat's profile page; only for the receiver of the gift
    pub is_saved: bool,
    /// True, if the gift is pinned to the top of the chat's profile page
    pub is_pinned: bool,
    /// True, if the gift is a regular gift that can be upgraded to a unique gift; only for the receiver of the gift
    pub can_be_upgraded: bool,
    /// True, if the gift is an upgraded gift that can be transferred to another owner; only for the receiver of the gift
    pub can_be_transferred: bool,
    /// True, if the gift was refunded and isn't available anymore
    pub was_refunded: bool,
    /// Point in time (Unix timestamp) when the gift was sent
    pub date: i32,
    /// The gift
    pub gift: crate::enums::SentGift,
    /// Identifiers of collections to which the gift is added; only for the receiver of the gift
    pub collection_ids: Vec<i32>,
    /// Number of Telegram Stars that can be claimed by the receiver instead of the regular gift; 0 if the gift can't be sold by the current user
    pub sell_star_count: i64,
    /// Number of Telegram Stars that were paid by the sender for the ability to upgrade the gift
    pub prepaid_upgrade_star_count: i64,
    /// True, if the upgrade was bought after the gift was sent. In this case, prepaid upgrade cost must not be added to the gift cost
    pub is_upgrade_separate: bool,
    /// Number of Telegram Stars that must be paid to transfer the upgraded gift; only for the receiver of the gift
    pub transfer_star_count: i64,
    /// Number of Telegram Stars that must be paid to drop original details of the upgraded gift; 0 if not available; only for the receiver of the gift
    pub drop_original_details_star_count: i64,
    /// Point in time (Unix timestamp) when the gift can be transferred to another owner; can be in the past; 0 if the gift can be transferred immediately or transfer isn't possible; only for the receiver of the gift
    pub next_transfer_date: i32,
    /// Point in time (Unix timestamp) when the gift can be resold to another user; can be in the past; 0 if the gift can't be resold; only for the receiver of the gift
    pub next_resale_date: i32,
    /// Point in time (Unix timestamp) when the upgraded gift can be transferred to the TON blockchain as an NFT; can be in the past; 0 if NFT export isn't possible; only for the receiver of the gift
    pub export_date: i32,
    /// If non-empty, then the user can pay for an upgrade of the gift using buyGiftUpgrade
    pub prepaid_upgrade_hash: String,
    /// Point in time (Unix timestamp) when the gift can be used to craft another gift can be in the past; only for the receiver of the gift
    pub craft_date: i32,
}
