#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a business bot that manages the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessBotManageBar {
    /// User identifier of the bot
    pub bot_user_id: i64,
    /// URL to be opened to manage the bot
    pub manage_url: String,
    /// True, if the bot is paused. Use toggleBusinessConnectedBotChatIsPaused to change the value of the field
    pub is_bot_paused: bool,
    /// True, if the bot can reply
    pub can_bot_reply: bool,
}
