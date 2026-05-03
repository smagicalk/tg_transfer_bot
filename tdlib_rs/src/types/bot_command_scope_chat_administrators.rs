#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A scope covering all administrators of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommandScopeChatAdministrators {
    /// Chat identifier
    pub chat_id: i64,
}
