#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes verification status provided by a bot
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotVerification {
    /// Identifier of the bot that provided the verification
    pub bot_user_id: i64,
    /// Identifier of the custom emoji that is used as the verification sign
    #[serde_as(as = "DisplayFromStr")]
    pub icon_custom_emoji_id: i64,
    /// Custom description of verification reason set by the bot. Can contain only Mention, Hashtag, Cashtag, PhoneNumber, BankCardNumber, Url, and EmailAddress entities
    pub custom_description: crate::types::FormattedText,
}
