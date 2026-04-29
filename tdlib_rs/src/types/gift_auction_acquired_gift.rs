#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a gift that was acquired by the current user on an auction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiftAuctionAcquiredGift {
    /// Receiver of the gift
    pub receiver_id: crate::enums::MessageSender,
    /// Point in time (Unix timestamp) when the gift was acquired
    pub date: i32,
    /// The number of Telegram Stars that were paid for the gift
    pub star_count: i64,
    /// Identifier of the auction round in which the gift was acquired
    pub auction_round_number: i32,
    /// Position of the user in the round among all auction participants
    pub auction_round_position: i32,
    /// Unique number of the gift among gifts upgraded from the same gift after upgrade; 0 if yet unassigned
    pub unique_gift_number: i32,
    /// Message added to the gift
    pub text: crate::types::FormattedText,
    /// True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    pub is_private: bool,
}
