#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message with a gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentGift {
    /// Number of Telegram Stars that sender paid for the gift
    pub star_count: i64,
    /// True, if the message is about prepaid upgrade of the gift by another user instead of actual receiving of a new gift
    pub is_prepaid_upgrade: bool,
}
