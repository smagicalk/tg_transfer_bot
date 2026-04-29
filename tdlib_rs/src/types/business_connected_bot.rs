#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a bot connected to a business account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessConnectedBot {
    /// User identifier of the bot
    pub bot_user_id: i64,
    /// Private chats that will be accessible to the bot
    pub recipients: crate::types::BusinessRecipients,
    /// Rights of the bot
    pub rights: crate::types::BusinessBotRights,
}
