#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes parameters of verification that is provided by a bot
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotVerificationParameters {
    /// Identifier of the custom emoji that is used as the verification sign
    #[serde_as(as = "DisplayFromStr")]
    pub icon_custom_emoji_id: i64,
    /// Name of the organization that provides verification
    pub organization_name: String,
    /// Default custom description of verification reason to be used as placeholder in setMessageSenderBotVerification; may be null if none
    pub default_custom_description: Option<crate::types::FormattedText>,
    /// True, if the bot is allowed to provide custom description for verified entities
    pub can_set_custom_description: bool,
}
