#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A scope covering a member of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommandScopeChatMember {
    /// Chat identifier
    pub chat_id: i64,
    /// User identifier
    pub user_id: i64,
}
