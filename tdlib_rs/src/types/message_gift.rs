#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A regular gift was received or sent by the current user, or the current user was notified about a channel gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGift {
    /// The gift
    pub gift: crate::types::Gift,
    /// Sender of the gift; may be null for outgoing messages about prepaid upgrade of gifts from unknown users
    pub sender_id: Option<crate::enums::MessageSender>,
    /// Receiver of the gift
    pub receiver_id: crate::enums::MessageSender,
    /// Unique identifier of the received gift for the current user; only for the receiver of the gift
    pub received_gift_id: String,
    /// Message added to the gift
    pub text: crate::types::FormattedText,
    /// Unique number of the gift among gifts upgraded from the same gift after upgrade; 0 if yet unassigned
    pub unique_gift_number: i32,
    /// Number of Telegram Stars that can be claimed by the receiver instead of the regular gift; 0 if the gift can't be sold by the receiver
    pub sell_star_count: i64,
    /// Number of Telegram Stars that were paid by the sender for the ability to upgrade the gift
    pub prepaid_upgrade_star_count: i64,
    /// True, if the upgrade was bought after the gift was sent. In this case, prepaid upgrade cost must not be added to the gift cost
    pub is_upgrade_separate: bool,
    /// True, if the message is a notification about a gift won on an auction
    pub is_from_auction: bool,
    /// True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    pub is_private: bool,
    /// True, if the gift is displayed on the user's or the channel's profile page; only for the receiver of the gift
    pub is_saved: bool,
    /// True, if the message is about prepaid upgrade of the gift by another user
    pub is_prepaid_upgrade: bool,
    /// True, if the gift can be upgraded to a unique gift; only for the receiver of the gift
    pub can_be_upgraded: bool,
    /// True, if the gift was converted to Telegram Stars; only for the receiver of the gift
    pub was_converted: bool,
    /// True, if the gift was upgraded to a unique gift
    pub was_upgraded: bool,
    /// True, if the gift was refunded and isn't available anymore
    pub was_refunded: bool,
    /// Identifier of the corresponding upgraded gift; may be empty if unknown. Use getReceivedGift to get information about the gift
    pub upgraded_received_gift_id: String,
    /// If non-empty, then the user can pay for an upgrade of the gift using buyGiftUpgrade
    pub prepaid_upgrade_hash: String,
}
