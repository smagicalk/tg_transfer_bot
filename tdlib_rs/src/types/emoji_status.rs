#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes an emoji to be shown instead of the Telegram Premium badge
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmojiStatus {
    /// Type of the emoji status
    pub r#type: crate::enums::EmojiStatusType,
    /// Point in time (Unix timestamp) when the status will expire; 0 if never
    pub expiration_date: i32,
}
