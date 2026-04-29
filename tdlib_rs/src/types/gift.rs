#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a gift that can be sent to another user or channel chat
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Gift {
    /// Unique identifier of the gift
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the chat that published the gift; 0 if none
    pub publisher_chat_id: i64,
    /// The sticker representing the gift
    pub sticker: crate::types::Sticker,
    /// Number of Telegram Stars that must be paid for the gift
    pub star_count: i64,
    /// Number of Telegram Stars that can be claimed by the receiver instead of the regular gift by default. If the gift was paid with just bought Telegram Stars, then full value can be claimed
    pub default_sell_star_count: i64,
    /// Number of Telegram Stars that must be paid to upgrade the gift; 0 if upgrade isn't possible
    pub upgrade_star_count: i64,
    /// Number of unique gift variants that are available for the upgraded gift; 0 if unknown
    pub upgrade_variant_count: i32,
    /// True, if the gift can be used to customize the user's name, and backgrounds of profile photo, reply header, and link preview
    pub has_colors: bool,
    /// True, if the gift is a birthday gift
    pub is_for_birthday: bool,
    /// True, if the gift can be bought only by Telegram Premium subscribers
    pub is_premium: bool,
    /// Information about the auction on which the gift can be purchased; may be null if the gift can be purchased directly
    pub auction_info: Option<crate::types::GiftAuction>,
    /// Point in time (Unix timestamp) when the gift can be sent next time by the current user; may be 0 or a date in the past.
    /// If the date is in the future, then call canSendGift to get the reason, why the gift can't be sent now
    pub next_send_date: i32,
    /// Number of times the gift can be purchased by the current user; may be null if not limited
    pub user_limits: Option<crate::types::GiftPurchaseLimits>,
    /// Number of times the gift can be purchased all users; may be null if not limited
    pub overall_limits: Option<crate::types::GiftPurchaseLimits>,
    /// Background of the gift
    pub background: crate::types::GiftBackground,
    /// Point in time (Unix timestamp) when the gift was send for the first time; for sold out gifts only
    pub first_send_date: i32,
    /// Point in time (Unix timestamp) when the gift was send for the last time; for sold out gifts only
    pub last_send_date: i32,
}
