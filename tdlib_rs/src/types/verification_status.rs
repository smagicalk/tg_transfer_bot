#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about verification status of a chat or a user
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VerificationStatus {
    /// True, if the chat or the user is verified by Telegram
    pub is_verified: bool,
    /// True, if the chat or the user is marked as scam by Telegram
    pub is_scam: bool,
    /// True, if the chat or the user is marked as fake by Telegram
    pub is_fake: bool,
    /// Identifier of the custom emoji to be shown as verification sign provided by a bot for the user; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub bot_verification_icon_custom_emoji_id: i64,
}
